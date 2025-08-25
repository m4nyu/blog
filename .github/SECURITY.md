# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in this project, please report it responsibly:

### Private Reporting (Recommended)
- Use GitHub's private vulnerability reporting feature
- Go to the "Security" tab in this repository
- Click "Report a vulnerability"

### Alternative Contact
- Email: [your-email@example.com] (replace with your actual email)
- Include detailed information about the vulnerability
- Provide steps to reproduce if possible

## Security Measures

This project implements several security measures:

### Infrastructure Security
- **Multi-region deployment** for high availability
- **CloudFront CDN** with HTTPS-only access
- **Cloudflare WAF** protection against common attacks
- **Origin Access Control** for secure S3 access
- **SSL/TLS encryption** end-to-end

### Dependency Security
- **Automated security audits** via GitHub Actions
- **Dependency review** on all pull requests
- **Regular updates** of dependencies
- **License compliance** checking

### Development Security
- **Strict TypeScript** compilation
- **Comprehensive linting** (Biome + Clippy)
- **Code formatting** enforcement
- **Conventional commits** for change tracking

## Security Automation

The following security checks run automatically:

- **Rust security audit** (`cargo audit`) on every push
- **Node.js security audit** (`bun audit`) on every push
- **Dependency review** on pull requests
- **Weekly scheduled** security scans
- **License compliance** verification

## Disclosure Timeline

- **Day 0**: Vulnerability reported
- **Day 1-3**: Initial triage and acknowledgment
- **Day 3-14**: Investigation and fix development
- **Day 14-21**: Fix testing and validation
- **Day 21+**: Public disclosure (after fix is released)

## Security Updates

Security updates will be:
- Released as soon as possible
- Documented in release notes
- Announced via GitHub releases
- Applied to supported versions

Thank you for helping keep this project secure! 🔒