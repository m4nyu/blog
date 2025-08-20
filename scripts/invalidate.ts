#!/usr/bin/env tsx

import { execSync } from "node:child_process";

class CloudFrontInvalidator {
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

  async invalidate(): Promise<void> {
    try {
      console.log("🔄 Creating CloudFront invalidation...\n");

      this.checkPrerequisites();

      const distributionId = this.getPulumiOutput("distributionId");

      const result = this.execCommand(
        `aws cloudfront create-invalidation --distribution-id ${distributionId} --paths "/*"`,
        "Creating CloudFront invalidation"
      );

      // Parse invalidation ID for tracking
      try {
        const invalidationData = JSON.parse(result) as { Invalidation?: { Id?: string } };
        const invalidationId = invalidationData.Invalidation?.Id;

        if (invalidationId) {
          console.log(`📝 Invalidation ID: ${invalidationId}`);
          console.log("⏳ Invalidation typically takes 10-15 minutes to complete globally");
        }
      } catch (parseError) {
        // JSON parse failed, continue without invalidation ID
        console.log("⚠️  Could not parse invalidation response (this is not critical)");
      }

      console.log("✅ CloudFront invalidation created successfully");
    } catch (error) {
      console.error("❌ CloudFront invalidation failed");
      console.error("Error:", error instanceof Error ? error.message : error);
      process.exit(1);
    }
  }
}

if (require.main === module) {
  const invalidator = new CloudFrontInvalidator();
  invalidator.invalidate();
}
