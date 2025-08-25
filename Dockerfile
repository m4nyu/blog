# Build arguments from Pulumi config
ARG RUST_VERSION=1.75
ARG DEBIAN_VERSION=bookworm-slim
ARG APP_BINARY=tailwind
ARG BUILD_FEATURES=ssr

FROM rust:${RUST_VERSION}-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy app source and cargo files
COPY app/src ./app/src
COPY app/public ./app/public
COPY Cargo.toml Cargo.lock ./
COPY posts ./posts

# Build with configurable features
RUN cargo build --release --features ${BUILD_FEATURES}

FROM debian:${DEBIAN_VERSION}

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary and assets with configurable name
ARG APP_BINARY
COPY --from=builder /app/target/release/${APP_BINARY} ./app
COPY --from=builder /app/posts ./posts
COPY --from=builder /app/target/site ./public

# Environment variables should be set at runtime via deployment configuration

# Run as non-root user for security
RUN useradd -u 1000 -s /bin/bash appuser && \
    chown -R appuser:appuser /app
USER appuser

EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/ || exit 1

CMD ["./app"]