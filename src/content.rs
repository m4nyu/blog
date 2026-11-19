//! Server side: reads Markdown posts with front matter from disk and renders
//! them to HTML blocks.
//!
//! Hardening: slugs are validated before any path is built, raw HTML in
//! Markdown is escaped (not rendered), and link/image URLs are limited to
//! http(s), mailto, fragments and site-relative paths.

use crate::model::{is_valid_slug, Block, Post, PostSummary};
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use std::path::{Path, PathBuf};

/// Directory holding `*.md` posts. `BLOG_POSTS_DIR`, default `posts`.
pub fn posts_dir() -> PathBuf {
    std::env::var_os("BLOG_POSTS_DIR").map_or_else(|| PathBuf::from("posts"), PathBuf::from)
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    MissingFrontMatter,
    Line(usize),
    Missing(&'static str),
    BadDate(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFrontMatter => write!(f, "missing --- front matter ---"),
            Self::Line(n) => write!(f, "front matter line {n} is not `key: value`"),
            Self::Missing(key) => write!(f, "front matter needs `{key}`"),
            Self::BadDate(d) => write!(f, "date {d:?} is not YYYY-MM-DD"),
        }
    }
}

/// A parsed file: summary, raw Markdown body, draft flag.
pub struct Source {
    pub summary: PostSummary,
    pub body: String,
    pub draft: bool,
}

/// Splits `---\n<front matter>\n---\n<markdown>`.
///
/// Front matter is a flat `key: value` list (a YAML subset): `title`, `date`
/// (required), `excerpt`, `draft: true`, and `tags` as `[a, b]`, `a, b` or
/// `- a` lines. Unknown keys are ignored.
pub fn parse(slug: &str, text: &str) -> Result<Source, ParseError> {
    let text = text
        .strip_prefix('\u{feff}')
        .unwrap_or(text)
        .replace("\r\n", "\n");
    let rest = text
        .strip_prefix("---\n")
        .ok_or(ParseError::MissingFrontMatter)?;
    let (head, body) = rest
        .split_once("\n---\n")
        .or_else(|| rest.strip_suffix("\n---").map(|h| (h, "")))
        .ok_or(ParseError::MissingFrontMatter)?;

    let (mut title, mut date, mut excerpt, mut tags, mut draft) =
        (None, None, String::new(), Vec::new(), false);
    let mut in_tag_list = false;
    for (i, raw) in head.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if in_tag_list {
            if let Some(item) = line.strip_prefix("- ") {
                tags.push(unquote(item));
                continue;
            }
            in_tag_list = false;
        }
        let (key, value) = line.split_once(':').ok_or(ParseError::Line(i + 1))?;
        let value = value.trim();
        match key.trim() {
            "title" => title = Some(unquote(value)),
            "date" => date = Some(unquote(value)),
            "excerpt" | "description" => excerpt = unquote(value),
            "draft" => draft = value == "true",
            "tags" if value.is_empty() => in_tag_list = true,
            "tags" => {
                tags = value
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(unquote)
                    .filter(|t| !t.is_empty())
                    .collect();
            }
            _ => {}
        }
    }

    let title = title
        .filter(|t| !t.is_empty())
        .ok_or(ParseError::Missing("title"))?;
    let date = date.ok_or(ParseError::Missing("date"))?;
    let date = iso_date(&date).ok_or(ParseError::BadDate(date))?;
    Ok(Source {
        summary: PostSummary {
            slug: slug.to_string(),
            title,
            date,
            excerpt,
            tags,
        },
        body: body.to_string(),
        draft,
    })
}

fn unquote(value: &str) -> String {
    let v = value.trim();
    let quoted = v.len() >= 2
        && ((v.starts_with('"') && v.ends_with('"')) || (v.starts_with('\'') && v.ends_with('\'')));
    if quoted { &v[1..v.len() - 1] } else { v }.to_string()
}

