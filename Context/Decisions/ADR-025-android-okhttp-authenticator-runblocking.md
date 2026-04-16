# ADR-025: `runBlocking` in `AuthAuthenticator` — Accepted Deviation

| Field | Value |
|---|---|
| **Status** | Accepted |
| **Date** | 2026-04-16 |
| **Feature** | 009-AndroidClient |

## Context

`kotlin-android.md` states: "No `runBlocking` outside of main function or tests."

`AuthAuthenticator` uses `runBlocking { authApi.refresh(...) }` to bridge OkHttp's synchronous `Authenticator` contract with the Retrofit suspend-based `authApi`.

## Decision

Accept `runBlocking` in `AuthAuthenticator` as a documented, narrow deviation from the project rule.

Add an inline comment citing the OkHttp contract so future reviewers understand the intent:
```kotlin
// runBlocking is intentional here: OkHttp's Authenticator is synchronous by contract
// (runs on OkHttp's thread pool, never the main thread). This is the standard bridge.
```

## Alternatives Considered

**Synchronous Retrofit call (`Call<T>.execute()`)**: Would eliminate the coroutine bridge by using Retrofit's blocking API directly. This is a valid alternative but requires a separate synchronous Retrofit interface, adding complexity.

**Custom OkHttp Interceptor (not Authenticator)**: Would allow coroutine suspension but loses OkHttp's built-in retry-after-refresh behavior. More code, same result.

## Rationale

OkHttp's `Authenticator` is always invoked on OkHttp's internal thread pool — never on the Android main thread. `runBlocking` on a background thread does not block the UI. The pattern is idiomatic for bridging OkHttp with coroutine-based auth APIs in Android and is widely documented in the OkHttp and Retrofit communities.

## Consequences

- The deviation is documented and intentional; future reviewers will not flag it as a mistake
- The rule in `kotlin-android.md` remains unchanged (this is a site-specific exception, not a rule change)
- If the project moves to a fully synchronous auth API, the `runBlocking` can be removed
