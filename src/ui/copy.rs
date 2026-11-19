use leptos::prelude::*;

/// Copies `text` to the clipboard. `done` reads true for two seconds after success.
pub fn copy_text(text: String, done: RwSignal<bool>) {
    #[cfg(feature = "hydrate")]
    {
        let promise = window().navigator().clipboard().write_text(&text);
        leptos::task::spawn_local(async move {
            if wasm_bindgen_futures::JsFuture::from(promise).await.is_ok() {
                done.set(true);
                set_timeout(move || done.set(false), std::time::Duration::from_secs(2));
            }
        });
    }
    #[cfg(not(feature = "hydrate"))]
    let _ = (text, done);
}

/// Absolute link to a post (the origin is only known in the browser).
fn post_url(slug: &str) -> String {
    #[cfg(feature = "hydrate")]
    let origin = window().location().origin().unwrap_or_default();
    #[cfg(not(feature = "hydrate"))]
    let origin = String::new();
    format!("{origin}/post/{slug}")
}

#[component]
pub fn ShareButton(slug: String) -> impl IntoView {
    let copied = RwSignal::new(false);
    view! {
        <button
            type="button"
            class="ghost-button"
            aria-label="Copy link to this post"
            on:click=move |_| copy_text(post_url(&slug), copied)
        >
            {move || if copied.get() { "link copied" } else { "share" }}
        </button>
    }
}
