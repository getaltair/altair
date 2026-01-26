---
name: gen-kmp-test
description: Generate Kotest test specifications for Kotlin Multiplatform code with proper expect/actual patterns
user-invocable: true
disable-model-invocation: false
---

# Multiplatform Test Generator

Generate comprehensive Kotest test specifications for Kotlin Multiplatform code following Altair's testing patterns.

## Usage

```
/gen-kmp-test <source-file-path> [test-style]
```

**Arguments**:
- `source-file-path`: Path to the Kotlin source file to test (required)
- `test-style`: Kotest spec style - `BehaviorSpec` (default), `DescribeSpec`, or `FunSpec`

**Examples**:
```
/gen-kmp-test shared/src/commonMain/kotlin/domain/auth/AuthManager.kt
/gen-kmp-test shared/src/commonMain/kotlin/data/repository/UserRepository.kt DescribeSpec
```

## Process

### 1. Analyze Source File

Read and analyze the source file for:
- **Platform-specific code**: `expect`/`actual` declarations
- **Error handling**: Arrow `Either<DomainError, T>` return types
- **Async code**: Kotlin `Flow`, coroutines, `suspend` functions
- **Dependencies**: Constructor parameters requiring mocking
- **Domain logic**: Business rules to test

### 2. Determine Test Location

Place test in the correct source set:
- `commonTest/` for `expect` declarations and common logic
- `jvmTest/` for JVM `actual` implementations
- `androidUnitTest/` for Android `actual` implementations
- `iosTest/` for iOS `actual` implementations

### 3. Generate Test Structure

Create test file using specified Kotest spec style:

**BehaviorSpec** (default - for integration and behavioral tests):
```kotlin
class FeatureNameTest : BehaviorSpec({
    given("initial state") {
        `when`("action occurs") {
            then("expected outcome") {
                // assertions
            }
        }
    }
})
```

**DescribeSpec** (for unit tests with hierarchical structure):
```kotlin
class ClassNameTest : DescribeSpec({
    describe("methodName") {
        it("should do expected behavior") {
            // assertions
        }
    }
})
```

**FunSpec** (for simple, straightforward tests):
```kotlin
class ClassNameTest : FunSpec({
    test("descriptive test name") {
        // assertions
    }
})
```

### 4. Include Required Testing Patterns

**Arrow Either Matchers**:
```kotlin
result.shouldBeRight()
result.shouldBeLeft()
result.leftOrNull().shouldBeInstanceOf<DomainError.NotFoundError>()
```

**Flow Testing with Turbine**:
```kotlin
flow.test {
    awaitItem() shouldBe expected
    awaitComplete()
}
```

**Mocking with Mokkery**:
```kotlin
val mockRepo = mock<UserRepository>()
everySuspend { mockRepo.getUser(any()) } returns Either.Right(user)
```

**Async Testing**:
```kotlin
eventually(5.seconds) {
    repository.getById(id).shouldBeRight()
}
```

**Lifecycle Hooks**:
```kotlin
beforeEach {
    // Setup before each test
}

afterEach {
    // Cleanup after each test
}
```

### 5. Follow Project Patterns

- Import from existing test utilities in the same module
- Use test data builders if available
- Follow naming conventions from existing tests
- Include property-based tests for pure functions where applicable
- Test both success and error cases for `Either` returns

## Test Coverage Requirements

For repository/service classes:
- ✅ Test all public methods
- ✅ Test success paths
- ✅ Test error paths (each error type)
- ✅ Test edge cases (null, empty, boundary values)
- ✅ Verify dependency interactions with mocks

For domain logic:
- ✅ Test business rule validations
- ✅ Test state transitions
- ✅ Test invariants with property-based tests

For UI components:
- ✅ Test user interactions
- ✅ Test state changes
- ✅ Test navigation

## Example Output

For `shared/src/commonMain/kotlin/domain/auth/AuthManager.kt`:

Creates `shared/src/commonTest/kotlin/domain/auth/AuthManagerTest.kt`:

```kotlin
package com.getaltair.altair.domain.auth

import app.cash.turbine.test
import arrow.core.Either
import com.getaltair.altair.domain.error.DomainError
import dev.mokkery.answering.returns
import dev.mokkery.everySuspend
import dev.mokkery.mock
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.shouldBe
import kotlin.time.Duration.Companion.seconds
import kotlinx.coroutines.delay

class AuthManagerTest : BehaviorSpec({
    given("an AuthManager with valid credentials") {
        val mockAuthService = mock<AuthService>()
        val mockTokenStorage = mock<TokenStorage>()
        val authManager = AuthManager(mockAuthService, mockTokenStorage)

        val validToken = "valid.jwt.token"
        everySuspend { mockAuthService.login(any()) } returns
            Either.Right(AuthResponse(validToken, "refresh"))

        `when`("user logs in") {
            val result = authManager.login("user@example.com", "password")

            then("should return success") {
                result.shouldBeRight()
            }

            then("should store token") {
                verifySuspend { mockTokenStorage.saveAccessToken(validToken) }
            }
        }
    }

    given("an AuthManager with invalid credentials") {
        val mockAuthService = mock<AuthService>()
        val authManager = AuthManager(mockAuthService, mock())

        everySuspend { mockAuthService.login(any()) } returns
            Either.Left(DomainError.AuthError.InvalidCredentials)

        `when`("user logs in with wrong password") {
            val result = authManager.login("user@example.com", "wrong")

            then("should return authentication error") {
                result.shouldBeLeft()
                result.leftOrNull().shouldBeInstanceOf<DomainError.AuthError.InvalidCredentials>()
            }
        }
    }
})
```

## Notes

- Always check for existing test utilities in the module
- Use the `/kotest` skill for detailed Kotest patterns and examples
- Consult CLAUDE.md for Arrow error handling patterns
- Run tests after generation: `./gradlew :module:test --tests ClassName`
