# Snyk Setup Guide

Quick guide to configure Snyk for Altair security scanning.

## Prerequisites

- GitHub repository with admin access
- Snyk account (free for open source)

## Setup Steps

### 1. Create Snyk Account

1. Go to <https://snyk.io>
2. Sign up with your GitHub account
3. Authorize Snyk to access your repositories

### 2. Generate Snyk API Token

1. Go to <https://app.snyk.io/account>
2. Click "General" → "Auth Token"
3. Click "Show" and copy the token
4. **Important**: Store this securely - you'll only see it once

### 3. Add Token to GitHub Secrets

1. Go to your GitHub repository
2. Click "Settings" → "Secrets and variables" → "Actions"
3. Click "New repository secret"
4. Name: `SNYK_TOKEN`
5. Value: [paste your token]
6. Click "Add secret"

### 4. Connect Repository to Snyk (Optional)

For additional features like Snyk PR checks:

1. Go to <https://app.snyk.io>
2. Click "Add project"
3. Select "GitHub"
4. Choose "getaltair/altair"
5. Select which manifests to monitor:
   - `services/auth-service/pyproject.toml`
   - `packages/altair-core/pubspec.yaml`
   - `packages/altair-auth/pubspec.yaml`
   - `packages/altair-ui/pubspec.yaml`
   - `packages/altair-db-service/pubspec.yaml`
   - `apps/altair_guidance/pubspec.yaml`

### 5. Configure Snyk Settings

1. Go to your project settings in Snyk
2. Set severity threshold: **High**
3. Enable automatic fix PRs
4. Configure test frequency: **Weekly**
5. Enable PR checks

### 6. Verify Setup

Run the security workflow manually:

```bash
# Push to trigger workflow
git push origin feat/security-hardening

# Or trigger manually in GitHub:
# Actions → Security → Run workflow
```

### 7. Review First Scan

1. Go to GitHub Actions tab
2. Find "Security" workflow
3. Review any vulnerabilities found
4. Click "View Snyk project" to see details

## Common Issues

### "SNYK_TOKEN not found"

**Solution**: Verify secret is added correctly

```bash
# Check if secret exists (won't show value)
gh secret list
```

### "Snyk CLI not found"

**Solution**: This is handled by GitHub Actions, no local install needed

### "Rate limit exceeded"

**Solution**: Snyk free tier has limits

- Wait for rate limit reset
- Or upgrade to paid plan
- Configure less frequent scans

## Local Testing

To test Snyk locally:

```bash
# Install Snyk CLI
npm install -g snyk

# Authenticate
snyk auth

# Test Python dependencies
cd services/auth-service
snyk test --file=pyproject.toml

# Test Flutter dependencies
cd packages/altair-core
snyk test --file=pubspec.yaml

# Test for all severity levels
snyk test --severity-threshold=low
```

## CI/CD Integration

The security workflow runs:

- **On push** to main/develop branches
- **On pull requests** to main/develop
- **Weekly** on Monday at 9 AM UTC (scheduled)

Workflow file: `.github/workflows/security.yml`

## Policy Configuration

Customize scanning in `.snyk` file:

```yaml
# Ignore specific vulnerabilities
ignore:
  'SNYK-JS-AXIOS-1234567':
    - '*':
        reason: 'False positive - not exploitable in our use case'
        expires: '2025-12-31'

# Patch specific vulnerabilities
patch:
  'npm:package:version':
    - package > dependency:
        patched: '2025-10-27'
```

## Monitoring

### View Scan Results

**In GitHub**:

- Actions → Security workflow
- Security tab → Dependabot alerts
- Security tab → Code scanning alerts

**In Snyk**:

- <https://app.snyk.io>
- Select "altair" project
- View vulnerabilities, licenses, dependencies

### Email Notifications

Configure in Snyk settings:

- New vulnerabilities
- Failed tests
- Fix PRs available

## Upgrading to Paid

Free tier limitations:

- 200 tests/month
- Limited historical data
- Basic reporting

Paid benefits:

- Unlimited tests
- Advanced reporting
- Priority support
- Custom policies
- SSO integration

Pricing: ~$52/developer/month

## Support

### Documentation

- Snyk Docs: <https://docs.snyk.io>
- GitHub Actions: <https://github.com/snyk/actions>

### Community

- Snyk Community: <https://community.snyk.io>
- Altair Discord: #security channel

### Issues

- Snyk Support: <https://support.snyk.io>
- Report in PR: #41

---

**Setup Complexity**: Easy (15 minutes)
**Maintenance**: Automated
**Cost**: Free for open source
