use leptos::*;

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
        <div class="flex items-center gap-2 text-sm text-gray-600">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
            </svg>
            <span>{move || views.get()} {move || if views.get() == 1 { "view" } else { "views" }}</span>
        </div>
    }
}