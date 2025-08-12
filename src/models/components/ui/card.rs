use leptos::*;

#[component]
pub fn Card(#[prop(optional)] class: Option<&'static str>, children: Children) -> impl IntoView {
    let base_class = "border bg-card text-card-foreground shadow-sm";
    let class = class
        .map(|c| format!("{} {}", base_class, c))
        .unwrap_or_else(|| base_class.to_string());

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base_class = "flex flex-col space-y-1 sm:space-y-1.5 p-3 sm:p-4 md:p-6";
    let class = class
        .map(|c| format!("{} {}", base_class, c))
        .unwrap_or_else(|| base_class.to_string());

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base_class = "text-base sm:text-lg md:text-2xl font-semibold leading-none tracking-tight";
    let class = class
        .map(|c| format!("{} {}", base_class, c))
        .unwrap_or_else(|| base_class.to_string());

    view! {
        <h3 class=class>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescription(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base_class = "text-xs sm:text-sm text-muted-foreground";
    let class = class
        .map(|c| format!("{} {}", base_class, c))
        .unwrap_or_else(|| base_class.to_string());

    view! {
        <p class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base_class = "p-3 sm:p-4 md:p-6 pt-0";
    let class = class
        .map(|c| format!("{} {}", base_class, c))
        .unwrap_or_else(|| base_class.to_string());

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base_class = "flex items-center p-3 sm:p-4 md:p-6 pt-0";
    let class = class
        .map(|c| format!("{} {}", base_class, c))
        .unwrap_or_else(|| base_class.to_string());

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardAction(
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base_class = "flex items-center";
    let class = class
        .map(|c| format!("{} {}", base_class, c))
        .unwrap_or_else(|| base_class.to_string());

    view! {
        <div class=class>
            {children()}
        </div>
    }
}
