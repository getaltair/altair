# Pre-Commit Quality Checks

Run all pre-commit quality checks and fix any issues found. Execute these steps in order:

## Step 1: Spotless (Code Formatting)

Run `./gradlew spotlessApply` to fix formatting issues. Then run `./gradlew spotlessCheck` to verify all issues are resolved.

**Repeat this step iteratively** until spotlessCheck reports zero issues with a clean exit status (exit code 0).

**IMPORTANT**: When ambiguity exists, multiple valid fix options are available, or user input is required for resolving an issue, **halt execution immediately** and prompt the user for their explicit decision before proceeding with any changes.

## Step 2: Detekt (Static Analysis)

Run `./gradlew detekt` to identify code quality issues. Fix any issues found.

**Repeat this step iteratively** until detekt reports zero issues with a clean exit status (exit code 0).

**IMPORTANT**: When ambiguity exists, multiple valid fix options are available, or user input is required for resolving an issue, **halt execution immediately** and prompt the user for their explicit decision before proceeding with any changes.

## Step 3: Unit Tests

Run `./gradlew test` to execute all unit tests. Fix any failing tests.

**Repeat this step iteratively** until all tests pass successfully with zero failures.

**CRITICAL TEST RULES - YOU MUST FOLLOW THESE**:
- **DO NOT** mark test cases as "skipped" or ignored
- **DO NOT** disable tests using annotations like `@Ignore`, `@Disabled`, or platform equivalents
- **DO NOT** comment out test code
- **DO NOT** modify test assertions to always pass (e.g., changing expected values to match incorrect actual values)
- **DO NOT** skip test execution in any way
- **DO NOT** delete or remove test cases

If any of the above actions seem necessary, you **MUST obtain explicit user approval first** before proceeding.

Any test modifications must:
- Preserve the original test intent
- Maintain the same test coverage
- Fix the underlying code issue rather than the test itself (when the test is correct)

## Completion

Report a summary of:
1. Number of spotless issues fixed
2. Number of detekt issues fixed
3. Number of test failures fixed
4. Final status: all checks passing or any remaining issues requiring user attention
