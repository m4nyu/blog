pub mod code;
pub mod header;
pub mod interactions;
pub mod markdown;
pub mod types;

// Re-export everything from types and interactions for convenience
pub use types::*;
pub use interactions::*;

// Re-export server functions conditionally
#[cfg(feature = "ssr")]
pub use types::{get_all_posts, get_post_by_slug, increment_view, update_vote};