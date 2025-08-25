use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
use crate::components::post::{get_post_by_slug, increment_view, update_vote};
use crate::components::post::{header::PostHeader, markdown::Markdown, PostInteractions};

#[server(GetPost, "/api")]
pub async fn get_post(
    slug: String,
) -> Result<Option<crate::components::post::BlogPost>, ServerFnError> {
    eprintln!("Server function get_post called with slug: {}", slug);
    let result = get_post_by_slug(&slug).await;
    match &result {
        Ok(Some(post)) => eprintln!("Successfully loaded post: {}", post.title),
        Ok(None) => eprintln!("Post not found for slug: {}", slug),
        Err(e) => eprintln!("Error loading post: {}", e),
    }
    result.map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(TrackView, "/api")]
pub async fn track_view(slug: String) -> Result<(), ServerFnError> {
    increment_view(&slug);
    Ok(())
}

#[server(SubmitVote, "/api")]
pub async fn submit_vote(slug: String, is_like: bool) -> Result<(), ServerFnError> {
    update_vote(&slug, is_like);
    Ok(())
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap_or_default());

    let post = create_resource(slug, |slug| async move {
        leptos::logging::log!("Calling get_post with slug: {}", slug);
        let result = get_post(slug).await;
        match &result {
            Ok(Some(_)) => leptos::logging::log!("Successfully loaded post"),
            Ok(None) => leptos::logging::log!("Post not found"),
            Err(e) => leptos::logging::log!("Error loading post: {:?}", e),
        }
        result
    });

    view! {
        <div class="max-w-4xl mx-auto px-4 sm:px-6 pt-3 pb-12 sm:pb-16 min-h-screen">
            <Suspense fallback=move || view! { <div></div> }>
                {move || {
                    post.get()
                        .map(|post| match post {
                            Ok(Some(post)) => {
                                view! {
                                    <article>
                                        <Title text=post.title.clone()/>
                                        <Meta name="description" content=post.excerpt.clone()/>

                                        // Back button
                                        <A href="/" class="text-sm sm:text-base md:text-lg text-primary hover:text-primary/80 mb-6 sm:mb-7 md:mb-8 inline-block">
                                            "← Back to posts"
                                        </A>

                                        // Post header with metadata and metrics
                                        <PostHeader
                                            slug=post.slug.clone()
                                            title=post.title.clone()
                                            date=post.date
                                            excerpt=post.excerpt.clone()
                                            tags=post.tags.clone()
                                            initial_views=post.metrics.views
                                        />

                                        // Post content
                                        <Markdown content=post.content base_path=format!("{}.md", post.slug)/>

                                        // Post interactions (sharing)
                                        <PostInteractions
                                            slug=post.slug.clone()
                                        />
                                    </article>
                                }.into_view()
                            }
                            Ok(None) => view! {
                                <Title text="Post not found - blog"/>
                                <div class="text-center py-12">
                                    <h1 class="text-3xl font-bold text-foreground mb-4">
                                        "Post not found"
                                    </h1>
                                    <A href="/" class="text-primary hover:text-primary/80">
                                        "Back to home"
                                    </A>
                                </div>
                            }.into_view(),
                            Err(e) => view! {
                                <Title text="Error - blog"/>
                                <p class="text-destructive">"Error loading post: " {e.to_string()}</p>
                            }.into_view(),
                        })
                }}
            </Suspense>
        </div>
    }
}
