import { Construct } from "constructs";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as s3Deploy from "aws-cdk-lib/aws-s3-deployment";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as cloudfrontOrigins from "aws-cdk-lib/aws-cloudfront-origins";
import * as acm from "aws-cdk-lib/aws-certificatemanager";
import { RemovalPolicy } from "aws-cdk-lib";

type WebResumeRustDistrubutionProps = {
  domainName: string;
  bucket: s3.Bucket;
  certificate: acm.DnsValidatedCertificate;
};

export class WebResumeRustDistribution extends Construct {
  distribution: cloudfront.IDistribution;
  constructor(scope: Construct, props: WebResumeRustDistrubutionProps) {
    super(scope, "web-resume-rust-distribution");

    this.distribution = new cloudfront.Distribution(
      this,
      "web-resume-rust-distribution",
      {
        defaultRootObject: "index.html",
        domainNames: [props.domainName],
        minimumProtocolVersion: cloudfront.SecurityPolicyProtocol.TLS_V1_2_2021,
        defaultBehavior: {
          origin: new cloudfrontOrigins.S3Origin(props.bucket),
          compress: true,
          allowedMethods: cloudfront.AllowedMethods.ALLOW_GET_HEAD_OPTIONS,
          viewerProtocolPolicy:
            cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        },
        certificate: props.certificate,
      }
    );
  }
}
