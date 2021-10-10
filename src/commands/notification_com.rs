use std::process::Command;

use crate::api::{make_json_request, url_shortener};
use crate::drawing::draw_box;
use crate::models::notifications::Notifications;
use anyhow::Result;
use notify_rust::Notification;

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &str = "message-new-instant";

#[cfg(target_os = "windows")]
static SOUND: &'static str = "Mail";

pub async fn show_notifications_cli() -> Result<()> {
    let notifications = make_json_request("https://api.github.com/notifications")
        .await?
        .json::<Notifications>()
        .await?;

    for n in notifications {
        let title = n.subject.title;
        let url = match n.subject.url {
            Some(url) => url,
            None => format!(
                "{}/discussions?discussions_q={}",
                n.repository.html_url,
                title.replace(" ", "+")
            ),
        };
        let subject_type = n.subject.subject_type;

        let url = url.replace("https://api.github.com/repos/", "https://github.com/");
        let url = url.replace("pulls", "pull");
        let url = if url.chars().count() >= 60 {
            url_shortener(&url).await?
        } else {
            url
        };

        let content = [title, url];

        draw_box(&content, subject_type)
    }

    Ok(())
}

fn xdg_open(url: String) {
    Command::new("xdg-open")
        .args([&url])
        .output()
        .expect("failed to open with browser");
}

pub async fn show_notifications_sys() -> Result<()> {
    let notifications = make_json_request("https://api.github.com/notifications")
        .await?
        .json::<Notifications>()
        .await?;

    for n in notifications {
        let title = n.subject.title;
        let full_name = n.repository.full_name;
        let url = match n.subject.url {
            Some(url) => url,
            None => format!(
                "{}/discussions?discussions_q={}",
                n.repository.html_url,
                title.replace(" ", "+")
            ),
        };
        let subject_type = n.subject.subject_type;

        let url = url.replace("https://api.github.com/repos/", "https://github.com/");
        let url = url.replace("pulls", "pull");
        let url = if url.chars().count() >= 60 {
            url_shortener(&url).await?
        } else {
            url
        };

        // xdg-open
        Notification::new()
            .summary(&title)
            .body(&[subject_type, full_name].join(" from "))
            .sound_name(SOUND)
            .icon("github")
            // .action("clicked", "Open")
            .timeout(0) // this however is
            .show()
            .unwrap()
            .wait_for_action(|action| match action {
                "__closed" => xdg_open(url),
                // "clicked" => xdg_open(url),
                _ => (),
            })
    }
    Ok(())
}
