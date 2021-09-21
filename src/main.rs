#![feature(format_args_capture)]
use dotenv::dotenv;
use structopt::StructOpt;

mod api;
mod commands;
mod drawing;
mod models;
mod options;

use commands::{notification_com::*, repo_com::*, user_com::*};
use options::Opt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let opt = Opt::from_args();
    let user = show_user().await?;

    if opt.repos {
        match opt.user {
            Some(u) => show_repos_user(u).await?,
            None => show_repos_user(String::from(user.login)).await?,
        }
    } else {
        show_notifications_cli().await?;
    }
    Ok(())
}
