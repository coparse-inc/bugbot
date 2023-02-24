use clap::Parser;

use crate::{github::{create_gh_issue, IssueArgs, Repo}, people::{Names, Engineer}};


#[derive(clap::Parser, Debug)]
#[command(name = "bug", arg_required_else_help = true)]
pub struct Command {
    /// Engineer who should be assigned the task
    #[arg(value_enum, short, long)]
    assignee: Option<Names>,

    /// Whether the bug is blocking for release
    #[arg(short, long, default_value_t = false)]
    blocking: bool,

    /// Open the issue against the bugbot repo for debugging
    #[arg(value_enum, short, long, default_value_t = Repo::Bugbot)]
    repo: Repo,

    /// Description of the bug
    description: Vec<String>,
}

impl Command {
    pub fn from_str(s: String) -> Result<Command, clap::Error> {
        Self::try_parse_from(format!("bug {}", s).split(' '))
    }

    fn get_assigned_engineer(&self) -> Option<Engineer> {
        let assignee = self.assignee.as_ref()?;
        Some(assignee.to_engineer())
    }

    fn get_repo_id(&self) -> String {
        self.repo.to_id().to_string()
    }

    pub async fn into_response_text(self) -> String {
        // // self.assignee.
        // let a =Names::from_str(self.assignee);
        let assignees: Vec<String> = match self.get_assigned_engineer() {
            Some(x) => vec![x.github().to_owned()],
            None => vec![],
        };

        let desc_string = self.description.join(" ");

        let res = create_gh_issue(IssueArgs::new(
            self.get_repo_id(),
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
