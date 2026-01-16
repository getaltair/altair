# .claude/rules/api-rules.md

---

paths:

- "shared/src/commonMain/kotlin/api/\*_/_.kt"

---

# API Layer Rules

- All network calls must use Arrow's Either<NetworkError, T>
- Ktor client configuration lives in NetworkModule
- Never expose Ktor types outside this package
