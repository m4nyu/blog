use crate::ui::{
    engineer::{EngineerHome, EngineerPost},
    home::HomePage,
    not_found::NotFound,
    post::PostPage,
    settings::{SettingsButton, SettingsDialog},
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, HashedStylesheet, Link, Meta, MetaTags};
use leptos_router::{
    components::{Route, Router, Routes},
    path, SsrMode,
};

/// Colour scheme. Stored in the `theme` cookie so the server renders the
/// right class on `<html>` and the page never flashes the wrong theme.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl Theme {
    pub const ALL: [Theme; 3] = [Theme::Light, Theme::Dark, Theme::System];

    pub fn parse(value: &str) -> Self {
        match value {
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::System,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }

    /// Class on `<html>`. System mode has none: CSS follows `prefers-color-scheme`.
    pub fn class(self) -> &'static str {
        match self {
            Self::System => "",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    /// Favicon: the adaptive SVG follows the OS; an explicit theme pins a variant.
    pub fn favicon(self) -> &'static str {
        match self {
            Self::System => "/icon.svg",
            Self::Light => "/icon-light.svg",
            Self::Dark => "/icon-dark.svg",
        }
    }

    /// Reads `theme=` from a `Cookie` header value.
    pub fn from_cookies(header: &str) -> Self {
        header
            .split(';')
            .find_map(|pair| pair.trim().strip_prefix("theme="))
            .map_or(Self::System, Self::parse)
    }

    /// The theme of the current request (server) or document (browser).
    pub fn current() -> Self {
        #[cfg(feature = "ssr")]
        {
            use_context::<http::request::Parts>()
                .and_then(|parts| {
                    let header = parts.headers.get(http::header::COOKIE)?;
                    header.to_str().ok().map(Self::from_cookies)
                })
                .unwrap_or_default()
        }
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsCast;
            document()
                .dyn_into::<web_sys::HtmlDocument>()
                .ok()
                .and_then(|d| d.cookie().ok())
                .map_or(Self::System, |c| Self::from_cookies(&c))
        }
        #[cfg(not(any(feature = "ssr", feature = "hydrate")))]
        {
            Self::System
        }
    }
}

pub const DEFAULT_SPEED_MS: u32 = 100;
pub const DEFAULT_DENSITY: f64 = 0.08;
pub const MAX_DENSITY: f64 = 0.25;

/// User settings, shared through context.
#[derive(Clone, Copy)]
pub struct Settings {
    /// Milliseconds per generation.
    pub speed: RwSignal<u32>,
    /// Initial share of live cells, 0.01 to 0.25.
    pub density: RwSignal<f64>,
    pub theme: RwSignal<Theme>,
    pub open: RwSignal<bool>,
}

impl Settings {
    fn new() -> Self {
        let (speed, density) = stored_life_settings();
        Self {
            speed: RwSignal::new(speed),
            density: RwSignal::new(density),
            theme: RwSignal::new(Theme::current()),
            open: RwSignal::new(false),
        }
    }
}

/// Speed and density from localStorage. Only the canvas depends on them, never
/// server-rendered markup, so reading them during hydration is safe.
fn stored_life_settings() -> (u32, f64) {
    #[cfg(feature = "hydrate")]
    {
        let storage = window().local_storage().ok().flatten();
        let get = |key: &str| {
            storage
                .as_ref()
                .and_then(|s| s.get_item(key).ok().flatten())
        };
        let speed = get("life-speed")
            .and_then(|v| v.parse().ok())
            .filter(|v| (50..=500).contains(v))
            .unwrap_or(DEFAULT_SPEED_MS);
        let density = get("life-density")
            .and_then(|v| v.parse().ok())
            .filter(|v: &f64| (0.01..=MAX_DENSITY).contains(v))
            .unwrap_or(DEFAULT_DENSITY);
        (speed, density)
    }
    #[cfg(not(feature = "hydrate"))]
    {
        (DEFAULT_SPEED_MS, DEFAULT_DENSITY)
    }
}

