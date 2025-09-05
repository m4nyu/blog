<div align="center">

<pre style="background: transparent;">
██████╗ ██╗      ██████╗  ██████╗ 
██╔══██╗██║     ██╔═══██╗██╔════╝ 
██████╔╝██║     ██║   ██║██║  ███╗
██╔══██╗██║     ██║   ██║██║   ██║
██████╔╝███████╗╚██████╔╝╚██████╔╝
╚═════╝ ╚══════╝ ╚═════╝  ╚═════╝ 
</pre>
[![GitHub](https://img.shields.io/badge/GitHub-m4nyu-181717?style=flat&logo=github)](https://github.com/m4nyu)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-Manuel%20Szedlak-0A66C2?style=flat&logo=linkedin)](https://www.linkedin.com/in/manuel-szedlak)
[![X](https://img.shields.io/badge/X-ManuelSzedlak-1DA1F2?style=flat&logo=x)](https://x.com/ManuelSzedlak)

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Leptos](https://img.shields.io/badge/Leptos-0.5.7-EF3939?style=flat&logo=rust&logoColor=white)
![TailwindCSS](https://img.shields.io/badge/TailwindCSS-06B6D4?style=flat&logo=tailwindcss&logoColor=white)
![Actix Web](https://img.shields.io/badge/Actix%20Web-4.8-000000?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green?style=flat)

</div>

## ▲ Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install cargo-leptos
```

## ▶ Run

```bash
cargo leptos watch
```

## ▲ Deploy

### Deploy to Render.com

1. **Create a New Web Service on Render**
   - Connect GitHub repository
   - Choose "Docker" as the environment

2. **Configure Build & Deploy Settings**
   - **Build Command**: (uses Dockerfile automatically)
   - **Start Command**: `./target/release/tailwind`
   - **Environment**: `Docker`
   - **Instance Type**: Free or paid tier as needed

3. **Set Environment Variables** (if needed)
   ```
   LEPTOS_SITE_ADDR=0.0.0.0:3000
   RUST_LOG=info
   ```

4. **Deploy**
   - Render will automatically build and deploy the blog
   - The blog will be available at `https://[app-name].onrender.com`


## ■ Adding Posts

Create markdown files in `posts/` directory:

```markdown
---
title: "Post Title"
date: 2024-01-15
excerpt: "Brief description"
tags: ["rust", "web"]
---

# Content here

## Images
![Alt text](./image.png)
- Place image files in posts/ alongside the .md file
- Supports: png, jpg, jpeg, gif, svg, webp

## Videos  
![Video description](./video.mp4)
- Use standard image syntax for videos
- Supports: mp4, webm
- Auto-renders as video player with controls

## Code Blocks
```javascript
console.log("This is executable!");
// Click 'Run' button to execute
```

## ◆ Structure

```
posts/           # Blog posts and media
├── post-1.md
├── post-2.md
└── image.png

app/             # Blog application code
├── src/         # Rust/Leptos source code
│   └── styles/  # CSS styles (Tailwind + global.css)
└── public/      # Static assets
```

