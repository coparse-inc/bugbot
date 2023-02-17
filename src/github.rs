use super::config_env_var;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct IssueArgs {
    title: String,
    body: String,
    assignees: Vec<String>,
    labels: Vec<String>,
}

impl IssueArgs {
    pub fn new(title: String, body: String, assignees: Vec<String>, labels: Vec<String>) -> Self {
        Self {
            title,
            body,
            assignees,
            labels,
        }
    }
}

#[derive(Serialize)]
struct GQLReq<T> {
    query: String,
    variables: T,
}

#[derive(Deserialize, Debug)]
struct GQLRes<T> {
    #[serde(flatten)]
    data: T,
}

#[derive(Deserialize, Debug)]
pub struct CreateIssuePayload {
    pub issue: Issue,
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub url: String,
}

pub async fn create_gh_issue(args: IssueArgs) -> Result<CreateIssuePayload> {
    let gh_token = config_env_var("GITHUB_TOKEN")?;
    let body = GQLReq {
        query: r##"
        mutation issue($title: String! $body: String! $assignees: [ID!] $labels: [ID!]) {
          createIssue(input: {
            repositoryId: "someID",
            title: $title,
            body: $body,
            assigneeIds: $assignees,
            labelIds: $labels
          }) {
            issue {
              id
              url
              title
            }
          }
        }
        "##
        .into(),
        variables: args,
    };
    let resp = reqwest::Client::new()
        .post("https://api.github.com/graphql")
        .json(&body)
        .bearer_auth(gh_token)
        .send()
        .await?;
    let js = resp.json::<GQLRes<CreateIssuePayload>>().await?;
    Ok(js.data)
}
