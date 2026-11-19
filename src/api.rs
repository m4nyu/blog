//! Server functions: the only API the browser calls. Both are read-only.

use crate::model::{Post, PostSummary};
use leptos::prelude::*;

#[server]
pub async fn list_posts() -> Result<Vec<PostSummary>, ServerFnError> {
    crate::content::list(&crate::content::posts_dir())
        .await
        .map_err(|e| {
            eprintln!("list_posts: {e}");
            ServerFnError::new("cannot read posts")
        })
}

#[server]
pub async fn get_post(slug: String) -> Result<Option<Post>, ServerFnError> {
    crate::content::load(&crate::content::posts_dir(), &slug)
        .await
        .map_err(|e| {
            eprintln!("get_post: {e}");
            ServerFnError::new("cannot read post")
        })
}
