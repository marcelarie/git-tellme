#![feature(format_args_capture)]
use anyhow::Result;
use dotenv::dotenv;
use structopt::StructOpt;

mod api;
mod commands;
mod drawing;
mod models;
mod options;

use commands::{notification_com::*, repo_com::*, user_com::*};
use options::Opt;
use std::time::Duration;
use tokio::{task, time};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let opt = Opt::from_args();
    let user = show_user().await?;

    if opt.repos {
        match opt.user {
            Some(u) => show_repos_user(u).await?,
            None => show_repos_user(String::from(user.login)).await?,
        }
    } else {
        if opt.system {
            let forever = task::spawn(async {
                let mut interval = time::interval(Duration::from_millis(1000));

                loop {
                    interval.tick().await;
                    show_notifications_sys().await;
                }
            });
            forever.await?;
        } else {
            show_notifications_cli().await;
        }
    }
    Ok(())
}
