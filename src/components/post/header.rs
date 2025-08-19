use super::interactions::PostMetrics;
use crate::components::ui::badge::{Badge, BadgeSize, BadgeVariant};
use crate::components::ui::button::{Button, ButtonVariant};
use chrono::{DateTime, Utc};
use leptos::*;

#[component]
pub fn PostHeader(
    slug: String,
    title: String,
    date: DateTime<Utc>,
    excerpt: String,
    tags: Vec<String>,
    initial_views: u64,
) -> impl IntoView {
    #[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
    let (is_shared, set_is_shared) = create_signal(false);
    view! {
        <div class="mb-8 sm:mb-10 md:mb-12 pb-6 sm:pb-7 md:pb-8 border-b-2 border-border">
            <h1 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold mb-3 sm:mb-3.5 md:mb-4 text-foreground leading-tight">{title.clone()}</h1>

            <div class="flex flex-wrap items-center gap-2 sm:gap-3 md:gap-4 mb-3 sm:mb-3.5 md:mb-4 text-xs sm:text-sm text-muted-foreground">
                <div class="flex items-center gap-1.5 sm:gap-2">
                    <svg class="w-3 h-3 sm:w-4 sm:h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                    </svg>
                    <span>{date.format("%B %d, %Y").to_string()}</span>
                </div>
            </div>

            <p class="text-base sm:text-lg md:text-xl text-muted-foreground mb-4 sm:mb-5 md:mb-6 leading-relaxed italic">{excerpt.clone()}</p>

            <div class="flex flex-wrap gap-1.5 sm:gap-2 mb-4">
                {tags.into_iter().map(|tag| {
                    view! {
                        <Badge variant=BadgeVariant::Primary size=BadgeSize::Medium>
                            {"#"}{tag}
                        </Badge>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="flex items-center justify-between">
                <PostMetrics slug=slug.clone() initial_views=initial_views />

                <Button
                    variant=ButtonVariant::Plain
                    onclick=Box::new({
                        let _slug = slug.clone();
                        let _set_is_shared = set_is_shared;
                        move || {
                            if is_shared.get() {
                                return;
                            }

                            // Set to shared immediately for visual feedback
                            _set_is_shared.set(true);

                            let _slug_for_async = _slug.clone();
                            let _set_is_shared_clone = _set_is_shared;

                            spawn_local(async move {
                                #[cfg(feature = "hydrate")]
                                {
                                    if let Some(window) = web_sys::window() {
                                        let full_url = format!("{}/post/{}", window.location().origin().unwrap(), _slug_for_async);

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
                                <>
                                    <svg class="w-4 h-4 mr-2 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                    </svg>
                                    <span class="text-green-500">"Copied"</span>
                                </>
                            }
                        } else {
                            view! {
                                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.367 2.684 3 3 0 00-5.367-2.684z"></path>
                                </svg>
                                "Share"
                            }
                        }
                    }}
                </Button>
            </div>
        </div>
    }
}
