use crate::app::set_not_found;
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFound() -> impl IntoView {
    set_not_found();
    view! {
        <Title text="not found"/>
        <div class="site">
            <main class="page">
                <h1>"404"</h1>
                <p>"Nothing here."</p>
                <p>
                    <a class="back" href="/">"← all posts"</a>
                </p>
            </main>
        </div>
    }
}
