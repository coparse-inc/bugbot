use crate::config_env_var;
use anyhow::Result;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Serialize, Debug)]
pub struct IssueArgs {
    repo: String,
    title: String,
    body: String,
    assignees: Vec<String>,
    labels: Vec<String>,
}


impl IssueArgs {
    pub fn new(repo: String, title: String, body: String, assignees: Vec<String>, labels: Vec<String>) -> Self {
        Self {
            repo,
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
    pub assignees: UserConnection
}

#[derive(Deserialize, Debug)]
pub struct UserConnection {
    pub nodes: Vec<User>
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: String
}

async fn gql_fetch<T, U>(args: &GQLReq<T>) -> Result<GQLRes<U>>
    where T: Serialize,
        U: DeserializeOwned
{
    let gh_token = config_env_var("GITHUB_TOKEN")?;
    let req = reqwest::Client::new()
        .post("https://api.github.com/graphql")
        .bearer_auth(gh_token)
        .header(reqwest::header::USER_AGENT, "curl")
        .json(args);


    let res = req.send().await?;
    res.json::<GQLRes<U>>().await.map_err(anyhow::Error::from)
}

pub async fn create_gh_issue(args: IssueArgs) -> Result<Issue> {
    let body = GQLReq {
        query: r##"
        mutation issue($title: String! $body: String! $assignees: [ID!] $labels: [ID!] $repo: ID!) {
          createIssue(input: {
            repositoryId: $repo,
            title: $title,
            body: $body,
            assigneeIds: $assignees,
            labelIds: $labels
          }) {
            issue {
              id
              url
              title
              assignees(first: 10) {
                nodes {
                    name
                }
              }
            }
          }
        }
        "##
        .into(),
        variables: args,
    };

    let js = gql_fetch::<_, CreateIssueMutation>(&body).await;

    Ok(js?.data.createIssue.issue)
}
