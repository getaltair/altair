# Branch Protection Setup for main

## Goal

Restrict the `main` branch to only accept merges from:

- `develop` branch
- `hotfix/*` branches

## Option 1: GitHub UI Configuration

### Step 1: Enable Branch Protection

1. Go to <https://github.com/getaltair/altair/settings/branches>
2. Click "Add branch protection rule"
3. Branch name pattern: `main`

### Step 2: Configure Protection Rules

Enable these options:

- ✅ **Require a pull request before merging**
  - ✅ Require approvals (recommended: 1)
  - ✅ Dismiss stale pull request approvals when new commits are pushed
  - ✅ Require review from Code Owners (optional)

- ✅ **Require status checks to pass before merging**
  - Search for and add: `test`, `build`, `lint` (whatever your CI jobs are named)
  - ✅ Require branches to be up to date before merging

- ✅ **Require conversation resolution before merging**

- ✅ **Require linear history** (prevents merge commits, uses squash/rebase)

- ✅ **Do not allow bypassing the above settings** (applies to administrators too)

### Step 3: Restrict Base Branches (GitHub Enterprise/Org only)

**Note**: This feature requires GitHub Enterprise or Organization plan with advanced security

If you have Enterprise:

1. Enable "Restrict who can push to matching branches"
2. Add only: `develop`, `hotfix/*` as allowed base branches

If you DON'T have Enterprise:

- Use a GitHub Action to enforce this (see Option 2)

## Option 2: GitHub Actions Enforcement (Works on Free Plan)

Create `.github/workflows/enforce-branch-policy.yml`:

```yaml
name: Enforce Branch Policy

on:
  pull_request:
    branches:
      - main

jobs:
  check-source-branch:
    runs-on: ubuntu-latest
    steps:
      - name: Check source branch
        run: |
          SOURCE_BRANCH="${{ github.head_ref }}"

          # Allow develop and hotfix/* branches
          if [[ "$SOURCE_BRANCH" == "develop" ]] || [[ "$SOURCE_BRANCH" == hotfix/* ]]; then
            echo "✅ Source branch '$SOURCE_BRANCH' is allowed to merge to main"
            exit 0
          else
            echo "❌ ERROR: Only 'develop' and 'hotfix/*' branches can merge to main"
            echo "Current source branch: $SOURCE_BRANCH"
            echo ""
            echo "Please merge your changes to 'develop' first, then create a PR from develop to main"
            exit 1
          fi
```

## Option 3: CLI Setup (Partial)

Run this command to set up basic protection:

```bash
gh api \
  --method PUT \
  -H "Accept: application/vnd.github+json" \
  /repos/getaltair/altair/branches/main/protection \
  -f required_status_checks='{"strict":true,"contexts":["test","build"]}' \
  -f enforce_admins=true \
  -f required_pull_request_reviews='{"dismiss_stale_reviews":true,"require_code_owner_reviews":false,"required_approving_review_count":1}' \
  -f restrictions=null \
  -f allow_force_pushes=false \
  -f allow_deletions=false \
  -f required_linear_history=true \
  -f required_conversation_resolution=true
```

**Note**: This doesn't restrict source branches, so combine with Option 2.

## Recommended Configuration for Your Workflow

Based on your `develop` → `main` git flow:

1. **Protect `main`**: Only accept PRs, require reviews, require CI
2. **Protect `develop`**: Require PRs from feature branches, require CI
3. **Enforce with GitHub Action**: Check source branch is `develop` or `hotfix/*`
4. **CODEOWNERS file**: Auto-assign reviewers

Example CODEOWNERS file (create `.github/CODEOWNERS`):

```
# Global owners
* @rghamilton3

# Flutter app
apps/altair_guidance/** @rghamilton3

# Core packages
packages/** @rghamilton3

# CI/CD
.github/workflows/** @rghamilton3
```

## Testing the Setup

1. Try creating a PR from a feature branch to `main` - should fail
2. Try creating a PR from `develop` to `main` - should succeed
3. Try creating a PR from `hotfix/something` to `main` - should succeed

## Current Status

- ❌ `main` branch: **Not protected**
- ❌ `develop` branch: **Not protected**
- ❌ Source branch restriction: **Not configured**

Run the setup commands or configure via GitHub UI to enable protection.
