use crate::api::make_json_request;
use crate::db::{hdel, hget, hkeys, hset};
use crate::drawing::draw_box;
use crate::models::repos::Repos;
use anyhow::Result;

// TODO: Refactor this
// this command will:
// 1. Check for the username send as parameter and check for new data
// The data:
//    - New repositories 📦
//    - New stars ⭐
//    - ...
//
//    If no user is given git-tellme will recommend some from the list of user follows
//    curl -H "Authorization: token xxx" https://api.github.com/user/following
//
pub async fn subscribe_to_user_cli(user: String) -> Result<()> {
    match hget("SUBS", &user) {
        Ok(_) => (),
        Err(_) => {
            let url = format!("https://api.github.com/users/{}/repos", &user);
            let res = make_json_request(&url).await?;
            let repos = res.json::<Repos>().await?;
            let _ = hset("SUBS", &user, &user);

            for r in repos {
                let id = r.id.unwrap().to_string();

                let _ = hset(&user, &id, &id);
            }
        }
    }
    Ok(())
}

pub async fn check_subs_cli(user: &str) -> Result<()> {
    let url =
        format!("https://api.github.com/users/{}/repos?per_page=5", &user);
    let res = make_json_request(&url).await?;
    let repos = res.json::<Repos>().await?;

    for r in repos {
        let id = r.id.unwrap().to_string();

        let _ = match hget(&user, &id) {
            Err(_) => {
                let name = r.name.unwrap_or(String::from("<MISSING NAME>"));
                let url = r.html_url.unwrap();

                let content = [name, url];

                hset(&user, &id, &id)?;
                draw_box(&content, id);
            }
            Ok(_) => (),
        };
    }
    Ok(())
}

pub async fn get_subs_cli() -> Result<()> {
    match hkeys("SUBS") {
        Ok(subs) => {
            for sub in subs {
                println!(" ⭐ {sub}");
                check_subs_cli(&sub).await?;
            }
        }
        Err(_) => (),
    }
    Ok(())
}

pub async fn unsubscribe(user: &str) -> Result<()> {
    let _ = hdel("SUBS", &user);
    Ok(())
}
