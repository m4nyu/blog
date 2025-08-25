import * as pulumi from "@pulumi/pulumi";
import { Values } from "./config.js";

export class Dns {
    public internalHostname: pulumi.Output<string>;
    public externalInstructions: pulumi.Output<string>;

    constructor(values: Values, ip: pulumi.Output<string>) {
        const config = new pulumi.Config();
        
        // Generate random internal hostname for staging/green environment
        const isProduction = values.environment.apply(env => env === "production");
        
        // For production: use main domain (managed in Namecheap)
        // For staging: use random internal string (Oracle Cloud internal only)
        this.internalHostname = pulumi.all([values.environment, values.domain]).apply(
            ([env, domain]) => {
                if (env === "production") {
                    return domain; // m4nuel.blog
                } else {
                    // Generate random staging hostname - internal only
                    const randomId = Math.random().toString(36).substring(2, 15);
                    return `staging-${randomId}.internal`;
                }
            }
        );

        // Instructions for DNS setup
        this.externalInstructions = pulumi.all([values.environment, ip, values.domain]).apply(
            ([env, publicIp, domain]) => {
                if (env === "production") {
                    return `
🌐 BLUE (Production) DNS Setup Required:
===========================================

1. Login to Namecheap DNS settings for ${domain}
2. Create/Update A record:
   
   Type: A
   Host: @ (root domain)
   Value: ${publicIp}
   TTL: 300 (5 minutes for fast switching)

3. Production will be available at: https://${domain}

⚠️  You MUST configure this A record in Namecheap for the site to work!
`;
                } else {
                    return `
🟢 GREEN (Staging) Environment:
===============================

Internal staging environment deployed.
This is for internal testing only - not publicly accessible.
Access via Oracle Cloud internal network: ${publicIp}

Once production is deployed, use blue-green switching to promote.
`;
                }
            }
        );
    }
}