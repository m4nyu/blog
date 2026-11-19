#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use blog::app::{shell, App};
    use http::{header, HeaderName, HeaderValue};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::set_header::SetResponseHeaderLayer;

    let conf = get_configuration(None).expect("invalid Leptos configuration");
    let options = conf.leptos_options;
    let addr = options.site_addr;
    let routes = generate_route_list(App);

    // Sent with every response. The Content-Security-Policy is set per page in
    // `shell`, because it carries that response's nonce.
    let headers = [
        (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        (header::X_FRAME_OPTIONS, "DENY"),
        (header::REFERRER_POLICY, "strict-origin-when-cross-origin"),
        (
            header::STRICT_TRANSPORT_SECURITY,
            "max-age=63072000; includeSubDomains",
        ),
        (
            HeaderName::from_static("cross-origin-opener-policy"),
            "same-origin",
        ),
        (
            HeaderName::from_static("cross-origin-resource-policy"),
            "same-origin",
        ),
        (
            HeaderName::from_static("permissions-policy"),
            "camera=(), microphone=(), geolocation=(), payment=(), usb=(), interest-cohort=()",
        ),
    ];

    use blog::seo::routes as seo;
    let mut app = Router::new()
        .route("/sitemap.xml", axum::routing::get(seo::sitemap))
        .route("/feed.xml", axum::routing::get(seo::feed))
        .route("/llms.txt", axum::routing::get(seo::llms))
        .route("/llms-full.txt", axum::routing::get(seo::llms_full))
        .leptos_routes(&options, routes, {
            let options = options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(options);
    for (name, value) in headers {
        app = app.layer(SetResponseHeaderLayer::overriding(
            name,
            HeaderValue::from_static(value),
        ));
    }
    // Brotli or gzip, as the client accepts; shrinks the WASM to about a third.
    let app = app.layer(tower_http::compression::CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("cannot bind address");
    println!("listening on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

/// Ctrl-C locally, SIGTERM from `docker stop`.
#[cfg(feature = "ssr")]
async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut s) => {
                s.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}

#[cfg(not(feature = "ssr"))]
fn main() {}
