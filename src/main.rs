use colored::*;
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
    Client, Response,
};
use std::env;
use structopt::StructOpt;

mod models;
mod options;
use models::notifications::Notifications;
use options::Opt;

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

fn draw_box(content: &[String], subject_type: String) {
    // get max size
    let mut max_width = 0;
    for item in content.iter() {
        if item.chars().count() > max_width {
            max_width = item.chars().count()
        };
    }
    let box_lines = ["┏", "━", "┓", "┃", "┗", "┛"];
    // generate box_width
    let mut box_width = box_lines[1].to_owned();
    for _ in 0 .. max_width {
        box_width.push_str(box_lines[1])
    }

    let upper_box = [
        box_lines[0],
        box_lines[1],
        &box_width,
        box_lines[1],
        box_lines[2],
    ]
    .join("");
    let lower_box = [
        box_lines[4],
        box_lines[1],
        &box_width,
        box_lines[1],
        box_lines[5],
    ]
    .join("");

    // print full box
    // color by type
    if subject_type == "Issue" {
        println!("{}", upper_box.truecolor(211, 160, 77))
    } else {
        println!("{}", upper_box.truecolor(123, 146, 70))
    };
    for item in content.iter() {
        let count = item.chars().count();
        let mut space_size = " ".to_owned();
        if count < max_width {
            for _ in 0 .. max_width - count {
                space_size.push_str(" ")
            }
        };

        if subject_type == "Issue" {
            println!(
                "{} {}{} {}",
                box_lines[3].truecolor(211, 160, 77),
                item,
                space_size,
                box_lines[3].truecolor(211, 160, 77)
            )
        } else {
            println!(
                "{} {}{} {}",
                box_lines[3].truecolor(123, 146, 70),
                item,
                space_size,
                box_lines[3].truecolor(123, 146, 70)
            )
        };
    }
    if subject_type == "Issue" {
        println!("{}", lower_box.truecolor(211, 160, 77));
    } else {
        println!("{}", lower_box.truecolor(123, 146, 70));
    };
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
        let subject_type = n.subject.subject_type;

        let url = url.replace("https://api.github.com/repos/", "https://github.com/");
        let url = url.replace("pulls", "pull");

        let content = [title, [full_name, url].join(" "), subject_type.clone()];

        draw_box(&content, subject_type)
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    dotenv().ok();
    show_notifications_cli().await?;
    Ok(())
}
