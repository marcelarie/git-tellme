use anyhow::Result;
use api::checks::check_token;
use dotenv::dotenv;

mod api;
mod commands;
mod db;
mod drawing;
mod models;
mod options;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    check_token().await?;
    Ok(())
}
