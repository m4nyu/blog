# blog

Markdown blog in Rust. Leptos renders every page on the server and hydrates it
in the browser as WASM; Conway's Game of Life runs in the background canvas.

- Rust only: no JavaScript sources, no Node toolchain. The only JS shipped is the
  loader `wasm-bindgen` generates.
- 100% safe Rust in this crate, read-only server, strict CSP. See [SECURITY.md](.github/SECURITY.md).

## Run

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked cargo-leptos --version 0.3.12
cargo leptos watch          # http://127.0.0.1:3000, rebuilds on change
```

## Check

```bash
cargo fmt --check
cargo clippy --features ssr --all-targets
cargo clippy --lib --features hydrate --target wasm32-unknown-unknown
cargo test --features ssr
```

## Write a post

`posts/<slug>.md`, slug in `[a-z0-9_-]`:

```markdown
---
title: Game of Life
date: 2026-11-19
excerpt: One line for the list and the meta description.
tags: [rust, wasm]
draft: false
---
Markdown body. Images and videos go in `public/assets/` and are referenced
relatively: ![board](board.png) resolves to /assets/board.png.
```

Raw HTML in posts is shown as text, not rendered.

## Deploy

```bash
docker build -t blog .
docker run --rm -p 127.0.0.1:3000:3000 --read-only --cap-drop ALL \
  --security-opt no-new-privileges blog
```

| Env | Default | |
|---|---|---|
| `LEPTOS_SITE_ADDR` | `0.0.0.0:3000` in the image | listen address |
| `BLOG_POSTS_DIR` | `/app/posts` in the image | Markdown directory |

## Layout

| Path | |
|---|---|
| `src/app.rs` | shell, routes, theme cookie, settings, CSP |
| `src/api.rs` | the two server functions |
| `src/content.rs` | front matter, Markdown rendering, URL and slug hardening (server) |
| `src/life/` | Game of Life engine (pure, tested) and canvas renderer (browser) |
| `src/ui/` | pages and components |
| `style/main.css` | the whole stylesheet |
