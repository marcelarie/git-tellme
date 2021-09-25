use crate::api::make_json_request;
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
        let url = n.subject.url;
        let title = n.subject.title;
        let subject_type = n.subject.subject_type;

        let url = url.replace("https://api.github.com/repos/", "https://github.com/");
        let url = url.replace("pulls", "pull");

        let content = [subject_type.clone(), title, [full_name, url].join(" ")];

        draw_box(&content, subject_type)
    }

    Ok(())
}
