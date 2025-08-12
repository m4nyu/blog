use leptos::*;

#[component]
pub fn CardShareButton(
    slug: String,
) -> impl IntoView {
    #[cfg_attr(not(feature = "hydrate"), allow(unused_variables))]
    let (is_shared, set_is_shared) = create_signal(false);

    let handle_share = {
        let _slug = slug.clone();
        
        move |ev: ev::MouseEvent| {
            ev.stop_propagation();
            ev.prevent_default();
            
            if is_shared.get() {
                return;
            }
            
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
                        if let Ok(_) = wasm_bindgen_futures::JsFuture::from(promise).await {
                            set_is_shared.set(true);
                            
                            gloo_timers::future::TimeoutFuture::new(2000).await;
                            set_is_shared.set(false);
                        }
                    }
                }
            });
        }
    };

    view! {
        <button
            class="inline-flex items-center justify-center p-2 h-8 w-8 rounded-full hover:bg-accent hover:text-accent-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            on:click=handle_share
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
        </button>
    }
}