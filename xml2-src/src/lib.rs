use std::{
    env,
    path::{Path, PathBuf},
};

mod c_files;
mod configure;
mod makefile;

pub const LIBXML2: &str = "libxml2";

pub struct Source {
    pub root: PathBuf,
    pub include: PathBuf,
}

pub fn build_lib() -> Source {
    let output = output_dir();
    let include_gen = output.join("include");
    let include_libxml2_gen = include_gen.join(LIBXML2);

    std::fs::create_dir_all(&include_libxml2_gen).expect("generated include dir created");

    std::fs::write(include_gen.join("config.h"), "// generated config.h")
        .expect("config.h being written");

    std::fs::write(
        include_libxml2_gen.join("xmlversion.h"),
        "// generated xmlversion.h",
    )
    .expect("xmlversion.h being written");

    let vendor = source_dir();
    let libxml2 = vendor.join(LIBXML2);
    let include = libxml2.join("include");

    println!("cargo:include={}", include.to_str().unwrap());

    cc::Build::new()
        .define("_REENTRANT", None)
        .flag_if_supported("-pedantic")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .include(&include)
        .include(&include_gen)
        .files(c_files::C_FILES)
        .compile("xml2");

    Source {
        root: libxml2,
        include,
    }
}

fn source_dir() -> PathBuf {
    dunce::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor")).expect("source dir")
}

fn output_dir() -> PathBuf {
    dunce::canonicalize(Path::new(
        &std::env::var("OUT_DIR").expect("OUT_DIR env variable, must be run inside build script"),
    ))
    .expect("output dir")
}
