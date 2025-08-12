use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: DateTime<Utc>,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
    pub metrics: PostMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PostMetrics {
    pub views: u64,
    pub likes: u64,
    pub dislikes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMeta {
    pub title: String,
    pub date: DateTime<Utc>,
    pub excerpt: String,
    pub tags: Vec<String>,
}