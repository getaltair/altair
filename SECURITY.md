# Security Policy

## TL;DR

**Found a security issue? Email [security@getaltair.app] - Don't post publicly.**

We take security seriously and respond quickly to reports.

---

## Reporting a Vulnerability

### Do This

1. **Email:** [security@getaltair.app] with details
2. **Include:**
   - What the vulnerability is
   - How to reproduce it
   - Potential impact
   - Your suggested fix (optional)
3. **Wait for response** - We'll acknowledge within 48 hours

### Don't Do This

- ❌ Don't post security issues publicly on GitHub
- ❌ Don't share vulnerability details with others
- ❌ Don't exploit the vulnerability

---

## What Qualifies as a Security Issue?

### Yes - Report These

- **Data leaks** - User data exposed without authorization
- **Authentication bypass** - Gaining access without proper credentials
- **Injection attacks** - SQL, XSS, or other code injection
- **Privilege escalation** - Users gaining unauthorized permissions
- **Cryptographic weaknesses** - Broken encryption or hashing
- **Dependency vulnerabilities** - Critical CVEs in dependencies

### No - Use Bug Reports Instead

- **Non-security bugs** - Use [bug report template](https://github.com/getaltair/altair/issues/new?template=bug_report.md)
- **Feature requests** - Use [feature request template](https://github.com/getaltair/altair/issues/new?template=feature_request.md)
- **Configuration issues** - Ask in [GitHub Discussions](https://github.com/getaltair/altair/discussions)
- **Outdated dependencies** - Open a regular issue

**Not sure?** Better to report it - we'll let you know.

---

## Response Timeline

### What to Expect

1. **Acknowledgment** - Within 48 hours
2. **Initial assessment** - Within 1 week
3. **Status updates** - Weekly until resolved
4. **Fix released** - Depends on severity
5. **Public disclosure** - After fix is deployed

### Severity Levels

- **Critical** - Fix within 24-48 hours
- **High** - Fix within 1 week
- **Medium** - Fix within 2-4 weeks
- **Low** - Fix in next regular release

---

## Our Commitment

### We Will

- **Acknowledge** your report promptly
- **Keep you updated** on progress
- **Credit you** in security advisories (if you want)
- **Not take legal action** against good-faith researchers
- **Patch quickly** based on severity

### We Won't

- **Ignore** security reports
- **Blame** reporters for issues
- **Delay** critical security fixes
- **Disclose** your identity without permission

---

## Supported Versions

Currently supported versions that receive security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

**Note:** As an early-stage project, we only support the latest release. Upgrade
regularly for security patches.

---

## Security Best Practices

### For Users

- **Keep updated** - Use the latest version
- **Secure local data** - Use disk encryption
- **Review permissions** - Only grant what's needed
- **Report issues** - See something, say something

### For Contributors

- **Review dependencies** - Check for known vulnerabilities
- **Follow secure coding** - No hardcoded secrets, sanitize inputs
- **Test thoroughly** - Include security test cases
- **Read guidelines** - Check CONTRIBUTING.md for standards

---

## Security Tools

We use:

- **SonarCloud** - Code security analysis
- **GitHub Dependabot** - Dependency vulnerability scanning
- **GitGuardian** - Secret detection

**See a security alert?** We're already on it, but feel free to confirm.

---

## Scope

### In Scope

- All code in the main repository
- Official Docker images
- Official releases and builds
- Backend services
- Database configurations

### Out of Scope

- Third-party integrations you've added
- Modified versions not from official sources
- User-created plugins or extensions
- Social engineering attacks

---

## Legal

### Safe Harbor

We won't pursue legal action against security researchers who:

- Act in good faith
- Don't violate privacy or laws
- Don't disrupt services
- Follow this disclosure policy
- Don't exploit beyond verification

### Bug Bounty

**Not currently available.** We're an open-source project maintained by
volunteers. We appreciate security research but can't offer financial
rewards at this time.

We'll credit you in:

- Security advisories
- CHANGELOG
- Project documentation

---

## Questions?

**Need clarification?** Email [security@getaltair.app]

**Want to help with security?** We welcome contributions! See CONTRIBUTING.md

---

> 🔒 **Remember:** Security is everyone's responsibility. Thank you for helping
> keep Altair safe!

**Last updated:** December 2024
