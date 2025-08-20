# Oracle Cloud Deployment Guide

This guide will help you deploy your Leptos blog to Oracle Cloud Infrastructure (OCI) using the Always Free tier.

## Prerequisites

1. **Oracle Cloud Account**: Sign up at [cloud.oracle.com](https://cloud.oracle.com)
2. **OCI CLI Setup**: Install and configure the OCI CLI
3. **Pulumi CLI**: Install Pulumi CLI from [pulumi.com](https://pulumi.com)

## Oracle Cloud Always Free Tier

Your blog will use these free tier resources:
- **VM.Standard.A1.Flex**: ARM-based compute with 1 OCPU and 6GB RAM
- **10GB boot volume**: For the operating system
- **Unlimited bandwidth**: No egress charges
- **Public IP**: One reserved public IP address

## Step 1: Set up OCI API Keys

1. Log into OCI Console
2. Go to Profile → User Settings → API Keys
3. Click "Add API Key"
4. Generate or upload a public key
5. Note down the configuration details shown

## Step 2: Configure Pulumi

Set the required configuration values:

```bash
# Set your OCI region (e.g., us-phoenix-1, us-ashburn-1, eu-frankfurt-1)
pulumi config set oci:region us-phoenix-1

# Set your OCI credentials
pulumi config set --secret oci:tenancyOcid <your-tenancy-ocid>
pulumi config set --secret oci:userOcid <your-user-ocid>
pulumi config set --secret oci:fingerprint <your-api-key-fingerprint>
pulumi config set --secret oci:privateKey "$(cat ~/.oci/oci_api_key.pem)"

# Set your domain (optional)
pulumi config set blog:domain m4nuel.blog
```

## Step 3: Deploy Infrastructure

Run the deployment:

```bash
# Install dependencies
npm install

# Deploy to OCI
pulumi up
```

The deployment will create:
- Virtual Cloud Network (VCN) with public subnet
- Security lists allowing HTTP, HTTPS, and SSH traffic
- VM.Standard.A1.Flex compute instance (ARM-based, Always Free)
- Reserved public IP address
- Automatic deployment of your blog via cloud-init

## Step 4: Configure DNS

After deployment, Pulumi will output the public IP address. Configure your DNS:

1. Go to your domain registrar (e.g., Cloudflare, Route53)
2. Add an A record pointing your domain to the instance IP
3. Wait for DNS propagation (5-10 minutes)

## Step 5: Set up SSL Certificate

SSH into your instance and set up HTTPS:

```bash
# SSH into the instance (use the command from Pulumi output)
ssh -i blog-key.pem ubuntu@<instance-ip>

# Set up SSL certificate with Let's Encrypt
sudo certbot --nginx -d m4nuel.blog

# Verify the blog is running
systemctl status blog
```

## Monitoring and Maintenance

### Check Blog Status
```bash
sudo systemctl status blog
sudo journalctl -u blog -f
```

### Update the Blog
```bash
cd /opt/blog
git pull
cargo leptos build --release
sudo systemctl restart blog
```

### Nginx Configuration
```bash
sudo nginx -t  # Test configuration
sudo systemctl reload nginx
```

## Troubleshooting

### If the blog fails to start:
1. Check the systemd logs: `sudo journalctl -u blog -f`
2. Verify the build completed: `ls -la /opt/blog/target/release/tailwind`
3. Check if port 4002 is available: `sudo lsof -i :4002`

### If SSL certificate fails:
1. Ensure your domain points to the instance IP
2. Check if port 80/443 are accessible: `sudo ufw status`
3. Retry certificate generation: `sudo certbot --nginx -d your-domain.com`

## Cost Monitoring

The Always Free tier includes:
- ✅ VM.Standard.A1.Flex (1 OCPU, 6GB RAM) - Always Free
- ✅ 10GB boot volume - Always Free  
- ✅ Reserved public IP - Always Free
- ✅ Unlimited outbound transfer - Always Free

Your blog should cost **$0/month** indefinitely on the Always Free tier.

## Security Best Practices

The deployment automatically configures:
- UFW firewall with minimal required ports
- Nginx reverse proxy
- Automatic security updates
- Non-root user for the blog service

Consider additional hardening:
- Change SSH port from 22
- Set up fail2ban for intrusion prevention
- Configure log monitoring
- Regular security updates: `sudo apt update && sudo apt upgrade`