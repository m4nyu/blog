use leptos::*;
use crate::components::ui::button::{Button, ButtonVariant};

// Component for displaying view count on post cards
#[component]
pub fn PostCardMetrics(
    views: u64,
) -> impl IntoView {
    view! {
        <div class="inline-flex items-center justify-center h-8 w-auto px-2 gap-1 text-xs sm:text-sm text-muted-foreground pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-200">
            // Views only
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
            </svg>
            <span>{views}" "{if views == 1 { "view" } else { "views" }}</span>
        </div>
    }
}


// Component for view tracking metrics in post header
#[component]
pub fn PostMetrics(
    slug: String,
    initial_views: u64,
) -> impl IntoView {
    #[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
    let (views, set_views) = create_signal(initial_views);

    // Track view on component mount
    create_effect({
        #[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
        let slug_for_effect = slug.clone();
        move |_| {
            #[cfg(feature = "hydrate")]
            {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let view_key = format!("viewed_{}", slug_for_effect);
                        if let Ok(None) = storage.get_item(&view_key) {
                            let slug_for_async = slug_for_effect.clone();
                            let view_key_for_async = view_key.clone();
                            spawn_local(async move {
                                use crate::routes::post::track_view;
                                if let Ok(_) = track_view(slug_for_async).await {
                                    set_views.update(|v| *v += 1);
                                    if let Some(window) = web_sys::window() {
                                        if let Ok(Some(storage)) = window.local_storage() {
                                            let _ = storage.set_item(&view_key_for_async, "true");
                                        }
                                    }
                                }
                            });
                        }
                    }
                }
            }
        }
    });

    view! {
        <div class="flex items-center gap-2 text-sm text-muted-foreground">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
            </svg>
            <span>{move || views.get()}" "{move || if views.get() == 1 { "view" } else { "views" }}</span>
        </div>
    }
}

// Component for post interactions at the bottom of post pages  
#[component]
pub fn PostInteractions(
    slug: String,
) -> impl IntoView {
    #[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
    let (is_shared, set_is_shared) = create_signal(false);

    let handle_share = {
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
    };

    view! {
        <div class="mt-12 pt-8 border-t border-border">
            <div class="flex justify-center">
                <div class="text-center">
                    <p class="text-sm text-muted-foreground mb-3">
                        "Share this post"
                    </p>
                    <Button 
                        variant=ButtonVariant::Plain
                        onclick=Box::new(handle_share)
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
        </div>
    }
}