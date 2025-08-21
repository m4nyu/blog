import * as oci from "@pulumi/oci";
import * as pulumi from "@pulumi/pulumi";

// Get configuration
const config = new pulumi.Config();
const ociConfig = new pulumi.Config("oci");
const tenancyId = ociConfig.require("tenancyOcid");
const region = ociConfig.get("region") || "us-phoenix-1";

// Get availability domains
const availabilityDomains = oci.identity.getAvailabilityDomains({
    compartmentId: tenancyId,
});

// Common tags for all resources
const commonTags = {
    Project: "m4nuel-blog",
    Environment: "production",
    ManagedBy: "pulumi",
    Owner: "manuel",
};

// Create VCN (Virtual Cloud Network)
const vcn = new oci.core.Vcn("blog-vcn", {
    compartmentId: tenancyId,
    cidrBlocks: ["10.0.0.0/16"],
    displayName: "Blog VCN",
    dnsLabel: "blogvcn",
    freeformTags: commonTags,
});

// Create Internet Gateway
const internetGateway = new oci.core.InternetGateway("blog-igw", {
    compartmentId: tenancyId,
    vcnId: vcn.id,
    displayName: "Blog Internet Gateway",
    enabled: true,
    freeformTags: commonTags,
});

// Create NAT Gateway for outbound traffic
const natGateway = new oci.core.NatGateway("blog-nat", {
    compartmentId: tenancyId,
    vcnId: vcn.id,
    displayName: "Blog NAT Gateway",
    freeformTags: commonTags,
});

// Create Service Gateway for Oracle services
const serviceGateway = new oci.core.ServiceGateway("blog-svc-gw", {
    compartmentId: tenancyId,
    vcnId: vcn.id,
    displayName: "Blog Service Gateway",
    services: [{
        serviceId: oci.core.getServices({}).then(s => 
            s.services.find(svc => svc.name.includes("Object Storage"))?.id || ""
        ),
    }],
    freeformTags: commonTags,
});

// Create Route Table for public subnet
const publicRouteTable = new oci.core.RouteTable("blog-public-rt", {
    compartmentId: tenancyId,
    vcnId: vcn.id,
    displayName: "Blog Public Route Table",
    routeRules: [{
        destination: "0.0.0.0/0",
        destinationType: "CIDR_BLOCK",
        networkEntityId: internetGateway.id,
    }],
    freeformTags: commonTags,
});

// Create Security List for web servers
const webSecurityList = new oci.core.SecurityList("blog-web-sl", {
    compartmentId: tenancyId,
    vcnId: vcn.id,
    displayName: "Blog Web Security List",
    
    // Egress rules - allow all outbound traffic
    egressSecurityRules: [
        {
            destination: "0.0.0.0/0",
            protocol: "all",
            stateless: false,
        }
    ],
    
    // Ingress rules - restrictive inbound access
    ingressSecurityRules: [
        // SSH access
        {
            source: "0.0.0.0/0",
            protocol: "6", // TCP
            stateless: false,
            tcpOptions: {
                min: 22,
                max: 22,
            },
            description: "SSH access",
        },
        // HTTP traffic
        {
            source: "0.0.0.0/0", 
            protocol: "6", // TCP
            stateless: false,
            tcpOptions: {
                min: 80,
                max: 80,
            },
            description: "HTTP traffic",
        },
        // HTTPS traffic
        {
            source: "0.0.0.0/0",
            protocol: "6", // TCP
            stateless: false,
            tcpOptions: {
                min: 443,
                max: 443,
            },
            description: "HTTPS traffic",
        },
        // ICMP for ping diagnostics
        {
            source: "0.0.0.0/0",
            protocol: "1", // ICMP
            stateless: false,
            icmpOptions: {
                type: 3,
                code: 4,
            },
            description: "ICMP Path Discovery",
        },
    ],
    freeformTags: commonTags,
});

// Create public subnet
const publicSubnet = new oci.core.Subnet("blog-public-subnet", {
    compartmentId: tenancyId,
    vcnId: vcn.id,
    cidrBlock: "10.0.2.0/24",
    displayName: "Blog Public Subnet",
    dnsLabel: "blogpublic",
    routeTableId: publicRouteTable.id,
    securityListIds: [webSecurityList.id],
    prohibitPublicIpOnVnic: false,
    freeformTags: commonTags,
});

