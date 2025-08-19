use leptos::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BadgeVariant {
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

impl BadgeVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "bg-muted text-muted-foreground border-border",
            BadgeVariant::Primary => "bg-primary/10 text-primary border-primary/20",
            BadgeVariant::Secondary => "bg-secondary text-secondary-foreground border-secondary/20",
            BadgeVariant::Success => "bg-green-100 text-green-800 border-green-200 dark:bg-green-900/20 dark:text-green-400 dark:border-green-800",
            BadgeVariant::Warning => "bg-yellow-100 text-yellow-800 border-yellow-200 dark:bg-yellow-900/20 dark:text-yellow-400 dark:border-yellow-800",
            BadgeVariant::Danger => "bg-red-100 text-red-800 border-red-200 dark:bg-red-900/20 dark:text-red-400 dark:border-red-800",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

impl BadgeSize {
    pub fn classes(&self) -> &'static str {
        match self {
            BadgeSize::Small => "px-1.5 sm:px-2 py-0.5 text-xs",
            BadgeSize::Medium => "px-2 sm:px-3 py-0.5 sm:py-1 text-xs sm:text-sm",
            BadgeSize::Large => "px-3 sm:px-4 py-1 sm:py-1.5 text-sm sm:text-base",
        }
    }
}

#[component]
pub fn Badge(
    #[prop(optional)] variant: Option<BadgeVariant>,
    #[prop(optional)] size: Option<BadgeSize>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(BadgeVariant::Default);
    let size = size.unwrap_or(BadgeSize::Medium);

    let base_classes =
        "inline-flex items-center font-medium border transition-colors hover:bg-opacity-80";
    let variant_classes = variant.classes();
    let size_classes = size.classes();
    let additional_classes = class.unwrap_or("");

    let combined_classes = format!(
        "{} {} {} {}",
        base_classes, variant_classes, size_classes, additional_classes
    );

    view! {
        <span class=combined_classes>
            {children()}
        </span>
    }
}
