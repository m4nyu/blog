---
title: Hello World - My First Blog Post
date: 2024-01-15T10:00:00Z
excerpt: Welcome to my new blog built with Leptos and Rust! In this post, I'll share my journey of building a blog with these amazing technologies.
tags:
  - rust
  - leptos
  - web-development
---

# Welcome to My Blog!

Hello and welcome to my new blog! I'm excited to share my thoughts and experiences with you. This blog is built using **Leptos**, a modern web framework for Rust that provides excellent performance and developer experience.

## Why Leptos?

I chose Leptos for several reasons:

1. **Performance**: Leptos compiles to WebAssembly, providing near-native performance
2. **Type Safety**: Rust's type system helps catch errors at compile time
3. **Reactive Programming**: Leptos provides a reactive programming model similar to SolidJS
4. **Server-Side Rendering**: Built-in SSR support for better SEO and initial load times

## Features of This Blog

This blog includes several cool features:

- **Markdown Support**: Write posts in Markdown with full syntax highlighting
- **Dark Mode**: Automatic dark mode support based on system preferences
- **Responsive Design**: Looks great on all devices
- **Fast Navigation**: Client-side routing for instant page transitions

## Code Example

Here's a simple example of a Leptos component:

```rust
#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Click me: " {count}
        </button>
    }
}
```

## What's Next?

I'll be writing about various topics including:

- Web development with Rust
- Building performant web applications
- System programming
- And much more!

Stay tuned for more posts!