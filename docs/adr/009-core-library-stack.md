# ADR-009: Core Library Stack

| Field             | Value           |
| ----------------- | --------------- |
| **Status**        | Accepted        |
| **Date**          | 2026-01-09      |
| **Deciders**      | Robert Hamilton |
| **Supersedes**    | —               |
| **Superseded by** | —               |

## Context

Altair requires foundational libraries for dependency injection, navigation, error handling, and
testing that work reliably across all Kotlin Multiplatform targets (Android, iOS, Desktop). The KMP
ecosystem has matured significantly, with clear winners emerging in each category.

Key requirements:

- Full multiplatform support (Android, iOS, Desktop)
- Production-proven stability
- Active maintenance
- Compose Multiplatform integration
- Testability

## Decision

### Dependency Injection: Koin 4.x

**Choice**: Koin over kotlin-inject, Kodein, or manual DI

| Factor          | Koin              | kotlin-inject | Kodein  |
| --------------- | ----------------- | ------------- | ------- |
| Stars           | 9.8k              | ~1.5k         | 3.3k    |
| Build impact    | Fast (no codegen) | Slower (KSP)  | Fast    |
| Error detection | Runtime           | Compile-time  | Runtime |
| Compose support | Excellent         | Good          | Good    |

Rationale:

- **14 million monthly downloads** - battle-tested at scale
- Google recommended Koin when migrating Jetcaster sample to KMP
- Zero code generation overhead keeps builds fast
- `koin-compose` provides identical API across all platforms
- `koin-core-viewmodel` for ViewModel support everywhere
- ~0.25ms startup overhead is negligible for productivity app

```kotlin
val appModule = module {
    singleOf(::QuestRepository)
    singleOf(::NoteRepository)
    viewModelOf(::GuidanceViewModel)
}
```

### Navigation: Decompose 3.x

**Choice**: Decompose over Voyager, JetBrains Navigation, or Appyx

| Factor          | Decompose     | Voyager | JetBrains Nav   |
| --------------- | ------------- | ------- | --------------- |
| Open issues     | 0             | 186     | N/A             |
| Desktop support | Excellent     | Good    | Good            |
| Back handling   | Best-in-class | Good    | Android-focused |
| UI coupling     | None          | Compose | Compose         |

Rationale:

- **0 open issues** against 271 closed - exceptional maintenance
- UI-agnostic architecture allows platform-specific adaptations
- Native back gesture handling on all platforms (predictive back Android, iOS sliding, ESC desktop)
- `InstanceKeeper` provides ViewModel-equivalent instance retention
- Clean separation of navigation logic from Compose UI

```kotlin
class RootComponent(componentContext: ComponentContext) : ComponentContext by componentContext {
    private val navigation = StackNavigation<Config>()
    val stack = childStack(source = navigation, initialConfiguration = Config.Home)

    fun navigateToDetails(id: String) = navigation.push(Config.Details(id))
}
```

### Error Handling: Arrow 2.x

**Choice**: Arrow arrow-core + arrow-optics over plain Result/exceptions

| Factor                     | Arrow Either | Kotlin Result       | Exceptions |
| -------------------------- | ------------ | ------------------- | ---------- |
| Typed errors               | Yes          | No (Throwable only) | No         |
| Error accumulation         | Yes          | No                  | No         |
| CancellationException safe | Yes          | No                  | N/A        |
| Nested state updates       | arrow-optics | Manual copy()       | Manual     |

Rationale:

- **ThoughtWorks Technology Radar "Adopt"** status (Dec 2024)
- Arrow 2.x removed complexity (no HKTs, typeclasses, or Arrow IO)
- `Either<DomainError, T>` explicitly models failure in types
- `zipOrAccumulate` collects multiple validation errors (forms)
- Arrow Optics simplifies nested immutable state updates
- Production use at Salesforce, Expedia Group, Katanox

```kotlin
// Typed errors
sealed interface QuestError {
    data object WipLimitExceeded : QuestError
    data class NotFound(val id: String) : QuestError
}

suspend fun startQuest(id: String): Either<QuestError, Quest> = either {
    val quest = repository.findById(id).bind()
    ensure(activeQuests.count() < 1) { QuestError.WipLimitExceeded }
    quest.copy(status = Status.Active)
}

// Optics for nested updates
@optics data class AppState(val guidance: GuidanceState)
@optics data class GuidanceState(val activeQuest: Quest?)

AppState.guidance.activeQuest.modify(state) { it?.copy(status = Status.Completed) }
```

### Testing: Mokkery 3.x + Fakes

**Choice**: Mokkery over MockK or Mockative

**Critical**: MockK cannot and will never support iOS/Native targets due to JVM reflection
requirements.

| Factor          | Mokkery         | MockK          | Mockative |
| --------------- | --------------- | -------------- | --------- |
| iOS support     | ✅              | ❌ Never       | ✅        |
| Implementation  | Compiler plugin | JVM reflection | KSP       |
| Open issues     | 5               | N/A            | 29        |
| API familiarity | MockK-inspired  | N/A            | Different |

Rationale:

- Compiler plugin approach works on all Kotlin targets
- MockK-inspired API eases migration from Android-only projects
- 5 open issues indicates solid maintenance
- Prefer hand-written fakes for most cases; Mokkery for edge cases

```kotlin
// Prefer fakes for repositories
class FakeQuestRepository : QuestRepository {
    private val quests = mutableMapOf<String, Quest>()
    override suspend fun findById(id: String) = quests[id]?.right() ?: QuestError.NotFound(id).left()
}

// Mokkery when needed
val repository = mock<QuestRepository> {
    everySuspend { findById(any()) } returns Quest(...).right()
}
```

### Additional Libraries

| Category      | Library               | Version | Purpose                  |
| ------------- | --------------------- | ------- | ------------------------ |
| Async         | kotlinx-coroutines    | 1.9+    | Coroutines and Flow      |
| Serialization | kotlinx-serialization | 1.7+    | JSON/CBOR encoding       |
| DateTime      | kotlinx-datetime      | 0.6+    | Multiplatform dates      |
| HTTP Client   | Ktor Client           | 3.1+    | API calls (if needed)    |
| Flow Testing  | Turbine               | 1.2+    | Flow assertions          |
| Resilience    | arrow-fx-coroutines   | 2.1+    | Retry, timeout, resource |

## Consequences

### Positive

- **Consistent patterns** across all platforms
- **Compile-time safety** for errors via Arrow Either
- **Exceptional navigation** with Decompose's 0-issue maintenance
- **Fast builds** with Koin's runtime DI
- **Testable code** with platform-agnostic Mokkery

### Negative

- **Arrow learning curve** - team must understand map/flatMap/fold patterns
- **Decompose verbosity** - more boilerplate than Voyager for simple cases
- **Runtime DI errors** - Koin catches issues at runtime, not compile time

### Mitigations

- Document Arrow patterns in AGENTS.md with examples
- Create Decompose component templates for common patterns
- Use Koin's `checkModules()` in tests to catch DI issues early

## References

- [Koin Documentation](https://insert-koin.io/)
- [Decompose Documentation](https://arkivanov.github.io/Decompose/)
- [Arrow 2.0 Release](https://arrow-kt.io/community/blog/2024/12/05/arrow-2-0/)
- [Mokkery Documentation](https://mokkery.dev/)
- [KMP Awesome List](https://github.com/terrakok/kmp-awesome)
