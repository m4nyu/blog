use crate::app::{Settings, Theme, MAX_DENSITY};
use leptos::prelude::*;

/// Fixed bottom-right control, styled like the icon buttons on m4nuel.com.
#[component]
pub fn SettingsButton() -> impl IntoView {
    let s = expect_context::<Settings>();
    view! {
        <div class="corner">
            <button
                type="button"
                class="icon-button"
                aria-label="Settings"
                aria-haspopup="dialog"
                aria-expanded=move || s.open.get().to_string()
                on:click=move |_| s.open.update(|open| *open = !*open)
            >
                <svg viewBox="0 0 256 256" aria-hidden="true">
                    <line x1="40" y1="88" x2="216" y2="88"></line>
                    <circle cx="160" cy="88" r="26"></circle>
                    <line x1="40" y1="168" x2="216" y2="168"></line>
                    <circle cx="96" cy="168" r="26"></circle>
                </svg>
            </button>
        </div>
    }
}

#[component]
pub fn SettingsDialog() -> impl IntoView {
    let s = expect_context::<Settings>();
    let close = move |_| s.open.set(false);

    view! {
        <Show when=move || s.open.get()>
            <div class="overlay" on:click=close></div>
            <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="settings-title">
                <header>
                    <h2 id="settings-title">"Settings"</h2>
                    <button type="button" class="ghost-button" aria-label="Close settings" on:click=close>
                        "close"
                    </button>
                </header>
                <label>
                    <span>"Generation every " {move || s.speed.get()} " ms"</span>
                    <input
                        type="range"
                        min="50"
                        max="500"
                        step="50"
                        prop:value=move || s.speed.get().to_string()
                        on:input=move |e| {
                            if let Ok(v) = event_target_value(&e).parse::<u32>() {
                                s.speed.set(v.clamp(50, 500));
                            }
                        }
                    />
                </label>
                <label>
                    <span>"Start density " {move || format!("{:.0}", s.density.get() * 100.0)} " %"</span>
                    <input
                        type="range"
                        min="1"
                        max="25"
                        step="1"
                        prop:value=move || format!("{:.0}", s.density.get() * 100.0)
                        on:change=move |e| {
                            if let Ok(v) = event_target_value(&e).parse::<f64>() {
                                s.density.set((v / 100.0).clamp(0.01, MAX_DENSITY));
                            }
                        }
                    />
                </label>
                <fieldset>
                    <legend>"Theme"</legend>
                    {Theme::ALL
                        .into_iter()
                        .map(|t| {
                            view! {
                                <button
                                    type="button"
                                    class:active=move || s.theme.get() == t
                                    aria-pressed=move || (s.theme.get() == t).to_string()
                                    on:click=move |_| s.theme.set(t)
                                >
                                    {t.label()}
                                </button>
                            }
                        })
                        .collect_view()}
                </fieldset>
            </div>
        </Show>
    }
}
