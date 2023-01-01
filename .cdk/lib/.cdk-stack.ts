import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { WebResumeRustBucket } from "./bucket";
import { WebResumeRustDistribution } from "./distribution";
import { WebResumeRustZone } from "./zone";
import { WebResumeRustCertificate } from "./certificate";

export class WebResumeRustStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const rootDomainName = "vpurush.com";
    const profileSubDomainName = "profile.vpurush.com";

    const zone = new WebResumeRustZone(this, {
      domainName: rootDomainName,
    });

    const certificate = new WebResumeRustCertificate(this, {
      domainName: rootDomainName,
      zone: zone.zone,
    });

    const bucket = new WebResumeRustBucket(this, {
      bucketName: rootDomainName,
    });

    const distribution = new WebResumeRustDistribution(this, {
      domainName: rootDomainName,
      bucket: bucket.bucket,
      certificate: certificate.certificate,
    });

    bucket.deploy(this, {
      distribution: distribution.distribution,
    });

    zone.createCNAMEForRoutingSubDomainToDistribution(this, {
      domainName: profileSubDomainName,
      distribution: distribution.distribution,
    });
  }
}
