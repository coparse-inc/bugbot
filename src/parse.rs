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
    fn to_engineer(&self) -> Engineer {
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

#[derive(clap::Parser, Debug)]
#[command(name = "bug", arg_required_else_help = true)]
pub struct Command {
    /// Engineer who should be assigned the task
    #[arg(short, long)]
    assignee: Option<String>,

    /// Whether the bug is blocking for release
    #[arg(short, long, default_value_t = false)]
    blocking: bool,

    /// Description of the bug
    description: Vec<String>,
}

impl Command {
    pub fn from_str(s: String) -> Result<Command, clap::Error> {
        Self::try_parse_from(format!("bug {}", s).split(' '))
    }

    fn get_assigned_engineer(&self) -> Option<Engineer> {
        let assignee = self.assignee.as_ref()?;
        Names::from_str(assignee.as_str())
            .ok()
            .map(|x| x.to_engineer())
    }

    pub async fn into_response_text(self) -> String {
        // // self.assignee.
        // let a =Names::from_str(self.assignee);
        let assignees: Vec<String> = match self.get_assigned_engineer() {
            Some(x) => vec![x.github_id.to_owned()],
            None => vec![],
        };

        let desc_string = self.description.join(" ");

        let res = create_gh_issue(IssueArgs::new(
            format!("[BugBot] {}", desc_string),
            format!("{}\n\nThis issue was created automatically by [BugBot](https://github.com/seanaye/bugbot)\nType `/bug --help` in the bug reports slack channel", desc_string),
            assignees,
            vec![],
        ))
        .await;

        match res {
            Ok(x) => {
                let assignee_names: Vec<String> =
                    x.assignees.nodes.into_iter().map(|n| n.name).collect();
                let suffix = assignee_names
                    .get(0)
                    .map_or("with no assignee".to_string(), |n| {
                        format!("and assigned {}", n)
                    });
                format!("Created a new <{}|Github Issue> {}", x.url, suffix)
            }
            Err(e) => format!("An error occurred {}", e),
        }
    }
}
