use crate::api::{make_json_request, url_shortener};
use crate::drawing::draw_box;
use crate::models::notifications::Notifications;
use anyhow::Result;

pub async fn show_notifications_cli() -> Result<()> {
    let notifications = make_json_request("https://api.github.com/notifications")
        .await?
        .json::<Notifications>()
        .await?;

    for n in notifications {
        let full_name = n.repository.full_name;
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

        // let content = [subject_type.clone(), title, [full_name, url].join(" ")];
        let content = [title, [full_name, url].join(" ")];

        draw_box(&content, subject_type)
    }

    Ok(())
}
