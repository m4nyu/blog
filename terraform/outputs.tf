output "vps_ip" {
  description = "Public IP address of the blog VPS"
  value       = ovh_cloud_project_instance.blog.addresses[0].ip
}

output "vps_name" {
  description = "Name of the VPS instance"
  value       = ovh_cloud_project_instance.blog.name
}

output "blog_url" {
  description = "URL to access the blog"
  value       = var.domain != "" ? "https://${var.domain}" : "http://${ovh_cloud_project_instance.blog.addresses[0].ip}:${var.blog_port}"
}

output "ssh_command" {
  description = "SSH command to connect to the VPS"
  value       = "ssh ubuntu@${ovh_cloud_project_instance.blog.addresses[0].ip}"
}

output "cloudflare_enabled" {
  description = "Whether Cloudflare DNS/proxy is active"
  value       = local.use_cloudflare
}

output "cloudflare_dns_record" {
  description = "Cloudflare DNS record ID"
  value       = local.use_cloudflare ? cloudflare_record.blog_a[0].id : "not configured"
}
