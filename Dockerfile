# Railway deployment Dockerfile
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy everything (filtered by .dockerignore)
COPY . .

# Build with SSR features for production
RUN cargo build --release --features ssr

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary and assets
COPY --from=builder /app/target/release/tailwind ./app
COPY --from=builder /app/app/posts ./posts
COPY --from=builder /app/target/site ./public

# Run as non-root user for security
RUN useradd -u 1000 -s /bin/bash appuser && \
    chown -R appuser:appuser /app
USER appuser

EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/ || exit 1

CMD ["./app"]