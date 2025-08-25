import * as oci from "@pulumi/oci";
import * as pulumi from "@pulumi/pulumi";
import { Values } from "./config.js";

export class Network {
    public vcn: oci.core.Vcn;
    public publicsubnet: oci.core.Subnet;

    constructor(values: Values) {
        // Single VCN (Always Free limit: 2 VCNs)
        this.vcn = new oci.core.Vcn("vcn", {
            compartmentId: values.tenancy,
            displayName: pulumi.interpolate`${values.project}-vcn`,
            cidrBlocks: ["10.0.0.0/16"],
            dnsLabel: "vcn",
        });

        // Internet Gateway for public access
        const gateway = new oci.core.InternetGateway("gateway", {
            compartmentId: values.tenancy,
            vcnId: this.vcn.id,
            displayName: pulumi.interpolate`${values.project}-gateway`,
            enabled: true,
        });

        // Route table for public subnet
        const publicroute = new oci.core.RouteTable("publicroute", {
            compartmentId: values.tenancy,
            vcnId: this.vcn.id,
            displayName: pulumi.interpolate`${values.project}-route`,
            routeRules: [{
                destination: "0.0.0.0/0",
                destinationType: "CIDR_BLOCK",
                networkEntityId: gateway.id,
            }],
        });

        // Security list allowing HTTP/HTTPS and SSH
        const security = new oci.core.SecurityList("security", {
            compartmentId: values.tenancy,
            vcnId: this.vcn.id,
            displayName: pulumi.interpolate`${values.project}-security`,
            egressSecurityRules: [{
                destination: "0.0.0.0/0",
                protocol: "all",
                stateless: false,
            }],
            ingressSecurityRules: [
                {
                    source: "0.0.0.0/0",
                    protocol: "6",
                    stateless: false,
                    tcpOptions: { min: 80, max: 80 },
                },
                {
                    source: "0.0.0.0/0",
                    protocol: "6",
                    stateless: false,
                    tcpOptions: { min: 443, max: 443 },
                },
                {
                    source: "0.0.0.0/0",
                    protocol: "6",
                    stateless: false,
                    tcpOptions: { min: 22, max: 22 },
                },
            ],
        });

        // Single public subnet for the compute instance
        this.publicsubnet = new oci.core.Subnet("publicsubnet", {
            compartmentId: values.tenancy,
            vcnId: this.vcn.id,
            cidrBlock: "10.0.1.0/24",
            displayName: pulumi.interpolate`${values.project}-public`,
            routeTableId: publicroute.id,
            securityListIds: [security.id],
            prohibitInternetIngress: false,
            prohibitPublicIpOnVnic: false,
            dnsLabel: "public",
        });
    }
}