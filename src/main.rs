use reqwest::{Client, header::{AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT}};
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let token = env::var("AUTH_TOKEN").unwrap();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    assert!(headers.contains_key(AUTHORIZATION));
    assert!(headers.contains_key(USER_AGENT));
    println!("{:#?}", headers);
    headers
}

async fn make_request() -> Result<(), Box<dyn std::error::Error>> {

    let res = Client::new()
        .get("https://api.github.com/notifications")
        .headers(construct_headers())
        .send().await?;

    let noti = res.json::<serde_json::Value>().await?;

    print!("git tell me now about {:#?} \n", &noti);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    make_request().await?;
    Ok(())
}
