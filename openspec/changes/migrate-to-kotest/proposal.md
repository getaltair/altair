# Change: Migrate All Testing to Kotest

## Why

The current test suite uses `kotlin.test` annotations with basic assertions, missing opportunities to leverage modern testing practices. This proposal establishes a comprehensive testing standard using Kotest's full feature set:

- **Behavior-driven testing** with expressive given/when/then structure
- **Property-based testing** for exhaustive domain model validation
- **Data-driven testing** for comprehensive edge case coverage
- **Structured test organization** with nested contexts and clear naming
- **First-class coroutine support** with deterministic time control
- **Powerful matchers** with detailed failure diagnostics

This is not a migration—it's a complete rewrite of the test suite to establish best practices.

## What Changes

### Testing Philosophy

**From**: Ad-hoc tests with basic assertions
**To**: Structured, behavior-driven tests with comprehensive coverage

- Tests describe behaviors, not implementation details
- Property tests validate invariants across input space
- Data-driven tests cover edge cases systematically
- Test names read as specifications

### Infrastructure Changes

- Add Kotest dependencies to all modules
- Configure JUnit Platform runner for JVM targets
- Add `AbstractProjectConfig` with strict settings (assertion mode, timeouts)
- Add `kotest-extensions-testcontainers` for server integration tests
- Configure property test defaults (iterations, seed logging)

### Test Suite Rewrite (43 test files across 3 modules)

**shared module** (14 test files):
- Domain model tests → BehaviorSpec with property-based validation
- Serialization tests → DescribeSpec with data-driven test cases
- Error handling tests → BehaviorSpec with exhaustive scenario coverage
- Auth tests → BehaviorSpec for security-critical flows

**composeApp module** (11 test files):
- Navigation tests → BehaviorSpec for user flow scenarios
- Component tests → DescribeSpec with lifecycle testing
- UI state tests → Property tests for state machine invariants

**server module** (18 test files):
- Repository tests → BehaviorSpec with Testcontainers extension
- RPC tests → BehaviorSpec for request/response scenarios
- Auth service tests → DescribeSpec with security edge cases

### Spec Style Standards

| Test Category | Spec Style | Rationale |
|--------------|------------|-----------|
| Domain validation | BehaviorSpec | Clear given/when/then for business rules |
| Serialization | DescribeSpec | Organize by type with nested describe blocks |
| Repository CRUD | BehaviorSpec | User-centric scenarios |
| Component behavior | DescribeSpec | Group by component method/state |
| Security/Auth | BehaviorSpec | Explicit preconditions and outcomes |
| Pure functions | FunSpec + property tests | Mathematical properties |
| Edge cases | Data-driven (`withData`) | Systematic coverage |

## Impact

- **Affected code**: All test files rewritten from scratch
- **Dependencies**: Kotest 5.9.1, kotest-extensions-testcontainers
- **Build configuration**: All module `build.gradle.kts` files
- **Documentation**: Testing guidelines added to CLAUDE.md
- **No runtime impact**: Testing infrastructure only

## References

- Kotest Skill: `.claude/skills/kotest/` - Comprehensive patterns and examples
- Kotest Official Docs: https://kotest.io/docs/
- Property Testing: `.claude/skills/kotest/references/proptest.md`
- Assertions: `.claude/skills/kotest/references/assertions.md`