/// Accepts `YYYY-MM-DD` or an RFC 3339 timestamp; returns `YYYY-MM-DD`.
fn iso_date(value: &str) -> Option<String> {
    let day = value.get(..10)?;
    let b = day.as_bytes();
    let digits = |r: std::ops::Range<usize>| b[r].iter().all(u8::is_ascii_digit);
    (digits(0..4) && b[4] == b'-' && digits(5..7) && b[7] == b'-' && digits(8..10))
        .then(|| day.to_string())
}

async fn read_source(dir: &Path, slug: &str) -> std::io::Result<Option<String>> {
    if !is_valid_slug(slug) {
        return Ok(None);
    }
    match tokio::fs::read_to_string(dir.join(format!("{slug}.md"))).await {
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

/// All published posts, newest first. Unparseable files are logged and skipped.
pub async fn list(dir: &Path) -> std::io::Result<Vec<PostSummary>> {
    let mut posts = Vec::new();
    let mut entries = match tokio::fs::read_dir(dir).await {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(posts),
        Err(e) => return Err(e),
    };
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let Some(slug) = path
            .file_stem()
            .and_then(|s| s.to_str())
            .filter(|_| path.extension().is_some_and(|e| e == "md"))
        else {
            continue;
        };
        match read_source(dir, slug).await?.map(|text| parse(slug, &text)) {
            Some(Ok(src)) if !src.draft => posts.push(src.summary),
            Some(Err(e)) => eprintln!("skipping {}: {e}", path.display()),
            _ => {}
        }
    }
    posts.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.slug.cmp(&b.slug)));
    Ok(posts)
}

/// One published post, rendered. `None` for invalid slugs, missing files and drafts.
pub async fn load(dir: &Path, slug: &str) -> std::io::Result<Option<Post>> {
    let Some(text) = read_source(dir, slug).await? else {
        return Ok(None);
    };
    match parse(slug, &text) {
        Ok(src) if !src.draft => Ok(Some(Post {
            blocks: render(&src.body),
            summary: src.summary,
        })),
        Ok(_) => Ok(None),
        Err(e) => {
            eprintln!("cannot render {slug}: {e}");
            Ok(None)
        }
    }
}

/// Markdown to blocks: HTML runs, with fenced code blocks split out as plain
/// text. Relative image paths resolve to `/assets/`, video files become `<video>`.
pub fn render(markdown: &str) -> Vec<Block> {
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION;

    let mut blocks = Vec::new();
    let mut pending: Vec<Event<'_>> = Vec::new();
    let mut code: Option<(String, String)> = None;
    // Inside the alt text of an image that became a <video>: drop events until its end.
    let mut video_alt = false;

    let flush = |pending: &mut Vec<Event<'_>>, blocks: &mut Vec<Block>| {
        if pending.is_empty() {
            return;
        }
        let mut out = String::new();
        html::push_html(&mut out, pending.drain(..));
        if !out.trim().is_empty() {
            blocks.push(Block::Html(out));
        }
    };

    for event in Parser::new_ext(markdown, options) {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                flush(&mut pending, &mut blocks);
                let lang = match kind {
                    CodeBlockKind::Fenced(info) => info
                        .split([' ', ',', '{'])
                        .next()
                        .unwrap_or("")
                        .chars()
                        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '#'))
                        .take(20)
                        .collect(),
                    CodeBlockKind::Indented => String::new(),
                };
                code = Some((lang, String::new()));
            }
            Event::Text(text) if code.is_some() => {
                if let Some((_, buf)) = code.as_mut() {
                    buf.push_str(&text);
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((lang, code)) = code.take() {
                    blocks.push(Block::Code { lang, code });
                }
            }
            // Raw HTML is shown as text, never interpreted.
            Event::Html(raw) | Event::InlineHtml(raw) => pending.push(Event::Text(raw)),
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => pending.push(Event::Start(Tag::Link {
                link_type,
                dest_url: CowStr::from(safe_url(&dest_url)),
                title,
                id,
            })),
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                let url = safe_url(&asset_url(&dest_url));
                match video_type(&url) {
                    Some(mime) => {
                        video_alt = true;
                        pending.push(Event::Html(CowStr::from(format!(
                            r#"<video controls preload="metadata"><source src="{}" type="{mime}"></video>"#,
                            escape_attr(&url)
                        ))));
                    }
                    None => pending.push(Event::Start(Tag::Image {
                        link_type,
                        dest_url: CowStr::from(url),
                        title,
                        id,
                    })),
                }
            }
            Event::End(TagEnd::Image) if video_alt => video_alt = false,
            _ if video_alt => {}
            other => pending.push(other),
        }
    }
    flush(&mut pending, &mut blocks);
    blocks
}

