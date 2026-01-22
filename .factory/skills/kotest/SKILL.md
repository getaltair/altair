---
name: kotest
description: Kotest testing framework patterns and examples for Kotlin. Use when writing tests, setting up test specs, configuring assertions, property-based testing, data-driven testing, or working with non-deterministic tests.
---

# Skill: Kotest Testing Framework

## Purpose

Provide comprehensive guidance on using Kotest 5.9.1+ for testing Kotlin multiplatform applications. This skill covers testing styles, assertions, property-based testing, data-driven testing, test configuration, and lifecycle management.

## When to use this skill

Use this skill when:

- **Writing unit tests** for Kotlin applications
- **Setting up test specs** using different testing styles (FunSpec, StringSpec, DescribeSpec, BehaviorSpec, etc.)
- **Writing assertions** with Kotest matchers (`shouldBe`, `shouldContain`, `shouldThrow`, etc.)
- **Testing collections** with inspectors (`forAll`, `forAtLeast`, `forNone`, etc.)
- **Creating custom matchers** for domain-specific testing
- **Property-based testing** with random inputs using `checkAll` or `forAll`
- **Data-driven testing** with multiple input/output combinations using `withData`
- **Non-deterministic testing** with `eventually` or `retry` for async/flaky code
- **Configuring tests** with lifecycle hooks, tags, timeouts, or parallelism
- **Migrating to Kotest 6.0** from 5.x versions

## Inputs

- **Test scenario**: What functionality is being tested
- **Test strategy**: Unit, integration, or property-based testing approach
- **Project context**: Kotlin multiplatform, specific targets (jvm, android, ios)
- **Existing test patterns**: Current testing conventions in the codebase

## Conventions

### Altair Project Conventions

Based on the project's coding guidelines:

- **Framework**: Kotest 5.9.1 for BDD-style testing framework
- **Test styles** (choose based on test type):
  - `BehaviorSpec`: BDD-style `given/when/then` for integration and behavioral tests
  - `DescribeSpec`: RSpec-style `describe/it` for unit tests with hierarchical structure
  - `FunSpec`: Simple `test("name") {}` for straightforward tests
- **Assertions**: Use Kotest matchers and Arrow Either matchers
- **Async testing**: Use `eventually` for non-deterministic operations
- **Mocking**: Use Mokkery for multiplatform mocking
- **Flow testing**: Use Turbine for testing Kotlin Flows

### File Structure

- **Unit tests**: `commonTest` for platform-agnostic logic
- **Platform tests**: `jvmTest`, `androidTest`, `iosTest` for platform-specific code

## Required behavior

1. **Follow appropriate test style**: Choose spec style based on test complexity (BehaviorSpec for integration, DescribeSpec for unit, FunSpec for simple)
2. **Use descriptive test names**: Test names should clearly describe what is being tested
3. **Cover both success and error paths**: Especially for Arrow Either returns
4. **Use property-based testing** for testing invariants across many inputs
5. **Add assertions**: Use Kotest matchers with proper context clues (`withClue`, `asClue`)

## Required artifacts

- Test classes using appropriate Kotest spec style
- Test cases covering success, error, and edge cases
- Property-based tests when applicable (especially for data transformations)
- Integration tests for cross-module interactions (desktop event bus, etc.)

## Implementation checklist

1. Choose appropriate test style based on test complexity
2. Identify test scenarios and expected behavior
3. Write test cases with clear names and structure
4. Add assertions using Kotest matchers
5. Consider property-based testing for data validation
6. Add lifecycle hooks for setup/teardown if needed
7. Tag tests appropriately for filtering
8. Run tests to verify they pass

## Verification

Run the following commands to verify the skill implementation:

```bash
# Run all tests
./gradlew test

# Run specific target tests
./gradlew :shared:jvmTest
./gradlew :composeApp:jvmTest
./gradlew :composeApp:iosSimulatorArm64Test

# Run all tests with aggregated report
./gradlew allTests
```

The skill is complete when:

- All tests pass on the relevant target platforms
- Tests cover success, error, and edge cases
- Arrow Either patterns are properly tested with `shouldBeRight()` / `shouldBeLeft()`
- Property-based tests use `checkAll` or `forAll` appropriately
- Tests follow project naming and structure conventions

## Safety and escalation

- If you encounter issues with platform-specific tests (e.g., iOS tests), focus on commonTest and jvmTest first
- If tests fail with timeouts or concurrency issues, adjust test configuration with appropriate timeouts and isolation modes
- For expensive database operations, consider using Testcontainers or in-memory alternatives

