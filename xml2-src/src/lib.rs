use std::{
    env,
    path::{Path, PathBuf},
};

mod c_files;
mod makefile;
mod configure;

pub const LIBXML2: &str = "libxml2-2.10.3";

pub struct Source {
    pub root: PathBuf,
    pub include: PathBuf,
}

pub fn build_lib() -> Source {
    let vendor = source_dir();
    let libxml2 = vendor.join(LIBXML2);
    let include = libxml2.join("include");

    println!("cargo:include={}", include.to_str().unwrap());

    cc::Build::new()
        .define("HAVE_CONFIG_H", None)
        .define("_REENTRANT", None)
        .flag_if_supported("-pedantic")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .include(&include)
        .files(c_files::C_FILES)
        .compile("xml2");

    Source {
        root: libxml2,
        include,
    }
}

fn source_dir() -> PathBuf {
    dunce::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("vendor")).unwrap()
}
