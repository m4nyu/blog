use leptos::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Destructive => {
                "bg-destructive text-destructive-foreground hover:bg-destructive/90"
            }
            ButtonVariant::Outline => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            }
            ButtonVariant::Secondary => {
                "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] onclick: Option<Box<dyn Fn() + 'static>>,
    #[prop(optional)] onclick_with_event: Option<Box<dyn Fn(ev::MouseEvent) + 'static>>,
    #[prop(optional)] href: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(ButtonVariant::Default);
    let base_class = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2";
    let variant_class = variant.as_str();
    let full_class = format!("{} {}", base_class, variant_class);
    let class = class
        .map(|c| format!("{} {}", full_class, c))
        .unwrap_or(full_class);

    if let Some(href) = href {
        view! {
            <a href=href class=class>
                {children()}
            </a>
        }
        .into_view()
    } else {
        view! {
            <button
                class=class
                on:click=move |ev| {
                    if let Some(ref onclick_with_event) = onclick_with_event {
                        onclick_with_event(ev);
                    } else if let Some(ref onclick) = onclick {
                        onclick();
                    }
                }
            >
                {children()}
            </button>
        }
        .into_view()
    }
}
