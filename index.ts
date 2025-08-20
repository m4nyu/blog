import * as aws from "@pulumi/aws";
import * as cloudflare from "@pulumi/cloudflare";
import * as pulumi from "@pulumi/pulumi";

// Configuration
const config = new pulumi.Config("blog");
const domain = config.get("domain");

// AWS Providers for different regions
const uswest = new aws.Provider("uswest", {
  region: "us-west-2",
});

const euwest = new aws.Provider("euwest", {
  region: "eu-west-1",
});

// ===== US WEST RESOURCES =====

// S3 Bucket for US West
const usbucket = new aws.s3.Bucket(
  "usbucket",
  {
    bucket: `${pulumi.getStack()}-blog-uswest`,
  },
  { provider: uswest }
);

// Website configuration for US West bucket
const uswebsite = new aws.s3.BucketWebsiteConfigurationV2(
  "uswebsite",
  {
    bucket: usbucket.bucket,
    indexDocument: {
      suffix: "index.html",
    },
    errorDocument: {
      key: "index.html",
    },
  },
  { provider: uswest }
);

// Public access block for US West
new aws.s3.BucketPublicAccessBlock(
  "uspab",
  {
    bucket: usbucket.bucket,
    blockPublicAcls: false,
    blockPublicPolicy: false,
    ignorePublicAcls: false,
    restrictPublicBuckets: false,
  },
  { provider: uswest }
);

// ===== EU WEST RESOURCES =====

// S3 Bucket for EU West
const eubucket = new aws.s3.Bucket(
  "eubucket",
  {
    bucket: `${pulumi.getStack()}-blog-euwest`,
  },
  { provider: euwest }
);

// Website configuration for EU West bucket
const euwebsite = new aws.s3.BucketWebsiteConfigurationV2(
  "euwebsite",
  {
    bucket: eubucket.bucket,
    indexDocument: {
      suffix: "index.html",
    },
    errorDocument: {
      key: "index.html",
    },
  },
  { provider: euwest }
);

// Public access block for EU West
new aws.s3.BucketPublicAccessBlock(
  "eupab",
  {
    bucket: eubucket.bucket,
    blockPublicAcls: false,
    blockPublicPolicy: false,
    ignorePublicAcls: false,
    restrictPublicBuckets: false,
  },
  { provider: euwest }
);

// ===== CLOUDFRONT DISTRIBUTION =====

// Origin Access Control
const oac = new aws.cloudfront.OriginAccessControl("oac", {
  name: `${pulumi.getStack()}-blog-oac`,
  description: "OAC for blog multi-region",
  originAccessControlOriginType: "s3",
  signingBehavior: "always",
  signingProtocol: "sigv4",
});

// CloudFront Distribution with multiple origins
const distribution = new aws.cloudfront.Distribution("distribution", {
  origins: [
    // US West Origin
    {
      domainName: uswebsite.websiteEndpoint,
      originId: `S3-US-${usbucket.bucket}`,
      originAccessControlId: oac.id,
      customOriginConfig: {
        httpPort: 80,
        httpsPort: 443,
        originProtocolPolicy: "http-only",
        originSslProtocols: ["TLSv1.2"],
      },
    },
    // EU West Origin
    {
      domainName: euwebsite.websiteEndpoint,
      originId: `S3-EU-${eubucket.bucket}`,
      originAccessControlId: oac.id,
      customOriginConfig: {
        httpPort: 80,
        httpsPort: 443,
        originProtocolPolicy: "http-only",
        originSslProtocols: ["TLSv1.2"],
      },
    },
  ],
  enabled: true,
  isIpv6Enabled: true,
  defaultRootObject: "index.html",

  // Default cache behavior (US West)
  defaultCacheBehavior: {
    targetOriginId: pulumi.interpolate`S3-US-${usbucket.bucket}`,
    viewerProtocolPolicy: "redirect-to-https",
    allowedMethods: ["DELETE", "GET", "HEAD", "OPTIONS", "PATCH", "POST", "PUT"],
    cachedMethods: ["GET", "HEAD"],
    compress: true,
    forwardedValues: {
      queryString: false,
      cookies: {
        forward: "none",
      },
    },
    // Use managed caching and security policies
    cachePolicyId: "4135ea2d-6df8-44a3-9df3-4b5a84be39ad", // CachingOptimized
    responseHeadersPolicyId: "5cc3b908-e619-4b99-88e5-2cf7f45965bd", // SecurityHeadersPolicy
  },

  // Custom error responses for SPA routing
  customErrorResponses: [
    {
      errorCode: 404,
      responseCode: 200,
      responsePagePath: "/index.html",
      errorCachingMinTtl: 10,
    },
    {
      errorCode: 403,
      responseCode: 200,
      responsePagePath: "/index.html",
      errorCachingMinTtl: 10,
    },
  ],

  // Use all price classes for global distribution
  priceClass: "PriceClass_All",

  restrictions: {
    geoRestriction: {
      restrictionType: "none",
    },
  },

  viewerCertificate: domain
    ? {
        acmCertificateArn: undefined,
        cloudfrontDefaultCertificate: false,
        sslSupportMethod: "sni-only",
        minimumProtocolVersion: "TLSv1.2_2021",
      }
    : {
        cloudfrontDefaultCertificate: true,
      },
});

