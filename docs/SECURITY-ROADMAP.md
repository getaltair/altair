# Security Roadmap

This document outlines planned security improvements for Altair, prioritized by impact and urgency.

## Current Security Posture (v0.1.0)

### ✅ Implemented

- **Cryptographically Secure Password Generation**: Random.secure() with 32+ character passwords
- **Platform-Specific Secure Storage**: Keychain (macOS), Credential Manager (Windows), Secret Service (Linux)
- **File Permission Hardening**: chmod 600 on Unix, ACL restrictions on Windows
- **Environment Variable Credentials**: Prevents exposure in process listings
- **Dependency Scanning**: Snyk integration for vulnerability detection
- **Secret Detection**: Gitleaks pre-commit and CI hooks
- **License Compliance**: Automated policy enforcement
- **OSSF Scorecard**: Security best practices validation
- **Code Coverage**: Codecov integration with quality gates

### ⚠️ Known Limitations

1. Service configuration files contain plaintext credentials
2. Credential file fallback lacks encryption at rest
3. No automated credential rotation
4. Limited security event logging
5. No runtime application security monitoring

## Phase 1: Critical Security Hardening (Q4 2024)

**Priority**: P0 - Critical
**Timeline**: 2-4 weeks

### 1.1 Eliminate Plaintext Credentials in Service Configs

**Issue**: systemd, launchd, and batch files store credentials in environment variables

**Solution**:

```yaml
Tasks:
  - Implement wrapper scripts for service startup
  - Scripts retrieve credentials from secure storage at runtime
  - Remove credentials from service configuration files
  - Linux: Read from CredentialManager via DBus
  - macOS: Read from Keychain via 'security' command
  - Windows: Read from Credential Manager via PowerShell

Files to Change:
  - packages/altair-db-service/lib/src/platform/linux_installer.dart
  - packages/altair-db-service/lib/src/platform/macos_installer.dart
  - packages/altair-db-service/lib/src/platform/windows_installer.dart

New Files:
  - packages/altair-db-service/scripts/start-surrealdb.sh
  - packages/altair-db-service/scripts/start-surrealdb.ps1

Testing:
  - Integration tests for each platform
  - Verify credentials not in process environment
  - Validate service starts correctly
```

**Success Criteria**:

- Zero plaintext credentials in service files
- Services start successfully on all platforms
- Credentials only in platform secure storage

### 1.2 Encrypt Credential File Fallback

**Issue**: Fallback JSON file stores credentials in plaintext

**Solution**:

```yaml
Tasks:
  - Implement AES-256-GCM encryption for credential files
  - Derive encryption key from machine-specific data:
    - Hardware UUID (system_info package)
    - MAC address (network_info package)
    - Installation timestamp
  - Use PBKDF2 for key derivation (100k iterations)
  - Store salt separately from encrypted data

Implementation:
  - Add encrypt_package dependency
  - Modify CredentialManager._storeCredentialsInFile()
  - Modify CredentialManager._getCredentialsFromFile()
  - Add encryption tests

Files to Change:
  - packages/altair-db-service/lib/src/security/credential_manager.dart
  - packages/altair-db-service/pubspec.yaml
  - packages/altair-db-service/test/security/credential_manager_test.dart
```

**Success Criteria**:

- Credential files encrypted at rest
- Machine-specific key derivation
- Backward compatibility with migration path
- 100% test coverage for encryption/decryption

### 1.3 Add Security Event Logging

**Issue**: Silent failures hide security issues

**Solution**:

```yaml
Tasks:
  - Integrate logger package
  - Create SecurityLogger class
  - Log all credential operations (store, retrieve, delete)
  - Log permission checks and failures
  - Log fallback from secure storage to file
  - Add structured logging (JSON format)
  - Implement log rotation
  - Add telemetry for monitoring (optional, privacy-respecting)

Log Levels:
  - DEBUG: All operations (development only)
  - INFO: Successful operations
  - WARN: Fallbacks, degraded security
  - ERROR: Failed operations, security violations
  - CRITICAL: Compromise indicators

Files to Change:
  - packages/altair-db-service/lib/src/security/credential_manager.dart
  - packages/altair-db-service/lib/src/security/security_logger.dart (new)

Privacy:
  - Never log actual credentials
  - Hash user identifiers
  - Respect user privacy settings
```

