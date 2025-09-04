use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "ssr")]
use crate::components::post::get_all_posts;
use crate::components::post::{BlogPost, PostCardMetrics};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::ui::life::Life;

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<crate::components::post::BlogPost>, ServerFnError> {
    get_all_posts()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn HomePage() -> impl IntoView {
    // Get global contexts
    let posts = use_context::<Resource<(), Result<Vec<BlogPost>, ServerFnError>>>()
        .expect("posts context not provided");

    // Get animation speed, population density, and search query from global context
    let animation_speed = use_context::<RwSignal<u64>>().unwrap_or_else(|| RwSignal::new(100u64));
    let population_density =
        use_context::<RwSignal<f64>>().unwrap_or_else(|| RwSignal::new(0.08f64));
    let search_query =
        use_context::<RwSignal<String>>().unwrap_or_else(|| RwSignal::new(String::new()));

    // Function to filter posts based on search query
    let filter_posts = move |posts: Vec<crate::components::post::BlogPost>| {
        let query = search_query.get().trim().to_lowercase();
        if query.is_empty() {
            posts
        } else {
            posts
                .into_iter()
                .filter(|post| {
                    post.title.to_lowercase().contains(&query)
                        || post.excerpt.to_lowercase().contains(&query)
                })
                .collect()
        }
    };

    view! {
        <>
            <Title text="blog"/>
            <Life animation_speed=animation_speed population_density=population_density />

            <div class="relative min-h-screen">
                <div class="relative z-10 max-w-5xl mx-auto px-2 sm:px-4 md:px-6 lg:px-8 pt-1 sm:pt-6 md:pt-8 pb-6 sm:pb-12 md:pb-16">
                <main>
                    <Suspense fallback=move || view! { <div></div> }>
                        {move || {
                            posts.get()
                                .map(|posts| match posts {
                                        Ok(posts) => {
                                            let filtered_posts = filter_posts(posts);
                                            if filtered_posts.is_empty() {
                                                let query = search_query.get();
                                                if query.trim().is_empty() {
                                                    view! {
                                                        <div class="text-center py-16">
                                                            <p class="text-muted-foreground text-lg">"No posts available"</p>
                                                        </div>
                                                    }.into_view()
                                                } else {
                                                    // Don't show any message when search has no results
                                                    view! { <div class="py-16"></div> }.into_view()
                                                }
                                            } else {
                                                view! {
                                                    <div class="space-y-2 sm:space-y-3 md:space-y-4">
                                                        {filtered_posts.into_iter()
                                                            .map(|post| {
                                                                let post_url = format!("/post/{}", post.slug);
                                                                let title = post.title.clone();
                                                                let excerpt = post.excerpt.clone();
                                                                let slug_for_share = post.slug.clone();

                                                                view! {
                                                                    <div class="relative w-full group">
                                                                        <A
                                                                            href=post_url
                                                                            class="block w-full"
                                                                        >
                                                                            <Card class="w-full h-20 sm:h-24 md:h-28 flex flex-col hover:bg-accent hover:border-ring focus-within:border-ring border-2 border-border transition-colors cursor-pointer">
                                                                                <CardHeader class="flex-shrink-0 !p-2 sm:!p-3 md:!p-4 !pb-1 sm:!pb-1 md:!pb-2">
                                                                                    <CardTitle class="text-card-foreground truncate !mb-0 sm:!mb-1 !text-sm sm:!text-base md:!text-xl font-bold">
                                                                                        {title.clone()}
                                                                                    </CardTitle>
                                                                                </CardHeader>
                                                                                <CardContent class="flex-1 flex flex-col !pt-0 !px-2 sm:!px-3 md:!px-4 !pb-2 sm:!pb-3 md:!pb-4">
                                                                                    <p class="text-muted-foreground truncate text-xs sm:text-sm md:text-base">
                                                                                        {excerpt.clone()}
                                                                                    </p>
                                                                                    <div class="flex-1"></div>
                                                                                </CardContent>
                                                                            </Card>
                                                                        </A>
                                                                        <div class="absolute bottom-1 left-2 sm:bottom-1 sm:left-3 md:bottom-1 md:left-4 pointer-events-none">
                                                                            <PostCardMetrics
                                                                                views=post.metrics.views
                                                                            />
                                                                        </div>
                                                                        <div class="absolute bottom-1 right-2 sm:bottom-1 sm:right-3 md:bottom-1 md:right-4 opacity-0 group-hover:opacity-100 transition-opacity">
                                                                            {
                                                                                let (is_shared, set_is_shared) = create_signal(false);
                                                                                view! {
                                                                                    <Button
                                                                                        variant=ButtonVariant::Plain
                                                                                        onclick=Box::new({
                                                                                            let slug_for_async = slug_for_share.clone();
                                                                                            let _set_is_shared = set_is_shared;
                                                                                            move || {
                                                                                                if is_shared.get() {
                                                                                                    return;
                                                                                                }

                                                                                                // Set to shared immediately for visual feedback
                                                                                                _set_is_shared.set(true);

                                                                                                let _slug_clone = slug_for_async.clone();
                                                                                                let _set_is_shared_clone = _set_is_shared;

                                                                                                spawn_local(async move {
                                                                                                    #[cfg(feature = "hydrate")]
                                                                                                    {
                                                                                                        if let Some(window) = web_sys::window() {
                                                                                                            let full_url = format!("{}/post/{}", window.location().origin().unwrap(), _slug_clone);

                                                                                                            let navigator = window.navigator();
                                                                                                            let clipboard = navigator.clipboard();
                                                                                                            let promise = clipboard.write_text(&full_url);
                                                                                                            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                                                                                        }
                                                                                                    }

                                                                                                    // Wait 2 seconds then reset
                                                                                                    #[cfg(feature = "hydrate")]
                                                                                                    {
                                                                                                        gloo_timers::future::TimeoutFuture::new(2000).await;
                                                                                                    }
                                                                                                    _set_is_shared_clone.set(false);
                                                                                                });
                                                                                            }
                                                                                        })
                                                                                        attr:class=move || if is_shared.get() { "text-green-500" } else { "text-muted-foreground hover:text-foreground" }
                                                                                    >
                                                                                        {move || {
                                                                                            if is_shared.get() {
                                                                                                view! {
                                                                                                    <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                                                                                    </svg>
                                                                                                }
                                                                                            } else {
                                                                                                view! {
                                                                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.367 2.684 3 3 0 00-5.367-2.684z"></path>
                                                                                                    </svg>
                                                                                                }
                                                                                            }
                                                                                        }}
                                                                                    </Button>
                                                                                }
                                                                            }
                                                                        </div>
                                                                    </div>
                                                                }
                                                            })
                                                            .collect_view()
                                                        }
                                                    </div>
                                                }.into_view()
                                            }
                                        }.into_view(),
                                        Err(e) => view! {
                                            <p class="text-destructive text-center text-lg">"Error loading posts: " {e.to_string()}</p>
                                        }.into_view(),
                                    })
                            }}
                        </Suspense>
                </main>
            </div>
        </div>
        </>
    }
}
