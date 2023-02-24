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

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Repo Id: {}", self.to_id())
    }
}
