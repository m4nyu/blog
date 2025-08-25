use super::code::CodeRunner;
use leptos::*;
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};

fn resolve_asset_url(url: &str, base_path: Option<&str>) -> String {
    // If URL is already absolute (starts with http, https, or /), return as-is
    if url.starts_with("http://") || url.starts_with("https://") || url.starts_with("/") {
        return url.to_string();
    }

    // Handle relative URLs - now content is at root level and served via /assets/
    match base_path {
        Some(_base) => {
            // Since all posts are now in content/ root, relative paths are relative to content/
            format!("/assets/{}", url)
        }
        None => format!("/assets/{}", url),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MarkdownElement {
    Html(String),
    CodeBlock { code: String, language: String },
}

pub fn parse_markdown_elements(content: &str) -> Vec<MarkdownElement> {
    parse_markdown_elements_with_base(content, None)
}

pub fn parse_markdown_elements_with_base(
    content: &str,
    base_path: Option<&str>,
) -> Vec<MarkdownElement> {
    if content.trim().is_empty() {
        return vec![MarkdownElement::Html(
            "<p>No content available.</p>".to_string(),
        )];
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(content, options);
    let mut elements = Vec::new();
    let mut current_html = String::new();
    let mut events = Vec::new();
    let mut code_block_lang = String::new();
    let mut code_block_content = String::new();
    let mut in_code_block = false;

    for event in parser {
        match &event {
            Event::Html(html) => {
                // Pass HTML through directly, preserving entities
                events.push(Event::Html(html.clone()));
                continue;
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                // Flush any accumulated HTML
                if !current_html.is_empty() || !events.is_empty() {
                    html::push_html(&mut current_html, events.drain(..));
                    if !current_html.trim().is_empty() {
                        elements.push(MarkdownElement::Html(current_html.clone()));
                    }
                    current_html.clear();
                }

                in_code_block = true;
                code_block_lang = lang.to_string();
                code_block_content.clear();
                continue;
            }
            Event::Text(text) if in_code_block => {
                code_block_content.push_str(text);
                continue;
            }
            Event::End(Tag::CodeBlock(_)) if in_code_block => {
                in_code_block = false;

                // Add code block as separate element
                elements.push(MarkdownElement::CodeBlock {
                    code: code_block_content.clone(),
                    language: code_block_lang.clone(),
                });
                continue;
            }
            // Handle images and videos
            Event::Start(Tag::Image(link_type, url, title)) => {
                let resolved_url = resolve_asset_url(url, base_path);

                // Convert image tags with .mp4/.webm extensions to video tags
                if url.ends_with(".mp4") || url.ends_with(".webm") || url.ends_with(".mov") {
                    let video_html = format!(
                        r#"<video controls class="w-full my-8 border-2 border-border">
                            <source src="{}" type="video/{}">
                            Your browser does not support the video tag.
                        </video>"#,
                        resolved_url,
                        if url.ends_with(".mp4") {
                            "mp4"
                        } else if url.ends_with(".webm") {
                            "webm"
                        } else {
                            "quicktime"
                        }
                    );
                    events.push(Event::Html(CowStr::from(video_html)));
                    continue;
                } else {
                    // Handle regular images with resolved URLs
                    events.push(Event::Start(Tag::Image(
                        link_type.clone(),
                        CowStr::from(resolved_url),
                        title.clone(),
                    )));
                    continue;
                }
            }
            _ => {}
        }
        events.push(event);
    }

    // Flush remaining HTML
    if !current_html.is_empty() || !events.is_empty() {
        html::push_html(&mut current_html, events.into_iter());
        if !current_html.trim().is_empty() {
            // Process images
            current_html = current_html.replace(
                "<img ",
                r#"<img class="w-full border-2 border-border my-8" "#,
            );
            elements.push(MarkdownElement::Html(current_html));
        }
    }

    elements
}

// Keep the old function for compatibility but make it use the new system
pub fn render_markdown(content: &str) -> String {
    let elements = parse_markdown_elements(content);
    let mut html_output = String::new();

    for element in elements {
        match element {
            MarkdownElement::Html(html) => {
                html_output.push_str(&html);
            }
            MarkdownElement::CodeBlock { code, language } => {
                // For the old HTML-based system, create a simple code block
                html_output.push_str(&format!(
                    r#"<pre class="bg-muted border-2 border-border p-3 sm:p-4 md:p-6 overflow-x-auto my-6 sm:my-7 md:my-8 relative text-sm sm:text-base font-mono leading-relaxed"><code class="language-{}">{}</code></pre>"#,
                    language,
                    html_escape::encode_text(&code)
                ));
            }
        }
    }

    html_output
}

#[component]
pub fn Markdown(content: String, #[prop(optional)] base_path: Option<String>) -> impl IntoView {
    let elements = create_memo(move |_| {
        if content.is_empty() {
            vec![MarkdownElement::Html(
                "<p>No content available.</p>".to_string(),
            )]
        } else {
            parse_markdown_elements_with_base(&content, base_path.as_deref())
        }
    });

    view! {
        <div class="prose text-foreground max-w-none text-sm sm:text-base md:text-lg leading-relaxed">
            {move || elements.get().into_iter().enumerate().map(|(_i, element)| {
                match element {
                    MarkdownElement::Html(html) => {
                        // Check if this contains ASCII art or is a pre tag with ASCII art
                        let is_ascii_art = html.contains("██") || html.contains("╗") || html.contains("╔") || html.contains("╚") || html.contains("═");

                        if is_ascii_art {
                            // If it's already wrapped in a pre tag, add transparent background styles
                            let styled_html = if html.contains("<pre>") {
                                html.replace("<pre>", r#"<pre style="background: transparent !important; border: none !important; padding: 0 !important;">"#)
                            } else {
                                // Replace <br> tags with actual newlines for proper display in pre tag
                                let formatted = html.replace("<br>", "\n").replace("<br/>", "\n").replace("<br />", "\n");
                                format!(r#"<pre style="background: transparent !important; border: none !important; padding: 0 !important;">{}</pre>"#, formatted)
                            };

                            view! {
                                <div class="flex justify-center font-mono" inner_html=styled_html></div>
                            }.into_view()
                        } else {
                            view! {
                                <div inner_html=html></div>
                            }.into_view()
                        }
                    },
                    MarkdownElement::CodeBlock { code, language } => view! {
                        <CodeRunner
                            code=code
                            language=language
                            show_copy=true
                            show_run=true
                        />
                    }.into_view(),
                }
            }).collect_view()}
        </div>
    }
}