// Get latest Ubuntu 22.04 ARM image for Always Free tier
const ubuntuImages = oci.core.getImages({
    compartmentId: tenancyId,
    operatingSystem: "Canonical Ubuntu",
    operatingSystemVersion: "22.04",
    shape: "VM.Standard.A1.Flex",
    sortBy: "TIMECREATED",
    sortOrder: "DESC",
});

// Get SSH public key from config
const sshPublicKey = config.requireSecret("sshPublicKey");

const cloudInitConfig = sshPublicKey.apply(key => `#cloud-config
package_update: true
packages:
  - docker.io
  - git
  - curl

ssh_authorized_keys:
  - ${key}

write_files:
  - content: |
      #!/bin/bash
      set -e
      
      echo "Starting deployment script..."
      
      # Ensure Docker is running
      sudo systemctl enable docker
      sudo systemctl start docker
      
      # Clone or update the repository
      cd /home/ubuntu
      if [ -d "blog" ]; then
        cd blog
        sudo -u ubuntu git pull origin main
      else
        sudo -u ubuntu git clone https://github.com/mszedlak/blog.git
        cd blog
      fi
      
      # Stop existing container if running
      docker stop blog-container || true
      docker rm blog-container || true
      
      # Build new image
      docker build -t blog-app .
      
      # Run container with proper port mapping
      docker run -d \
        --name blog-container \
        --restart unless-stopped \
        -p 80:80 \
        -p 443:443 \
        blog-app
      
      echo "Deployment complete!"
    path: /home/ubuntu/deploy.sh
    permissions: '0755'
    owner: ubuntu:ubuntu

runcmd:
  - systemctl enable docker
  - systemctl start docker
  - usermod -aG docker ubuntu
  - /home/ubuntu/deploy.sh
`);

// Create compute instance
const blogInstance = new oci.core.Instance("blog-instance", {
    compartmentId: tenancyId,
    availabilityDomain: availabilityDomains.then(ads => 
        ads.availabilityDomains[1]?.name || ads.availabilityDomains[0].name
    ),
    shape: "VM.Standard.A1.Flex",
    shapeConfig: {
        ocpus: 1,
        memoryInGbs: 6,
    },
    sourceDetails: {
        sourceType: "image", 
        sourceId: ubuntuImages.then(imgs => imgs.images[0].id),
        bootVolumeSizeInGbs: "50",
    },
    createVnicDetails: {
        subnetId: publicSubnet.id,
        displayName: "Blog Instance VNIC",
        assignPublicIp: "true",
        hostnameLabel: "blog-clean",
        nsgIds: [], // No network security groups for simplicity
    },
    metadata: {
        "ssh_authorized_keys": sshPublicKey,
        "user_data": cloudInitConfig.apply(config => Buffer.from(config).toString("base64")),
    },
    displayName: "Blog Instance",
    freeformTags: {
        ...commonTags,
        Name: "blog-server",
        Role: "webserver",
    },
});

// Export important values
export const tenancyOcid = tenancyId;
export const regionName = region;
export const vcnId = vcn.id;
export const subnetId = publicSubnet.id;
export const instanceId = blogInstance.id;
export const publicIp = blogInstance.publicIp;
export const privateIp = blogInstance.privateIp;
export const websiteUrl = pulumi.interpolate`http://${blogInstance.publicIp}`;
export const domainUrl = "https://m4nuel.blog";

// Log deployment information
pulumi.all([instanceId, publicIp, privateIp, vcnId, subnetId, websiteUrl]).apply(
    ([instId, pubIp, privIp, vId, sId, url]) => {
        pulumi.log.info(`Oracle Cloud Infrastructure Deployment Complete:
- Region: ${region}
- Instance ID: ${instId}
- Public IP: ${pubIp} 
- Private IP: ${privIp}
- VCN ID: ${vId}
- Subnet ID: ${sId}
- Website URL: ${url}
- Domain: https://m4nuel.blog`);
    }
);