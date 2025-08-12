use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::ui::life::Life;
use crate::components::post::card_metrics::PostCardMetrics;
use crate::components::post::card_share_button::CardShareButton;
use crate::components::post::BlogPost;
#[cfg(feature = "ssr")]
use crate::components::post::get_all_posts;

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<crate::components::post::BlogPost>, ServerFnError> {
    eprintln!("Server function get_posts called");
    let result = get_all_posts().await;
    match &result {
        Ok(posts) => eprintln!("Successfully loaded {} posts", posts.len()),
        Err(e) => eprintln!("Error loading posts: {}", e),
    }
    result.map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn HomePage() -> impl IntoView {
    // Get global contexts
    let posts = use_context::<Resource<(), Result<Vec<BlogPost>, ServerFnError>>>()
        .expect("posts context not provided");
    
    // Get animation speed, population density, and search query from global context
    let animation_speed = use_context::<RwSignal<u64>>().unwrap_or_else(|| RwSignal::new(100u64));
    let population_density = use_context::<RwSignal<f64>>().unwrap_or_else(|| RwSignal::new(0.08f64));
    let search_query = use_context::<RwSignal<String>>().unwrap_or_else(|| RwSignal::new(String::new()));

    // Function to filter posts based on search query
    let filter_posts = move |posts: Vec<crate::components::post::BlogPost>| {
        let query = search_query.get().trim().to_lowercase();
        if query.is_empty() {
            posts
        } else {
            posts.into_iter()
                .filter(|post| {
                    post.title.to_lowercase().contains(&query) ||
                    post.excerpt.to_lowercase().contains(&query)
                })
                .collect()
        }
    };

    view! {
        <>
            <Title text="blog"/>
            <Life animation_speed=animation_speed population_density=population_density />
            
            <div class="relative min-h-screen">
                <div class="relative z-10 max-w-5xl mx-auto px-2 sm:px-4 md:px-6 lg:px-8 pt-3 sm:pt-6 md:pt-8 pb-6 sm:pb-12 md:pb-16">
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
                                                                    <A
                                                                        href=post_url
                                                                        class="block w-full"
                                                                    >
                                                                        <Card class="w-full h-20 sm:h-24 md:h-28 flex flex-col hover:bg-accent hover:border-ring focus-within:border-ring border-2 border-border transition-colors cursor-pointer group">
                                                                            <CardHeader class="flex-shrink-0 !p-2 sm:!p-3 md:!p-4 !pb-1 sm:!pb-1 md:!pb-2">
                                                                                <CardTitle class="text-card-foreground truncate !mb-0 sm:!mb-1 !text-sm sm:!text-base md:!text-xl font-bold">
                                                                                    {title.clone()}
                                                                                </CardTitle>
                                                                            </CardHeader>
                                                                            <CardContent class="flex-1 flex flex-col !pt-0 !px-2 sm:!px-3 md:!px-4 !pb-2 sm:!pb-3 md:!pb-4">
                                                                                <p class="text-muted-foreground truncate text-xs sm:text-sm md:text-base">
                                                                                    {excerpt.clone()}
                                                                                </p>
                                                                                <div class="flex items-end justify-between">
                                                                                    <PostCardMetrics 
                                                                                        views=post.metrics.views
                                                                                        likes=post.metrics.likes
                                                                                        dislikes=post.metrics.dislikes
                                                                                    />
                                                                                    <CardShareButton 
                                                                                        slug=slug_for_share
                                                                                    />
                                                                                </div>
                                                                            </CardContent>
                                                                        </Card>
                                                                    </A>
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
