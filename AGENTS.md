# AGENTS.md

Altair is an ADHD-focused productivity app built with Kotlin Multiplatform + Compose Multiplatform +
SurrealDB/SQLite.

## Commands

```bash
# Install dependencies (Gradle handles this automatically)
./gradlew build

# Development - Desktop
./gradlew :composeApp:run

# Development - Android (requires device/emulator)
./gradlew :composeApp:installDebug

# Development - iOS (requires macOS + Xcode)
open iosApp/iosApp.xcodeproj  # Then run from Xcode

# Run server locally
./gradlew :server:run

# Docker Compose (full stack)
docker compose up -d

# Run all tests
./gradlew test

# Run desktop tests only
./gradlew :composeApp:jvmTest

# Lint and format
./gradlew detekt
./gradlew ktlintCheck
./gradlew ktlintFormat
```

## Project Structure

```text
altair/
├── composeApp/           # Compose Multiplatform app (desktop, Android, iOS)
│   ├── src/
│   │   ├── commonMain/   # Shared UI, ViewModels, domain logic
│   │   ├── jvmMain/      # Desktop-specific (SurrealDB, window management)
│   │   ├── androidMain/  # Android-specific (SQLite, permissions)
│   │   └── iosMain/      # iOS-specific (SQLite, system integration)
├── shared/               # Pure Kotlin shared code (no Compose)
│   └── src/commonMain/   # Domain entities, validation, RPC interfaces
├── server/               # Ktor server
│   └── src/main/         # RPC implementations, AI services, sync engine
├── docs/                 # Architecture docs, PRDs, ADRs (read these for context)
└── docker-compose.yml    # Self-hosted deployment
```

## Core Libraries

| Library               | Version | Purpose                          |
| --------------------- | ------- | -------------------------------- |
| Koin                  | 4.x     | Dependency injection             |
| Decompose             | 3.x     | Navigation (UI-agnostic)         |
| Arrow                 | 2.x     | Typed errors, validation, optics |
| Mokkery               | 3.x     | Multiplatform mocking            |
| Turbine               | 1.x     | Flow testing                     |
| SQLDelight            | 2.x     | Type-safe SQLite (mobile)        |
| kotlinx-serialization | 1.7+    | JSON/CBOR encoding               |
| kotlinx-datetime      | 0.6+    | Multiplatform dates              |

## Conventions

### Dependency Injection (Koin)

```kotlin
// Define modules in di/
val guidanceModule = module {
    singleOf(::QuestRepository)
    factoryOf(::StartQuestUseCase)
    viewModelOf(::GuidanceViewModel)
}

// Inject in Composables
@Composable
fun GuidanceScreen() {
    val viewModel = koinViewModel<GuidanceViewModel>()
}

// Inject in Components (Decompose)
class GuidanceComponent(
    componentContext: ComponentContext,
    private val repository: QuestRepository = get()  // Or constructor injection
) : ComponentContext by componentContext
```

### Navigation (Decompose)

```kotlin
// Define configs as sealed class
@Serializable
sealed interface RootConfig {
    @Serializable data object Guidance : RootConfig
    @Serializable data object Knowledge : RootConfig
    @Serializable data object Tracking : RootConfig
    @Serializable data class QuestDetail(val id: String) : RootConfig
}

// Create component with navigation
class RootComponent(
    componentContext: ComponentContext
) : ComponentContext by componentContext {

    private val navigation = StackNavigation<RootConfig>()

    val stack = childStack(
        source = navigation,
        serializer = RootConfig.serializer(),
        initialConfiguration = RootConfig.Guidance,
        childFactory = ::createChild
    )

    fun navigateTo(config: RootConfig) = navigation.push(config)
    fun goBack() = navigation.pop()
}

// Render in Compose
@Composable
fun RootContent(component: RootComponent) {
    Children(component.stack) { child ->
        when (val instance = child.instance) {
            is RootChild.Guidance -> GuidanceContent(instance.component)
            is RootChild.Knowledge -> KnowledgeContent(instance.component)
            // ...
        }
    }
}
```

### Error Handling (Arrow)

```kotlin
// Define domain errors as sealed interfaces
sealed interface QuestError {
    data object WipLimitExceeded : QuestError
    data class NotFound(val id: String) : QuestError
    data class ValidationFailed(val errors: NonEmptyList<String>) : QuestError
}

// Use Either for operations that can fail
suspend fun startQuest(id: String): Either<QuestError, Quest> = either {
    val quest = repository.findById(id)
        .toEither { QuestError.NotFound(id) }
        .bind()

    ensure(activeQuestCount() < 1) { QuestError.WipLimitExceeded }

    repository.update(quest.copy(status = Status.Active)).bind()
}

// Handle in ViewModel
viewModelScope.launch {
    startQuest(id).fold(
        ifLeft = { error -> _state.update { it.copy(error = error.toMessage()) } },
        ifRight = { quest -> _state.update { it.copy(activeQuest = quest) } }
    )
}

// Accumulate validation errors
fun validateQuest(input: QuestInput): EitherNel<String, ValidQuest> = either {
    zipOrAccumulate(
        { ensure(input.title.isNotBlank()) { "Title required" } },
        { ensure(input.energyCost in 1..5) { "Energy must be 1-5" } },
        { ensure(input.title.length <= 100) { "Title too long" } }
    ) { _, _, _ -> ValidQuest(input) }
}
```

