import { Construct } from "constructs";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as s3Deploy from "aws-cdk-lib/aws-s3-deployment";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import { RemovalPolicy } from "aws-cdk-lib";

type WebResumeRustBucketProps = {
  bucketName: string;
};

type WebResumeRustBucketDeploymentProps = {
  distribution: cloudfront.IDistribution;
};

export class WebResumeRustBucket extends Construct {
  bucket: s3.Bucket;
  constructor(scope: Construct, props: WebResumeRustBucketProps) {
    super(scope, "web-resume-rust-bucket");
    this.bucket = new s3.Bucket(this, "web-resume-rust-bucket", {
      publicReadAccess: true,
      removalPolicy: RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      bucketName: props.bucketName,
    });
  }

  deploy(scope: Construct, props: WebResumeRustBucketDeploymentProps) {
    const bucketDeployment = new s3Deploy.BucketDeployment(
      scope,
      "web-resume-rust-bucket-deployment",
      {
        destinationBucket: this.bucket,
        sources: [s3Deploy.Source.asset("../.perseus/dist/exported")],
        distribution: props.distribution,
        distributionPaths: ["/*"],
      }
    );
  }
}
