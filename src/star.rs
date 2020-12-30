use anyhow::{anyhow, bail};
use structopt::StructOpt;

use hubcaps::{Credentials, Github};
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
";

pub async fn run(
    api_key: String,
    args: Star,
) -> anyhow::Result<()> {
    let github = Github::new(
        USER_AGENT,
    Credentials::Token(api_key),
  )?;
    let Star { dryrun } = args;
    // Does not consume rate limit
    let status = github.rate_limit().get().await?;
    println!("{:#?}", status);
    if !dryrun {
            let repositories = github.user_repos("davidmaceachern").list(&Default::default()).await?;
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
//         let result = run();
//         assert_eq!(result, expected);
//     }
// }