import * as oci from "@pulumi/oci";
import * as pulumi from "@pulumi/pulumi";
import { Values } from "./config.js";
import { Network } from "./network.js";

export class Compute {
    public instance: oci.core.Instance;
    public publicIp: oci.core.PublicIp;

    constructor(values: Values, network: Network) {
        // Get additional config for container environment
        const config = new pulumi.Config();
        const leptosOutputName = config.get("leptosOutputName") || "blog";
        const leptosSiteAddr = config.get("leptosSiteAddr") || "0.0.0.0:3000";
        const rustLog = config.get("rustLog") || "info";
        // Use a platform image specifically for E2.1.Micro (x86_64)
        // This is a hard-coded Oracle-provided platform image that works with E2.1.Micro
        // Oracle Linux 8 platform image for x86_64
        const image = pulumi.output("ocid1.image.oc1.phx.aaaaaaaafx6vdtqpjsqdjaietaoxnz74dnp4llnybotatih5iqopomfhseaa");

        // Cloud-init script to install Docker, nginx, and set up SSL with Let's Encrypt
        const cloudInit = pulumi.interpolate`#cloud-config
packages:
  - docker
  - docker-compose
  - nginx
  - certbot
  - python3-certbot-nginx

runcmd:
  # Set up Docker
  - systemctl enable docker
  - systemctl start docker
  - usermod -aG docker opc
  
  # Start the blog container with proper environment configuration
  - docker run -d --name blog --restart always -p 3000:3000 -e LEPTOS_OUTPUT_NAME="${leptosOutputName}" -e LEPTOS_SITE_ADDR="${leptosSiteAddr}" -e RUST_LOG="${rustLog}" ${values.region}.ocir.io/${values.project}/blog:latest || echo "Container registry not yet available - will start later"
  
  # Set up nginx
  - systemctl enable nginx
  - systemctl start nginx
  
  # Create nginx config for the blog
  - |
    cat > /etc/nginx/conf.d/blog.conf << 'EOF'
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
  - nginx -t && systemctl reload nginx
  
  # Wait for DNS to propagate, then get SSL certificate
  - sleep 120
  - certbot --nginx -d ${values.domain} --non-interactive --agree-tos --email ${values.email} || echo "SSL cert setup failed - will retry later"
  
  # Set up auto-renewal
  - systemctl enable crond
  - systemctl start crond
  - echo "0 0,12 * * * root python3 -c 'import random; import time; time.sleep(random.random() * 3600)' && /usr/bin/certbot renew --quiet" >> /etc/crontab
  
  - echo "Setup complete for ${values.domain}"

write_files:
  - path: /home/opc/update-blog.sh
    permissions: '0755'
    content: |
      #!/bin/bash
      docker pull ${values.region}.ocir.io/${values.project}/blog:latest
      docker stop blog || true
      docker rm blog || true
      docker run -d --name blog --restart always -p 3000:3000 -e LEPTOS_OUTPUT_NAME="${leptosOutputName}" -e LEPTOS_SITE_ADDR="${leptosSiteAddr}" -e RUST_LOG="${rustLog}" ${values.region}.ocir.io/${values.project}/blog:latest
      
  - path: /home/opc/setup-ssl.sh
    permissions: '0755'
    content: |
      #!/bin/bash
      # Manual SSL setup script in case cloud-init fails
      sudo certbot --nginx -d ${values.domain} --non-interactive --agree-tos --email ${values.email}
`;

        // Single Always Free E2.1.Micro instance (1/8 OCPU, 1GB RAM) - x86_64
        // This is the OTHER Always Free option besides ARM
        this.instance = new oci.core.Instance("instance", {
            compartmentId: values.tenancy,
            displayName: pulumi.interpolate`${values.project}-blog`,
            availabilityDomain: values.tenancy.apply(async id => {
                const ads = await oci.identity.getAvailabilityDomains({
                    compartmentId: id,
                });
                return ads.availabilityDomains[0].name;
            }),
            shape: "VM.Standard.E2.1.Micro",
            // E2.1.Micro doesn't need shapeConfig - it's fixed size
            sourceDetails: {
                sourceType: "image",
                sourceId: image,
                bootVolumeSizeInGbs: "50", // Use 50GB of the 200GB Always Free storage
            },
            createVnicDetails: {
                subnetId: network.publicsubnet.id,
                assignPublicIp: "true",
            },
            metadata: {
                user_data: pulumi.output(cloudInit).apply(ci => Buffer.from(ci).toString('base64')),
            },
        });

        // Reserved public IP for the instance
        this.publicIp = new oci.core.PublicIp("publicip", {
            compartmentId: values.tenancy,
            displayName: pulumi.interpolate`${values.project}-ip`,
            lifetime: "RESERVED",
            privateIpId: this.instance.privateIp,
        });
    }

    ip(): pulumi.Output<string> {
        return this.publicIp.ipAddress;
    }
}