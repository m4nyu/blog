use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::models::markdown::Markdown;
use crate::models::components::ui::badge::{Badge, BadgeVariant, BadgeSize};
#[cfg(feature = "ssr")]
use crate::models::post::get_post_by_slug;

#[server(GetPost, "/api")]
pub async fn get_post(
    slug: String,
) -> Result<Option<crate::models::post::BlogPost>, ServerFnError> {
    eprintln!("Server function get_post called with slug: {}", slug);
    let result = get_post_by_slug(&slug).await;
    match &result {
        Ok(Some(post)) => eprintln!("Successfully loaded post: {}", post.title),
        Ok(None) => eprintln!("Post not found for slug: {}", slug),
        Err(e) => eprintln!("Error loading post: {}", e),
    }
    result.map_err(|e| ServerFnError::ServerError(e.to_string()))
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

                                        // Just back button and markdown content - responsive
                                        <A href="/" class="text-sm sm:text-base md:text-lg text-primary hover:text-primary/80 mb-6 sm:mb-7 md:mb-8 inline-block">
                                            "← Back to posts"
                                        </A>

                                        // Frontmatter display - responsive
                                        <div class="mb-8 sm:mb-10 md:mb-12 pb-6 sm:pb-7 md:pb-8 border-b-2 border-border">
                                            <h1 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold mb-3 sm:mb-3.5 md:mb-4 text-foreground leading-tight">{post.title.clone()}</h1>
                                            
                                            <div class="flex flex-wrap items-center gap-2 sm:gap-3 md:gap-4 mb-3 sm:mb-3.5 md:mb-4 text-xs sm:text-sm text-muted-foreground">
                                                <div class="flex items-center gap-1.5 sm:gap-2">
                                                    <svg class="w-3 h-3 sm:w-4 sm:h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                                                    </svg>
                                                    <span>{post.date.format("%B %d, %Y").to_string()}</span>
                                                </div>
                                            </div>
                                            
                                            <p class="text-base sm:text-lg md:text-xl text-muted-foreground mb-4 sm:mb-5 md:mb-6 leading-relaxed italic">{post.excerpt.clone()}</p>
                                            
                                            <div class="flex flex-wrap gap-1.5 sm:gap-2">
                                                {post.tags.clone().into_iter().map(|tag| {
                                                    view! {
                                                        <Badge variant=BadgeVariant::Primary size=BadgeSize::Medium>
                                                            {"#"}{tag}
                                                        </Badge>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>

                                        <Markdown content=post.content/>
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
