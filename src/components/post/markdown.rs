use leptos::*;
use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag, CodeBlockKind};
use super::code::CodeRunner;

#[derive(Debug, Clone, PartialEq)]
pub enum MarkdownElement {
    Html(String),
    CodeBlock { code: String, language: String },
}

pub fn parse_markdown_elements(content: &str) -> Vec<MarkdownElement> {
    if content.trim().is_empty() {
        return vec![MarkdownElement::Html("<p>No content available.</p>".to_string())];
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(content, options);
    let mut elements = Vec::new();
    let mut current_html = String::new();
    let mut events = Vec::new();
    let mut code_block_lang = String::new();
    let mut code_block_content = String::new();
    let mut in_code_block = false;

    for event in parser {
        match &event {
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
            // Convert image tags with .mp4/.webm extensions to video tags
            Event::Start(Tag::Image(_, url, _)) => {
                if url.ends_with(".mp4") || url.ends_with(".webm") || url.ends_with(".mov") {
                    let video_html = format!(
                        r#"<video controls class="w-full my-8 border-2 border-border">
                            <source src="{}" type="video/{}">
                            Your browser does not support the video tag.
                        </video>"#,
                        url,
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
pub fn Markdown(content: String) -> impl IntoView {
    let elements = create_memo(move |_| {
        if content.is_empty() {
            vec![MarkdownElement::Html("<p>No content available.</p>".to_string())]
        } else {
            parse_markdown_elements(&content)
        }
    });

    view! {
        <div class="prose text-foreground max-w-none text-sm sm:text-base md:text-lg leading-relaxed">
            {move || elements.get().into_iter().enumerate().map(|(_i, element)| {
                match element {
                    MarkdownElement::Html(html) => view! {
                        <div inner_html=html></div>
                    }.into_view(),
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
