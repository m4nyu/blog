use crate::api::get_post;
use crate::model::{display_date, Block, Post};
use crate::ui::copy::{copy_text, ShareButton};
use crate::ui::not_found::NotFound;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params_map;

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();
    let post = Resource::new(
        move || params.read().get("slug").unwrap_or_default(),
        get_post,
    );

    view! {
        <Suspense fallback=|| ()>
            {move || Suspend::new(async move {
                match post.await {
                    Ok(Some(post)) => view! { <Article post/> }.into_any(),
                    Ok(None) => view! { <NotFound/> }.into_any(),
                    Err(e) => {
                        view! {
                            <div class="site">
                                <main class="page">
                                    <p class="error">{e.to_string()}</p>
                                </main>
                            </div>
                        }
                            .into_any()
                    }
                }
            })}
        </Suspense>
    }
}

#[component]
fn Article(post: Post) -> impl IntoView {
    let Post { summary, blocks } = post;
    let date = display_date(&summary.date);
    view! {
        <Title text=summary.title.clone()/>
        <Meta name="description" content=summary.excerpt.clone()/>
        <div class="site">
            <main class="page post">
                <article>
                    <a class="back" href="/">"← all posts"</a>
                    <header class="post-head">
                        <h1>{summary.title}</h1>
                        <p class="meta">
                            <time datetime=summary.date>{date}</time>
                        </p>
                        {(!summary.excerpt.is_empty())
                            .then(|| view! { <p class="excerpt">{summary.excerpt}</p> })}
                        <ul class="tags">
                            {summary.tags.into_iter().map(|t| view! { <li>"#" {t}</li> }).collect_view()}
                        </ul>
                    </header>
                    <div class="prose">
                        {blocks
                            .into_iter()
                            .map(|block| match block {
                                Block::Html(html) => view! { <div inner_html=html></div> }.into_any(),
                                Block::Code { lang, code } => view! { <CodeBlock lang code/> }.into_any(),
                            })
                            .collect_view()}
                    </div>
                    <footer class="post-foot">
                        <ShareButton slug=summary.slug/>
                    </footer>
                </article>
            </main>
        </div>
    }
}

#[component]
fn CodeBlock(lang: String, code: String) -> impl IntoView {
    let copied = RwSignal::new(false);
    let label = if lang.is_empty() {
        "text".to_string()
    } else {
        lang
    };
    let text = code.clone();
    view! {
        <figure class="code">
            <figcaption>
                <span>{label}</span>
                <button
                    type="button"
                    class="ghost-button"
                    aria-label="Copy code"
                    on:click=move |_| copy_text(text.clone(), copied)
                >
                    {move || if copied.get() { "copied" } else { "copy" }}
                </button>
            </figcaption>
            <pre>
                <code>{code}</code>
            </pre>
        </figure>
    }
}