**Success Criteria**:

- All security operations logged
- No sensitive data in logs
- Logs rotated automatically
- Integration tests verify logging

## Phase 2: Advanced Security Features (Q1 2025)

**Priority**: P1 - High
**Timeline**: 4-6 weeks

### 2.1 Credential Rotation and Expiry

**Issue**: Credentials never expire, no rotation policy

**Solution**:

```yaml
Tasks:
  - Implement credential age tracking (use existing created_at)
  - Add rotation policy configuration (default: 90 days)
  - Create credential rotation API
  - Implement expiry warnings (14 days, 7 days, 1 day)
  - Graceful rotation during service restart
  - UI notifications for rotation needed
  - Automated rotation option

Configuration:
  rotation_policy:
    enabled: true
    max_age_days: 90
    warning_days: [14, 7, 1]
    auto_rotate: false  # Manual by default

Files to Create:
  - packages/altair-db-service/lib/src/security/rotation_policy.dart
  - packages/altair-db-service/lib/src/security/rotation_manager.dart

UI Changes:
  - Add rotation status indicator
  - Add manual rotation button
  - Show days until expiry
```

**Success Criteria**:

- Credentials expire after 90 days
- Users warned before expiry
- Rotation doesn't disrupt service
- Tests cover rotation scenarios

### 2.2 Container Security Scanning

**Issue**: Docker images not scanned for vulnerabilities

**Solution**:

```yaml
Tasks:
  - Add Snyk container scanning to CI
  - Scan base images (python:3.12-slim, etc.)
  - Implement multi-stage builds
  - Use distroless images where possible
  - Add SBOM generation
  - Scan for misconfigurations

Files to Change:
  - .github/workflows/security.yml
  - services/auth-service/Dockerfile
  - services/sync-service/Dockerfile (when created)

Best Practices:
  - Pin base image versions
  - Minimize layers
  - Remove unnecessary packages
  - Run as non-root user
  - Use read-only filesystem
```

**Success Criteria**:

- Zero high/critical vulnerabilities
- SBOM generated for each image
- Images pass CIS benchmark checks
- Automated scanning on every build

### 2.3 Infrastructure as Code Security

**Issue**: IaC files not scanned for misconfigurations

**Solution**:

```yaml
Tasks:
  - Add Snyk IaC scanning
  - Scan Docker Compose files
  - Scan Kubernetes manifests (when added)
  - Validate security best practices
  - Check for exposed secrets
  - Verify network policies

Scan Targets:
  - docker-compose.yml files
  - Kubernetes YAML manifests
  - Terraform configs (if added)
  - GitHub Actions workflows

Files to Change:
  - .github/workflows/security.yml
  - Add .iac-ignore for false positives
```

**Success Criteria**:

- All IaC scanned on commit
- Zero high/critical misconfigurations
- Security policies enforced
- Documentation of exceptions

## Phase 3: Enterprise Security (Q2 2025)

**Priority**: P2 - Medium
**Timeline**: 6-8 weeks

### 3.1 Static Application Security Testing (SAST)

**Solution**:

```yaml
Tasks:
  - Implement Snyk Code for Dart analysis
  - Add CodeQL for Python analysis
  - Create custom security rules
  - Integrate with PR checks
  - Set up automated remediation

Scan For:
  - SQL injection vulnerabilities
  - XSS vulnerabilities
  - Path traversal
  - Insecure deserialization
  - Authentication bypasses
  - Authorization issues
```

### 3.2 Dynamic Application Security Testing (DAST)

**Solution**:

```yaml
Tasks:
  - Set up OWASP ZAP scanning
  - Create baseline scan profiles
  - Automate scans in staging
  - Configure authenticated scanning
  - Integrate with CI/CD

Scan Coverage:
  - API endpoints
  - Authentication flows
  - Authorization checks
  - Input validation
  - Session management
```

