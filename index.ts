import * as pulumi from "@pulumi/pulumi";
import { Config } from "./app/infra/config.js";
import { App } from "./app/infra/app.js";

// Get config values from Pulumi
const config = new Config();
const values = config.values();

// Create app with explicit key-value pairs
const app = new App(
    values.tenancy,
    values.region,
    values.project,
    values.environment,
    values.domain,
    values.email
);

// Pulumi stack exports for Always Free tier
export const vcn = app.network.vcn.id;
export const instance = app.compute.instance.id;
export const publicIp = app.compute.publicIp.ipAddress;
export const registry = app.registry.registry.id;
export const registryUrl = app.registry.imageUrl();
export const namespace = app.registry.namespace;
export const internalHostname = app.dns.internalHostname;
export const dnsInstructions = app.dns.externalInstructions;
export const domain = pulumi.interpolate`https://${values.domain}`;
export const setupScript = app.apps.setupScript;
export const tls = "letsencrypt-enabled";
export const status = pulumi.all([
    app.compute.instance.id,
    app.compute.publicIp.ipAddress,
    app.registry.registry.id,
    values.environment,
]).apply(([instanceId, ip, registryId, env]) => ({
    ready: !!(instanceId && ip && registryId),
    environment: env,
    instance: instanceId ? "READY" : "PENDING",
    ip: ip || "PENDING",
    registry: registryId ? "READY" : "PENDING",
    nginx: "CONFIGURED",
    tls: "ENABLED",
    containerized: "YES",
    alwaysFree: "YES",
    dnsManagement: env === "production" ? "NAMECHEAP_REQUIRED" : "INTERNAL_ONLY",
}));