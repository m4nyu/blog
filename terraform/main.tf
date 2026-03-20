provider "ovh" {
  endpoint           = var.ovh_endpoint
  application_key    = var.ovh_application_key
  application_secret = var.ovh_application_secret
  consumer_key       = var.ovh_consumer_key
}

# ── SSH Key ─────────────────────────────────────────────────────────────────

resource "ovh_cloud_project_kube_nodepool" "this" {
  # Not used — placeholder for future k8s if needed
  count = 0
}

resource "ovh_cloud_project_ssh_key" "blog" {
  service_name = var.service_name
  name         = "blog-deploy-key"
  public_key   = file(pathexpand(var.ssh_public_key_path))
}

# ── VPS Instance ────────────────────────────────────────────────────────────

resource "ovh_cloud_project_instance" "blog" {
  service_name = var.service_name
  name         = "blog-vps"
  region       = var.vps_region
  flavor_name  = var.vps_flavor
  image_name   = var.vps_image

  ssh_key      = ovh_cloud_project_ssh_key.blog.name

  user_data = templatefile("${path.module}/cloud-init.yaml", {
    blog_port = var.blog_port
  })
}

# ── Firewall / Security Group ───────────────────────────────────────────────

resource "ovh_cloud_project_network_private" "blog_net" {
  service_name = var.service_name
  name         = "blog-network"
  regions      = [var.vps_region]
}
