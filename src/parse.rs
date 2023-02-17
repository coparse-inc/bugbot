use std::str::FromStr;

use clap::Parser;

use crate::github::{create_gh_issue, IssueArgs};

struct Engineer {
    github_id: &'static str,
    notion_id: &'static str,
}

#[derive(Debug, PartialEq)]
enum Names {
    Sean,
    Will,
    Ness,
    Chase,
    Sam,
}

impl FromStr for Names {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_uppercase().as_str() {
            "SEAN" => Ok(Self::Sean),
            "WILL" => Ok(Self::Will),
            "NESS" => Ok(Self::Ness),
            "CHASE" => Ok(Self::Chase),
            "SAM" => Ok(Self::Sam),
            _ => Err(()),
        }
    }
}

impl Names {
    fn to_engineer(self) -> Engineer {
        match self {
            Self::Sean => Engineer {
                github_id: "",
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

#[derive(clap::Parser, Debug)]
#[command(name = "bug", arg_required_else_help = true)]
pub struct Command {
    /// Engineer who should be assigned the task
    #[arg(short, long)]
    assignee: Option<String>,

    /// Whether the bug is blocking for release
    #[arg(short, long, default_value_t = true)]
    blocking: bool,

    description: String,
}

impl Command {
    pub fn from_str(s: String) -> Result<Command, clap::Error> {
        Self::try_parse_from(s.split(' '))
    }

    fn get_assigned_engineer(&self) -> Option<Engineer> {
        let assignee = self.assignee.as_ref()?;
        Names::from_str(assignee.as_str()).ok().map(|x| x.to_engineer())
    }

    pub async fn into_response_text(self) -> String {
        // // self.assignee.
        // let a =Names::from_str(self.assignee);
        let assignees: Vec<String> = match self.get_assigned_engineer() {
            Some(x) => vec![x.github_id.to_owned()],
            None => vec![]
        };
        


        let res = create_gh_issue(IssueArgs::new(
            format!("[BugBot] {}", self.description),
            self.description,
            assignees,
            vec![]
        )).await;



        match res {
            Ok(x) => format!("Created <{}|Github Issue>, {}", x.issue.url, x.issue.title),
            Err(e) => format!("An error occurred {}", e)
        }
    }
}
