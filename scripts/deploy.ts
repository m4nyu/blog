#!/usr/bin/env tsx

import { execSync } from "node:child_process";
import { existsSync } from "node:fs";
import * as path from "node:path";

interface DeploymentConfig {
  buildDir: string;
  regions: {
    usWest: string;
    euWest: string;
  };
}

class BlogDeployer {
  private config: DeploymentConfig;
  private stackName: string;

  constructor() {
    this.config = {
      buildDir: "target/site",
      regions: {
        usWest: "us-west-2",
        euWest: "eu-west-1",
      },
    };

    // Get current Pulumi stack
    try {
      this.stackName = execSync("pulumi stack --show-name", { encoding: "utf8" }).trim();
    } catch (error) {
      console.error("❌ Failed to get Pulumi stack name. Make sure you're in a Pulumi project.");
      process.exit(1);
    }
  }

  private execCommand(command: string, description: string): string {
    console.log(`🔄 ${description}...`);
    try {
      const result = execSync(command, {
        encoding: "utf8",
        stdio: ["inherit", "pipe", "inherit"],
      });
      return result.trim();
    } catch (error) {
      console.error(`❌ Failed: ${description}`);
      console.error(`Command: ${command}`);
      throw error;
    }
  }

  private async checkPrerequisites(): Promise<void> {
    console.log("🔍 Checking prerequisites...");

    // Check if build directory exists
    if (!existsSync(this.config.buildDir)) {
      throw new Error(`Build directory ${this.config.buildDir} not found. Run 'bun run build' first.`);
    }

    // Check AWS CLI
    try {
      execSync("aws --version", { stdio: "ignore" });
    } catch {
      throw new Error("AWS CLI not found. Please install and configure AWS CLI.");
    }

    // Check Pulumi CLI
    try {
      execSync("pulumi version", { stdio: "ignore" });
    } catch {
      throw new Error("Pulumi CLI not found. Please install Pulumi CLI.");
    }

    // Verify AWS credentials
    try {
      execSync("aws sts get-caller-identity", { stdio: "ignore" });
      console.log("✅ AWS credentials verified");
    } catch {
      throw new Error("AWS credentials not configured. Run 'aws configure'.");
    }

    console.log("✅ Prerequisites check passed");
  }

  private async deployInfrastructure(): Promise<void> {
    console.log("☁️  Deploying infrastructure with Pulumi...");

    try {
      // Run pulumi up
      this.execCommand("pulumi up --yes --skip-preview", "Deploying infrastructure");
      console.log("✅ Infrastructure deployment complete");
    } catch (error) {
      console.error("❌ Infrastructure deployment failed");
      throw error;
    }
  }

  private getPulumiOutput(outputName: string): string {
    try {
      const result = execSync(`pulumi stack output ${outputName}`, { encoding: "utf8" }).trim();
      if (!result) {
        throw new Error(`Empty output for '${outputName}'`);
      }
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      throw new Error(`Failed to get Pulumi output '${outputName}': ${errorMessage}`);
    }
  }

  private async syncToS3(): Promise<void> {
    console.log("📦 Syncing files to S3 buckets...");

    try {
      // Get bucket names from Pulumi outputs
      const usWestBucket = this.getPulumiOutput("bucketUSWest");
      const euWestBucket = this.getPulumiOutput("bucketEUWest");

      console.log(`🇺🇸 Syncing to US West bucket: ${usWestBucket}`);
      this.execCommand(
        `aws s3 sync ${this.config.buildDir}/ s3://${usWestBucket} --delete --region ${this.config.regions.usWest}`,
        "Syncing to US West S3 bucket"
      );

      console.log(`🇪🇺 Syncing to EU West bucket: ${euWestBucket}`);
      this.execCommand(
        `aws s3 sync ${this.config.buildDir}/ s3://${euWestBucket} --delete --region ${this.config.regions.euWest}`,
        "Syncing to EU West S3 bucket"
      );

      console.log("✅ S3 sync complete");
    } catch (error) {
      console.error("❌ S3 sync failed");
      throw error;
    }
  }

