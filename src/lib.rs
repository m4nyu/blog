pub mod api;
pub mod app;
pub mod life;
pub mod model;
pub mod seo;
pub mod ui;

#[cfg(feature = "ssr")]
pub mod content;

/// Browser entry point, called by the loader script after the WASM module loads.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    leptos::mount::hydrate_body(app::App);
}