// Bucket policies for CloudFront access
new aws.s3.BucketPolicy(
  "uspolicy",
  {
    bucket: usbucket.bucket,
    policy: pulumi.all([usbucket.arn, distribution.arn]).apply(([bucketArn, distArn]) =>
      JSON.stringify({
        Version: "2012-10-17",
        Statement: [
          {
            Sid: "AllowCloudFrontServicePrincipal",
            Effect: "Allow",
            Principal: {
              Service: "cloudfront.amazonaws.com",
            },
            Action: "s3:GetObject",
            Resource: `${bucketArn}/*`,
            Condition: {
              StringEquals: {
                "AWS:SourceArn": distArn,
              },
            },
          },
        ],
      })
    ),
  },
  { provider: uswest }
);

new aws.s3.BucketPolicy(
  "eupolicy",
  {
    bucket: eubucket.bucket,
    policy: pulumi.all([eubucket.arn, distribution.arn]).apply(([bucketArn, distArn]) =>
      JSON.stringify({
        Version: "2012-10-17",
        Statement: [
          {
            Sid: "AllowCloudFrontServicePrincipal",
            Effect: "Allow",
            Principal: {
              Service: "cloudfront.amazonaws.com",
            },
            Action: "s3:GetObject",
            Resource: `${bucketArn}/*`,
            Condition: {
              StringEquals: {
                "AWS:SourceArn": distArn,
              },
            },
          },
        ],
      })
    ),
  },
  { provider: euwest }
);

// ===== CLOUDFLARE SECURITY =====

// Cloudflare configuration - requires existing zone
const cfconfig = new pulumi.Config("cloudflare");
const zoneid = cfconfig.get("zoneId");

if (domain && zoneid) {
  // DNS record pointing to CloudFront
  new cloudflare.Record("dns", {
    zoneId: zoneid,
    name: "@",
    value: distribution.domainName,
    type: "CNAME",
    proxied: true,
  });

  // WAF Rules for security
  new cloudflare.Ruleset("waf", {
    zoneId: zoneid,
    name: "Blog WAF Rules",
    description: "Web Application Firewall rules for blog security",
    kind: "zone",
    phase: "http_request_firewall_custom",
    rules: [
      {
        action: "block",
        expression:
          '(http.request.uri.path contains "wp-admin") or (http.request.uri.path contains "phpmyadmin") or (http.request.uri.path contains ".env")',
        description: "Block common attack paths",
        enabled: true,
      },
      {
        action: "challenge",
        expression: "(cf.client.bot) or (cf.threat_score > 14)",
        description: "Challenge suspicious requests and bots",
        enabled: true,
      },
      {
        action: "block",
        expression:
          '(http.request.uri.query contains "union") or (http.request.uri.query contains "select") or (http.request.uri.query contains "script")',
        description: "Block SQL injection attempts",
        enabled: true,
      },
    ],
  });

  // Page Rules for caching and security
  new cloudflare.PageRule("caching", {
    zoneId: zoneid,
    target: `${domain}/pkg/*`,
    priority: 1,
    status: "active",
    actions: {
      cacheLevel: "cache_everything",
      securityLevel: "medium",
      ssl: "strict",
    },
  });

  new cloudflare.PageRule("root", {
    zoneId: zoneid,
    target: `${domain}/*`,
    priority: 2,
    status: "active",
    actions: {
      securityLevel: "high",
      ssl: "strict",
    },
  });

  // SSL/TLS and security settings
  new cloudflare.ZoneSettingsOverride("ssl", {
    zoneId: zoneid,
    settings: {
      ssl: "strict",
      alwaysUseHttps: "on",
      minTlsVersion: "1.2",
      tls13: "on",
      automaticHttpsRewrites: "on",
      securityLevel: "high",
      challengeTtl: 1800,
      browserCheck: "on",
      hotlinkProtection: "on",
      emailObfuscation: "on",
      serverSideExclude: "on",
    },
  });
}

// ===== EXPORTS =====

export const bucketUSWest = usbucket.bucket;
export const bucketEUWest = eubucket.bucket;
export const distributionId = distribution.id;
export const usEndpoint = uswebsite.websiteEndpoint;
export const euEndpoint = euwebsite.websiteEndpoint;
export const websiteUrl = domain ? `https://${domain}` : pulumi.interpolate`https://${distribution.domainName}`;
export const cloudflareZone = zoneid;