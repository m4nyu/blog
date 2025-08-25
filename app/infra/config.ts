import * as pulumi from "@pulumi/pulumi";

export interface Values {
    tenancy: pulumi.Output<string>;
    region: pulumi.Output<string>;
    project: pulumi.Output<string>;
    environment: pulumi.Output<string>;
    domain: pulumi.Output<string>;
    email: pulumi.Output<string>;
    leptosOutputName?: pulumi.Output<string>;
    leptosSiteAddr?: pulumi.Output<string>;
    rustLog?: pulumi.Output<string>;
}

export class Config {
    values(): Values {
        const config = new pulumi.Config();
        const oci = new pulumi.Config("oci");
        const dns = new pulumi.Config("dns");
        const tls = new pulumi.Config("tls");

        return {
            tenancy: oci.requireSecret("tenancyOcid"),
            region: config.requireSecret("region"),
            project: config.requireSecret("project"),
            environment: config.requireSecret("environment"),
            domain: dns.requireSecret("domain"),
            email: tls.requireSecret("acmeEmail"),
            leptosOutputName: pulumi.output(config.get("leptosOutputName") || "blog"),
            leptosSiteAddr: pulumi.output(config.get("leptosSiteAddr") || "0.0.0.0:3000"),
            rustLog: pulumi.output(config.get("rustLog") || "info"),
        };
    }
}