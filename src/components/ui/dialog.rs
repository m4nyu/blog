use leptos::*;

#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
#[cfg(feature = "hydrate")]
use web_sys;

#[component]
pub fn Dialog(
    show_settings: RwSignal<bool>,
    animation_speed: RwSignal<u64>,
    population_density: RwSignal<f64>,
    theme_mode: RwSignal<String>,
    on_theme_change: Callback<String>,
) -> impl IntoView {
    // Handle animation speed changes
    #[cfg(feature = "hydrate")]
    let handle_speed_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input = target.unchecked_into::<web_sys::HtmlInputElement>();
        if let Ok(speed) = input.value().parse::<u64>() {
            animation_speed.set(speed);
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let handle_speed_change = move |_ev| {};

    // Handle density changes
    #[cfg(feature = "hydrate")]
    let handle_density_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input = target.unchecked_into::<web_sys::HtmlInputElement>();
        if let Ok(density) = input.value().parse::<f64>() {
            // Clamp density to maximum 25% and convert percentage to decimal
            let clamped_density = (density / 100.0).min(0.25);
            population_density.set(clamped_density);
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let handle_density_change = move |_ev| {};

    view! {
        <Show when=move || show_settings.get() fallback=|| ()>
            <div class="fixed inset-0 z-[100]">
                // Full-screen backdrop with blur
                <div
                    class="absolute inset-0 backdrop-blur-sm"
                    on:click=move |_| {
                        show_settings.set(false);
                    }
                ></div>

                // Semi-transparent overlay for darkening
                <div class="absolute inset-0 bg-black/50 pointer-events-none"></div>

                // Dialog - absolutely centered in full viewport
                <div
                    class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-popover shadow-xl border border-border p-3 sm:p-4 w-[85vw] sm:w-96 max-w-[90vw]"
                    on:click=move |e| {
                        e.stop_propagation();
                    }
                >
                    <div class="flex items-center justify-between mb-3 sm:mb-4">
                        <h3 class="text-base sm:text-lg font-semibold text-popover-foreground">Settings</h3>
                        <button
                            class="text-muted-foreground hover:text-foreground cursor-pointer"
                            on:click=move |_| {
                                show_settings.set(false);
                            }
                        >
                            <svg class="w-4 sm:w-5 h-4 sm:h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                            </svg>
                        </button>
                    </div>

                    <div class="space-y-4 sm:space-y-6">
                        // Animation Speed Control
                        <div>
                            <label class="block text-xs sm:text-sm font-medium text-popover-foreground mb-1.5 sm:mb-2">
                                Animation Speed
                            </label>
                            <div class="space-y-1.5 sm:space-y-2">
                                <div class="relative">
                                    <input
                                        type="range"
                                        min="50"
                                        max="500"
                                        step="50"
                                        class="w-full range-slider"
                                        prop:value=move || animation_speed.get().to_string()
                                        on:input=handle_speed_change
                                    />
                                </div>
                                <div class="flex justify-between text-[10px] sm:text-xs text-muted-foreground">
                                    <span>Fast</span>
                                    <span class="font-medium">{move || format!("{}ms", animation_speed.get())}</span>
                                    <span>Slow</span>
                                </div>
                            </div>
                        </div>

                        // Population Density Control
                        <div>
                            <label class="block text-xs sm:text-sm font-medium text-popover-foreground mb-1.5 sm:mb-2">
                                Population Density
                            </label>
                            <div class="space-y-1.5 sm:space-y-2">
                                <div class="relative">
                                    <input
                                        type="range"
                                        min="1"
                                        max="25"
                                        step="1"
                                        class="w-full range-slider"
                                        prop:value=move || (population_density.get() * 100.0).round().to_string()
                                        on:input=handle_density_change
                                        on:change=handle_density_change
                                    />
                                </div>
                                <div class="flex justify-between text-[10px] sm:text-xs text-muted-foreground">
                                    <span>Sparse</span>
                                    <span class="font-medium">{move || format!("{}%", (population_density.get() * 100.0).round())}</span>
                                    <span>Dense</span>
                                </div>
                            </div>
                        </div>

                        // Theme Mode Control
                        <div>
                            <label class="block text-xs sm:text-sm font-medium text-popover-foreground mb-1.5 sm:mb-2">
                                Theme
                            </label>
                            <div class="grid grid-cols-3 gap-1">
                                <button
                                    class=move || format!(
                                        "px-2 sm:px-3 py-1.5 sm:py-2 text-[10px] sm:text-xs font-medium border transition-colors cursor-pointer {}",
                                        if theme_mode.get() == "light" {
                                            "bg-foreground text-background border-foreground"
                                        } else {
                                            "bg-transparent text-foreground border-border hover:bg-muted"
                                        }
                                    )
                                    on:click=move |_| on_theme_change.call("light".to_string())
                                >
                                    Light
                                </button>
                                <button
                                    class=move || format!(
                                        "px-2 sm:px-3 py-1.5 sm:py-2 text-[10px] sm:text-xs font-medium border transition-colors cursor-pointer {}",
                                        if theme_mode.get() == "dark" {
                                            "bg-foreground text-background border-foreground"
                                        } else {
                                            "bg-transparent text-foreground border-border hover:bg-muted"
                                        }
                                    )
                                    on:click=move |_| on_theme_change.call("dark".to_string())
                                >
                                    Dark
                                </button>
                                <button
                                    class=move || format!(
                                        "px-2 sm:px-3 py-1.5 sm:py-2 text-[10px] sm:text-xs font-medium border transition-colors cursor-pointer {}",
                                        if theme_mode.get() == "system" {
                                            "bg-foreground text-background border-foreground"
                                        } else {
                                            "bg-transparent text-foreground border-border hover:bg-muted"
                                        }
                                    )
                                    on:click=move |_| on_theme_change.call("system".to_string())
                                >
                                    System
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
