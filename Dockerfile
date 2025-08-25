FROM rust:1.75-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

WORKDIR /app

# Copy source and build
COPY . .
RUN cargo build --release --features ssr

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN useradd -u 1000 -m -s /bin/bash appuser

WORKDIR /app

# Copy binary and assets with proper ownership
COPY --from=builder --chown=appuser:appuser /app/target/release/tailwind ./app
COPY --from=builder --chown=appuser:appuser /app/app/posts ./posts
COPY --from=builder --chown=appuser:appuser /app/target/site ./public

USER appuser
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/ || exit 1

CMD ["./app"]