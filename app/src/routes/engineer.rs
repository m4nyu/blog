use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::post::markdown::render_markdown;
use crate::components::ui::life::Life;
use crate::routes::home::get_posts;
use crate::routes::post::get_post;

#[component]
pub fn EngineerHome() -> impl IntoView {
    let posts = create_resource(|| (), |_| async { get_posts().await });
    let animation_speed =
        use_context::<RwSignal<u64>>().unwrap_or_else(|| RwSignal::new(100u64));
    let population_density =
        use_context::<RwSignal<f64>>().unwrap_or_else(|| RwSignal::new(0.08f64));

    view! {
        <Title text="blog [engineer mode]"/>
        <Life animation_speed=animation_speed population_density=population_density />
        <div style="position:relative;z-index:1;padding:1em;max-width:72ch;margin:0 auto;background:rgba(255,255,255,0.92)">
            <h1>"blog"</h1>
            <pre>"mode: engineer | styling: none | runtime: wasm"</pre>
            <hr/>
            <a href="/">"[normal mode]"</a>
            <hr/>
            <Suspense fallback=move || view! { <pre>"loading posts..."</pre> }>
                {move || {
                    posts.get().map(|result| match result {
                        Ok(posts) => {
                            view! {
                                <ul>
                                    {posts.into_iter().map(|post| {
                                        let url = format!("/engineer/{}", post.slug);
                                        let date = post.date.format("%Y-%m-%d").to_string();
                                        view! {
                                            <li>
                                                <a href=url>{&post.title}</a>
                                                " ("{date}")"
                                                <br/>
                                                <small>{&post.excerpt}</small>
                                            </li>
                                        }
                                    }).collect_view()}
                                </ul>
                            }.into_view()
                        }
                        Err(e) => view! { <pre>"error: " {e.to_string()}</pre> }.into_view(),
                    })
                }}
            </Suspense>
            <hr/>
            <small>"game of life running in background canvas"</small>
        </div>
    }
}

#[component]
pub fn EngineerPost() -> impl IntoView {
    let params = use_params_map();
    let slug =
        move || params.with(|params| params.get("slug").cloned().unwrap_or_default());

    let post = create_resource(slug, |slug| async move { get_post(slug).await });

    let animation_speed =
        use_context::<RwSignal<u64>>().unwrap_or_else(|| RwSignal::new(100u64));
    let population_density =
        use_context::<RwSignal<f64>>().unwrap_or_else(|| RwSignal::new(0.08f64));

    view! {
        <Life animation_speed=animation_speed population_density=population_density />
        <div style="position:relative;z-index:1;padding:1em;max-width:72ch;margin:0 auto;background:rgba(255,255,255,0.92)">
            <a href="/engineer">"[back]"</a>
            <hr/>
            <Suspense fallback=move || view! { <pre>"loading..."</pre> }>
                {move || {
                    post.get().map(|result| match result {
                        Ok(Some(post)) => {
                            let html = render_markdown(&post.content);
                            let tags = post.tags.join(", ");
                            view! {
                                <Title text=format!("{} [engineer]", post.title)/>
                                <article>
                                    <h1>{&post.title}</h1>
                                    <pre>
                                        "date:  " {post.date.format("%Y-%m-%d %H:%M").to_string()} "\n"
                                        "tags:  " {tags} "\n"
                                        "views: " {post.metrics.views.to_string()}
                                    </pre>
                                    <blockquote>{&post.excerpt}</blockquote>
                                    <hr/>
                                    <div inner_html=html></div>
                                </article>
                            }.into_view()
                        }
                        Ok(None) => view! {
                            <Title text="404 [engineer]"/>
                            <h1>"404"</h1>
                            <pre>"post not found"</pre>
                        }.into_view(),
                        Err(e) => view! {
                            <pre>"error: " {e.to_string()}</pre>
                        }.into_view(),
                    })
                }}
            </Suspense>
        </div>
    }
}
