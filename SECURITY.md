# Security Policy

## Supported Versions
Only the latest major release branch receives security updates.

| Version | Supported          |
| ------- | ------------------ |
| Latest  | :white_check_mark: |
| < Last  | :x:                |

## Reporting a Vulnerability
**Please do NOT create public GitHub issues for security vulnerabilities.**

To report security issues:
1. Send an email to [popxh@outlook.com](mailto:popxh@outlook.com)
2. Include as much information as possible:
   - Affected version(s)
   - Steps to reproduce
   - Impact description
   - Proof-of-concept (if available, avoid destructive payloads)

We will acknowledge receipt at the earliest opportunity and provide regular updates on investigation and fix progress.

### What to expect
- We will confirm the vulnerability.
- Work on a patch and prepare release.
- Coordinate disclosure timeline with you.
- Credit you in the security advisory (unless you wish to remain anonymous).

## Security Best Practices for Users
- Always use the latest released version.
- Keep dependencies updated.
- Validate inputs, avoid running untrusted configuration or files.
- Restrict file system / network permissions when executing this software.

## Scope
This security policy covers the source code in this repository.
Third-party dependencies are governed by their own security policies.

## Out of Scope
- Vulnerabilities in underlying operating system, runtime, or external libraries not maintained in this repo.
- Issues requiring physical access, social engineering, or compromised credentials.
- Denial-of-service attacks that require excessive resources or non-standard environments.
