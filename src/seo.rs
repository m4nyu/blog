//! Search and AI discoverability, derived from the posts with no manual step:
//! canonical URLs, Open Graph / Twitter cards and JSON-LD on every page; a
//! sitemap, an Atom feed, `llms.txt` and `llms-full.txt` from the server.

use crate::model::PostSummary;
use leptos::prelude::*;
use leptos::serde_json;
use leptos_meta::{Link, Meta, Script};

pub const SITE_URL: &str = "https://m4nuel.blog";
pub const SITE_NAME: &str = "m4nuel.blog";
pub const DESCRIPTION: &str = "Notes on software by Manuel Szedlak, written and served in Rust.";
pub const AUTHOR: &str = "Manuel Szedlak";
pub const AUTHOR_URL: &str = "https://m4nuel.net";
pub const OG_IMAGE: &str = "https://m4nuel.blog/og.png";

pub fn post_url(slug: &str) -> String {
    format!("{SITE_URL}/post/{slug}")
}

/// JSON for an inline `<script>`: `<`, `>` and `&` become `\u` escapes, so the
/// text can never close the tag or need HTML escaping.
fn script_json(value: &serde_json::Value) -> String {
    value
        .to_string()
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
}

fn author() -> serde_json::Value {
    serde_json::json!({ "@type": "Person", "name": AUTHOR, "url": AUTHOR_URL })
}

/// Title, description, canonical URL, Open Graph, Twitter card and JSON-LD.
#[component]
pub fn PageMeta(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] url: String,
    /// `None` for the home page, `Some` for a post.
    #[prop(optional)]
    post: Option<PostSummary>,
) -> impl IntoView {
    let kind = if post.is_some() { "article" } else { "website" };
    let ld = match &post {
        Some(p) => serde_json::json!({
            "@context": "https://schema.org",
            "@type": "BlogPosting",
            "headline": p.title,
            "description": p.excerpt,
            "datePublished": p.date,
            "dateModified": p.date,
            "url": url,
            "mainEntityOfPage": url,
            "image": OG_IMAGE,
            "keywords": p.tags.join(", "),
            "inLanguage": "en",
            "author": author(),
            "publisher": author(),
        }),
        None => serde_json::json!({
            "@context": "https://schema.org",
            "@type": "Blog",
            "name": SITE_NAME,
            "url": SITE_URL,
            "description": DESCRIPTION,
            "inLanguage": "en",
            "author": author(),
        }),
    };
    let published = post.as_ref().map(|p| p.date.clone());
    let tags = post.map(|p| p.tags).unwrap_or_default();

    view! {
        <Meta name="description" content=description.clone()/>
        <Link rel="canonical" href=url.clone()/>
        <Meta property="og:type" content=kind/>
        <Meta property="og:site_name" content=SITE_NAME/>
        <Meta property="og:title" content=title.clone()/>
        <Meta property="og:description" content=description.clone()/>
        <Meta property="og:url" content=url/>
        <Meta property="og:image" content=OG_IMAGE/>
        <Meta property="og:image:width" content="1200"/>
        <Meta property="og:image:height" content="630"/>
        <Meta property="og:image:alt" content="m4nuel.blog: Conway's Game of Life and the M mark"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:creator" content="@ManuelSzedlak"/>
        <Meta name="twitter:title" content=title/>
        <Meta name="twitter:description" content=description/>
        <Meta name="twitter:image" content=OG_IMAGE/>
        {published.map(|d| view! { <Meta property="article:published_time" content=d/> })}
        {tags.into_iter().map(|t| view! { <Meta property="article:tag" content=t/> }).collect_view()}
        <Script type_="application/ld+json">{script_json(&ld)}</Script>
    }
}

#[cfg(feature = "ssr")]
pub mod routes {
    //! Feeds and indexes for crawlers and AI agents, built from the posts.

    use super::{post_url, AUTHOR, DESCRIPTION, SITE_NAME, SITE_URL};
    use crate::content::{posts_dir, sources, Source};
    use axum::http::{header, StatusCode};
    use axum::response::{IntoResponse, Response};

    fn xml_escape(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }

