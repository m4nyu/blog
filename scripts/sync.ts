#!/usr/bin/env tsx

import { execSync } from "node:child_process";
import { existsSync } from "node:fs";

class S3Syncer {
  private buildDir = "target/site";
  private regions = {
    usWest: "us-west-2",
    euWest: "eu-west-1",
  };

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
      throw new Error(`Failed to get Pulumi output '${outputName}'. Run deployment first. ${errorMessage}`);
    }
  }

  private checkPrerequisites(): void {
    if (!existsSync(this.buildDir)) {
      throw new Error(`Build directory ${this.buildDir} not found. Run 'bun run build' first.`);
    }

    try {
      execSync("aws --version", { stdio: "ignore" });
    } catch {
      throw new Error("AWS CLI not found. Please install and configure AWS CLI.");
    }

    try {
      execSync("aws sts get-caller-identity", { stdio: "ignore" });
    } catch {
      throw new Error("AWS credentials not configured. Run 'aws configure'.");
    }
  }

  async sync(): Promise<void> {
    try {
      console.log("📦 Starting S3 sync to both regions...\n");

      this.checkPrerequisites();

      const usWestBucket = this.getPulumiOutput("bucketUSWest");
      const euWestBucket = this.getPulumiOutput("bucketEUWest");

      console.log(`🇺🇸 Syncing to US West: ${usWestBucket}`);
      this.execCommand(
        `aws s3 sync ${this.buildDir}/ s3://${usWestBucket} --delete --region ${this.regions.usWest}`,
        "Syncing to US West S3"
      );

      console.log(`🇪🇺 Syncing to EU West: ${euWestBucket}`);
      this.execCommand(
        `aws s3 sync ${this.buildDir}/ s3://${euWestBucket} --delete --region ${this.regions.euWest}`,
        "Syncing to EU West S3"
      );

      console.log("✅ S3 sync completed successfully");
    } catch (error) {
      console.error("❌ S3 sync failed");
      console.error("Error:", error instanceof Error ? error.message : error);
      process.exit(1);
    }
  }
}

if (require.main === module) {
  const syncer = new S3Syncer();
  syncer.sync();
}
