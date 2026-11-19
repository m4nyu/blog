//! Types shared by server and browser.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostSummary {
    pub slug: String,
    pub title: String,
    /// ISO date, `YYYY-MM-DD`. Sorts correctly as a string.
    pub date: String,
    pub excerpt: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub summary: PostSummary,
    pub blocks: Vec<Block>,
}

/// A post body, rendered on the server. Code blocks stay separate so the
/// browser can attach a copy button to each one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Block {
    /// HTML from Markdown; raw HTML in the source was escaped.
    Html(String),
    /// Plain text, rendered as an escaped text node.
    Code { lang: String, code: String },
}

/// `2026-11-19` -> `November 19, 2026`. Anything else is returned unchanged.
pub fn display_date(iso: &str) -> String {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let mut parts = iso.splitn(3, '-');
    let (Some(y), Some(m), Some(d)) = (parts.next(), parts.next(), parts.next()) else {
        return iso.to_string();
    };
    match (m.parse::<usize>(), d.parse::<u32>()) {
        (Ok(m @ 1..=12), Ok(d)) => format!("{} {d}, {y}", MONTHS[m - 1]),
        _ => iso.to_string(),
    }
}

/// Slugs are file stems: lowercase ASCII letters, digits, `-` and `_`.
/// Anything else (`..`, `/`, `%`, uppercase) is rejected before a path is built.
pub fn is_valid_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.len() <= 128
        && slug
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_dates() {
        assert_eq!(display_date("2026-11-19"), "November 19, 2026");
        assert_eq!(display_date("2026-01-05"), "January 5, 2026");
        assert_eq!(display_date("2026-13-01"), "2026-13-01");
        assert_eq!(display_date("soon"), "soon");
    }

    #[test]
    fn slug_rules() {
        assert!(is_valid_slug("hello-world_2"));
        for bad in [
            "",
            "..",
            "../etc/passwd",
            "a/b",
            "a\\b",
            "Hello",
            "a.md",
            "%2e%2e",
            "a b",
            "é",
        ] {
            assert!(!is_valid_slug(bad), "{bad}");
        }
        assert!(!is_valid_slug(&"a".repeat(129)));
    }
}
