---
title: Asset Demo Post
date: 2024-01-26T10:00:00Z
excerpt: Demonstrating how to use relative paths for images and videos in content.
tags:
  - demo
  - assets
  - markdown
---

# Asset Demo

This post demonstrates how to use relative paths for assets stored in the posts directory.

## Images

You can reference images relative to the posts directory:

![Sample Image](./images/sample-image.jpg)

Or from subdirectories:

![Another Image](./assets/another-image.png)

## Videos

Videos work the same way:

![Sample Video](./videos/sample-video.mp4)

## File Structure

With this setup, you can organize your content like this:

```
posts/
  assets-demo.md
  hello-world.md
  rust-web-development.md
  images/
    sample-image.jpg
  videos/
    sample-video.mp4
  assets/
    another-image.png
    document.pdf
```

All paths are relative to the posts directory and work seamlessly!