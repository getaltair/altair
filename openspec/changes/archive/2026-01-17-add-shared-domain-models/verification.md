# Verification Report: {{CHANGE_ID}}

> **Instructions:** Copy this template to `openspec/changes/<change-id>/verification.md` after running `/openspec:apply`. Complete each section to verify the implementation.

**Date:** YYYY-MM-DD  
**Verifier:** [Your Name]  
**Change:** [Brief description of the change]

---

## Quick Summary

| Category | Status | Notes |
|----------|--------|-------|
| Task Completion | ☐ PASS / ☐ FAIL | |
| Spec Compliance | ☐ PASS / ☐ FAIL | |
| Code Quality | ☐ PASS / ☐ FAIL | |

**Overall:** ☐ APPROVED / ☐ REJECTED

---

## 1. Task Completion

### 1.1 Checklist Audit

Review `tasks.md` and verify all tasks are marked `[x]`:

| Task ID | Description | Status | Evidence (file:line or commit) |
|---------|-------------|--------|-------------------------------|
| 1.1 | | ☐ ✓ / ☐ ✗ | |
| 1.2 | | ☐ ✓ / ☐ ✗ | |
| 2.1 | | ☐ ✓ / ☐ ✗ | |

### 1.2 Deliverable Existence

Verify all expected files/artifacts exist:

```bash
# Run these checks
[ -f "path/to/expected/file.py" ] && echo "✓ exists" || echo "✗ missing"
```

| Deliverable | Expected Location | Exists |
|-------------|-------------------|--------|
| | | ☐ Yes / ☐ No |
| | | ☐ Yes / ☐ No |

### 1.3 Acceptance Criteria

| Task | Acceptance Criteria | Verification Method | Result |
|------|---------------------|---------------------|--------|
| | | | ☐ Pass / ☐ Fail |
| | | | ☐ Pass / ☐ Fail |

**Section Result:** ☐ PASS / ☐ FAIL

---

## 2. Specification Compliance

### 2.1 Requirement Traceability

Map each SHALL/MUST requirement to its implementation:

| Req ID | Spec Statement (from specs/*.md) | Implementation Location | Verified |
|--------|----------------------------------|------------------------|----------|
| REQ-001 | | `file.py:line` | ☐ |
| REQ-002 | | `file.py:line` | ☐ |

### 2.2 Scenario Coverage

For each scenario in the spec delta:

#### Scenario: [Name from spec]
- **WHEN:** [condition]
- **THEN:** [expected result]
- **Test:** `tests/test_file.py::test_name`
- **Result:** ☐ Pass / ☐ Fail

#### Scenario: [Name from spec]
- **WHEN:** [condition]
- **THEN:** [expected result]
- **Test:** `tests/test_file.py::test_name`
- **Result:** ☐ Pass / ☐ Fail

### 2.3 Delta Alignment

| Delta Type | Count | All Implemented | Notes |
|------------|-------|-----------------|-------|
| ADDED | | ☐ Yes / ☐ No | |
| MODIFIED | | ☐ Yes / ☐ No | |
| REMOVED | | ☐ Yes / ☐ No | |

### 2.4 Scope Check

- [ ] No undocumented features added (scope creep)
- [ ] No requirements missed (spec drift)
- [ ] No orphaned code from removed requirements

**Section Result:** ☐ PASS / ☐ FAIL

---

## 3. Code Quality

### 3.1 Static Analysis

```bash
# Run linting
ruff check .           # Python
eslint .               # JavaScript/TypeScript
```

| Tool | Command | Exit Code | Errors | Warnings |
|------|---------|-----------|--------|----------|
| Linter | | | | |
| Type Checker | | | | |
| Formatter | | | | |

### 3.2 Convention Compliance

| Convention | Expected | Actual | ☐ |
|------------|----------|--------|---|
| File naming | `snake_case.py` | | ☐ |
| Function naming | `snake_case` | | ☐ |
| Class naming | `PascalCase` | | ☐ |
| Import ordering | stdlib → external → internal | | ☐ |
| Docstrings | All public APIs | | ☐ |

### 3.3 Test Results

```bash
# Run tests
pytest --tb=short -v   # Python
npm test               # JavaScript/TypeScript
```

| Metric | Value |
|--------|-------|
| Tests Run | |
| Passed | |
| Failed | |
| Skipped | |
| Coverage (new code) | % |

### 3.4 Security Review (if applicable)

- [ ] No hardcoded secrets
- [ ] All user inputs validated/sanitized
- [ ] Parameterized queries only (no SQL injection)
- [ ] Proper authentication/authorization
- [ ] ITAR compliance verified (if applicable)

**Section Result:** ☐ PASS / ☐ FAIL

---

## 4. Hallucination Detection

### 4.1 Import Verification

```bash
# Check for unknown imports
# Python: verify all imports are installed packages
# Node: npm ls --depth=0
```

| Import | Package | Installed | Real |
|--------|---------|-----------|------|
| | | ☐ | ☐ |
| | | ☐ | ☐ |

### 4.2 API Call Verification

| API/Function Called | Source | Documented | Real |
|---------------------|--------|------------|------|
| | | ☐ | ☐ |
| | | ☐ | ☐ |

### 4.3 Database/Schema Verification

| Table/Column Referenced | Exists in Schema |
|------------------------|------------------|
| | ☐ Yes / ☐ No |
| | ☐ Yes / ☐ No |

**Section Result:** ☐ PASS / ☐ FAIL

---

## Issues Found

List any issues discovered during verification:

| # | Severity | Description | Resolution |
|---|----------|-------------|------------|
| 1 | High/Medium/Low | | |
| 2 | High/Medium/Low | | |

---

## Sign-off

- [ ] All sections completed
- [ ] All critical issues resolved
- [ ] Ready for archive

**Verified by:** _________________________ **Date:** _____________

**Next step:** `openspec archive {{CHANGE_ID}} --yes`

---

*Template version: 1.0 | Compatible with OpenSpec 0.15+*
