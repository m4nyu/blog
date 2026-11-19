# Build with the Rust toolchain only; run as non-root on distroless (no shell,
# no package manager). Base images are pinned to a release line.
FROM rust:1.97-slim-trixie AS build
ARG CARGO_LEPTOS_VERSION=0.3.12
RUN rustup target add wasm32-unknown-unknown \
 && cargo install --locked cargo-leptos --version "${CARGO_LEPTOS_VERSION}"
WORKDIR /src
COPY . .
RUN cargo leptos build --release

FROM gcr.io/distroless/cc-debian13:nonroot
WORKDIR /app
COPY --from=build /src/target/release/blog /src/target/release/hash.txt ./
COPY --from=build /src/target/site ./site
COPY posts ./posts
ENV LEPTOS_ENV=PROD \
    LEPTOS_OUTPUT_NAME=blog \
    LEPTOS_SITE_ROOT=site \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_SITE_ADDR=0.0.0.0:3000 \
    LEPTOS_HASH_FILES=true \
    BLOG_POSTS_DIR=/app/posts
EXPOSE 3000
ENTRYPOINT ["/app/blog"]
