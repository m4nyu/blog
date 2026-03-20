variable "cloudflare_api_token" {
  description = "Cloudflare API token with Zone:Read, DNS:Edit permissions"
  type        = string
  sensitive   = true
  default     = ""
}

variable "cloudflare_zone_id" {
  description = "Cloudflare zone ID for the domain"
  type        = string
  default     = ""
}

locals {
  use_cloudflare = var.cloudflare_api_token != "" && var.cloudflare_zone_id != "" && var.domain != ""
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

# ── DNS A record pointing domain to VPS ─────────────────────────────────────

resource "cloudflare_record" "blog_a" {
  count   = local.use_cloudflare ? 1 : 0
  zone_id = var.cloudflare_zone_id
  name    = var.domain
  content = ovh_cloud_project_instance.blog.addresses[0].ip
  type    = "A"
  proxied = true
  ttl     = 1 # Auto when proxied
}

# ── DNS AAAA record if IPv6 available ───────────────────────────────────────

resource "cloudflare_record" "blog_aaaa" {
  count   = local.use_cloudflare && length(ovh_cloud_project_instance.blog.addresses) > 1 ? 1 : 0
  zone_id = var.cloudflare_zone_id
  name    = var.domain
  content = ovh_cloud_project_instance.blog.addresses[1].ip
  type    = "AAAA"
  proxied = true
  ttl     = 1
}

# ── www redirect ────────────────────────────────────────────────────────────

resource "cloudflare_record" "blog_www" {
  count   = local.use_cloudflare ? 1 : 0
  zone_id = var.cloudflare_zone_id
  name    = "www"
  content = ovh_cloud_project_instance.blog.addresses[0].ip
  type    = "A"
  proxied = true
  ttl     = 1
}

# ── SSL/TLS mode ────────────────────────────────────────────────────────────

resource "cloudflare_zone_settings_override" "blog_ssl" {
  count   = local.use_cloudflare ? 1 : 0
  zone_id = var.cloudflare_zone_id

  settings {
    ssl                      = "full"
    always_use_https         = "on"
    min_tls_version          = "1.2"
    automatic_https_rewrites = "on"
    brotli                   = "on"
    minify {
      css  = "on"
      js   = "on"
      html = "on"
    }
  }
}

# ── Cache rules ─────────────────────────────────────────────────────────────

resource "cloudflare_ruleset" "blog_cache" {
  count   = local.use_cloudflare ? 1 : 0
  zone_id = var.cloudflare_zone_id
  name    = "Blog cache rules"
  kind    = "zone"
  phase   = "http_request_cache_settings"

  rules {
    action = "set_cache_settings"
    action_parameters {
      cache = true
      edge_ttl {
        mode    = "override_origin"
        default = 86400 # 1 day for static assets
      }
      browser_ttl {
        mode    = "override_origin"
        default = 3600 # 1 hour browser cache
      }
    }
    expression  = "(http.request.uri.path matches \"^/pkg/.*\")"
    description = "Cache static assets (WASM, JS, CSS)"
    enabled     = true
  }
}
