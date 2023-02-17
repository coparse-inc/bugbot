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
    data: T,
}

#[derive(Deserialize, Debug)]
struct CreateIssueMutation {
    pub createIssue: CreateIssuePayload
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

pub async fn create_gh_issue(args: IssueArgs) -> Result<Issue> {
    let gh_token = config_env_var("GITHUB_TOKEN")?;
    let body = GQLReq {
        query: r##"
        mutation issue($title: String! $body: String! $assignees: [ID!] $labels: [ID!]) {
          createIssue(input: {
            repositoryId: "R_kgDOI-bUuw",
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
    let req = reqwest::Client::new()
        .post("https://api.github.com/graphql")
        .bearer_auth(gh_token)
        .header(reqwest::header::USER_AGENT, "curl")
        .json(&body);

    println!("{req:#?}");

    let res = req.send().await;
    let text = res?.text().await;
    println!("{:#?}", &text);
    let js: GQLRes<CreateIssueMutation> = serde_json::from_str(text?.as_str())?;
    println!("output: {js:#?}");
    Ok(js.data.createIssue.issue)
}