/// Keeps http(s), mailto, `#fragment` and relative URLs; anything else
/// (`javascript:`, `data:`, `vbscript:`) becomes `#`.
fn safe_url(url: &str) -> String {
    let lower = url.trim().to_ascii_lowercase();
    let scheme_end = lower.find([':', '/', '?', '#']);
    let has_scheme = scheme_end.is_some_and(|i| lower.as_bytes()[i] == b':');
    let allowed = !has_scheme
        || ["http:", "https:", "mailto:"]
            .iter()
            .any(|s| lower.starts_with(s));
    if allowed {
        url.trim().to_string()
    } else {
        "#".to_string()
    }
}

fn asset_url(url: &str) -> String {
    if url.starts_with('/') || url.contains(':') {
        url.to_string()
    } else {
        format!("/assets/{}", url.trim_start_matches("./"))
    }
}

fn video_type(url: &str) -> Option<&'static str> {
    let path = url
        .split(['?', '#'])
        .next()
        .unwrap_or(url)
        .to_ascii_lowercase();
    [
        (".mp4", "video/mp4"),
        (".webm", "video/webm"),
        (".mov", "video/quicktime"),
    ]
    .into_iter()
    .find_map(|(ext, mime)| path.ends_with(ext).then_some(mime))
}

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    const POST: &str = "---\ntitle: Hello\ndate: 2026-11-19\nexcerpt: \"First: post\"\ntags: [rust, life]\n---\n# Hi\n\nText.\n";

    fn html_of(md: &str) -> String {
        render(md)
            .into_iter()
            .map(|b| match b {
                Block::Html(h) => h,
                Block::Code { code, .. } => code,
            })
            .collect()
    }

    #[test]
    fn parses_front_matter() {
        let src = parse("hello", POST).unwrap();
        assert_eq!(src.summary.title, "Hello");
        assert_eq!(src.summary.date, "2026-11-19");
        assert_eq!(src.summary.excerpt, "First: post");
        assert_eq!(src.summary.tags, ["rust", "life"]);
        assert!(!src.draft);
        assert!(src.body.starts_with("# Hi"));
    }

    #[test]
    fn front_matter_variants() {
        let text = "---\r\ntitle: 'T'\r\ndate: 2025-08-11T10:00:00Z\r\ntags:\r\n  - a\r\n  - \"b\"\r\ndraft: true\r\nauthor: me\r\n---\r\nbody";
        let src = parse("t", text).unwrap();
        assert_eq!(src.summary.date, "2025-08-11");
        assert_eq!(src.summary.tags, ["a", "b"]);
        assert!(src.draft);
        assert_eq!(src.body, "body");
        assert_eq!(
            parse("t", "---\ntitle: T\ndate: 2025-01-01\ntags: x, y\n---")
                .unwrap()
                .summary
                .tags,
            ["x", "y"]
        );
    }

    #[test]
    fn rejects_bad_front_matter() {
        assert_eq!(
            parse("x", "# none").err(),
            Some(ParseError::MissingFrontMatter)
        );
        assert_eq!(
            parse("x", "---\ntitle T\n---\n").err(),
            Some(ParseError::Line(1))
        );
        assert_eq!(
            parse("x", "---\ndate: 2026-01-01\n---\n").err(),
            Some(ParseError::Missing("title"))
        );
        assert_eq!(
            parse("x", "---\ntitle: T\n---\n").err(),
            Some(ParseError::Missing("date"))
        );
        assert!(matches!(
            parse("x", "---\ntitle: T\ndate: soon\n---\n"),
            Err(ParseError::BadDate(_))
        ));
    }

    #[test]
    fn splits_code_blocks_out() {
        let blocks = render("Intro\n\n```rust,ignore\nfn main() {}\n```\n\nOutro");
        assert_eq!(blocks.len(), 3);
        assert!(matches!(&blocks[0], Block::Html(h) if h.contains("<p>Intro</p>")));
        assert_eq!(
            blocks[1],
            Block::Code {
                lang: "rust".into(),
                code: "fn main() {}\n".into()
            }
        );
        assert!(matches!(&blocks[2], Block::Html(h) if h.contains("Outro")));
    }

    #[test]
    fn code_language_is_sanitized() {
        let blocks = render("```\"><img src=x>\nx\n```");
        assert!(
            matches!(&blocks[0], Block::Code { lang, .. } if lang == "img"),
            "{blocks:?}"
        );
    }

    #[test]
    fn raw_html_is_escaped() {
        let html = html_of("<script>alert(1)</script>\n\nhi <img src=x onerror=alert(1)> there");
        assert!(!html.contains("<script"), "{html}");
        assert!(!html.contains("<img"), "{html}");
        assert!(html.contains("&lt;script&gt;"), "{html}");
    }

    #[test]
    fn dangerous_urls_are_neutralized() {
        for url in [
            "javascript:alert(1)",
            "JaVaScRiPt:alert(1)",
            " javascript:x",
            "data:text/html,x",
            "vbscript:x",
        ] {
            let html = html_of(&format!("[x]({url}) ![y]({url})"));
            assert!(
                !html.to_ascii_lowercase().contains("script:"),
                "{url}: {html}"
            );
            assert!(!html.contains("data:"), "{url}: {html}");
        }
        let html =
            html_of("[a](https://x.dev) [b](/post/a) [c](#top) [d](mailto:me@x.dev) [e](rel/path)");
        for href in [
            "https://x.dev",
            "/post/a",
            "#top",
            "mailto:me@x.dev",
            "rel/path",
        ] {
            assert!(html.contains(&format!("href=\"{href}\"")), "{href}: {html}");
        }
    }

    #[test]
    fn resolves_assets_and_videos() {
        let html = html_of("![a](img/x.png)\n\n![b](/abs.png)\n\n![clip *x*](./v/demo.mp4) after");
        assert!(html.contains(r#"src="/assets/img/x.png""#), "{html}");
        assert!(html.contains(r#"src="/abs.png""#));
        assert!(
            html.contains(r#"<source src="/assets/v/demo.mp4" type="video/mp4">"#),
            "{html}"
        );
        assert!(!html.contains("clip"), "{html}");
        assert!(html.contains("</video> after"), "{html}");
    }

    #[tokio::test]
    async fn lists_and_loads_from_disk() {
        let dir = std::env::temp_dir().join(format!("blog-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("hello.md"), POST).unwrap();
        std::fs::write(
            dir.join("older.md"),
            POST.replace("2026-11-19", "2020-01-01"),
        )
        .unwrap();
        std::fs::write(
            dir.join("wip.md"),
            POST.replace("tags:", "draft: true\ntags:"),
        )
        .unwrap();
        std::fs::write(dir.join("broken.md"), "no front matter").unwrap();
        std::fs::write(dir.join("Bad Name.md"), POST).unwrap();
        std::fs::write(dir.join("notes.txt"), POST).unwrap();

        let slugs: Vec<_> = list(&dir)
            .await
            .unwrap()
            .into_iter()
            .map(|p| p.slug)
            .collect();
        assert_eq!(slugs, ["hello", "older"]);
        assert!(load(&dir, "hello").await.unwrap().is_some());
        for missing in ["wip", "broken", "missing", "../hello", "Bad Name"] {
            assert!(load(&dir, missing).await.unwrap().is_none(), "{missing}");
        }
        assert!(list(&dir.join("nope")).await.unwrap().is_empty());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
