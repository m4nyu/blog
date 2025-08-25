import * as oci from "@pulumi/oci";
import * as pulumi from "@pulumi/pulumi";
import { Values } from "./config.js";

export class Registry {
    public registry: oci.artifacts.ContainerRepository;
    public namespace: pulumi.Output<string>;

    constructor(private values: Values) {
        this.registry = new oci.artifacts.ContainerRepository("registry", {
            compartmentId: values.tenancy,
            displayName: pulumi.interpolate`${values.project}/blog`,
            isPublic: false,
        });

        // Get the registry namespace for image URLs
        this.namespace = values.tenancy.apply(tenancyId => 
            oci.artifacts.getContainerConfiguration({
                compartmentId: tenancyId,
            }).then(config => config.namespace)
        );
    }

    imageUrl(tag: string = "latest"): pulumi.Output<string> {
        return pulumi.all([this.values.region, this.namespace, this.values.project]).apply(
            ([region, ns, project]) => `${region}.ocir.io/${ns}/${project}/blog:${tag}`
        );
    }
}