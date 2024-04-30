use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub article_id: u64,
    pub title: String,
    pub content: String,
}