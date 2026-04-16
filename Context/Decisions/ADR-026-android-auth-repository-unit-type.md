# ADR-026: `AuthRepository` Returns `Unit` — Exception-Based Error Propagation

| Field | Value |
|---|---|
| **Status** | Accepted |
| **Date** | 2026-04-16 |
| **Feature** | 009-AndroidClient |

## Context

`AuthRepository.login()` and `register()` return `Unit`. Success/failure is expressed through exceptions. `AuthViewModel` catches `Exception` and extracts `e.message`, which is nullable and opaque.

The alternative is a sealed `AuthResult` type (e.g., `Success`, `InvalidCredentials`, `NetworkError`, `UnknownError`) that makes the error domain visible in the type system.

## Decision

Accept the current exception-based approach for the `AuthRepository` interface. Do not introduce a sealed `AuthResult` type at this time.

## Rationale

- The project uses Retrofit with suspend functions. Retrofit already converts HTTP errors into typed `HttpException` (with `code()`) and network failures into `IOException`. The exception hierarchy is rich enough to distinguish error categories at the call site if needed.
- `AuthViewModel` currently presents a single error string to the user. There is no UI that needs to distinguish "wrong password" from "server down" with different actions — both show an error message. A sealed type would add complexity without changing user-visible behavior.
- If business logic requires distinct handling (e.g., showing "try a different password" vs. "check your connection"), introduce `AuthResult` at that time.

## Consequences

- `AuthViewModel` catch blocks use `e.message ?: "fallback"` — this is acceptable given the current single-string error display
- If a future feature requires differentiating error types (rate limiting, email verification, etc.), refactor `AuthRepository` to return a sealed `AuthResult` at that point
- The `kotlin-android.md` guideline for "sealed classes for error states" applies to domain operations; auth failure strings are treated as UI-layer concerns here

## Future Trigger

Revisit this decision if:
- A screen needs to show different UI for credential errors vs. network errors
- Rate limiting or account lockout requires distinguishing `429` from `401`
