# ADR-022: Android HTTP Client and Serialization Library

| Field | Value |
|---|---|
| **Status** | Accepted |
| **Date** | 2026-04-16 |
| **Feature** | 009-AndroidClient |

## Context

The Android client (Feature 009) requires an HTTP client for two purposes:

1. **Auth endpoints** — login, register, token refresh (`/api/auth/*`)
2. **PowerSync CRUD upload** — batching local writes to the server REST API

The `apps/android/app/src/main/java/com/getaltair/altair/contracts/Dtos.kt` already defines DTOs with a note that "JSON serialization annotations are deferred to Step 8 (Android Client) when the JSON library is chosen." A serialization library must be chosen now.

Three options were considered:

**Option 1 — OkHttp + Retrofit + kotlinx.serialization**
- OkHttp: industry standard, first-class auth interceptor API (`Interceptor` / `Authenticator`)
- Retrofit: type-safe API interface definitions, reduces boilerplate vs raw OkHttp
- kotlinx.serialization: Kotlin-native, compile-time code generation, no runtime reflection, KMP-compatible

**Option 2 — OkHttp + Retrofit + Gson**
- Same HTTP story as Option 1
- Gson: runtime reflection, not recommended for obfuscation-heavy production apps; not Kotlin-native

**Option 3 — Ktor Client + kotlinx.serialization**
- Full Kotlin/KMP HTTP client
- Adds an unfamiliar dependency; the server uses Axum/REST (not Ktor-specific protocols); no benefit over Retrofit for this use case
- Higher integration risk for the auth interceptor pattern required by the Spec (FA-019)

## Decision

**Accept Option 1: OkHttp + Retrofit + kotlinx.serialization.**

OkHttp's `Authenticator` interface maps directly to the 401-refresh-retry flow required by FA-019. Retrofit provides compile-time interface-based API definitions for the Altair REST endpoints. kotlinx.serialization is Kotlin-native, generates code at compile time, and avoids the Proguard/R8 configuration complexity of reflection-based libraries.

The `Dtos.kt` file will be annotated with `@Serializable` from `kotlinx.serialization` during implementation.

## Gradle Coordinates

```toml
# libs.versions.toml
[versions]
okhttp = "4.12.0"
retrofit = "2.11.0"
kotlinx-serialization = "1.8.0"

[libraries]
okhttp = { module = "com.squareup.okhttp3:okhttp", version.ref = "okhttp" }
okhttp-logging = { module = "com.squareup.okhttp3:logging-interceptor", version.ref = "okhttp" }
retrofit = { module = "com.squareup.retrofit2:retrofit", version.ref = "retrofit" }
retrofit-kotlinx-serialization = { module = "com.squareup.retrofit2:converter-kotlinx-serialization", version.ref = "retrofit" }
kotlinx-serialization-json = { module = "org.jetbrains.kotlinx:kotlinx-serialization-json", version.ref = "kotlinx-serialization" }
```

## Consequences

- The `NetworkModule` (Koin) provides a single `OkHttpClient` instance with the auth interceptor, and a `Retrofit` instance configured with the kotlinx.serialization converter factory.
- The auth interceptor attaches the JWT from DataStore to every request and refreshes on 401 (FA-019).
- `Dtos.kt` contracts require `@Serializable` annotations — added during implementation (S-DI step per Steps.md).
- Ktor is not introduced; any future KMP migration can substitute Ktor behind the repository interface without changing calling code.
