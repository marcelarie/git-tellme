use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
    Client,
};
use std::env;

mod models;
use models::notifications::Notifications;

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let token = env::var("AUTH_TOKEN").unwrap();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    assert!(headers.contains_key(AUTHORIZATION));
    assert!(headers.contains_key(USER_AGENT));
    headers
}

async fn make_notifications_request() -> Result<Notifications, Box<dyn std::error::Error>> {
    let res = Client::new()
        .get("https://api.github.com/notifications")
        .headers(construct_headers())
        .send()
        .await?;

    let notifications = res.json::<Notifications>().await?;
    println!("{}", notifications[0].id);
    Ok(notifications)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    make_notifications_request().await?;
    Ok(())
}
