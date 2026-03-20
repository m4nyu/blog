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
