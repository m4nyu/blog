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

### Prerequisites (one-time setup)
```bash
# Install Pulumi CLI
curl -fsSL https://get.pulumi.com | sh

# Configure AWS
aws configure

# Initialize Pulumi stack
pulumi stack init production
```

### Deploy to AWS (Multi-Region + CloudFront)
```bash
# Single command deployment
bun run deploy
```

This will:
- ✅ Build your Leptos blog
- ✅ Deploy to US West & EU West regions
- ✅ Set up global CloudFront CDN
- ✅ Provide enterprise-grade security

### Optional: Add Custom Domain + Cloudflare Security
```bash
pulumi config set blog:domain yourdomain.com
pulumi config set cloudflare:zoneId YOUR_ZONE_ID
pulumi config set cloudflare:apiToken YOUR_TOKEN --secret
bun run deploy
```

### Individual Operations
```bash
bun run preview      # Preview infrastructure changes
bun run sync         # Sync files to S3 buckets  
bun run invalidate   # Clear CloudFront cache
```

## ■ Adding Posts

Create markdown files in `content/` directory:

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
- Place image files in content/ alongside the .md file
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
content/         # Your blog posts and media
├── post-1.md
├── post-2.md
└── image.png

src/             # Blog engine (Rust/Leptos)
public/          # Static assets
```

