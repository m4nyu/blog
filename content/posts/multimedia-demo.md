---
title: Multimedia Content in Markdown
date: 2024-01-25T16:00:00Z
excerpt: Demonstrating how this blog handles images, videos, and rich content in markdown files.
tags:
  - multimedia
  - markdown
  - demo
---

# Rich Content Support

This blog supports all kinds of multimedia content directly in markdown files. You can embed images, videos, code blocks, tables, and more!

## Images

Images work just like in regular markdown. They'll be automatically styled with rounded corners and shadows:

![A beautiful landscape](https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=800&q=80)

## Videos

You can embed videos by using the image syntax with video file extensions (.mp4, .webm, .mov):

<!-- ![Sample Video](/videos/sample.mp4) -->
*Note: Video functionality is demonstrated in the markdown parser - it converts .mp4/.webm/.mov image syntax to video tags*

## Code Blocks

Code blocks support syntax highlighting:

```rust
fn main() {
    println!("Hello from Rust!");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    
    println!("Sum: {}", sum);
}
```

```javascript
// JavaScript example
const greet = (name) => {
    console.log(`Hello, ${name}!`);
};

greet('World');
```

## Tables

Tables are fully supported with proper styling:

| Feature | Supported | Notes |
|---------|-----------|-------|
| Images | ✅ | Auto-styled with Tailwind |
| Videos | ✅ | .mp4, .webm, .mov |
| Code | ✅ | Syntax highlighting |
| Tables | ✅ | Responsive design |
| Lists | ✅ | Ordered and unordered |

## Lists

### Unordered Lists
- First item
- Second item
  - Nested item
  - Another nested item
- Third item

### Ordered Lists
1. First step
2. Second step
   1. Sub-step A
   2. Sub-step B
3. Third step

## Task Lists

- [x] Completed task
- [ ] Pending task
- [x] Another completed task

## Blockquotes

> "The best way to predict the future is to invent it."
> 
> — Alan Kay

## Inline Elements

You can use **bold text**, *italic text*, ~~strikethrough~~, and `inline code`.

## Links

Check out [Leptos](https://leptos.dev) for more information about the framework powering this blog.

## Horizontal Rule

---

## HTML Support

You can even include raw HTML when needed:

<div class="bg-blue-100 dark:bg-blue-900 p-4 rounded-lg my-4">
  <p class="text-blue-900 dark:text-blue-100">
    This is a custom HTML block with Tailwind classes!
  </p>
</div>

## Conclusion

As you can see, this blog supports a wide variety of content types, making it perfect for technical writing, tutorials, and creative content!