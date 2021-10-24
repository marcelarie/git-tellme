pub mod checks;

use super::commands;
use super::db::{self, get_token};
use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
    Client, Response,
};
use std::env;

pub fn construct_headers() -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    let token = get_token()?;
    let token = String::from("token ")
        + &env::var("AUTH_TOKEN").unwrap_or_else(|_| token);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(token.trim()).unwrap());
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    assert!(headers.contains_key(AUTHORIZATION));
    assert!(headers.contains_key(USER_AGENT));
    Ok(headers)
}

pub async fn make_json_request(url: &str) -> Result<Response> {
    let res = Client::new()
        .get(url)
        .headers(construct_headers()?)
        .send()
        .await?;
    Ok(res)
}

pub async fn url_shortener(url: &str) -> Result<String> {
    let res = Client::new()
        .post("https://git.io")
        .form(&[("url", url)])
        .send()
        .await?;
    Ok(match res.headers().get("location") {
        Some(short) => short.to_str().unwrap_or_default().to_owned(),
        None => panic!("Can't shorten the given url {}", &url),
    })
}
