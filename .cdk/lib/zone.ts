import { Construct } from "constructs";
import * as route53 from "aws-cdk-lib/aws-route53";
import * as route53Targets from "aws-cdk-lib/aws-route53-targets";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";

type WebResumeRustZoneProps = {
  domainName: string;
};

type WebResumeRustZoneArecordProps = {
  domainName: string;
  //   subDomainName: string;
  cnameRecord: route53.CnameRecord;
};

type WebResumeRustZoneCNAMErecordProps = {
  domainName: string;
  distribution: cloudfront.IDistribution;
};

export class WebResumeRustZone extends Construct {
  zone: route53.IHostedZone;
  constructor(scope: Construct, props: WebResumeRustZoneProps) {
    super(scope, "web-resume-rust-zone");

    this.zone = route53.HostedZone.fromLookup(
      scope,
      "web-resume-rust-hostedzone",
      {
        domainName: props.domainName,
      }
    );
  }

  createARecordForRoutingRootDomainToSubDomain(
    scope: Construct,
    props: WebResumeRustZoneArecordProps
  ) {
    // new route53.ARecord(scope, "web-resume-rust-zone-arecord", {
    //   recordName: props.domainName,
    //   target: route53.RecordTarget.fromValues("profile.vpurush.com"),
    //   zone: this.zone,
    // }).node.addDependency(props.cnameRecord);
    // return new route53.CnameRecord(scope, "web-resume-rust-zone-root-cname", {
    //   recordName: props.domainName,
    //   domainName: "profile.vpurush.com",
    //   zone: this.zone,
    // });
  }

  createCNAMEForRoutingSubDomainToDistribution(
    scope: Construct,
    props: WebResumeRustZoneCNAMErecordProps
  ) {
    return new route53.CnameRecord(
      scope,
      "web-resume-rust-zone-profile-subdomain-cname",
      {
        recordName: props.domainName,
        domainName: props.distribution.distributionDomainName,
        zone: this.zone,
      }
    );
  }
}
