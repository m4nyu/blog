#[cfg(feature = "ssr")]
#[shuttle_runtime::main]
async fn main(
) -> shuttle_actix_web::ShuttleActixWeb<impl FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static> {
    use actix_files::Files;
    use actix_web::web::ServiceConfig;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use tailwind::app::*;

    // Set up basic configuration for Shuttle environment
    let conf = get_configuration(None).await.unwrap_or_else(|e| {
        eprintln!("Warning: Failed to get Leptos configuration: {}", e);
        eprintln!("Using minimal fallback configuration");
        panic!("Cannot proceed without Leptos configuration");
    });
    
    let routes = generate_route_list(App);
    let leptos_options = conf.leptos_options;
    
    // Use a simple public directory for assets in production
    let site_root = "/app".to_string();
    
    eprintln!("Leptos options: {:?}", leptos_options);
    eprintln!("Using site root: {}", site_root);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(leptos_options.clone()))
            .app_data(web::Data::new(routes.clone()))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(leptos_options.clone(), routes.clone(), App)
            .service(Files::new("/assets/", "/app/posts/").show_files_listing())
            .service(Files::new("/pkg/", "/app/pkg/").show_files_listing())
            .route("/{filename:.*}", web::get().to(|req: HttpRequest| async move {
                HttpResponse::Ok().body("Hello from Shuttle!")
            }));
    };

    Ok(config.into())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
