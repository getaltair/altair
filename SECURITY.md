# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **<security@getaltair.com>**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

### What to Include

Please include the following information in your report:

- Type of vulnerability (e.g., SQL injection, XSS, credentials exposure, etc.)
- Full paths of source file(s) related to the manifestation of the vulnerability
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the vulnerability, including how an attacker might exploit it

This information will help us triage your report more quickly.

## Security Update Process

1. **Report Received**: We acknowledge receipt within 48 hours
2. **Assessment**: We assess the vulnerability within 7 days
3. **Fix Development**: We develop a fix (timeline varies by severity)
4. **Security Advisory**: We publish a GitHub Security Advisory
5. **Patch Release**: We release a patched version
6. **Public Disclosure**: We disclose the vulnerability after patch is available

### Severity Levels

We use the following severity levels based on CVSS scores:

- **Critical** (9.0-10.0): Immediate attention, patch within 24-48 hours
- **High** (7.0-8.9): Urgent attention, patch within 7 days
- **Medium** (4.0-6.9): Important, patch within 30 days
- **Low** (0.1-3.9): Scheduled for next release

## Security Measures

### Current Security Implementations

- **Credential Security**: Cryptographically secure password generation with platform-specific secure storage (Keychain, Credential Manager, Secret Service API)
- **Dependency Scanning**: Automated vulnerability scanning via Snyk
- **Dependency Updates**: Automated updates via Dependabot
- **Secret Scanning**: Gitleaks integration to prevent credential leaks
- **Code Quality**: Pre-commit hooks with security linting
- **Access Control**: File permissions hardening (chmod 600 on sensitive files)
- **OSSF Scorecard**: Open source security best practices validation

### Security Best Practices

When contributing to Altair:

1. **Never commit secrets**: Use environment variables and secure storage
2. **Validate all inputs**: Sanitize user input to prevent injection attacks
3. **Use parameterized queries**: Prevent SQL injection
4. **Keep dependencies updated**: Run `flutter pub outdated` and `uv lock --upgrade` regularly
5. **Follow secure coding guidelines**: See [CONTRIBUTING.md](CONTRIBUTING.md)
6. **Test security features**: Add tests for authentication and authorization
7. **Review third-party code**: Audit dependencies before adding them

## Known Security Considerations

### Current Limitations (Tracked for Future Improvement)

1. **Service Configuration Files**: Platform service files (systemd, launchd, batch) contain credentials in environment variables
   - **Mitigation**: Files have restrictive permissions (600)
   - **Future**: Implement runtime retrieval from secure storage

2. **Credential File Fallback**: Fallback credential storage uses JSON without encryption at rest
   - **Mitigation**: File permissions set to 600 (owner-only)
   - **Future**: Implement AES-256 encryption with machine-specific key derivation

3. **No Credential Rotation**: Credentials don't expire automatically
   - **Future**: Implement 90-day rotation policy with automated warnings

4. **Limited Logging**: Security operations have minimal logging
   - **Future**: Implement comprehensive security audit logging

See [docs/SECURITY-ROADMAP.md](docs/SECURITY-ROADMAP.md) for detailed improvement plans.

## Disclosure Policy

We follow **coordinated disclosure**:

1. Vulnerability is reported privately
2. We develop and test a fix
3. We release a patch
4. We publish a security advisory with details
5. Full disclosure occurs 90 days after patch release (or earlier if agreed with reporter)

## Bug Bounty Program

We currently do not have a bug bounty program. However, we deeply appreciate security research and will:

- Acknowledge your contribution in our security advisories
- Give you credit in release notes (if you wish)
- Provide updates on the fix progress

## Security Contacts

- **Primary**: <security@getaltair.com>
- **GPG Key**: [Coming Soon]
- **Security Team**: @rghamilton3

## Additional Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)
- [Altair Security Documentation](docs/SECURITY-ROADMAP.md)

---

**Last Updated**: 2025-10-27
**Policy Version**: 1.0