  private async invalidateCloudFront(): Promise<void> {
    console.log("🔄 Creating CloudFront invalidation...");

    try {
      const distributionId = this.getPulumiOutput("distributionId");

      const invalidationCommand = `aws cloudfront create-invalidation --distribution-id ${distributionId} --paths "/*"`;
      const result = this.execCommand(invalidationCommand, "Creating CloudFront invalidation");

      // Parse invalidation ID for tracking
      try {
        const invalidationData = JSON.parse(result) as { Invalidation?: { Id?: string } };
        const invalidationId = invalidationData.Invalidation?.Id;

        if (invalidationId) {
          console.log(`📝 Invalidation ID: ${invalidationId}`);
          console.log("⏳ Note: Invalidation may take 10-15 minutes to complete globally");
        }
      } catch (parseError) {
        // JSON parse failed, continue without invalidation ID
        console.log("⚠️  Could not parse invalidation response (this is not critical)");
      }

      console.log("✅ CloudFront invalidation created");
    } catch (error) {
      console.error("❌ CloudFront invalidation failed");
      throw error;
    }
  }

  private async displayResults(): Promise<void> {
    console.log("\n🎉 Deployment Summary");
    console.log("=".repeat(50));

    try {
      const websiteUrl = this.getPulumiOutput("websiteUrl");
      const distributionId = this.getPulumiOutput("distributionId");
      const usWestEndpoint = this.getPulumiOutput("usEndpoint");
      const euWestEndpoint = this.getPulumiOutput("euEndpoint");

      console.log(`🌐 Website URL: ${websiteUrl}`);
      console.log(`📡 CloudFront Distribution: ${distributionId}`);
      console.log(`🇺🇸 US West Endpoint: https://${usWestEndpoint}`);
      console.log(`🇪🇺 EU West Endpoint: https://${euWestEndpoint}`);

      // Try to get Cloudflare info if available
      try {
        const cloudflareZoneId = this.getPulumiOutput("cloudflareZone");
        if (cloudflareZoneId && cloudflareZoneId !== "undefined") {
          console.log(`🛡️  Cloudflare Zone ID: ${cloudflareZoneId}`);

          // Note: nameServers output no longer available in simplified setup
        }
      } catch {
        // Cloudflare outputs not available (domain not configured)
      }

      console.log("\n🛡️  Security Features Enabled:");
      console.log("  • Multi-region S3 deployment");
      console.log("  • CloudFront global CDN");
      console.log("  • SSL/TLS encryption");
      console.log("  • Cloudflare DDoS protection (if domain configured)");
      console.log("  • WAF security rules (if domain configured)");
    } catch (error) {
      console.error("❌ Failed to get deployment outputs:", error);
    }
  }

  public async deploy(): Promise<void> {
    const startTime = Date.now();

    try {
      console.log("🚀 Starting blog deployment...\n");

      await this.checkPrerequisites();
      await this.deployInfrastructure();
      await this.syncToS3();
      await this.invalidateCloudFront();
      await this.displayResults();

      const duration = ((Date.now() - startTime) / 1000).toFixed(1);
      console.log(`\n✅ Deployment completed successfully in ${duration}s`);
    } catch (error) {
      const duration = ((Date.now() - startTime) / 1000).toFixed(1);
      console.error(`\n❌ Deployment failed after ${duration}s`);
      console.error("Error:", error instanceof Error ? error.message : error);
      process.exit(1);
    }
  }

  public async preview(): Promise<void> {
    try {
      console.log("👀 Previewing infrastructure changes...\n");
      await this.checkPrerequisites();
      this.execCommand("pulumi preview", "Generating preview");
    } catch (error) {
      console.error("❌ Preview failed");
      console.error("Error:", error instanceof Error ? error.message : error);
      process.exit(1);
    }
  }
}

// CLI Interface
async function main() {
  const deployer = new BlogDeployer();

  const command = process.argv[2];

  switch (command) {
    case "preview":
      await deployer.preview();
      break;
    case "deploy":
    case undefined:
      await deployer.deploy();
      break;
    default:
      console.log("Usage: tsx deploy.ts [preview|deploy]");
      process.exit(1);
  }
}

if (require.main === module) {
  main().catch((error) => {
    console.error("Unexpected error:", error);
    process.exit(1);
  });
}