## Quick reference

### Basic test structure

**FunSpec - Simple tests**:
```kotlin
class MyTests : FunSpec({
    test("String length should return the length of the string") {
        "sammy".length shouldBe 5
        "".length shouldBe 0
    }
})
```

**DescribeSpec - Hierarchical tests**:
```kotlin
class MyTests : DescribeSpec({
    describe("String.length") {
        it("should return the length of the string") {
            "sammy".length shouldBe 5
        }
    }
})
```

**BehaviorSpec - BDD style**:
```kotlin
class MyTests : BehaviorSpec({
    given("a user") {
        `when`("logging in with valid credentials") {
            then("access should be granted") {
                // test logic
            }
        }
    }
})
```

### Assertions

```kotlin
// Basic matchers
"substring".shouldContain("str")
user.email.shouldBeLowerCase()
cityMap.shouldContainKey("London")

// Arrow Either matchers
result.shouldBeRight()
result.shouldBeLeft()
result.leftOrNull().shouldBeInstanceOf<DomainError.NotFoundError>()

// Exception testing
shouldThrow<IllegalAccessException> {
    // code that should throw
}

// Collection inspectors
xs.forAtLeast(2) { it.shouldHaveMinLength(7) }
xs.forNone { it.shouldContain("x") }
```

### Arrow Either testing

```kotlin
class MyTest : BehaviorSpec({
    given("a user repository") {
        `when`("fetching an existing user") {
            then("should return Right with user") {
                val result = repository.getById(userId)
                result.shouldBeRight()
                result.valueOrNull()?.id shouldBe userId
            }
        }
        `when`("fetching a non-existent user") {
            then("should return Left with NotFoundError") {
                val result = repository.getById(nonExistentId)
                result.shouldBeLeft()
                result.leftOrNull().shouldBeInstanceOf<NotFoundError>()
            }
        }
    }
})
```

### Property-based testing

```kotlin
class PropertyExample : StringSpec({
    "String size" {
        checkAll<String, String> { a, b ->
            (a + b).length shouldBe a.length + b.length
        }
    }
})
```

### Data-driven testing

```kotlin
withData(
    Triple(1, 5, 5),
    Triple(1, 0, 1),
    Triple(0, 0, 0)
) { (a, b, max) ->
    Math.max(a, b) shouldBe max
}
```

### Non-deterministic testing

```kotlin
eventually(5.seconds) {
    repository.getById(id).shouldBeRight()
}
```

### Lifecycle hooks

```kotlin
class TestSpec : WordSpec({
    beforeTest {
        println("Starting a test $it")
    }
    afterTest { (test, result) ->
        println("Finished spec with result $result")
    }
})
```

## Dependencies

The project uses Kotest 5.9.1 via Gradle:

```kotlin
testImplementation("io.kotest:kotest-runner-junit5:5.9.1")
testImplementation("io.kotest:kotest-assertions-core:5.9.1")
testImplementation("io.kotest:kotest-property:5.9.1")
```

## Kotest 6.0 Migration Notes

If you need to upgrade to Kotest 6.0:

- Minimum JDK 11 and Kotlin 2.2 required
- Extension registration: Use `override val extensions` instead of `fun extensions()`
- Classpath scanning removed: Extensions must be explicitly registered
- `withData` merged into core: No separate dependency needed
- `InstancePerRoot` added as new isolation mode
- `InstancePerLeaf` and `InstancePerTest` deprecated

## Project-specific resources

- `@CLAUDE.md` - Testing strategy and framework setup
- `@docs/architecture/` - System architecture for integration test context
- Search for existing test patterns: `rg "Either.*shouldBeRight"` or `rg "class.*: BehaviorSpec"`

## Reference documentation

Additional documentation is available in the `references/` directory:

| File | Description |
|------|-------------|
| **framework.md** | Core framework: testing styles, lifecycle hooks, configuration |
| **assertions.md** | Assertions and matchers, inspectors, custom matchers |
| **proptest.md** | Property-based testing: generators, seeds, assumptions |
| **extensions.md** | Extensions documentation and usage |
| **6.0.md** | Kotest 6.0 specific features and migration guide |
| **5.9.x.md** | Version 5.9.x documentation and changelog |
| **5.8.x.md** | Version 5.8.x documentation and changelog |
| **next.md** | Upcoming/development version documentation |
| **other.md** | Additional documentation and miscellaneous topics |
