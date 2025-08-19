#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use tailwind::app::*;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;
        let routes = &routes;

        App::new()
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(Files::new("/assets/", "content/").show_files_listing())
            .service(Files::new("/", site_root))
            .wrap(middleware::DefaultHeaders::new().add(("Cache-Control", "no-cache")))
            .wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
