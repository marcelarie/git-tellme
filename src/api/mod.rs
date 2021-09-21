use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
    Client, Response,
};
use std::env;

pub fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let token = env::var("AUTH_TOKEN").expect("No AUTH_TOKEN found on the .env file");
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    assert!(headers.contains_key(AUTHORIZATION));
    assert!(headers.contains_key(USER_AGENT));
    headers
}

pub async fn make_json_request(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let res = Client::new()
        .get(url)
        .headers(construct_headers())
        .send()
        .await?;
    Ok(res)
}
