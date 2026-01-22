# Altair (Kotlin Multiplatform)

Altair is a life management ecosystem built with Kotlin Multiplatform + Compose Multiplatform, with a self-hosted Ktor backend.

# Core Commands

- Full test suite (fast default): `./gradlew test`
- Shared JVM tests only: `./gradlew :shared:jvmTest`
- ComposeApp JVM tests only: `./gradlew :composeApp:jvmTest`
- All checks (tests + lint where applicable): `./gradlew check`

- Run desktop app: `./gradlew :composeApp:run`
- Build Android debug APK: `./gradlew :composeApp:assembleDebug`

# Project Layout

- `composeApp/` — Compose Multiplatform UI (Android/iOS/Desktop)
- `shared/` — Kotlin Multiplatform domain + data access abstractions
- `server/` — Ktor backend (sync, AI, auth)

# Architecture & Key Conventions

- Error handling: use Arrow `Either<DomainError, T>` for expected failures; avoid throwing for business/domain errors.
- Navigation: Decompose (`ComponentContext`) for lifecycle-aware navigation/state.
- DI: Koin modules; prefer existing module patterns over new DI approaches.
- Data isolation: every persisted entity includes `user_id`; all queries must scope to the authenticated user.

# Data / Persistence

- Desktop/Server: SurrealDB (embedded)
- Mobile: SQLite via SQLDelight (migrations under `shared/src/commonMain/sqldelight/migrations/`)

# Testing

- Tests use Kotest (BDD-friendly). Prefer adding/adjusting tests when changing logic.

# Security

- Never commit secrets (tokens/keys/passwords). Be careful with config files and logs.
- Server auth uses JWT; `JWT_SECRET` must be set (minimum 32 chars). Other common env vars: `JWT_ISSUER`, `JWT_AUDIENCE`, expiry settings.

# Git / PR Expectations

- Use conventional-style commit messages (`feat:`, `fix:`, `chore:`, etc.).
- Before opening/updating a PR, ensure validators pass (`./gradlew test` at minimum; `./gradlew check` when changing Kotlin/Gradle).
