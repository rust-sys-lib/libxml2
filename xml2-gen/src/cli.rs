use std::{fmt::Display, path::PathBuf, str::FromStr};

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Run generation with current configuration without customization.
    #[arg(long)]
    pub no_ui: bool,
    /// Output directory.
    #[arg(short, long, default_value_t = out_dir())]
    pub out: PathBufString,
}

fn out_dir() -> PathBufString {
    PathBufString(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("parent folder")
            .join("xml2")
            .join("src")
            .join("gen"),
    )
}

#[derive(Clone)]
pub struct PathBufString(PathBuf);

impl PathBufString {
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl Display for PathBufString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

impl FromStr for PathBufString {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PathBuf::from_str(s).map(Self)
    }
}
