import * as oci from "@pulumi/oci";
import * as pulumi from "@pulumi/pulumi";
import * as tls from "@pulumi/tls";

// Configuration
const config = new pulumi.Config("blog");
const domain = config.get("domain") || "m4nuel.blog";
const ociConfig = new pulumi.Config("oci");

// Get the list of availability domains
const availabilityDomains = oci.identity.getAvailabilityDomains({
    compartmentId: ociConfig.require("tenancyOcid"),
});

// Create a VCN (Virtual Cloud Network)
const vcn = new oci.core.Vcn("blog-vcn", {
    cidrBlocks: ["10.0.0.0/16"],
    compartmentId: ociConfig.require("tenancyOcid"),
    displayName: "Blog VCN",
    dnsLabel: "blogvcn",
});

// Create an Internet Gateway
const internetGateway = new oci.core.InternetGateway("blog-igw", {
    compartmentId: ociConfig.require("tenancyOcid"),
    vcnId: vcn.id,
    displayName: "Blog Internet Gateway",
    enabled: true,
});

// Create a Route Table
const routeTable = new oci.core.RouteTable("blog-rt", {
    compartmentId: ociConfig.require("tenancyOcid"),
    vcnId: vcn.id,
    displayName: "Blog Route Table",
    routeRules: [{
        destinationTypeRaw: "CIDR_BLOCK",
        destination: "0.0.0.0/0",
        networkEntityId: internetGateway.id,
    }],
});

// Create a Public Subnet
const publicSubnet = new oci.core.Subnet("blog-public-subnet", {
    cidrBlock: "10.0.1.0/24",
    compartmentId: ociConfig.require("tenancyOcid"),
    vcnId: vcn.id,
    displayName: "Blog Public Subnet",
    dnsLabel: "blogpub",
    routeTableId: routeTable.id,
    securityListIds: [],
    prohibitPublicIpOnVnic: false,
});

// Create a Security List for the subnet
const securityList = new oci.core.SecurityList("blog-security-list", {
    compartmentId: ociConfig.require("tenancyOcid"),
    vcnId: vcn.id,
    displayName: "Blog Security List",
    egressSecurityRules: [{
        destination: "0.0.0.0/0",
        protocol: "all",
        stateless: false,
    }],
    ingressSecurityRules: [
        // Allow SSH
        {
            source: "0.0.0.0/0",
            protocol: "6", // TCP
            stateless: false,
            tcpOptions: {
                min: 22,
                max: 22,
            },
        },
        // Allow HTTP
        {
            source: "0.0.0.0/0",
            protocol: "6", // TCP
            stateless: false,
            tcpOptions: {
                min: 80,
                max: 80,
            },
        },
        // Allow HTTPS
        {
            source: "0.0.0.0/0",
            protocol: "6", // TCP
            stateless: false,
            tcpOptions: {
                min: 443,
                max: 443,
            },
        },
        // Allow Leptos app port
        {
            source: "0.0.0.0/0",
            protocol: "6", // TCP
            stateless: false,
            tcpOptions: {
                min: 4002,
                max: 4002,
            },
        },
    ],
});

// Update subnet to use the security list
const subnetWithSecurity = new oci.core.Subnet("blog-subnet-with-security", {
    cidrBlock: "10.0.2.0/24",
    compartmentId: ociConfig.require("tenancyOcid"),
    vcnId: vcn.id,
    displayName: "Blog Subnet",
    dnsLabel: "blogsub",
    routeTableId: routeTable.id,
    securityListIds: [securityList.id],
    prohibitPublicIpOnVnic: false,
});

// Get the Ubuntu 22.04 image for ARM (Ampere A1) - Always Free tier
const ubuntuImage = oci.core.getImages({
    compartmentId: ociConfig.require("tenancyOcid"),
    operatingSystem: "Canonical Ubuntu",
    operatingSystemVersion: "22.04",
    shape: "VM.Standard.A1.Flex",
    sortBy: "TIMECREATED",
    sortOrder: "DESC",
});

