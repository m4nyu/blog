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

    let conf = get_configuration(None).await.unwrap();
    let routes = generate_route_list(App);
    let leptos_options = conf.leptos_options;
    let site_root = leptos_options.site_root.clone();

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(leptos_options.clone()))
            .app_data(web::Data::new(routes.clone()))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(leptos_options.clone(), routes.clone(), App)
            .service(Files::new("/assets/", "posts/").show_files_listing())
            .service(Files::new("/", &site_root));
    };

    Ok(config.into())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
