# Branch Protection Rules

> Recommended branch protection settings for Altair repository

## Quick Setup

Go to: **Settings → Branches → Branch protection rules**

Or use the direct link: `https://github.com/getaltair/altair/settings/branches`

---

## Protection for `main` Branch

### Branch Name Pattern

```
main
```

### Protection Rules

#### ✅ Require Pull Request Before Merging

- **Required approvals:** 1
- ✅ Dismiss stale pull request approvals when new commits are pushed
- ✅ Require review from Code Owners (optional, if CODEOWNERS file exists)

#### ✅ Require Status Checks to Pass

**Required status checks:**

- `Test Suite / test`
- `Test Suite / integration-test`
- `Test Suite / build`
- `CI / lint-flutter`
- `CI / test-flutter`
- `CI / lint-python` (if backend changes)
- `CI / test-python` (if backend changes)

**Settings:**

- ✅ Require branches to be up to date before merging
- ✅ Require status checks to pass before merging

#### ✅ Require Conversation Resolution Before Merging

- All PR comments must be resolved before merge

#### ✅ Require Signed Commits (Optional)

- Enforces GPG-signed commits for security

#### ✅ Require Linear History

- Prevents merge commits, requires rebase or squash

#### ✅ Include Administrators

- Even admins must follow these rules

#### ❌ Allow Force Pushes

- **Disabled** - Prevents history rewriting on main

#### ❌ Allow Deletions

- **Disabled** - Prevents accidental branch deletion

---

## Protection for `develop` Branch

### Branch Name Pattern

```
develop
```

### Protection Rules

#### ✅ Require Pull Request Before Merging

- **Required approvals:** 1 (can be 0 for solo development)
- ✅ Dismiss stale pull request approvals when new commits are pushed

#### ✅ Require Status Checks to Pass

**Required status checks:**

- `Test Suite / test`
- `Test Suite / integration-test`
- `Test Suite / build`
- `CI / lint-flutter`
- `CI / test-flutter`

**Settings:**

- ✅ Require branches to be up to date before merging
- ✅ Require status checks to pass before merging

#### ✅ Require Conversation Resolution Before Merging

- All PR comments must be resolved

#### ⚠️  Require Linear History (Optional)

- Can be enabled for cleaner history
- May slow down integration if many features

#### ❌ Include Administrators (Optional)

- Can allow admins to bypass for urgent fixes
- Recommended: Keep enabled for discipline

#### ❌ Allow Force Pushes

- **Disabled** - Prevents history rewriting

#### ❌ Allow Deletions

- **Disabled** - Prevents accidental deletion

---

## Protection for Feature Branches

### Branch Name Pattern

```
feature/*
fix/*
docs/*
```

### Protection Rules

**Minimal protection for flexibility:**

#### ✅ Require Status Checks to Pass (Recommended)

- `CI / lint-flutter`
- `CI / test-flutter`

#### ❌ Require Pull Requests

- Not required (developer manages their own feature branch)

#### ✅ Allow Force Pushes

- **Enabled** - Developers can rebase/clean up commits

#### ✅ Allow Deletions

- **Enabled** - Can delete after merge

---

## Recommended Setup Order

### 1. Start Conservative (Solo Development)

```
main:
  - Require 1 approval
  - Require status checks
  - No force push
  - No deletion

develop:
  - Require 0 approvals (can self-merge)
  - Require status checks
  - No force push
  - No deletion

feature/*:
  - No restrictions
```

### 2. Team Development

```
main:
  - Require 2 approvals
  - Require status checks
  - Require conversation resolution
  - Require linear history
  - No force push
  - No deletion
  - Include administrators

develop:
  - Require 1 approval
  - Require status checks
  - Require conversation resolution
  - No force push
  - No deletion

feature/*:
  - Require status checks
  - Allow force push
  - Allow deletion
```

---

## Using GitHub CLI (Alternative Method)

If you have `gh` CLI with admin permissions:

```bash
# Enable branch protection for main
gh api repos/getaltair/altair/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["test","build"]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":1}' \
  --field restrictions=null

# Enable branch protection for develop
gh api repos/getaltair/altair/branches/develop/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["test","build"]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":1}' \
  --field restrictions=null
```

**Note:** This requires repository admin permissions and may need adjustment based on your exact CI check names.

---

## Verification

After setting up branch protection:

### Test `main` Protection

```bash
# Try to push directly to main (should fail)
git checkout main
echo "test" >> test.txt
git add test.txt
git commit -m "test: verify protection"
git push origin main
# Expected: Error - protected branch requires PR
```

### Test `develop` Protection

```bash
# Create a test PR to develop
git checkout -b test/branch-protection
echo "test" >> test.txt
git add test.txt
git commit -m "test: verify protection"
git push -u origin test/branch-protection
gh pr create --base develop
# Expected: PR created, must pass checks before merge
```

---

## Adjusting Settings

### Allow Bypasses for Urgent Hotfixes

In `main` branch protection:

1. Disable "Include administrators" temporarily
2. Make hotfix
3. Re-enable "Include administrators"

### Add More Required Checks

As you add CI workflows:

1. Go to branch protection settings
2. Add new check names under "Require status checks"
3. Save changes

---

## Rulesets (New GitHub Feature)

GitHub now supports **Branch Rulesets** as a more flexible alternative to branch protection rules.

### Benefits

- Apply rules to multiple branches with patterns
- More granular permissions
- Better enforcement options

### Creating a Ruleset

1. Go to **Settings → Rules → Rulesets**
2. Click **New ruleset**
3. Name: "Production Protection" or "Development Protection"
4. Target branches: `main`, `develop`, `feature/*`, etc.
5. Configure rules (same as branch protection)
6. Set bypass permissions if needed

**Recommended Rulesets:**

1. **Production Ruleset** - For `main`
   - Strictest rules
   - No bypasses except owner

2. **Integration Ruleset** - For `develop`
   - Moderate rules
   - Admin bypass allowed

3. **Feature Ruleset** - For `feature/*`, `fix/*`, `docs/*`
   - CI checks only
   - Full flexibility

---

## Troubleshooting

### "Required status check 'test' not found"

**Problem:** CI check name doesn't match protection rule

**Solution:**

1. Run a PR to see actual check names
2. Update protection rules with exact names
3. Check `.github/workflows/*.yml` for job names

### "Cannot merge - PR requires review"

**Expected behavior** when protection is working correctly.

**Override (admin only):**

1. Temporarily disable "Include administrators"
2. Merge PR
3. Re-enable protection

### "Force push rejected"

**Expected behavior** on protected branches.

**Solution:** Use feature branches for development, only merge via PR.

---

## Resources

- [GitHub Branch Protection Docs](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- [GitHub Rulesets Docs](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets)
- [Git Workflow Guide](./GIT-WORKFLOW.md)

---

**Last Updated:** October 23, 2025
**Action Required:** Repository admin must configure these settings in GitHub
