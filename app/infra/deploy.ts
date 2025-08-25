#!/usr/bin/env tsx

import { execSync } from "child_process";
import { readFileSync, writeFileSync } from "fs";

interface DeploymentConfig {
    blueStack: string;
    greenStack: string;
    domain: string;
    passphrase: string;
}

class BlueGreenDeployment {
    private config: DeploymentConfig;

    constructor() {
        this.config = {
            blueStack: "production",
            greenStack: "staging", 
            domain: "m4nuel.blog",
            passphrase: "blog_0105_1706_1610"
        };
    }

    private runCommand(command: string): string {
        console.log(`Running: ${command}`);
        return execSync(command, { 
            encoding: "utf-8",
            env: { ...process.env, PULUMI_CONFIG_PASSPHRASE: this.config.passphrase }
        });
    }


    private deployProduction(): void {
        console.log(`🔄 Deploying to BLUE (production) environment...`);
        
        try {
            // Deploy production stack
            this.runCommand(`pulumi up --yes -s ${this.config.blueStack}`);
            
            const productionIp = this.runCommand(`pulumi stack output publicIp -s ${this.config.blueStack}`);
            const dnsInstructions = this.runCommand(`pulumi stack output dnsInstructions -s ${this.config.blueStack}`);
            
            console.log(`✅ BLUE (production) deployment complete`);
            console.log(`${dnsInstructions}`);
            
        } catch (error) {
            console.error("❌ Production deployment failed:", error);
            throw error;
        }
    }

    public async deployToGreen(): Promise<void> {
        console.log("🚀 Deploying to GREEN (staging) environment...");
        
        try {
            // Deploy to green stack
            this.runCommand(`pulumi up --yes -s ${this.config.greenStack}`);
            
            const greenOutput = this.runCommand(`pulumi stack output publicIp -s ${this.config.greenStack}`);
            const greenIp = greenOutput.trim();
            
            const dnsInstructions = this.runCommand(`pulumi stack output dnsInstructions -s ${this.config.greenStack}`);
            
            console.log(`✅ GREEN deployment complete in us-phoenix-1`);
            console.log(`🔗 Internal staging IP: ${greenIp}`);
            console.log(`📝 This is internal staging - test via Oracle Cloud network`);
            console.log(`\n${dnsInstructions}`);
            
        } catch (error) {
            console.error("❌ Green deployment failed:", error);
            throw error;
        }
    }

    public async promoteGreenToBlue(): Promise<void> {
        console.log("🔄 Promoting GREEN (staging) to BLUE (production)...");
        console.log("⚠️  This replaces the current production deployment!");
        
        try {
            // Deploy the staging environment configuration to production stack
            console.log("📦 Deploying staging build to production stack...");
            this.deployProduction();
            
            console.log("✅ GREEN promoted to BLUE (production)");
            console.log(`🌐 https://${this.config.domain} now serves the promoted environment`);
            console.log(`📝 Remember to update Namecheap A record if IP changed!`);
        } catch (error) {
            console.error("❌ Promotion failed:", error);
            throw error;
        }
    }

    public async rollbackToBlue(): Promise<void> {
        console.log("⚡ ROLLBACK: This requires manual restoration from backup or previous deployment");
        console.log("⚠️  In this architecture, rollback means redeploying previous version to production");
        
        console.log("\n📋 Rollback Options:");
        console.log("1. Redeploy previous container image tag to production");
        console.log("2. Restore from infrastructure backup");
        console.log("3. Keep current production, fix issues in staging first");
        
        console.log("\n🔄 To rollback, use:");
        console.log(`tsx app/infra/deploy.ts deploy-blue`);
        console.log("(After ensuring previous good state is ready)");
    }

    public showStatus(): void {
        console.log("📊 Blue-Green Deployment Status");
        console.log("================================");
        
        try {
            console.log(`🔴 Production: BLUE environment (https://${this.config.domain})`);
            console.log(`🟢 Staging: GREEN environment (internal only)`);
            
            // Get IPs from both stacks (suppress error output)
            try {
                const blueIp = execSync(`pulumi stack output publicIp -s ${this.config.blueStack}`, { 
                    encoding: "utf-8", 
                    stdio: ["ignore", "pipe", "ignore"],
                    env: { ...process.env, PULUMI_CONFIG_PASSPHRASE: this.config.passphrase }
                }).trim();
                console.log(`🔵 BLUE (${this.config.blueStack}): https://${this.config.domain} (${blueIp})`);
            } catch {
                console.log(`🔵 BLUE (${this.config.blueStack}): Not deployed`);
            }
            
            try {
                const greenIp = execSync(`pulumi stack output publicIp -s ${this.config.greenStack}`, { 
                    encoding: "utf-8", 
                    stdio: ["ignore", "pipe", "ignore"],
                    env: { ...process.env, PULUMI_CONFIG_PASSPHRASE: this.config.passphrase }
                }).trim();
                console.log(`🟢 GREEN (${this.config.greenStack}): Internal staging (${greenIp})`);
            } catch {
                console.log(`🟢 GREEN (${this.config.greenStack}): Not deployed`);
            }
            
            console.log(`🌐 Production: https://${this.config.domain} (via Namecheap DNS)`);
            
        } catch (error) {
            console.error("❌ Failed to get status:", error);
        }
    }
}

// CLI Interface
const deployment = new BlueGreenDeployment();

async function main() {
    const command = process.argv[2];
    
    switch (command) {
        case "deploy":
        case "deploy-green":
            await deployment.deployToGreen();
            break;
        case "deploy-blue":
        case "deploy-production":
            deployment.deployProduction();
            break;
        case "promote":
            await deployment.promoteGreenToBlue();
            break;
        case "rollback":
            await deployment.rollbackToBlue();
            break;
        case "status":
            deployment.showStatus();
            break;
        default:
            console.log("Blue-Green Deployment Manager");
            console.log("============================");
            console.log("Usage:");
            console.log("  tsx app/infra/deploy.ts deploy        - Deploy to GREEN (staging)");
            console.log("  tsx app/infra/deploy.ts deploy-blue   - Deploy to BLUE (production)");
            console.log("  tsx app/infra/deploy.ts promote       - Promote GREEN to BLUE");
            console.log("  tsx app/infra/deploy.ts rollback      - Rollback info");
            console.log("  tsx app/infra/deploy.ts status        - Show current status");
            console.log("");
            console.log("Workflow:");
            console.log("1. deploy        -> Deploy to internal staging");
            console.log("2. Test staging environment internally");
            console.log("3. deploy-blue   -> Deploy to production (or promote)");
            console.log("4. Configure Namecheap A record to point to production IP");
            console.log("5. https://m4nuel.blog will serve production");
    }
}

main().catch(console.error);