import { Network } from "./network.js";
import { Compute } from "./cluster.js";
import { Registry } from "./registry.js";
import { Apps } from "./apps.js";
import { Dns } from "./dns.js";
import * as pulumi from "@pulumi/pulumi";

export class App {
    public network: Network;
    public compute: Compute;
    public registry: Registry;
    public apps: Apps;
    public dns: Dns;

    constructor(
        tenancy: pulumi.Output<string>,
        region: pulumi.Output<string>,
        project: pulumi.Output<string>,
        environment: pulumi.Output<string>,
        domain: pulumi.Output<string>,
        email: pulumi.Output<string>
    ) {
        const values = { tenancy, region, project, environment, domain, email };
        
        // Always Free tier infrastructure  
        this.network = new Network(values);
        this.compute = new Compute(values, this.network);
        this.registry = new Registry(values);
        this.apps = new Apps(values);
        this.dns = new Dns(values, this.compute.ip());
    }
}