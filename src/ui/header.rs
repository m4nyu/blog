use crate::app::Settings;
use leptos::prelude::*;

/// Search bar with inline completion from post titles, plus the settings button.
/// Hides while scrolling down, comes back when scrolling up.
#[component]
pub fn Header(query: RwSignal<String>, titles: Vec<String>) -> impl IntoView {
    let settings = expect_context::<Settings>();
    let input = NodeRef::<leptos::html::Input>::new();
    let hidden = RwSignal::new(false);
    let titles = StoredValue::new(titles);

    // Remainder of the first title that starts with the query (case-insensitive).
    let completion = Memo::new(move |_| {
        let q = query.get();
        if q.trim().is_empty() {
            return String::new();
        }
        let lower = q.to_lowercase();
        titles.with_value(|all| {
            all.iter()
                .find(|t| t.to_lowercase().starts_with(&lower))
                .map(|t| t.chars().skip(q.chars().count()).collect())
                .unwrap_or_default()
        })
    });

    let on_key = move |e: leptos::ev::KeyboardEvent| match e.key().as_str() {
        "Tab" | "ArrowRight" => {
            let rest = completion.get_untracked();
            if !rest.is_empty() {
                e.prevent_default();
                query.update(|q| q.push_str(&rest));
            }
        }
        "Escape" => query.set(String::new()),
        _ => {}
    };

    #[cfg(feature = "hydrate")]
    {
        use std::cell::Cell;
        let shortcut = window_event_listener(leptos::ev::keydown, move |e| {
            if (e.ctrl_key() || e.meta_key()) && e.key().eq_ignore_ascii_case("k") {
                e.prevent_default();
                if let Some(el) = input.get() {
                    let _ = el.focus();
                }
            }
        });
        let last_y = Cell::new(0.0);
        let scroll = window_event_listener(leptos::ev::scroll, move |_| {
            let y = window().scroll_y().unwrap_or(0.0);
            hidden.set(y > last_y.get() && y > 64.0);
            last_y.set(y);
        });
        on_cleanup(move || {
            shortcut.remove();
            scroll.remove();
        });
    }

    view! {
        <header class="topbar" class:hidden=move || hidden.get()>
            <div class="search">
                <span class="completion" aria-hidden="true">
                    <span class="typed">{move || query.get()}</span>
                    {move || completion.get()}
                </span>
                <input
                    node_ref=input
                    type="text"
                    placeholder="Search  (Ctrl K)"
                    aria-label="Search posts"
                    autocomplete="off"
                    spellcheck="false"
                    maxlength="100"
                    bind:value=query
                    on:keydown=on_key
                />
            </div>
            <button
                type="button"
                class="ghost-button"
                aria-haspopup="dialog"
                on:click=move |_| settings.open.update(|open| *open = !*open)
            >
                "settings"
            </button>
        </header>
    }
}
