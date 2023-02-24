use std::fmt::Display;

use clap::ValueEnum;


#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Repo {
    Bugbot,
    AppMonorepo,
}

impl Repo {
    pub fn to_id(self) -> &'static str {
        match self {
            Self::Bugbot =>  "R_kgDOI-bUuw",
            Self::AppMonorepo => ""
        }
    }
}

