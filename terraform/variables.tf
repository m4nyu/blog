variable "ovh_application_key" {
  description = "OVH API application key"
  type        = string
  sensitive   = true
}

variable "ovh_application_secret" {
  description = "OVH API application secret"
  type        = string
  sensitive   = true
}

variable "ovh_consumer_key" {
  description = "OVH API consumer key"
  type        = string
  sensitive   = true
}

variable "ovh_endpoint" {
  description = "OVH API endpoint (ovh-eu, ovh-ca, ovh-us)"
  type        = string
  default     = "ovh-eu"
}

variable "service_name" {
  description = "OVH public cloud project ID"
  type        = string
}

variable "vps_region" {
  description = "OVH region for the VPS"
  type        = string
  default     = "GRA11"
}

variable "vps_flavor" {
  description = "VPS flavor (instance type)"
  type        = string
  default     = "d2-2"
}

variable "vps_image" {
  description = "OS image for the VPS"
  type        = string
  default     = "Ubuntu 24.04"
}

variable "ssh_public_key_path" {
  description = "Path to SSH public key for VPS access"
  type        = string
  default     = "~/.ssh/id_rsa.pub"
}

variable "domain" {
  description = "Custom domain for the blog (optional)"
  type        = string
  default     = ""
}

variable "blog_port" {
  description = "Port the blog server listens on"
  type        = number
  default     = 3000
}
