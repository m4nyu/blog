use leptos::*;
use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn PostInteractions(
    slug: String,
    initial_likes: u64,
    initial_dislikes: u64,
) -> impl IntoView {
    let (likes, set_likes) = create_signal(initial_likes);
    let (dislikes, set_dislikes) = create_signal(initial_dislikes);
    let (voted, set_voted) = create_signal::<Option<bool>>(None);

    create_effect({
        #[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
        let slug_for_effect = slug.clone();
        move |_| {
            #[cfg(feature = "hydrate")]
            {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let vote_key = format!("vote_{}", slug_for_effect);
                        if let Ok(Some(stored_vote)) = storage.get_item(&vote_key) {
                            if let Ok(is_like) = stored_vote.parse::<bool>() {
                                set_voted.set(Some(is_like));
                            }
                        }
                    }
                }
            }
        }
    });

    view! {
        <div class="mt-12 pt-8 border-t border-border">
            <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-6">
                <div>
                    <h3 class="text-lg font-semibold text-foreground mb-4">
                        "Was this post helpful?"
                    </h3>
                    <div class="flex items-center gap-4">
                        // Thumbs up button
                        <Button
                            variant=if voted.get() == Some(true) { ButtonVariant::Default } else { ButtonVariant::Outline }
                            onclick=Box::new({
                                let slug = slug.clone();
                                move || {
                                    if voted.get() == Some(true) {
                                        return;
                                    }
                                    
                                    let slug_clone = slug.clone();
                                    spawn_local(async move {
                                        use crate::routes::post::submit_vote;
                                        let _ = submit_vote(slug_clone, true).await;
                                    });
                                    
                                    set_likes.update(|l| *l += 1);
                                    set_voted.set(Some(true));
                                    
                                    #[cfg(feature = "hydrate")]
                                    {
                                        if let Some(window) = web_sys::window() {
                                            if let Ok(Some(storage)) = window.local_storage() {
                                                let vote_key = format!("vote_{}", slug);
                                                let _ = storage.set_item(&vote_key, "true");
                                            }
                                        }
                                    }
                                }
                            })
                        >
                            <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M2 10.5a1.5 1.5 0 113 0v6a1.5 1.5 0 01-3 0v-6zM6 10.333v5.43a2 2 0 001.106 1.79l.05.025A4 4 0 008.943 18h5.416a2 2 0 001.962-1.608l1.2-6A2 2 0 0015.56 8H12V4a2 2 0 00-2-2 1 1 0 00-1 1v.667a4 4 0 01-.8 2.4L6.8 7.933a4 4 0 00-.8 2.4z"></path>
                            </svg>
                            {move || likes.get().to_string()}
                        </Button>

                        // Thumbs down button  
                        <Button
                            variant=if voted.get() == Some(false) { ButtonVariant::Destructive } else { ButtonVariant::Outline }
                            onclick=Box::new({
                                let slug = slug.clone();
                                move || {
                                    if voted.get() == Some(false) {
                                        return;
                                    }
                                    
                                    let slug_clone = slug.clone();
                                    spawn_local(async move {
                                        use crate::routes::post::submit_vote;
                                        let _ = submit_vote(slug_clone, false).await;
                                    });
                                    
                                    set_dislikes.update(|d| *d += 1);
                                    set_voted.set(Some(false));
                                    
                                    #[cfg(feature = "hydrate")]
                                    {
                                        if let Some(window) = web_sys::window() {
                                            if let Ok(Some(storage)) = window.local_storage() {
                                                let vote_key = format!("vote_{}", slug);
                                                let _ = storage.set_item(&vote_key, "false");
                                            }
                                        }
                                    }
                                }
                            })
                        >
                            <svg class="w-5 h-5 mr-2 rotate-180" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M2 10.5a1.5 1.5 0 113 0v6a1.5 1.5 0 01-3 0v-6zM6 10.333v5.43a2 2 0 001.106 1.79l.05.025A4 4 0 008.943 18h5.416a2 2 0 001.962-1.608l1.2-6A2 2 0 0015.56 8H12V4a2 2 0 00-2-2 1 1 0 00-1 1v.667a4 4 0 01-.8 2.4L6.8 7.933a4 4 0 00-.8 2.4z"></path>
                            </svg>
                            {move || dislikes.get().to_string()}
                        </Button>
                    </div>
                </div>
                
                <div class="text-center sm:text-right">
                    <p class="text-sm text-muted-foreground mb-3">
                        "Share this post"
                    </p>
                                            <Button 
                            variant=ButtonVariant::Outline
                            onclick=Box::new({
                                let _slug = slug.clone();
                                move || {
                                    #[cfg(feature = "hydrate")]
                                    let slug_for_async = _slug.clone();
                                    spawn_local(async move {
                                        #[cfg(feature = "hydrate")]
                                        {
                                            if let Some(window) = web_sys::window() {
                                                let full_url = format!("{}/{}", window.location().origin().unwrap(), slug_for_async);
                                                
                                                let navigator = window.navigator();
                                                let clipboard = navigator.clipboard();
                                                let promise = clipboard.write_text(&full_url);
                                                let _result = wasm_bindgen_futures::JsFuture::from(promise).await;
                                            }
                                        }
                                    });
                                }
                            })
                        >
                        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.367 2.684 3 3 0 00-5.367-2.684z"></path>
                        </svg>
                        "Share"
                    </Button>
                </div>
            </div>
        </div>
    }
}