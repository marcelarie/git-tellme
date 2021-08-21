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
    Ok(notifications)
}

async fn show_notifications_cli() ->Result<(), Box<dyn std::error::Error>> {
    let notifications  = make_notifications_request().await?;

    for n in notifications {
        let full_name = n.repository.full_name;
        let url = n.subject.url;
        let title = n.subject.title;

        let first_bar_size = full_name.chars().count() + url.chars().count();
        let second_bar_size = title.chars().count();
        let first_bar_is_bigger =  first_bar_size > second_bar_size;

        let total_width = if first_bar_is_bigger { first_bar_size } else { second_bar_size };

        let mut box_size = "━".to_owned();
        for _ in 0..total_width { box_size.push_str("━") };


        let mut first_bar_space_size = " ".to_owned();
        let mut second_bar_space_size = " ".to_owned();
        if first_bar_is_bigger {
            for _ in 0..first_bar_size - second_bar_size { second_bar_space_size.push_str(" ") };
        } else {
            for _ in 0..first_bar_size - second_bar_size { first_bar_space_size.push_str(" ") };
        }

        println!("┏━{}━┓", box_size);
        println!("┃ {} {}{}┃", full_name, url, first_bar_space_size );
        println!("┃ {} {}┃",  title, second_bar_space_size );
        println!("┗━{}━┛", box_size);
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    show_notifications_cli().await?;
    Ok(())
}
