use std::{
    env,
    path::{Path, PathBuf},
};

#[cfg(all(not(feature = "static"), feature = "bindings"))]
const VERSION: &str = "2.10.4";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    #[cfg(feature = "static")]
    let include_dirs = {
        println!("cargo:rustc-link-lib=z");
        vec![xml2_src::build_lib().include]
    };

    #[cfg(not(feature = "static"))]
    let include_dirs: Vec<PathBuf> = {
        println!("cargo:rustc-link-lib=xml2");
        #[cfg(feature = "bindings")]
        {
            pkg_config::Config::new()
                .atleast_version(VERSION)
                .probe("libxml-2.0")
                .expect("libxml-2.0 pkg-config configuration")
                .include_paths
        }
        #[cfg(not(feature = "bindings"))]
        {
            if let Some(lib_dir) = std::env::var_os("LIBXML2_PATH") {
                let lib_dir = Path::new(&lib_dir);
                let dylib_name = format!(
                    "{}xml2{}",
                    std::env::consts::DLL_PREFIX,
                    std::env::consts::DLL_SUFFIX
                );
                if lib_dir.join(dylib_name).exists()
                    || lib_dir.join("libxml2.a").exists()
                    || lib_dir.join("xml2.lib").exists()
                {
                    println!("cargo:rustc-link-search=native={}", lib_dir.display());
                }
            }
            vec![]
        }
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let output_bindings = out_path.join("bindings.rs");

    let crate_path = Path::new(env!("CARGO_MANIFEST_DIR"));

    if cfg!(feature = "bindings") {
        let mut builder = bindgen::builder()
            .header(crate_path.join("wrapper.h").to_string_lossy())
            .derive_default(false)
            .allowlist_file(".*/libxml/.*.h")
            .generate_comments(false)
            .enable_function_attribute_detection()
            .disable_header_comment()
            .parse_callbacks(Box::new(bindgen::CargoCallbacks));
        for include_dir in include_dirs {
            builder = builder.clang_arg(format!("-I{}", include_dir.to_string_lossy()));
        }
        let bindings = builder.generate().expect("Unable to generate bindings");
        bindings
            .write_to_file(output_bindings)
            .expect("Couldn't write bindings!");
        #[cfg(feature = "__update")]
        bindings
            .write_to_file(crate_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    } else {
        let bundled_bindings = crate_path.join("bindings.rs");
        std::fs::copy(bundled_bindings, output_bindings)
            .expect("copy bundled bindings.rs to output dir");
    }
}
