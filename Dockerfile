FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release --features ssr

FROM debian:bookworm-slim

# Install nginx, supervisor, and required packages
RUN apt-get update && apt-get install -y \
    nginx \
    supervisor \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy application files
COPY --from=builder /app/target/release/tailwind /app/tailwind
COPY --from=builder /app/public /app/public
COPY --from=builder /app/posts /app/posts
COPY --from=builder /app/Leptos.toml /app/Leptos.toml

# Copy nginx configuration
COPY nginx.conf /etc/nginx/sites-available/default
RUN rm -f /etc/nginx/sites-enabled/default && \
    ln -s /etc/nginx/sites-available/default /etc/nginx/sites-enabled/

# Create supervisor configuration
RUN mkdir -p /etc/supervisor/conf.d
COPY <<EOF /etc/supervisor/conf.d/supervisord.conf
[supervisord]
nodaemon=true
user=root

[program:nginx]
command=nginx -g "daemon off;"
autostart=true
autorestart=true
stderr_logfile=/var/log/nginx.err.log
stdout_logfile=/var/log/nginx.out.log

[program:leptos]
command=/app/tailwind
directory=/app
autostart=true
autorestart=true
stderr_logfile=/var/log/leptos.err.log
stdout_logfile=/var/log/leptos.out.log
environment=LEPTOS_SITE_ADDR="127.0.0.1:3000"
EOF

WORKDIR /app

# Expose HTTP and HTTPS ports
EXPOSE 80 443

# Start supervisor to manage both nginx and the Leptos app
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]