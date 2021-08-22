use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
    Client, Response,
};
use std::env;

mod models;
use models::issue::Issue;
use models::notifications::Notifications;
use models::pull::PullRequest;

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let token = env::var("AUTH_TOKEN").unwrap();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    assert!(headers.contains_key(AUTHORIZATION));
    assert!(headers.contains_key(USER_AGENT));
    headers
}

async fn make_json_request(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let res = Client::new()
        .get(url)
        .headers(construct_headers())
        .send()
        .await?;
    Ok(res)
}

fn draw_box(content: &[String]) {
    // get max size
    let mut max_width = 0;
    for item in content.iter() {
        if item.chars().count() > max_width {
            max_width = item.chars().count()
        };
    }
    // generate box_width
    let mut box_width = "━".to_owned();
    for _ in 0 .. max_width {
        box_width.push_str("━")
    }

    // print full box
    println!("┏━{}━┓", box_width);
    for item in content.iter() {
        let count = item.chars().count();
        let mut space_size = " ".to_owned();
        if count < max_width {
            for _ in 0 .. max_width - count {
                space_size.push_str(" ")
            }
        };
        println!("┃ {}{} ┃", item, space_size);
    }
    println!("┗━{}━┛", box_width);
}

async fn show_notifications_cli() -> Result<(), Box<dyn std::error::Error>> {
    let notifications = make_json_request("https://api.github.com/notifications")
        .await?
        .json::<Notifications>()
        .await?;

    for n in notifications {
        let full_name = n.repository.full_name;
        let url = n.subject.url;
        let title = n.subject.title;

        let html_url = if n.subject.subject_type == "Issue" {
            make_json_request(&url)
                .await?
                .json::<Issue>()
                .await?
                .html_url
        } else {
            make_json_request(&url)
                .await?
                .json::<PullRequest>()
                .await?
                .html_url
        };

        let content = [[full_name, html_url].join(" "), title];

        draw_box(&content)
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    show_notifications_cli().await?;
    Ok(())
}
