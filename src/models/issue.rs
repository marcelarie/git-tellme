use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub user: Assignee,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    pub assignee: Assignee,
    pub assignees: Vec<Assignee>,
    pub milestone: Option<serde_json::Value>,
    pub comments: i64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub author_association: String,
    pub active_lock_reason: Option<serde_json::Value>,
    pub body: String,
    pub closed_by: Option<Assignee>,
    pub performed_via_github_app: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assignee {
    pub login:               String,
    pub id:                  i64,
    pub node_id:             String,
    pub avatar_url:          String,
    pub gravatar_id:         String,
    pub url:                 String,
    pub html_url:            String,
    pub followers_url:       String,
    pub following_url:       String,
    pub gists_url:           String,
    pub starred_url:         String,
    pub subscriptions_url:   String,
    pub organizations_url:   String,
    pub repos_url:           String,
    pub events_url:          String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub assignee_type:       String,
    pub site_admin:          bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id:            i64,
    pub node_id:       String,
    pub url:           String,
    pub name:          String,
    pub color:         String,
    #[serde(rename = "default")]
    pub label_default: bool,
    pub description:   Option<serde_json::Value>,
}
