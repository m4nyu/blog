//! Engineer mode: the same posts as plain HTML, browser default styling.

use crate::api::{get_post, list_posts};
use crate::app::set_not_found;
use crate::life::view::Life;
use crate::model::Block;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

#[component]
pub fn EngineerHome() -> impl IntoView {
    let posts = Resource::new(|| (), |_| list_posts());
    view! {
        <Title text="blog [engineer]"/>
        <Life/>
        <div class="engineer">
            <h1>"blog"</h1>
            <pre>"mode: engineer | styling: none | runtime: wasm"</pre>
            <p>
                <a href="/">"[normal mode]"</a>
            </p>
            <hr/>
            <Suspense fallback=|| view! { <pre>"loading..."</pre> }>
                {move || Suspend::new(async move {
                    match posts.await {
                        Ok(posts) if posts.is_empty() => view! { <p>"no posts"</p> }.into_any(),
                        Ok(posts) => {
                            view! {
                                <ul>
                                    {posts
                                        .into_iter()
                                        .map(|p| {
                                            view! {
                                                <li>
                                                    <a href=format!("/engineer/{}", p.slug)>{p.title}</a>
                                                    " (" {p.date} ")"
                                                    <br/>
                                                    <small>{p.excerpt}</small>
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            }
                                .into_any()
                        }
                        Err(e) => view! { <pre>"error: " {e.to_string()}</pre> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
pub fn EngineerPost() -> impl IntoView {
    let params = use_params_map();
    let post = Resource::new(
        move || params.read().get("slug").unwrap_or_default(),
        get_post,
    );
    view! {
        <Life/>
        <div class="engineer">
            <a href="/engineer">"[back]"</a>
            <hr/>
            <Suspense fallback=|| view! { <pre>"loading..."</pre> }>
                {move || Suspend::new(async move {
                    match post.await {
                        Ok(Some(post)) => {
                            let s = post.summary;
                            view! {
                                <Title text=format!("{} [engineer]", s.title)/>
                                <article>
                                    <h1>{s.title}</h1>
                                    <pre>"date: " {s.date} "\ntags: " {s.tags.join(", ")}</pre>
                                    <blockquote>{s.excerpt}</blockquote>
                                    <hr/>
                                    {post
                                        .blocks
                                        .into_iter()
                                        .map(|block| match block {
                                            Block::Html(html) => view! { <div inner_html=html></div> }.into_any(),
                                            Block::Code { code, .. } => view! { <pre><code>{code}</code></pre> }.into_any(),
                                        })
                                        .collect_view()}
                                </article>
                            }
                                .into_any()
                        }
                        Ok(None) => {
                            set_not_found();
                            view! {
                                <Title text="404 [engineer]"/>
                                <h1>"404"</h1>
                                <pre>"post not found"</pre>
                            }
                                .into_any()
                        }
                        Err(e) => view! { <pre>"error: " {e.to_string()}</pre> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
