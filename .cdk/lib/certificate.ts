import { Construct } from "constructs";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as s3Deploy from "aws-cdk-lib/aws-s3-deployment";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as cloudfrontOrigins from "aws-cdk-lib/aws-cloudfront-origins";
import * as acm from "aws-cdk-lib/aws-certificatemanager";
import * as route53 from "aws-cdk-lib/aws-route53";
import { RemovalPolicy } from "aws-cdk-lib";

type WebResumeRustCertificateProps = {
  domainName: string;
  zone: route53.IHostedZone;
};

export class WebResumeRustCertificate extends Construct {
  certificate: acm.DnsValidatedCertificate;
  constructor(scope: Construct, props: WebResumeRustCertificateProps) {
    super(scope, "web-resume-rust-certificate");

    this.certificate = new acm.DnsValidatedCertificate(
      this,
      "SiteCertificate",
      {
        domainName: props.domainName,
        hostedZone: props.zone,
        region: "us-east-1", // Cloudfront only checks this region for certificates.
      }
    );
  }
}
