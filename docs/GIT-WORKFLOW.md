# Git Workflow

> Git branching strategy and workflow for Altair development

## Quick Reference

**Branches:**

- `main` - Stable releases only (v0.1.0, v0.2.0, etc.)
- `develop` - Active development, integration branch
- `feature/*` - Individual features, branch from `develop`

**Workflow:**

```bash
# 1. Start new feature
git checkout develop
git pull origin develop
git checkout -b feature/my-feature

# 2. Work on feature
git add .
git commit -m "feat(scope): description"

# 3. Push and create PR
git push -u origin feature/my-feature
gh pr create --base develop

# 4. After PR approval, merge via GitHub
# 5. Clean up local branch
git checkout develop
git pull origin develop
git branch -d feature/my-feature
```

---

## Branching Strategy

We use **Git Flow** with two permanent branches and temporary feature branches.

### Permanent Branches

#### `main` - Production Branch

- Contains stable, production-ready code
- Only updated via merges from `develop`
- Every merge is tagged with a version (v0.1.0, v0.2.0, etc.)
- Protected: Requires PR approval and CI passing

**When to update:**

- Ready for production release
- All Phase milestones complete
- Beta testing successful
- No critical bugs

#### `develop` - Integration Branch

- Contains latest development changes
- Integration branch for all features
- Always buildable and testable
- May contain unreleased features

**When to update:**

- Merge approved feature PRs
- Continuous integration of ongoing work
- Daily development activity

### Temporary Branches

#### Feature Branches (`feature/*`)

**Purpose:** Individual features or enhancements

**Naming:** `feature/descriptive-name`

```bash
feature/filter-menu
feature/ai-settings-ui
feature/knowledge-app-foundation
```

**Lifecycle:**

1. Branch from `develop`
2. Develop feature with commits
3. Create PR to `develop`
4. After merge, delete branch

#### Fix Branches (`fix/*`)

**Purpose:** Bug fixes

**Naming:** `fix/descriptive-name`

```bash
fix/task-deletion-crash
fix/ai-timeout-error
```

**Lifecycle:** Same as feature branches

#### Documentation Branches (`docs/*`)

**Purpose:** Documentation updates only

**Naming:** `docs/descriptive-name`

```bash
docs/api-documentation
docs/mobile-testing-guide
```

**Lifecycle:** Same as feature branches

---

## Workflow Examples

### Starting a New Feature

```bash
# 1. Ensure develop is up to date
git checkout develop
git pull origin develop

# 2. Create feature branch
git checkout -b feature/filter-menu

# 3. Make changes and commit
git add .
git commit -m "feat(ui): add filter menu component"

# 4. Push to remote
git push -u origin feature/filter-menu

# 5. Create PR via GitHub CLI
gh pr create --base develop \
  --title "feat(ui): Add filter menu" \
  --body "Implements filter menu for task list. Closes #123"
```

### Working on Existing Feature

```bash
# Pull latest changes from develop
git checkout develop
git pull origin develop

# Switch to your feature branch
git checkout feature/filter-menu

# Merge latest develop changes
git merge develop

# Continue development
git add .
git commit -m "feat(ui): add filter options"
git push
```

### Fixing Conflicts

```bash
# If your PR has conflicts with develop
git checkout feature/filter-menu
git fetch origin
git merge origin/develop

# Resolve conflicts in your editor
# Then commit the resolution
git add .
git commit -m "merge: resolve conflicts with develop"
git push
```

### After PR is Merged

```bash
# Switch to develop and pull
git checkout develop
git pull origin develop

# Delete your local feature branch
git branch -d feature/filter-menu

# Delete remote branch (if not auto-deleted)
git push origin --delete feature/filter-menu
```

---

## Release Process

When ready to release a new version:

### 1. Prepare Release on `develop`

```bash
# Ensure develop is stable
git checkout develop
git pull origin develop

# Run all tests
flutter test
pytest

# Update version numbers
# Update CHANGELOG.md
git add .
git commit -m "chore(release): prepare v0.2.0"
git push origin develop
```

### 2. Create Release PR

```bash
# Create PR from develop to main
gh pr create --base main --head develop \
  --title "Release v0.2.0" \
  --body "Release notes...

## Changes
- Feature 1
- Feature 2
- Bug fixes
"
```

