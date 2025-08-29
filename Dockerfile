# Use Rust 1.82 for ICU dependency compatibility
FROM rust:1.82-alpine AS builder

# Install build dependencies including Node.js for Tailwind
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    nodejs \
    npm

# Add wasm32 target and install wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli --version 0.2.93

WORKDIR /app

# Copy all files for leptos build
COPY . .

# Install Node dependencies for Tailwind
RUN npm install

# Build Tailwind CSS
RUN npx tailwindcss -i app/src/styles/global.css -o target/site/pkg/blog.css --minify

# Create target directories
RUN mkdir -p target/site/pkg target/server

# Build Rust server binary 
RUN cargo build --release --features ssr --bin blog

# Build WASM for client side
RUN cargo build --release --features hydrate --target wasm32-unknown-unknown --lib

# Generate WASM bindings
RUN wasm-bindgen target/wasm32-unknown-unknown/release/blog.wasm --out-dir target/site/pkg --web --no-typescript --target web --omit-default-module-path

# Copy server binary to expected location
RUN cp target/release/blog target/server/blog

# Runtime stage
FROM alpine:3.18

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    curl

# Create non-root user
RUN adduser -D -u 1000 appuser

WORKDIR /app

# Copy binary and assets with proper ownership
COPY --from=builder --chown=appuser:appuser /app/target/server/blog ./blog
COPY --from=builder --chown=appuser:appuser /app/app/posts ./posts  
# Copy the compiled site artifacts (CSS, JS, WASM)
COPY --from=builder --chown=appuser:appuser /app/target/site ./site
# Copy public assets (favicon, etc.) to the site root so they're served at root paths
COPY --from=builder --chown=appuser:appuser /app/app/public/* ./site/

USER appuser

# Set environment variables
ENV LEPTOS_OUTPUT_NAME="blog"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV RUST_LOG="info"

# Railway will set PORT, default to 3000 for local
ENV PORT=3000
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:${PORT}/ || exit 1

# Use shell form to expand PORT variable
CMD LEPTOS_SITE_ADDR="0.0.0.0:${PORT}" ./blog
