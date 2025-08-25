import * as pulumi from "@pulumi/pulumi";
import { Values } from "./config.js";

export class Apps {
    public setupScript: pulumi.Output<string>;

    constructor(values: Values) {
        // Cloud-init script for setting up nginx with SSL using certbot
        this.setupScript = pulumi.interpolate`#!/bin/bash

# Install nginx and certbot
sudo dnf update -y
sudo dnf install -y nginx certbot python3-certbot-nginx

# Enable and start nginx
sudo systemctl enable nginx
sudo systemctl start nginx

# Create nginx config for the blog
sudo tee /etc/nginx/conf.d/blog.conf > /dev/null << 'EOF'
server {
    listen 80;
    server_name ${values.domain};
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
EOF

# Test nginx config and reload
sudo nginx -t && sudo systemctl reload nginx

# Wait for DNS to propagate, then get SSL certificate
sleep 60
sudo certbot --nginx -d ${values.domain} --non-interactive --agree-tos --email ${values.email}

# Set up auto-renewal
sudo systemctl enable crond
sudo systemctl start crond
echo "0 0,12 * * * root python3 -c 'import random; import time; time.sleep(random.random() * 3600)' && /usr/bin/certbot renew --quiet" | sudo tee -a /etc/crontab

echo "SSL setup complete for ${values.domain}"
`;
    }
}