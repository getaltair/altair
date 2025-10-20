# Linting and Code Quality

This document explains the linting and code quality checks used in the Altair project, ensuring consistency between local development and CI pipeline.

## Overview

**Critical Principle**: Local pre-commit hooks MUST catch the same issues as CI pipeline checks to prevent failed builds.

## Pre-commit Hooks vs CI Pipeline

The project is configured to ensure pre-commit hooks match CI pipeline checks exactly:

| Check | Pre-commit | CI | Notes |
|-------|-----------|----|----|
| **Python: Ruff lint** | ✅ `--fix` | ✅ `check` | Pre-commit auto-fixes for dev convenience |
| **Python: Ruff format** | ✅ Auto-fix | ✅ Check only | Pre-commit auto-fixes for dev convenience |
| **Python: mypy** | ✅ | ✅ | Identical behavior |
| **Flutter: analyze** | ✅ | ✅ | **Identical** - Catches all analysis warnings |
| **Markdown: lint** | ✅ Auto-fix | ❌ | Local-only, auto-fixes formatting |
| **Secrets: gitleaks** | ✅ | ❌ | Local-only, prevents secret commits |
| **File checks** | ✅ | ❌ | Local-only (whitespace, EOF, line endings) |

## Python Linting

### Ruff Configuration

**Pre-commit behavior:**

```bash
# Automatically fixes issues and forces you to stage the fixes
ruff check . --fix --exit-non-zero-on-fix
ruff format .
```

**CI behavior:**

```bash
# Check-only mode, fails if any issues found
ruff check .
ruff format --check .
```

**Why the difference?**

- Local: Auto-fixes for developer convenience
- CI: Check-only to ensure all fixes are committed
- Both catch the same issues ✅

### Mypy Type Checking

**Identical behavior in both pre-commit and CI:**

```bash
mypy app/
```

- Runs on `services/auth-service` only
- Same strict type checking in both environments

## Flutter Linting

### Flutter Analyze

**NEW**: Flutter analyze is now part of pre-commit hooks!

**Pre-commit behavior:**

```bash
# Runs flutter analyze on all Flutter packages
scripts/pre-commit/flutter-analyze.sh
```

**CI behavior:**

```bash
# Runs flutter analyze on each package
flutter analyze
```

**Packages checked** (identical in both):

- `packages/altair-core`
- `packages/altair-auth`
- `packages/altair-ui`
- `apps/altair_guidance`

**Why this matters:**

- ❌ Before: Local commits could pass but CI would fail on Flutter warnings
- ✅ After: Flutter analyze runs locally, catching issues before commit

## Local-Only Checks

These checks run only in pre-commit hooks for developer convenience:

### Markdown Linting

```bash
markdownlint --fix
```

- Auto-fixes markdown formatting issues
- Not critical for CI, so kept local-only

### Secrets Detection

```bash
gitleaks
```

- Prevents committing secrets
- Critical for security, runs locally only

### File Quality Checks

- Trailing whitespace
- End of files
- YAML/JSON/TOML validation
- Large files
- Merge conflicts
- Case conflicts
- Line ending consistency (LF)
- Private key detection

## How to Use

### First-Time Setup

Install pre-commit hooks:

```bash
pre-commit install
```

### Running Hooks Manually

Run all hooks on staged files:

```bash
pre-commit run
```

Run all hooks on all files:

```bash
pre-commit run --all-files
```

Run specific hook:

```bash
pre-commit run flutter-analyze
pre-commit run ruff
pre-commit run mypy
```

### Updating Hooks

Update to latest versions:

```bash
pre-commit autoupdate
```

## CI Pipeline Alignment

### Verification

To verify your code will pass CI before pushing:

```bash
# Run all pre-commit hooks
pre-commit run --all-files

# Manually run CI-equivalent commands:

# Python (from services/auth-service)
uv run ruff check .
uv run ruff format --check .
uv run mypy app/

# Flutter (from each package)
flutter analyze
flutter test
```

### Why Pre-commit Matches CI

**Problem solved:** Before adding Flutter analyze to pre-commit:

- Developers would commit code that passed local checks
- CI would fail on Flutter analysis warnings
- This wastes time and creates frustration

**Solution:**

- Flutter analyze now runs in pre-commit hooks
- Catches ALL issues that CI would catch
- Developers see failures immediately, not after pushing

## Troubleshooting

### Pre-commit Hook Fails but Code Looks Fine

If Flutter analyze fails:

```bash
# Run manually to see details
cd packages/altair-ui
flutter analyze
```

Common issues:

- Unused variables/imports
- Deprecated API usage
- Type errors
- Invalid constant values
- Missing getters/properties

### Skipping Hooks (Not Recommended)

Only skip hooks for emergency fixes:

```bash
git commit --no-verify
```

⚠️ **Warning:** Skipping hooks may cause CI failures!

### Hooks Are Slow

Flutter analyze can take 5-10 seconds. This is normal and ensures code quality.

To speed up:

- Only commit Dart files when working on Flutter code
- Use `git commit -n` to skip hooks if you're just updating docs (not recommended)

## Best Practices

1. **Let hooks auto-fix**: Pre-commit hooks will fix formatting issues automatically
2. **Stage the fixes**: After hooks auto-fix, stage the changes and commit again
3. **Don't skip hooks**: Skipping hooks leads to CI failures
4. **Run manually before big commits**: Run `pre-commit run --all-files` before large commits
5. **Keep hooks updated**: Run `pre-commit autoupdate` monthly

## Adding New Checks

When adding new lint checks:

1. Add to `.pre-commit-config.yaml`
2. Add identical check to `.github/workflows/ci.yml`
3. Update this documentation
4. Test with `pre-commit run --all-files`
5. Verify CI passes on a test PR

## Summary

✅ **Local pre-commit hooks now match CI exactly**

- Flutter analyze catches all warnings before commit
- Python linting is identical (Ruff, mypy)
- No more surprise CI failures on lint checks
- Faster development workflow

⚠️ **Key takeaway**: If pre-commit passes, CI will pass (for lint checks)!
