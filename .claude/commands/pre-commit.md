# Pre-Commit Quality Checks

Run all pre-commit quality checks and fix any issues found.

## Step 1: Run Full Check Suite

Run `./gradlew check` to execute all quality checks in one go:
- Spotless code formatting
- Detekt static analysis
- All unit tests (JVM, iOS, Android)
- Android lint
- All other configured checks

**IMPORTANT**: When ambiguity exists, multiple valid fix options are available, or user input is required for resolving an issue, **halt execution immediately** and prompt the user for their explicit decision before proceeding with any changes.

## Step 2: Fix Issues Iteratively

If `./gradlew check` fails, analyze the errors and fix them in this order:

### 2a. Formatting Issues (Spotless)
- Run `./gradlew spotlessApply` to auto-fix formatting
- Re-run `./gradlew spotlessCheck` to verify

### 2b. Static Analysis Issues (Detekt)
- Read the Detekt error messages
- Fix code quality issues manually
- Re-run `./gradlew detekt` to verify

### 2c. Test Failures
- Read test failure messages
- Fix the underlying code issues (or tests if they're incorrect)
- Re-run the specific test task to verify (e.g., `./gradlew :composeApp:jvmTest`)

### 2d. Lint Issues
- Read Android lint warnings/errors
- Fix issues or suppress with justification
- Re-run `./gradlew lint` to verify

## Step 3: Repeat

**Repeat Step 1 and Step 2 iteratively** until `./gradlew check` passes with a clean exit status (exit code 0).

## CRITICAL TEST RULES - YOU MUST FOLLOW THESE

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
4. Number of lint issues fixed
5. Final status: all checks passing or any remaining issues requiring user attention
