# Blue-Green Deployment Guide

This infrastructure supports blue-green deployments on Oracle Cloud Always Free tier with Namecheap DNS management.

## Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│             Namecheap DNS Management                │
│  m4nuel.blog -> A Record -> Production IP           │
│  (Manual DNS update required)                       │
└─────────────────────────────────────────────────────┘
           │                           │
           ▼                           ▼
┌─────────────────────┐    ┌─────────────────────────┐
│  BLUE (Production)  │    │  GREEN (Staging)        │
│  Stack: production  │    │  Stack: staging         │
│  m4nuel.blog        │    │  internal-staging-xyz   │
│  Public via DNS     │    │  Internal Oracle only   │
│  Always Free ARM    │    │  Always Free ARM        │
│  Docker + nginx     │    │  Docker + nginx         │
│  SSL Auto-renewal   │    │  SSL Auto-renewal       │
└─────────────────────┘    └─────────────────────────┘
```

## Deployment Workflow

### 1. Deploy to Green (Staging)
```bash
tsx app/infra/deploy.ts deploy
```
- Deploys new version to internal staging stack
- Creates internal staging environment (not publicly accessible)
- Keeps production (blue) running

### 2. Test Green Environment
- Access: Internal Oracle Cloud network only
- SSH to staging instance for testing
- Validate functionality internally

### 3. Deploy to Production
```bash
tsx app/infra/deploy.ts deploy-blue
# OR
tsx app/infra/deploy.ts promote
```
- Deploys to production stack
- Provides Namecheap DNS instructions
- Manual A record update required

### 4. Rollback (if needed)
```bash
tsx app/infra/deploy.ts rollback
```
- Shows rollback options and instructions
- Requires redeployment of previous version
- Or manual A record change in Namecheap

### 5. Check Status
```bash
tsx app/infra/deploy.ts status
```
Shows current deployment status and URLs.

## Always Free Tier Compliance

Each environment uses:
- ✅ 1x ARM Ampere A1 instance (4 cores, 24GB RAM)
- ✅ 100GB boot storage (within 200GB limit)
- ✅ 1x VCN with public subnet
- ✅ 1x reserved public IP
- ✅ OCI Container Registry
- ✅ Oracle DNS with A/CNAME records

**Total usage: 2 ARM instances, 2 VCNs, 2 IPs - within Always Free limits**

## DNS Configuration

### Namecheap Management Required
- `m4nuel.blog` -> A record pointing to production IP
- DNS managed in Namecheap dashboard
- TTL: 300 seconds (5 minutes) for reasonable switching time

### Internal Staging
- Internal hostnames only (not public DNS)
- Accessible via Oracle Cloud internal network
- Random string hostnames for security

## Container Deployment

Both environments run:
```bash
docker run -d --name blog --restart always \
  -p 3000:3000 \
  -e LEPTOS_OUTPUT_NAME="blog" \
  -e LEPTOS_SITE_ADDR="0.0.0.0:3000" \
  -e RUST_LOG="info" \
  us-phoenix-1.ocir.io/m4nuel-blog/blog:latest
```

With nginx reverse proxy and automatic SSL via Let's Encrypt.

## Manual Promotion Benefits

- **Cost Effective**: Uses Always Free tier resources
- **Testing Isolation**: Green environment completely internal
- **Production Control**: Manual DNS management prevents accidents
- **Simple Operations**: No complex DNS orchestration
- **Secure Staging**: Internal-only staging environment

## Troubleshooting

### If ARM capacity is unavailable:
1. Wait for ARM capacity to become available in us-phoenix-1
2. Monitor Oracle Cloud status for capacity availability
3. ARM instances are in high demand - may need multiple deployment attempts

### DNS propagation issues:
- DNS TTL is set to 60 seconds
- Use `dig m4nuel.blog` to check propagation
- Clear browser DNS cache if needed

### Container registry access:
- Ensure OCI credentials are configured for us-phoenix-1
- Push latest image before deployment:
  ```bash
  docker push us-phoenix-1.ocir.io/m4nuel-blog/blog:latest
  ```

## Security

- ✅ Always Free tier resources only
- ✅ HTTPS with automatic Let's Encrypt certificates
- ✅ Security groups allow only HTTP/HTTPS/SSH
- ✅ Container runs as non-root user
- ✅ All secrets managed via Pulumi configuration