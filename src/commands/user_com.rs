use crate::api::make_json_request;
use crate::models::user::User;
use anyhow::Result;

pub async fn show_user() -> Result<User> {
    let url = format!("https://api.github.com/user");
    let user = make_json_request(&url).await?.json::<User>().await?;

    Ok(user)
}