    async fn published() -> Result<Vec<Source>, StatusCode> {
        sources(&posts_dir()).await.map_err(|e| {
            eprintln!("seo: reading posts: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
    }

    fn reply(content_type: &'static str, body: String) -> Response {
        ([(header::CONTENT_TYPE, content_type)], body).into_response()
    }

    pub async fn sitemap() -> Result<Response, StatusCode> {
        let posts = published().await?;
        let mut xml = String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
        );
        let newest = posts.first().map(|p| p.summary.date.clone());
        xml.push_str(&format!("  <url><loc>{SITE_URL}/</loc>"));
        if let Some(d) = newest {
            xml.push_str(&format!("<lastmod>{d}</lastmod>"));
        }
        xml.push_str("</url>\n");
        for p in &posts {
            xml.push_str(&format!(
                "  <url><loc>{}</loc><lastmod>{}</lastmod></url>\n",
                xml_escape(&post_url(&p.summary.slug)),
                p.summary.date
            ));
        }
        xml.push_str("</urlset>\n");
        Ok(reply("application/xml; charset=utf-8", xml))
    }

    pub async fn feed() -> Result<Response, StatusCode> {
        let posts = published().await?;
        let updated = posts
            .first()
            .map_or("1970-01-01".to_string(), |p| p.summary.date.clone());
        let mut xml = format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<feed xmlns=\"http://www.w3.org/2005/Atom\">\n  <title>{}</title>\n  <subtitle>{}</subtitle>\n  <link href=\"{SITE_URL}/\"/>\n  <link rel=\"self\" href=\"{SITE_URL}/feed.xml\"/>\n  <id>{SITE_URL}/</id>\n  <updated>{updated}T00:00:00Z</updated>\n  <author><name>{}</name></author>\n",
            xml_escape(SITE_NAME),
            xml_escape(DESCRIPTION),
            xml_escape(AUTHOR)
        );
        for p in &posts {
            let s = &p.summary;
            let url = xml_escape(&post_url(&s.slug));
            xml.push_str(&format!(
                "  <entry>\n    <title>{}</title>\n    <link href=\"{url}\"/>\n    <id>{url}</id>\n    <updated>{}T00:00:00Z</updated>\n    <summary>{}</summary>\n{}  </entry>\n",
                xml_escape(&s.title),
                s.date,
                xml_escape(&s.excerpt),
                s.tags
                    .iter()
                    .map(|t| format!("    <category term=\"{}\"/>\n", xml_escape(t)))
                    .collect::<String>()
            ));
        }
        xml.push_str("</feed>\n");
        Ok(reply("application/atom+xml; charset=utf-8", xml))
    }

    fn llms_index(posts: &[Source]) -> String {
        let mut out = format!(
            "# {SITE_NAME}\n\n> {DESCRIPTION}\n\nEvery post is Markdown with a date and tags. Full text of all posts: {SITE_URL}/llms-full.txt\n\n## Posts\n\n"
        );
        if posts.is_empty() {
            out.push_str("No posts yet.\n");
        }
        for p in posts {
            let s = &p.summary;
            out.push_str(&format!(
                "- [{}]({}) ({}): {}\n",
                s.title,
                post_url(&s.slug),
                s.date,
                s.excerpt
            ));
        }
        out.push_str("\n## Elsewhere\n\n- [m4nuel.net](https://m4nuel.net): who Manuel is and what he works on\n- [m4nuel.com](https://m4nuel.com): links and contact\n");
        out
    }

    pub async fn llms() -> Result<Response, StatusCode> {
        Ok(reply(
            "text/plain; charset=utf-8",
            llms_index(&published().await?),
        ))
    }

    pub async fn llms_full() -> Result<Response, StatusCode> {
        let posts = published().await?;
        let mut out = format!("# {SITE_NAME}\n\n> {DESCRIPTION}\n");
        for p in &posts {
            let s = &p.summary;
            out.push_str(&format!(
                "\n---\n\n# {}\n\nURL: {}\nDate: {}\nTags: {}\n\n{}\n",
                s.title,
                post_url(&s.slug),
                s.date,
                s.tags.join(", "),
                p.body.trim()
            ));
        }
        Ok(reply("text/plain; charset=utf-8", out))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::model::PostSummary;

        fn source(slug: &str, title: &str) -> Source {
            Source {
                summary: PostSummary {
                    slug: slug.into(),
                    title: title.into(),
                    date: "2026-11-19".into(),
                    excerpt: "An <excerpt> & more".into(),
                    tags: vec!["rust".into()],
                },
                body: "Body".into(),
                draft: false,
            }
        }

        #[test]
        fn escapes_xml() {
            assert_eq!(
                xml_escape(r#"<a href="x">&'"#),
                "&lt;a href=&quot;x&quot;&gt;&amp;&apos;"
            );
        }

        #[test]
        fn llms_index_lists_posts_and_links() {
            let text = llms_index(&[source("life", "Game of Life")]);
            assert!(text.contains("- [Game of Life](https://m4nuel.blog/post/life) (2026-11-19)"));
            assert!(text.contains("llms-full.txt"));
            assert!(llms_index(&[]).contains("No posts yet."));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_json_cannot_break_out() {
        let v = serde_json::json!({ "t": "</script><b>&" });
        let s = script_json(&v);
        assert!(
            !s.contains('<') && !s.contains('>') && !s.contains('&'),
            "{s}"
        );
        assert!(s.contains("\\u003c/script\\u003e"));
    }
}
