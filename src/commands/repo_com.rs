use crate::api::make_json_request;
use crate::drawing::draw_box;
use crate::models::repos::Repos;
use anyhow::Result;

pub async fn show_repos_user(user: String) -> Result<()> {
    let url = format!("https://api.github.com/users/{}/repos", user);

    let res = make_json_request(&url).await?;
    // let content_length = res.content_length().unwrap();
    let repos = res.json::<Repos>().await?;

    for r in repos {
        let name = r.name.unwrap_or(String::from("<MISSING NAME>"));
        let id = r.id.unwrap();
        let url = r.html_url.unwrap();

        let content = [name, url];

        draw_box(&content, id.to_string())
    }

    Ok(())
}
