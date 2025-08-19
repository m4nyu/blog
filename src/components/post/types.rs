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

// Server-side functions for handling posts and metrics
#[cfg(feature = "ssr")]
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use std::sync::Mutex;

#[cfg(feature = "ssr")]
pub async fn get_all_posts() -> Result<Vec<BlogPost>, std::io::Error> {
    use std::path::Path;
    use tokio::fs;

    let posts_dir = Path::new("content");
    let mut posts = Vec::new();

    if posts_dir.exists() {
        let mut entries = fs::read_dir(posts_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(content) = fs::read_to_string(&path).await {
                    if let Some(post) = parse_post(&content, &path) {
                        posts.push(post);
                    }
                }
            }
        }
    }

    // Sort posts by date (newest first)
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(posts)
}

#[cfg(feature = "ssr")]
pub async fn get_post_by_slug(slug: &str) -> Result<Option<BlogPost>, std::io::Error> {
    use std::path::Path;
    use tokio::fs;

    let post_path = Path::new("content").join(format!("{}.md", slug));

    if post_path.exists() {
        let content = fs::read_to_string(&post_path).await?;
        Ok(parse_post(&content, &post_path))
    } else {
        Ok(None)
    }
}

#[cfg(feature = "ssr")]
fn parse_post(content: &str, path: &std::path::Path) -> Option<BlogPost> {
    use gray_matter::engine::YAML;
    use gray_matter::Matter;

    let matter = Matter::<YAML>::new();
    let result = matter.parse(content);

    // Parse the frontmatter directly from the Pod
    let data = result.data?;
    let map = data.as_hashmap().ok()?;

    let title = map.get("title")?.as_string().ok()?;
    let date_str = map.get("date")?.as_string().ok()?;

    let date = DateTime::parse_from_rfc3339(&date_str)
        .ok()?
        .with_timezone(&Utc);

    let excerpt = map
        .get("excerpt")
        .and_then(|v| v.as_string().ok())
        .unwrap_or_default();

    let tags = map
        .get("tags")
        .and_then(|v| v.as_vec().ok())
        .map(|vec| vec.iter().filter_map(|v| v.as_string().ok()).collect())
        .unwrap_or_default();

    let slug = path.file_stem()?.to_str()?.to_string();

    Some(BlogPost {
        slug: slug.clone(),
        title,
        date,
        excerpt,
        content: result.content,
        tags,
        metrics: get_post_metrics(&slug),
    })
}

#[cfg(feature = "ssr")]
static METRICS_STORE: std::sync::LazyLock<Mutex<HashMap<String, PostMetrics>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(feature = "ssr")]
pub fn get_post_metrics(slug: &str) -> PostMetrics {
    let store = METRICS_STORE.lock().unwrap();
    store.get(slug).cloned().unwrap_or_default()
}

#[cfg(feature = "ssr")]
pub fn increment_view(slug: &str) {
    let mut store = METRICS_STORE.lock().unwrap();
    let metrics = store.entry(slug.to_string()).or_default();
    metrics.views += 1;
}

#[cfg(feature = "ssr")]
pub fn update_vote(slug: &str, is_like: bool) {
    let mut store = METRICS_STORE.lock().unwrap();
    let metrics = store.entry(slug.to_string()).or_default();
    if is_like {
        metrics.likes += 1;
    } else {
        metrics.dislikes += 1;
    }
}
