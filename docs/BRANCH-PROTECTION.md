# Repository Rulesets

> Modern branch protection using GitHub Rulesets (replaces legacy branch protection rules)

## Quick Setup

Go to: **Settings → Rules → Rulesets**

Or use the direct link: `https://github.com/getaltair/altair/settings/rules`

**What are Rulesets?**

Rulesets are GitHub's modern approach to branch protection that offer:

- Multiple rulesets can apply simultaneously (most restrictive wins)
- Can be enabled/disabled without deletion
- Visible to all users (not just admins)
- Support for commit metadata rules

---

## Ruleset for `main` Branch

### Step-by-Step Setup

1. **Go to Settings → Rules → Rulesets → New ruleset → New branch ruleset**

2. **Ruleset Name:** `main-protection`

3. **Enforcement status:** **Active** (enforce immediately)

4. **Bypass list:** Leave empty (or add specific users with "Pull request only" if needed)

5. **Target branches:**
   - Add target: **Include by pattern**
   - Pattern: `main`

6. **Branch protections** - Enable these rules:

#### ✅ Restrict deletions

- Prevents accidental branch deletion

#### ✅ Block force pushes

- Prevents history rewriting (enabled by default)

#### ✅ Require linear history

- Prevents merge commits, enforces squash or rebase

#### ✅ Require pull request before merging

- **Required approvals:** 1
- ✅ Dismiss stale pull request approvals when new commits are pushed
- ✅ Require review from Code Owners (if you create a CODEOWNERS file)
- ✅ Require approval of the most recent reviewable push
- ✅ Require conversation resolution before merging

#### ✅ Require status checks to pass

- ✅ Require branches to be up to date before merging

**Add these status checks:**

- `test` (from Test Suite workflow)
- `integration-test` (from Test Suite workflow)
- `build` (from Test Suite workflow)
- `Lint Flutter Code` (from CI workflow)
- `Test Flutter Code` (from CI workflow)
- `Android Build & Test` (from Mobile CI/CD workflow)
- `Lint Python Code` (if backend changes)
- `Test Python Code` (if backend changes)

#### ✅ Require signed commits (Optional but recommended)

- Ensures all commits are GPG signed

1. **Click "Create"** at the bottom

---

## Ruleset for `develop` Branch

### Step-by-Step Setup

1. **Go to Settings → Rules → Rulesets → New ruleset → New branch ruleset**

2. **Ruleset Name:** `develop-protection`

3. **Enforcement status:** **Active**

4. **Bypass list:**
   - For solo dev: Can add yourself with full bypass
   - For team: Leave empty or add leads with "Pull request only"

5. **Target branches:**
   - Add target: **Include by pattern**
   - Pattern: `develop`

6. **Branch protections** - Enable these rules:

#### ✅ Restrict deletions

- Prevents accidental branch deletion

#### ✅ Block force pushes

- Prevents history rewriting

#### ✅ Require pull request before merging

- **Required approvals:** 0 for solo, 1 for team
- ✅ Dismiss stale pull request approvals when new commits are pushed
- ✅ Require conversation resolution before merging

#### ✅ Require status checks to pass

- ✅ Require branches to be up to date before merging

**Add these status checks:**

- `test`
- `integration-test`
- `build`
- `Lint Flutter Code`
- `Test Flutter Code`
- `Android Build & Test`

#### ⚠️ Require linear history (Optional)

- Enable for cleaner git history
- May slow integration if many concurrent features

1. **Click "Create"**

---

## Ruleset for Feature Branches (Optional)

### Step-by-Step Setup

1. **Go to Settings → Rules → Rulesets → New ruleset → New branch ruleset**

2. **Ruleset Name:** `feature-branch-checks`

3. **Enforcement status:** **Active** (or **Evaluate** to test without blocking)

4. **Bypass list:** Leave empty (all developers can push)

5. **Target branches:**
   - Add target: **Include by pattern**
   - Pattern: `feature/*`
   - Add another: `fix/*`
   - Add another: `docs/*`
   - Add another: `test/*`

6. **Branch protections** - Minimal rules for flexibility:

#### ✅ Require status checks to pass (Recommended)

- **Do NOT** require branches to be up to date (too restrictive for feature work)

**Add these status checks:**

- `Lint Flutter Code`
- `Test Flutter Code`

**Leave DISABLED:**

- Restrict deletions (developers can delete their feature branches)
- Block force pushes (developers need to rebase/amend)
- Require pull requests (not needed for work-in-progress)

1. **Click "Create"**

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

## Using GitHub CLI (Advanced)

Rulesets can also be created via the GitHub API, though the web interface is recommended for initial setup.

See [GitHub REST API - Rulesets](https://docs.github.com/en/rest/repos/rules) for API documentation.

**Note:** The ruleset API is more complex than the legacy branch protection API. Use the web UI unless you need to automate ruleset creation across many repositories.

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

## Managing Rulesets

### Temporarily Disable a Ruleset

For urgent hotfixes or testing:

1. Go to **Settings → Rules → Rulesets**
2. Click on the ruleset (e.g., `main-protection`)
3. Change **Enforcement status** to **Disabled**
4. Make your changes
5. Re-enable by setting status back to **Active**

**Tip:** Use **Evaluate** status to test ruleset changes without blocking contributors.

### Add Bypass Permissions

To allow specific users to bypass rules:

1. Edit the ruleset
2. Scroll to **Bypass list**
3. Click **Add bypass**
4. Select user/team/app
5. Choose bypass mode:
   - **Always** - Full bypass (use sparingly)
   - **Pull request only** - Must still create PR (recommended for admins)

### Add More Status Checks

As you add CI workflows:

1. Edit the ruleset
2. Scroll to **Require status checks to pass**
3. Click **Add checks**
4. Enter exact check name (e.g., `Mobile CI Summary`)
5. Save changes

**Tip:** Run a PR first to see exact check names in the "Checks" tab.

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