// Create an SSH key pair for instance access
const sshKeyPair = new tls.PrivateKey("blog-ssh-key", {
    algorithm: "RSA",
    rsaBits: 2048,
});

// Cloud-init script to set up the Leptos blog
const cloudInitScript = `#!/bin/bash
set -e

# Update system
apt-get update
apt-get upgrade -y

# Install essential packages
apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    nginx \
    certbot \
    python3-certbot-nginx \
    ufw

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Add wasm target
rustup target add wasm32-unknown-unknown

# Install cargo-leptos
cargo install cargo-leptos

# Clone the blog repository (you'll need to update this with your repo)
cd /opt
git clone https://github.com/m4nyu/blog.git
cd blog

# Build the application
cargo leptos build --release

# Create systemd service for the blog
cat > /etc/systemd/system/blog.service << 'EOF'
[Unit]
Description=Leptos Blog
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/opt/blog
Environment="LEPTOS_SITE_ADDR=127.0.0.1:4002"
ExecStart=/opt/blog/target/release/tailwind
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

# Configure Nginx as reverse proxy
cat > /etc/nginx/sites-available/blog << 'EOF'
server {
    listen 80;
    listen [::]:80;
    server_name ${domain};

    location / {
        proxy_pass http://127.0.0.1:4002;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
EOF

# Enable the site
ln -s /etc/nginx/sites-available/blog /etc/nginx/sites-enabled/
rm -f /etc/nginx/sites-enabled/default

# Configure firewall
ufw allow 22/tcp
ufw allow 80/tcp
ufw allow 443/tcp
ufw --force enable

# Start services
systemctl daemon-reload
systemctl enable blog
systemctl start blog
systemctl restart nginx

echo "Blog deployment complete!"
`;

// Create a Compute Instance (Free tier: VM.Standard.A1.Flex with 1 OCPU and 6GB RAM)
const blogInstance = new oci.core.Instance("blog-instance", {
    availabilityDomain: availabilityDomains.then(ads => ads.availabilityDomains[0].name),
    compartmentId: ociConfig.require("tenancyOcid"),
    shape: "VM.Standard.A1.Flex",
    shapeConfig: {
        ocpus: 1,
        memoryInGbs: 6,
    },
    sourceDetails: {
        sourceType: "image",
        sourceId: ubuntuImage.then(imgs => imgs.images[0].id),
    },
    createVnicDetails: {
        assignPublicIp: true,
        subnetId: subnetWithSecurity.id,
        displayName: "Blog Instance VNIC",
    },
    metadata: {
        ssh_authorized_keys: sshKeyPair.publicKeyOpenssh,
        user_data: Buffer.from(cloudInitScript).toString("base64"),
    },
    displayName: "Blog Instance",
    freeformTags: {
        "Name": "Blog Server",
        "Environment": "Production",
    },
});

// Create a Reserved Public IP
const publicIp = new oci.core.PublicIp("blog-public-ip", {
    compartmentId: ociConfig.require("tenancyOcid"),
    lifetime: "RESERVED",
    displayName: "Blog Public IP",
});

// Associate the Reserved IP with the instance
const privateIp = blogInstance.createVnicDetails.apply(details => 
    oci.core.getPrivateIps({
        subnetId: subnetWithSecurity.id,
    }).then(ips => ips.privateIps[0])
);

// ===== EXPORTS =====
export const instanceId = blogInstance.id;
export const instancePublicIp = blogInstance.publicIp;
export const reservedPublicIp = publicIp.ipAddress;
export const sshPrivateKey = sshKeyPair.privateKeyPem;
export const sshCommand = pulumi.interpolate`ssh -i blog-key.pem ubuntu@${blogInstance.publicIp}`;
export const websiteUrl = pulumi.interpolate`http://${blogInstance.publicIp}`;
export const domain = domain;

// Instructions for DNS setup
export const dnsInstructions = pulumi.interpolate`
To complete the setup:
1. Add an A record in your DNS provider pointing ${domain} to ${blogInstance.publicIp}
2. SSH into the instance: ${sshCommand}
3. Run: sudo certbot --nginx -d ${domain} to set up SSL
`;