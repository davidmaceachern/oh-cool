use anyhow::{anyhow, bail};
use hubcaps::{Credentials, Github};
use reqwest::{header, Client};
use serde_json::json;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
pub struct Star {
    /// Run without actually updating account
    #[structopt(short, long)]
    dryrun: bool,
}
static USER_AGENT: &str = "oh-cool (https://github.com/davidmaceachern/oh-cool)";

static GRAPHQL_QUERY_STARRED: &str = "
query {
  viewer {
    login
    name
    starredRepositories {
      edges {
        cursor
        node {
          id
          name
          primaryLanguage {
            id
            name
            color
          }
        }
      }
    }
  }
}
";

pub async fn graphql(api_key: String) -> anyhow::Result<()> {
    let endpoint = "https://api.github.com/graphql";
    let client = Client::new();
    let resp = client
        .post(endpoint)
        .header(header::AUTHORIZATION, format!("token {}", api_key))
        .header(header::USER_AGENT, USER_AGENT)
        .json(&json!({
            "query": GRAPHQL_QUERY_STARRED,
        }))
        .send()
        .await?;
    let body = resp.text().await?;
    println!("{}", body);
    Ok(())
}

pub fn diff() {
    unimplemented!()
}

pub async fn run(api_key: String, args: Star) -> anyhow::Result<()> {
    let github = Github::new(USER_AGENT, Credentials::Token(api_key.clone()))?;
    let Star { dryrun } = args;
    graphql(api_key.clone()).await?;
    // Does not consume rate limit
    let status = github.rate_limit().get().await?;
    println!("{:#?}", status);
    if dryrun {
        let repositories = github
            .user_repos("davidmaceachern")
            .list(&Default::default())
            .await?;
        // println!("You would have starred the following repositories");
        println!("{:?}", repositories);
    }
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_compares_lists() {
//         let result = diff();
//         assert_eq!(result, expected);
//     }
// }
