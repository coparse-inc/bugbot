
pub struct Engineer {
    github_id: &'static str,
    notion_id: &'static str,
}

impl Engineer {
    pub fn github(&self) -> &'static str {
        self.github_id
    }

    pub fn notion(&self) -> &'static str {
        self.notion_id
    }
}

#[derive(Debug, clap::ValueEnum, Clone, Copy)]
pub enum Names {
    Sean,
    Will,
    Ness,
    Chase,
    Sam,
}

impl Names {
    pub fn to_engineer(&self) -> Engineer {
        match self {
            Self::Sean => Engineer {
                github_id: "MDQ6VXNlcjI0NDk2ODIy",
                notion_id: "",
            },
            Self::Chase => Engineer {
                github_id: "",
                notion_id: "",
            },
            Self::Will => Engineer {
                github_id: "",
                notion_id: "",
            },
            Self::Ness => Engineer {
                github_id: "",
                notion_id: "",
            },
            Self::Sam => Engineer {
                github_id: "",
                notion_id: "",
            },
        }
    }
}
