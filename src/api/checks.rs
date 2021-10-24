use crate::options::Opt;

use super::commands::{notification_com::*, repo_com::*, user_com::*};
use super::db::{get_token, save_token};
use anyhow::Result;
use colored::Colorize;
use std::time::Duration;
use structopt::StructOpt;
use tokio::{task, time};

fn no_token_message() {
    let git_tellme = String::from("git-tellme").truecolor(255, 215, 0);
    println!("<{}>", String::from("WARNING").truecolor(255, 69, 0));
    println!("    No GitHub token was found. ");
    println!("    To generate one go to: https://github.com/settings/tokens");
    println!("    To save it use the --token or -t parameter:\n");
    println!("{}{}", git_tellme, " --token <YOUR_GITHUB_TOKEN>\n");
    println!("For more information try --help");
}

pub async fn check_token() -> Result<()> {
    let opt = Opt::from_args();
    match get_token() {
        Ok(_) => check_options().await?,
        _error => match opt.github_token {
            Some(t) => save_token(&t)?,
            None => no_token_message(),
        },
    }
    Ok(())
}

pub async fn check_options() -> Result<()> {
    let opt = Opt::from_args();
    if opt.repos {
        match opt.user {
            Some(u) => show_repos_user(u).await?,
            None => {
                let user = show_user().await?;
                show_repos_user(String::from(user.login)).await?
            }
        }
    } else {
        if opt.system {
            let forever = task::spawn(async {
                let mut interval = time::interval(Duration::from_millis(1000));

                loop {
                    interval.tick().await;
                    show_notifications_sys().await.unwrap();
                }
            });
            forever.await?;
        } else {
            show_notifications_cli().await?;
        }
    }
    Ok(())
}
