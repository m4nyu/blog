FROM rust:1.82-alpine AS builder

# Install build dependencies  
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev

WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source for dependency caching
RUN mkdir -p app/src && echo "fn main() {}" > app/src/main.rs && echo "" > app/src/lib.rs
RUN cargo build --release --features ssr
RUN rm -rf app/src

# Copy actual source and build
COPY . .
RUN cargo build --release --features ssr

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
COPY --from=builder --chown=appuser:appuser /app/target/release/tailwind ./app
COPY --from=builder --chown=appuser:appuser /app/app/posts ./posts
COPY --from=builder --chown=appuser:appuser /app/app/public ./public

USER appuser

# Set environment variables
ENV LEPTOS_OUTPUT_NAME="blog"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="."
ENV RUST_LOG="info"

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/ || exit 1

CMD ["./app"]
