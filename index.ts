import * as oci from "@pulumi/oci";
import * as pulumi from "@pulumi/pulumi";

// Get the root compartment ID (tenancy OCID)
const tenancyId = new pulumi.Config("oci").require("tenancyOcid");

// Get availability domains
const availabilityDomains = oci.identity.getAvailabilityDomains({
    compartmentId: tenancyId,
});

// Create a VCN (Virtual Cloud Network)
const vcn = new oci.core.Vcn("blog-vcn", {
    compartmentId: tenancyId,
    cidrBlocks: ["10.0.0.0/16"],
    displayName: "Blog VCN",
    dnsLabel: "blogvcn",
});

// Create Internet Gateway
const internetGateway = new oci.core.InternetGateway("blog-igw", {
    compartmentId: tenancyId,
    vcnId: vcn.id,
    displayName: "Blog Internet Gateway",
    enabled: true,
});

// Create Route Table
const routeTable = new oci.core.RouteTable("blog-rt", {
    compartmentId: tenancyId,
    vcnId: vcn.id,
    displayName: "Blog Route Table",
    routeRules: [{
        destination: "0.0.0.0/0",
        destinationType: "CIDR_BLOCK",
        networkEntityId: internetGateway.id,
    }],
});

// Create Security List
const securityList = new oci.core.SecurityList("blog-security-list", {
    compartmentId: tenancyId,
    vcnId: vcn.id,
    displayName: "Blog Security List",
    egressSecurityRules: [{
        destination: "0.0.0.0/0",
        protocol: "all",
        stateless: false,
    }],
    ingressSecurityRules: [
        // SSH
        {
            source: "0.0.0.0/0",
            protocol: "6", // TCP
            stateless: false,
            tcpOptions: {
                min: 22,
                max: 22,
            },
        },
        // HTTP
        {
            source: "0.0.0.0/0",
            protocol: "6", // TCP
            stateless: false,
            tcpOptions: {
                min: 80,
                max: 80,
            },
        },
        // HTTPS
        {
            source: "0.0.0.0/0",
            protocol: "6", // TCP
            stateless: false,
            tcpOptions: {
                min: 443,
                max: 443,
            },
        },
    ],
});

// Create Subnet
const subnet = new oci.core.Subnet("blog-subnet", {
    compartmentId: tenancyId,
    vcnId: vcn.id,
    cidrBlock: "10.0.1.0/24",
    displayName: "Blog Subnet",
    dnsLabel: "blogsub",
    routeTableId: routeTable.id,
    securityListIds: [securityList.id],
    prohibitPublicIpOnVnic: false,
});

// Get Ubuntu 22.04 ARM image for Always Free tier
const images = oci.core.getImages({
    compartmentId: tenancyId,
    operatingSystem: "Canonical Ubuntu",
    operatingSystemVersion: "22.04",
    shape: "VM.Standard.A1.Flex",
    sortBy: "TIMECREATED",
    sortOrder: "DESC",
});

// Cloud-init script to set up the blog
const cloudInit = `#!/bin/bash
set -e

# Update system
apt-get update && apt-get upgrade -y

# Install dependencies
apt-get install -y curl git build-essential pkg-config libssl-dev nginx

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source /root/.cargo/env
echo 'source /root/.cargo/env' >> /root/.bashrc

# Add wasm target
/root/.cargo/bin/rustup target add wasm32-unknown-unknown

# Install cargo-leptos
/root/.cargo/bin/cargo install cargo-leptos

# Clone and build the blog
cd /opt
git clone https://github.com/m4nyu/blog.git
cd blog
/root/.cargo/bin/cargo leptos build --release

# Create systemd service
cat > /etc/systemd/system/blog.service << 'EOF'
[Unit]
Description=Leptos Blog
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/blog
Environment="LEPTOS_SITE_ADDR=127.0.0.1:3000"
ExecStart=/opt/blog/target/release/tailwind
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

# Configure nginx
cat > /etc/nginx/sites-available/blog << 'EOF'
server {
    listen 80 default_server;
    listen [::]:80 default_server;

    location / {
        proxy_pass http://127.0.0.1:3000;
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

# Enable site
rm -f /etc/nginx/sites-enabled/default
ln -s /etc/nginx/sites-available/blog /etc/nginx/sites-enabled/blog

# Start services
systemctl daemon-reload
systemctl enable blog nginx
systemctl start blog nginx

echo "Blog deployment complete!"
`;

// Create compute instance using Always Free tier
const instance = new oci.core.Instance("blog-instance", {
    compartmentId: tenancyId,
    availabilityDomain: availabilityDomains.then(ads => ads.availabilityDomains[0].name),
    shape: "VM.Standard.A1.Flex",
    shapeConfig: {
        ocpus: 1,
        memoryInGbs: 6,
    },
    sourceDetails: {
        sourceType: "image",
        sourceId: images.then(imgs => imgs.images[0].id),
        bootVolumeSizeInGbs: "50",
    },
    createVnicDetails: {
        subnetId: subnet.id,
        displayName: "Blog Instance VNIC",
        assignPublicIp: "true",
        hostnameLabel: "blog",
    },
    metadata: {
        user_data: Buffer.from(cloudInit).toString("base64"),
    },
    displayName: "Blog Instance",
    freeformTags: {
        Name: "Blog",
        Environment: "Production",
    },
});

// Export stack outputs for external reference
export const instanceId = instance.id;
export const publicIp = instance.publicIp;
export const privateIp = instance.privateIp;
export const vcnId = vcn.id;
export const subnetId = subnet.id;
export const websiteUrl = pulumi.interpolate`http://${instance.publicIp}`;

// Log outputs for debugging
pulumi.log.info(`Instance will be created with ID: ${instance.id}`);