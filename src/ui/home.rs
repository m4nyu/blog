use crate::api::list_posts;
use crate::life::view::Life;
use crate::model::PostSummary;
use crate::ui::{copy::ShareButton, header::Header};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    let posts = Resource::new(|| (), |_| list_posts());
    let query = RwSignal::new(String::new());

    view! {
        <Title text="blog"/>
        <Life/>
        <div class="site">
            <Suspense fallback=|| ()>
                {move || Suspend::new(async move {
                    match posts.await {
                        Ok(posts) => {
                            let titles = posts.iter().map(|p| p.title.clone()).collect();
                            view! {
                                <Header query titles/>
                                <main class="page home">
                                    <PostList posts query/>
                                </main>
                            }
                                .into_any()
                        }
                        Err(e) => {
                            view! {
                                <main class="page">
                                    <p class="error">{e.to_string()}</p>
                                </main>
                            }
                                .into_any()
                        }
                    }
                })}
            </Suspense>
        </div>
    }
}

fn matches(post: &PostSummary, query: &str) -> bool {
    query.is_empty()
        || post.title.to_lowercase().contains(query)
        || post.excerpt.to_lowercase().contains(query)
        || post.tags.iter().any(|t| t.to_lowercase().contains(query))
}

#[component]
fn PostList(posts: Vec<PostSummary>, query: RwSignal<String>) -> impl IntoView {
    if posts.is_empty() {
        return view! { <p class="empty">"No posts yet."</p> }.into_any();
    }
    let posts = StoredValue::new(posts);
    let visible = move || {
        let q = query.get().trim().to_lowercase();
        posts.with_value(|all| {
            all.iter()
                .filter(|p| matches(p, &q))
                .cloned()
                .collect::<Vec<_>>()
        })
    };
    view! {
        <ul class="posts">
            <For each=visible key=|p| p.slug.clone() children=|post| view! { <PostCard post/> }/>
        </ul>
    }
    .into_any()
}

#[component]
fn PostCard(post: PostSummary) -> impl IntoView {
    let href = format!("/post/{}", post.slug);
    view! {
        <li class="card">
            <a href=href>
                <h2>{post.title}</h2>
                <p>{post.excerpt}</p>
            </a>
            <div class="card-actions">
                <ShareButton slug=post.slug/>
            </div>
        </li>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_matches_title_excerpt_and_tags() {
        let post = PostSummary {
            slug: "s".into(),
            title: "Game of Life".into(),
            date: "2026-01-01".into(),
            excerpt: "Cellular automata".into(),
            tags: vec!["Rust".into()],
        };
        for q in ["", "game", "automata", "rust"] {
            assert!(matches(&post, q), "{q}");
        }
        assert!(!matches(&post, "python"));
    }
}
