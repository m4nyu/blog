# Build stage
FROM rust:1.89-bookworm as builder

# Install system dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-leptos and wasm target
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --locked

# Install Node.js and Tailwind CSS
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# Set working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./
COPY src/lib.rs src/lib.rs

# Cache dependencies
RUN cargo build --release --features ssr
RUN rm src/lib.rs

# Copy source code
COPY . .

# Build the application with leptos
RUN cargo leptos build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false -m -d /app app

# Set working directory
WORKDIR /app

# Copy built application and assets
COPY --from=builder /app/target/release/tailwind /app/tailwind
COPY --from=builder /app/target/site /app/target/site
COPY --from=builder /app/Cargo.toml /app/Cargo.toml
COPY --from=builder /app/posts /app/posts
COPY --from=builder /app/public /app/public

# Change ownership
RUN chown -R app:app /app
USER app

# Set environment
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"

# Expose port
EXPOSE 8080

# Run the application
CMD ["./tailwind"]