### 3. After Approval, Tag Release

```bash
# Merge PR via GitHub UI
# Then locally:
git checkout main
git pull origin main

# Create version tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

### 4. Merge Back to Develop

```bash
# Ensure develop has any hotfixes from main
git checkout develop
git merge main
git push origin develop
```

---

## Hotfixes

For critical bugs in production (`main`):

```bash
# 1. Branch from main
git checkout main
git pull origin main
git checkout -b hotfix/critical-bug

# 2. Fix bug and commit
git add .
git commit -m "fix(core): resolve critical bug"

# 3. Create PR to main
gh pr create --base main \
  --title "hotfix: Critical bug fix" \
  --label "hotfix"

# 4. After merge to main, also merge to develop
git checkout develop
git merge main
git push origin develop

# 5. Tag new patch version
git checkout main
git tag -a v0.1.1 -m "Hotfix v0.1.1"
git push origin v0.1.1
```

---

## Best Practices

### Commits

✅ **Do:**

- Use conventional commits: `feat:`, `fix:`, `docs:`, etc.
- Keep commits focused and atomic
- Write clear commit messages
- Reference issues: `fix(auth): resolve token refresh (#123)`

❌ **Don't:**

- Mix multiple changes in one commit
- Use vague messages like "fix stuff" or "update code"
- Commit broken code
- Commit secrets or credentials

### Branches

✅ **Do:**

- Keep feature branches short-lived (< 1 week)
- Regularly merge `develop` into your feature branch
- Delete branches after merging
- Use descriptive branch names

❌ **Don't:**

- Work directly on `main` or `develop`
- Let feature branches get stale (> 2 weeks)
- Create branches from other feature branches
- Push to someone else's feature branch without permission

### Pull Requests

✅ **Do:**

- Keep PRs focused on one feature/fix
- Write detailed PR descriptions
- Link related issues
- Request reviews promptly
- Respond to feedback quickly

❌ **Don't:**

- Create massive PRs (> 500 lines changed)
- Merge your own PRs without review
- Force push to PR branches after review
- Leave PRs open indefinitely

---

## Common Scenarios

### Scenario 1: Need to switch features mid-work

```bash
# Save current work
git add .
git commit -m "wip: partial implementation"

# Or use stash
git stash

# Switch to other feature
git checkout other-feature

# Come back later
git checkout original-feature

# If you stashed
git stash pop
```

### Scenario 2: Accidentally committed to wrong branch

```bash
# If you committed to develop instead of feature branch
git checkout develop

# Move commit to new branch
git checkout -b feature/oops
git push -u origin feature/oops

# Reset develop
git checkout develop
git reset --hard origin/develop
```

### Scenario 3: Need to update feature branch with develop

```bash
# Option 1: Merge (preserves history)
git checkout feature/my-feature
git merge develop

# Option 2: Rebase (cleaner history, but rewrites commits)
git checkout feature/my-feature
git rebase develop
# Resolve conflicts if any
git push --force-with-lease  # Be careful with this!
```

---

## Troubleshooting

### "Your branch is behind 'origin/develop'"

```bash
git pull origin develop
```

### "Your branch has diverged from 'origin/feature'"

```bash
# If you haven't shared your commits
git pull --rebase

# If others are working on same branch
git pull --no-rebase
```

### "Merge conflict"

```bash
# 1. See conflicted files
git status

# 2. Open files and resolve conflicts
# Look for <<<<<<< HEAD and >>>>>>> markers

# 3. Mark as resolved
git add <resolved-files>

# 4. Complete merge
git commit
```

---

## CI/CD Integration

All branches trigger CI checks:

- `main` - Full CI + deployment to production
- `develop` - Full CI + deployment to staging
- `feature/*` - CI checks only (lint, test, build)

**Required checks before merge:**

- ✅ All tests pass
- ✅ Linting passes
- ✅ Build succeeds
- ✅ Code coverage maintained

See [.github/workflows/](.github/workflows/) for CI configuration.

---

## References

- [Git Flow Original](https://nvie.com/posts/a-successful-git-branching-model/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub Flow](https://githubflow.github.io/)
- [CONTRIBUTING.md](../CONTRIBUTING.md)

---

**Last Updated:** October 23, 2025
**Maintained By:** Altair Development Team
