# Security Policy

## Supported Versions

Security fixes are provided for the latest `zsmtp` 0.1.x release. Please use
the newest available patch release when checking whether an issue still exists.

| Version | Supported |
| ------- | --------- |
| 0.1.x   | ✅        |
| < 0.1   | ❌        |

## Reporting a Vulnerability

Please do not open a public GitHub issue or discussion for a suspected
security vulnerability.

Report vulnerabilities privately by emailing
[team@zsmtp.dev](mailto:team@zsmtp.dev). Include, when possible:

- A description of the vulnerability and its potential impact
- The affected `zsmtp` version and platform
- Steps to reproduce the issue or a minimal proof of concept
- Whether the issue involves configuration, message submission, protocol or
  cryptographic handling, or optional OpenTelemetry export
- Any suggested mitigation or fix

The maintainers will review the report and coordinate a fix and disclosure
timeline with you if the issue is confirmed.

Please keep the report confidential until a fixed release is available or a
disclosure timeline has been agreed upon.

## Scope

Reports about vulnerabilities in `zsmtp` itself or in the way it uses its
dependencies are in scope. General support questions and vulnerabilities that
only affect an upstream dependency should be reported to the relevant upstream
project unless `zsmtp` uses the dependency in an exploitable way.
