use crate::api::list_posts;
use crate::life::view::Life;
use crate::model::PostSummary;
use crate::seo::{PageMeta, DESCRIPTION, SITE_URL};
use crate::ui::copy::ShareButton;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let posts = Resource::new(|| (), |_| list_posts());

    view! {
        <PageMeta title="m4nuel.blog" description=DESCRIPTION url=format!("{SITE_URL}/")/>
        <Life/>
        <div class="site">
            <main class="page home">
                <Suspense fallback=|| ()>
                    {move || Suspend::new(async move {
                        match posts.await {
                            Ok(posts) => view! { <PostList posts/> }.into_any(),
                            Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                        }
                    })}
                </Suspense>
            </main>
        </div>
    }
}

#[component]
fn PostList(posts: Vec<PostSummary>) -> impl IntoView {
    if posts.is_empty() {
        return view! { <p class="empty">"No posts yet."</p> }.into_any();
    }
    view! {
        <ul class="posts">
            {posts.into_iter().map(|post| view! { <PostCard post/> }).collect_view()}
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
