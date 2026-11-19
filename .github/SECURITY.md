# Security

Report vulnerabilities privately through GitHub: **Security** tab, **Report a vulnerability**.
Please do not open public issues for them.

## Design

- 100% safe Rust in this crate (`unsafe_code = "forbid"`).
- The server is read-only: two server functions (`list_posts`, `get_post`), no
  writes, no accounts, no database.
- Slugs are validated (`[a-z0-9_-]`) before any file path is built.
- Raw HTML in Markdown is escaped; link and image URLs are limited to http(s),
  mailto, fragments and relative paths.
- Every HTML response carries a nonce-based Content-Security-Policy with no
  inline scripts or styles, plus HSTS, nosniff, frame and referrer headers.
- The container runs as non-root on a distroless image.
- CI runs `cargo audit` and pins every GitHub Action to a commit SHA.