### 3.3 Software Bill of Materials (SBOM)

**Solution**:

```yaml
Tasks:
  - Generate SBOM for all releases
  - Use CycloneDX or SPDX format
  - Include all dependencies
  - Track vulnerability disclosures
  - Provide SBOM to enterprise users

Tools:
  - syft for SBOM generation
  - grype for vulnerability matching
  - Automate in release workflow
```

### 3.4 Security Compliance

**Solution**:

```yaml
Tasks:
  - Document security controls
  - Create compliance matrix
  - Implement audit logging
  - Add compliance reporting

Target Frameworks:
  - SOC 2 Type II
  - GDPR compliance
  - HIPAA (for healthcare users)
  - PCI DSS (if payment processing added)
```

## Phase 4: Advanced Protection (Q3 2025)

**Priority**: P3 - Low
**Timeline**: Ongoing

### 4.1 Runtime Application Self-Protection (RASP)

**Solution**:

```yaml
Capabilities:
  - Real-time threat detection
  - Automatic attack blocking
  - Anomaly detection
  - Behavioral analysis

Implementation:
  - Integrate Sqreen or similar
  - Monitor API calls
  - Detect malicious payloads
  - Block suspicious requests
```

### 4.2 Zero Trust Architecture

**Solution**:

```yaml
Components:
  - Mutual TLS (mTLS)
  - Service mesh (Istio/Linkerd)
  - Policy enforcement
  - Micro-segmentation
  - Identity-based access
```

### 4.3 Threat Intelligence Integration

**Solution**:

```yaml
Tasks:
  - Subscribe to threat feeds
  - Integrate MISP
  - Automated IoC blocking
  - Threat hunting
  - Incident response automation
```

## Metrics and KPIs

### Security Health Indicators

```yaml
Target Metrics:
  - Critical vulnerabilities: 0
  - High vulnerabilities: <5
  - Medium vulnerabilities: <20
  - OSSF Scorecard: >8.0
  - Test coverage: >80%
  - Mean time to remediate (MTTR):
    - Critical: <24 hours
    - High: <7 days
    - Medium: <30 days

Monitoring:
  - Weekly vulnerability scans
  - Monthly security reviews
  - Quarterly penetration testing
  - Annual third-party audit
```

## Setup Instructions for Future Work

### Prerequisites

1. **Snyk Account** (already configured)
   - Create account at <https://snyk.io>
   - Generate API token
   - Add to GitHub secrets as `SNYK_TOKEN`

2. **Future Tool Setup**

```bash
# Container scanning (Phase 2)
snyk container test <image>

# IaC scanning (Phase 2)
snyk iac test docker-compose.yml

# Code analysis (Phase 3)
snyk code test

# SBOM generation (Phase 3)
syft packages altair-guidance:latest -o cyclonedx-json

# Vulnerability matching (Phase 3)
grype sbom:./sbom.json
```

### Cost Estimates

```yaml
Free Tier (Current):
  - Snyk: 200 tests/month
  - GitHub: Advanced Security (public repos)
  - OSSF Scorecard: Free
  - Gitleaks: Free

Paid Tier (When Needed):
  - Snyk Team: $420/year (for private repos)
  - Container scanning: Included
  - IaC scanning: Included
  - Code analysis: Included

Enterprise (Future):
  - Snyk Enterprise: Custom pricing
  - Pen testing: $5k-15k annually
  - Third-party audit: $10k-30k annually
```

## Getting Help

### Resources

- **Snyk Documentation**: <https://docs.snyk.io>
- **OWASP Resources**: <https://owasp.org>
- **CWE Database**: <https://cwe.mitre.org>
- **NIST Guidelines**: <https://csrc.nist.gov>

### Support Channels

- **Security Team**: <security@getaltair.com>
- **Community**: Discord #security channel
- **Issues**: GitHub Security Advisories

---

**Last Updated**: 2025-10-27
**Roadmap Version**: 1.0
**Next Review**: 2025-11-27