/// Strict CSP for one HTML response. Scripts only run with this response's
/// nonce (Leptos puts it on its loader script); `strict-dynamic` lets that
/// script import the hashed `/pkg/*.js`, `wasm-unsafe-eval` lets it compile
/// the WASM. No inline scripts, styles, plugins, frames or form posts.
#[cfg(feature = "ssr")]
fn content_security_policy() {
    let Some(res) = use_context::<leptos_axum::ResponseOptions>() else {
        return;
    };
    let nonce = leptos::nonce::use_nonce()
        .map(|n| n.to_string())
        .unwrap_or_default();
    // `cargo leptos watch` live reload talks over a websocket on another port.
    let dev_ws = if std::env::var_os("LEPTOS_WATCH").is_some() {
        " ws:"
    } else {
        ""
    };
    let policy = format!(
        "default-src 'none'; script-src 'nonce-{nonce}' 'strict-dynamic' 'wasm-unsafe-eval'; \
         style-src 'self'; img-src 'self' https:; media-src 'self' https:; \
         connect-src 'self'{dev_ws}; manifest-src 'self'; base-uri 'none'; \
         form-action 'none'; frame-ancestors 'none'"
    );
    if let Ok(value) = http::HeaderValue::from_str(&policy) {
        res.insert_header(http::header::CONTENT_SECURITY_POLICY, value);
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    #[cfg(feature = "ssr")]
    content_security_policy();
    let theme = Theme::current();
    view! {
        <!DOCTYPE html>
        <html lang="en" class=theme.class()>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone() defer=true/>
                <HashedStylesheet options id="leptos"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let settings = Settings::new();
    provide_context(settings);

    #[cfg(feature = "hydrate")]
    persist(settings);

    // Escape closes the settings dialog wherever focus is.
    #[cfg(feature = "hydrate")]
    {
        let handle = window_event_listener(leptos::ev::keydown, move |e| {
            if e.key() == "Escape" && settings.open.get_untracked() {
                settings.open.set(false);
            }
        });
        on_cleanup(move || handle.remove());
    }

    view! {
        <Meta name="color-scheme" content="light dark"/>
        <Link id="favicon" rel="icon" type_="image/svg+xml" href=settings.theme.get_untracked().favicon()/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png"/>
        <Link rel="apple-touch-icon" href="/favicon-192x192.png"/>
        <Link rel="manifest" href="/manifest.json"/>
        <Link rel="alternate" type_="application/atom+xml" title="m4nuel.blog" href="/feed.xml"/>
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=HomePage ssr=SsrMode::Async/>
                <Route path=path!("/post/:slug") view=PostPage ssr=SsrMode::Async/>
                <Route path=path!("/engineer") view=EngineerHome ssr=SsrMode::Async/>
                <Route path=path!("/engineer/:slug") view=EngineerPost ssr=SsrMode::Async/>
            </Routes>
        </Router>
        <SettingsButton/>
        <SettingsDialog/>
    }
}

/// Writes settings back: theme to cookie + `<html>` class, the rest to localStorage.
#[cfg(feature = "hydrate")]
fn persist(settings: Settings) {
    use wasm_bindgen::JsCast;

    Effect::new(move |_| {
        let theme = settings.theme.get();
        if let Some(root) = document().document_element() {
            root.set_class_name(theme.class());
        }
        if let Some(icon) = document().get_element_by_id("favicon") {
            let _ = icon.set_attribute("href", theme.favicon());
        }
        if let Ok(doc) = document().dyn_into::<web_sys::HtmlDocument>() {
            let _ = doc.set_cookie(&format!(
                "theme={}; Path=/; Max-Age=31536000; SameSite=Lax",
                theme.as_str()
            ));
        }
    });

    let store = |key: &'static str, value: String| {
        if let Some(storage) = window().local_storage().ok().flatten() {
            let _ = storage.set_item(key, &value);
        }
    };
    Effect::new(move |_| store("life-speed", settings.speed.get().to_string()));
    Effect::new(move |_| store("life-density", settings.density.get().to_string()));
}

/// Marks the server response 404. No-op in the browser.
pub fn set_not_found() {
    #[cfg(feature = "ssr")]
    if let Some(res) = use_context::<leptos_axum::ResponseOptions>() {
        res.set_status(http::StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
mod tests {
    use super::Theme;

    #[test]
    fn theme_from_cookies() {
        assert_eq!(Theme::from_cookies("a=1; theme=dark; b=2"), Theme::Dark);
        assert_eq!(Theme::from_cookies("theme=light"), Theme::Light);
        assert_eq!(Theme::from_cookies("theme=neon"), Theme::System);
        assert_eq!(Theme::from_cookies("xtheme=dark"), Theme::System);
        assert_eq!(Theme::from_cookies(""), Theme::System);
    }

    #[test]
    fn theme_round_trips() {
        for t in Theme::ALL {
            assert_eq!(Theme::parse(t.as_str()), t);
        }
    }
}
