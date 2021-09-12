use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub welcome_type: String,
    pub site_admin: bool,
    pub name: String,
    pub company: String,
    pub blog: String,
    pub location: String,
    pub email: Option<serde_json::Value>,
    pub hireable: bool,
    pub bio: String,
    pub twitter_username: Option<serde_json::Value>,
    pub public_repos: i64,
    pub public_gists: i64,
    pub followers: i64,
    pub following: i64,
    pub created_at: String,
    pub updated_at: String,
    pub private_gists: i64,
    pub total_private_repos: i64,
    pub owned_private_repos: i64,
    pub disk_usage: i64,
    pub collaborators: i64,
    pub two_factor_authentication: bool,
    pub plan: Plan,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    pub name: String,
    pub space: i64,
    pub collaborators: i64,
    pub private_repos: i64,
}