### Arrow Optics (Nested State Updates)

```kotlin
@optics data class AppState(
    val guidance: GuidanceState,
    val knowledge: KnowledgeState
) { companion object }

@optics data class GuidanceState(
    val activeQuest: Quest?,
    val quests: List<Quest>
) { companion object }

// Instead of:
// state.copy(guidance = state.guidance.copy(activeQuest = state.guidance.activeQuest?.copy(status = Done)))

// Use optics:
AppState.guidance.activeQuest.status.modify(state) { Status.Done }
```

### Testing (Mokkery + Fakes)

```kotlin
// PREFER: Hand-written fakes for repositories
class FakeQuestRepository : QuestRepository {
    private val quests = mutableMapOf<String, Quest>()
    var findByIdCalled = 0

    override suspend fun findById(id: String): Quest? {
        findByIdCalled++
        return quests[id]
    }

    fun givenQuest(quest: Quest) { quests[quest.id] = quest }
}

// USE MOKKERY: When fakes are impractical
@Test
fun `startQuest fails when wip limit exceeded`() = runTest {
    val repository = mock<QuestRepository> {
        everySuspend { findById("123") } returns someQuest
        everySuspend { countActive() } returns 1  // Already at limit
    }

    val result = StartQuestUseCase(repository).invoke("123")

    result shouldBeLeft QuestError.WipLimitExceeded
}

// Flow testing with Turbine
@Test
fun `state updates on quest completion`() = runTest {
    viewModel.state.test {
        awaitItem() shouldBe initialState

        viewModel.completeQuest("123")

        awaitItem().activeQuest shouldBe null
    }
}
```

### Database (Desktop - SurrealDB)

- Tables are singular snake_case: `quest`, `note`, `item`
- Namespace: `altair`, Database: `main`
- Use `RELATE` for graph edges between entities
- Repositories in `data/repository/surreal/`
- Return `Either<DbError, T>` from repository methods

### Database (Mobile - SQLite)

- Use SQLDelight for type-safe queries
- Schema in `composeApp/src/commonMain/sqldelight/`
- Repositories in `data/repository/sqlite/`
- Mirror SurrealDB schema structure where possible
- Return `Either<DbError, T>` from repository methods

### Server (Ktor)

- RPC services implement interfaces from `shared` module
- Routes in `routes/`, services in `services/`
- AI providers in `ai/providers/`
- Sync logic in `sync/`

### Compose (composeApp)

- Screens go in `ui/screens/`, one package per module (guidance, knowledge, tracking)
- Components go in `ui/components/`, organized by type
- Use Compose Unstyled primitives, wrap with Altair theme styling
- Design tokens in `ui/theme/AltairTheme.kt`

## Domain Vocabulary

| Term          | Meaning                                              |
| ------------- | ---------------------------------------------------- |
| Quest         | A focused task with energy cost (1-5), WIP limit = 1 |
| Epic          | A goal containing multiple Quests                    |
| Checkpoint    | Optional sub-step within a Quest                     |
| Energy Budget | Daily capacity (default 5), soft limit               |
| Note          | Markdown content with wiki-links `[[Title]]`         |
| Item          | Physical object in inventory                         |
| Location      | Fixed place (Room → Shelf)                           |
| Container     | Movable storage (Box, Toolbox)                       |

## Gotchas

- **WIP=1**: Only one Quest can be `active` at a time—reject with `WipLimitExceeded`
- **Energy is soft**: Warn but don't block if completing Quest exceeds budget
- **AI is server-side**: Desktop can fallback to local, mobile requires server
- **Embeddings lazy load**: Server loads models on first request, not startup
- **Note titles unique per folder**: Same title OK in different folders
- **Item location XOR container**: Item has location_id OR container_id, never both
- **Sync versions**: All entities track `sync_version` for conflict detection
- **Platform targets**: Desktop first, then Android, iOS is nice-to-have
- **MockK doesn't work on iOS**: Use Mokkery or fakes for multiplatform tests

## Verification

Before completing any task:

1. `./gradlew test` — all tests pass
2. `./gradlew detekt` — no static analysis issues
3. `./gradlew ktlintCheck` — code style valid
4. Desktop app runs with `./gradlew :composeApp:run`
5. Server runs with `./gradlew :server:run`

## When Stuck

- Read `docs/architecture/` for technical patterns
- Read `docs/requirements/` for acceptance criteria
- Read `docs/adr/` for decision rationale (especially ADR-009 for library patterns)
- Ask a clarifying question rather than guessing

**NEVER** commit code that fails verification checks. **NEVER** use cloud AI APIs as defaults—server
uses local models by default. **NEVER** hard delete records—use soft delete with `deleted_at`.
**NEVER** access database directly from UI layer—go through repositories. **NEVER** use MockK in
commonTest—it doesn't support iOS/Native. **NEVER** throw exceptions for expected failures—use
`Either<Error, T>`.
