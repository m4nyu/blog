use leptos::*;

#[cfg(feature = "hydrate")]
use wasm_bindgen::{JsCast, closure::Closure};
#[cfg(feature = "hydrate")]
use web_sys::{Event, HtmlInputElement, KeyboardEvent};

use crate::components::post::BlogPost;

#[component]
pub fn Header(
    #[prop(optional)] _on_animation_speed_change: Option<Callback<u64>>,
    #[prop(optional)] on_theme_change: Option<Callback<String>>,
) -> impl IntoView {
    let search_query = use_context::<RwSignal<String>>().expect("search_query context not provided");
    let show_settings = use_context::<RwSignal<bool>>().expect("show_settings context not provided");
    let posts = use_context::<Resource<(), Result<Vec<BlogPost>, ServerFnError>>>()
        .expect("posts context not provided");
    let theme_mode = RwSignal::new("system".to_string());
    let is_scrolled = RwSignal::new(false);
    // Generate inline text suggestion based on current query
    let current_suggestion = create_memo(move |_| {
        let query_full = search_query.get();
        let query = query_full.trim();
        if query.is_empty() {
            String::new()
        } else {
            posts.get()
                .and_then(|posts_result| posts_result.ok())
                .and_then(|posts| {
                    posts.into_iter()
                        .find(|post| {
                            post.title.to_lowercase().starts_with(&query.to_lowercase())
                        })
                        .map(|post| post.title.clone())
                })
                .unwrap_or_default()
        }
    });

    // Handle search input
    #[cfg(feature = "hydrate")]
    let handle_search_input = move |ev: Event| {
        let target = ev.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>();
        let value = input.value();
        search_query.set(value);
    };

    // Handle keyboard navigation for suggestions
    #[cfg(feature = "hydrate")]
    let handle_search_keydown = move |ev: KeyboardEvent| {
        let key = ev.key();
        let suggestion = current_suggestion.get();
        let current_query = search_query.get();
        
        match key.as_str() {
            "Tab" | "ArrowRight" => {
                if !suggestion.is_empty() && suggestion != current_query {
                    ev.prevent_default();
                    search_query.set(suggestion);
                }
            },
            "Escape" => {
                // Clear the search
                search_query.set(String::new());
            },
            _ => {}
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let handle_search_input = move |_ev| {};

    #[cfg(not(feature = "hydrate"))]
    let handle_search_keydown = move |_ev| {};

    // Handle theme mode change
    let _handle_theme_change = move |mode: String| {
        theme_mode.set(mode.clone());
        if let Some(callback) = on_theme_change {
            callback.call(mode);
        }
    };

    // Scroll detection for header styling
    #[cfg(feature = "hydrate")]
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            let scroll_handler = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Some(window) = web_sys::window() {
                    let scroll_y = window.scroll_y().unwrap_or(0.0);
                    let scrolled = scroll_y > 0.0;
                    leptos::logging::log!("Scroll Y: {}, Is Scrolled: {}", scroll_y, scrolled);
                    is_scrolled.set(scrolled);
                }
            }) as Box<dyn FnMut(_)>);
            
            let _ = window.add_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref());
            scroll_handler.forget();
        }
    });

    // Keyboard shortcut for search (Cmd+K / Ctrl+K)
    #[cfg(feature = "hydrate")]
    create_effect(move |_| {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let keyboard_handler = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                let is_cmd_or_ctrl = event.meta_key() || event.ctrl_key();
                let is_k_key = event.key() == "k" || event.key() == "K";
                
                if is_cmd_or_ctrl && is_k_key {
                    event.prevent_default();
                    
                    // Focus the search input
                    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                        if let Ok(Some(search_input)) = document.query_selector("input[placeholder='Search (⌘K)']") {
                            if let Ok(input_element) = search_input.dyn_into::<web_sys::HtmlInputElement>() {
                                let _ = input_element.focus();
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            let _ = document.add_event_listener_with_callback("keydown", keyboard_handler.as_ref().unchecked_ref());
            keyboard_handler.forget();
        }
    });

    view! {
        <header class=move || format!(
            "fixed left-0 right-0 z-50 bg-background backdrop-blur-sm border-b border-border shadow-sm transition-all duration-300 {}",
            if is_scrolled.get() {
                "-top-16"  // Slide up and hide completely
            } else {
                "top-0"    // Visible at top
            }
        )>
            <div class="max-w-7xl mx-auto px-2 sm:px-4 py-1 sm:py-1.5 flex items-center justify-between gap-1 sm:gap-2">
                // Left spacer - hidden on mobile
                <div class="hidden sm:block w-20"></div>
                
                // Center - Search field with inline suggestions
                <div class="flex-1 max-w-full sm:max-w-md mx-0 sm:mx-auto">
                    <div class="relative">
                        <svg class="absolute left-2 sm:left-3 top-1/2 transform -translate-y-1/2 w-3 sm:w-4 h-3 sm:h-4 text-muted-foreground z-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m21 21-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                        </svg>
                        
                        // Suggestion text overlay (appears behind the input)
                        <div class="absolute inset-0 pl-7 sm:pl-10 pr-2 sm:pr-4 py-1 sm:py-1.5 text-xs sm:text-sm pointer-events-none flex items-center">
                            <span class="text-muted-foreground/50">
                                {move || {
                                    let query = search_query.get();
                                    let suggestion = current_suggestion.get();
                                    if !query.is_empty() && !suggestion.is_empty() && suggestion != query && suggestion.to_lowercase().starts_with(&query.to_lowercase()) {
                                        // Calculate the completion part
                                        let completion = suggestion[query.len()..].to_string();
                                        view! {
                                            <>
                                                <span class="text-transparent">{query.clone()}</span>
                                                <span>{completion}</span>
                                            </>
                                        }
                                    } else {
                                        view! {
                                            <>
                                                <span class="text-transparent"></span>
                                            </>
                                        }
                                    }
                                }}
                            </span>
                        </div>
                        
                        // Main input field (appears over the suggestion)
                        <input
                            type="text"
                            placeholder="Search (⌘K)"
                            class="relative w-full pl-7 sm:pl-10 pr-2 sm:pr-4 py-1 sm:py-1.5 text-xs sm:text-sm bg-transparent border-2 border-border focus:outline-none focus:ring-0 focus:border-ring text-foreground placeholder:text-muted-foreground transition-colors z-10"
                            on:input=handle_search_input
                            on:keydown=handle_search_keydown
                            prop:value=move || search_query.get()
                        />
                        
                        // Helper text for keyboard shortcuts - hidden on mobile
                        {move || {
                            let query = search_query.get();
                            let suggestion = current_suggestion.get();
                            if !query.is_empty() && !suggestion.is_empty() && suggestion != query && suggestion.to_lowercase().starts_with(&query.to_lowercase()) {
                                view! {
                                    <div class="hidden sm:block absolute -bottom-5 left-0 text-xs text-muted-foreground">
                                        "Press Tab or → to accept"
                                    </div>
                                }.into_view()
                            } else {
                                view! { <div></div> }.into_view()
                            }
                        }}
                    </div>
                </div>
                
                // Right - Settings button
                <div class="relative flex-shrink-0">
                    <button
                        class="p-1.5 sm:p-2 text-muted-foreground hover:text-foreground hover:bg-accent rounded-full transition-colors cursor-pointer"
                        on:click=move |_| {
                            let current = show_settings.get_untracked();
                            show_settings.set(!current);
                        }
                    >
                        <svg class="w-4 sm:w-5 h-4 sm:h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                        </svg>
                    </button>
                </div>
                
            </div>
        </header>
    }
}