use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
    Client, Response,
};
use std::{env, process::Command};

fn get_token() -> String {
    String::from_utf8(
        Command::new("pass")
            .arg("git-tellme/token")
            .output()
            .expect("Token not found")
            .stdout,
    )
    .expect("No AUTH_TOKEN found on the .env file")
}

pub fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let token = env::var("AUTH_TOKEN").unwrap_or_else(|_| get_token());
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    assert!(headers.contains_key(AUTHORIZATION));
    assert!(headers.contains_key(USER_AGENT));
    headers
}

pub async fn make_json_request(url: &str) -> Result<Response> {
    let res = Client::new()
        .get(url)
        .headers(construct_headers())
        .send()
        .await?;
    Ok(res)
}
