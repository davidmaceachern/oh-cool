mod star;

use star::Star;
use std::env;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Star::from_args();
    star::run( 
        env::var("GITHUB_API_KEY")
            .map_err(|_| anyhow::anyhow!(
                "Please export a GITHUB_API_KEY env variable.\n  ▶ You can generate one by visiting https://github.com/settings/tokens/new"
            ))?,
        args,
    )
    .await?;
    Ok(())
}