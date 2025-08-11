use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::models::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::models::components::life::Life;
use crate::models::post::BlogPost;
#[cfg(feature = "ssr")]
use crate::models::post::get_all_posts;

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<crate::models::post::BlogPost>, ServerFnError> {
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
    let filter_posts = move |posts: Vec<crate::models::post::BlogPost>| {
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
                <div class="relative z-10 max-w-5xl mx-auto px-4 sm:px-6 md:px-8 pt-6 sm:pt-8 pb-12 sm:pb-16">
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
                                                    <div class="space-y-6 sm:space-y-8">
                                                        {filtered_posts.into_iter()
                                                            .map(|post| {
                                                                let post_url = format!("/post/{}", post.slug);
                                                                view! {
                                                                    <A
                                                                        href=post_url
                                                                        class="block w-full"
                                                                    >
                                                                        <Card class="w-full h-24 sm:h-28 md:h-32 flex flex-col hover:bg-accent transition-colors cursor-pointer group">
                                                                            <CardHeader class="flex-shrink-0 p-3 sm:p-4 md:p-5 pb-1 sm:pb-2 md:pb-2">
                                                                                <CardTitle class="text-card-foreground truncate mb-1 sm:mb-2 text-lg sm:text-xl md:text-2xl font-bold">
                                                                                    {&post.title}
                                                                                </CardTitle>
                                                                            </CardHeader>
                                                                            <CardContent class="flex-1 flex flex-col pt-0 px-3 sm:px-4 md:px-5 pb-3 sm:pb-4 md:pb-5">
                                                                                <p class="text-muted-foreground truncate text-sm sm:text-base md:text-lg">
                                                                                    {&post.excerpt}
                                                                                </p>
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
