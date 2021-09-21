use crate::api::make_json_request;
use crate::drawing::draw_box;
use crate::models::repos::Repos;

pub async fn show_repos_user(user: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/users/{}/repos", user);

    let repos = make_json_request(&url).await?.json::<Repos>().await?;

    for r in repos {
        let name = r.name.unwrap();
        let id = r.id.unwrap();

        let content = [id.to_string(), name];

        draw_box(&content, id.to_string())
    }

    Ok(())
}
