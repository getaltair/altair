# Kotlin / Android Conventions

Applies to: `apps/android/`

## Framework

- Kotlin with Jetpack Compose for UI
- MVVM architecture: Screen -> ViewModel -> Repository -> DAO/Service
- Koin for dependency injection
- Room (SQLite) for local persistence
- Coroutines + Flow for async operations

## File Naming

- Classes: `PascalCase.kt` matching class name
- Packages: lowercase dot-separated (`com.getaltair.altair.domain.entity`)
- Composables: `PascalCase.kt` matching the composable function name
- Tests: `ClassNameTest.kt` co-located or in test source set

## Composable Functions

- Public composables must accept a `Modifier` parameter
- No heavy computation inside `@Composable` functions; move to ViewModel
- Use `remember {}` with key parameters when value depends on inputs
- `mutableStateOf` must be wrapped in `remember` to survive recomposition
- Side effects go in `LaunchedEffect`, `SideEffect`, or `DisposableEffect`
- Immutable parameters only; no mutable state objects as composable args

## Coroutines & Flow

- Structured concurrency only: no `GlobalScope.launch` or `GlobalScope.async`
- No `runBlocking` outside of main function or tests (exception: OkHttp `Authenticator` — see ADR-025)
- When catching `Exception` broadly, always add `catch (e: CancellationException) { throw e }` as the first catch clause. Bare `catch (e: Exception)` that silently swallows cancellation breaks structured concurrency. A `rethrowCancellation()` extension or the pattern `if (e is CancellationException) throw e` are also acceptable.
- Use `delay()` in coroutine context, never `Thread.sleep()`
- `Flow.collect` must be lifecycle-aware (use `repeatOnLifecycle` or `collectAsState`)
- Use `SupervisorJob` for isolated error handling in child coroutines

## Data Layer

- Repository interfaces in `domain/repository/`
- Repository implementations in `data/repository/`
- Room entities in `data/local/entity/`
- Room DAOs in `data/local/dao/`
- Domain models in `domain/entity/`
- Mappers between entity and domain layers in `data/local/mapper/`

## Kotlin Style

- Prefer `val` over `var`; flag unnecessary `var` declarations
- No `!!` (non-null assertion) except in rare justified cases
- No `lateinit var` for nullable types; use nullable instead
- Always use `kotlinx-datetime` (`Clock.System.now().toString()`) for timestamp generation. `LocalDateTime.now()` is prohibited — it strips timezone information and produces incorrect `updated_at` ordering against server `timestamptz` columns.
- Use a logging framework, never `println()` for logging
- Concrete types behind interfaces in constructors (dependency inversion)

## Error Handling

- Sealed classes or sealed interfaces for error states
- `UiState` pattern: `Loading`, `Success(data)`, `Error(message)` — all three states are required; omitting `Loading` leaves write operations with no in-flight feedback
- ViewModels that observe a single entity by ID via DAO **must** wrap the result in `UiState<T>`. `StateFlow<T?>` is insufficient because `null` conflates "Loading" and "Not Found" (e.g., a record deleted server-side and synced). Use the pattern: `daoFlow.map { UiState.Success(it) }.catch { emit(UiState.Error(it.message ?: "Unknown error")) }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), UiState.Loading)`
- Every `viewModelScope.launch` block that performs a database write or network call must include a `try/catch` that: (1) rethrows `CancellationException`, (2) logs the failure with `Log.e`, and (3) emits to a `_uiState` error state. Unhandled coroutine exceptions produce opaque crash-level logs with no UI feedback.
- Never swallow exceptions silently in catch blocks
- ViewModel state must be exposed as `StateFlow<T>` (read-only). Back all mutable state with `private val _field: MutableStateFlow` and expose `val field: StateFlow` via `_field.asStateFlow()`. Composables must never write directly to ViewModel state fields — `MutableStateFlow` must never be `public`.

## Testing

- Unit tests with JUnit 5 and Turbine for Flow testing
- Compose UI tests with `createComposeRule`
- Koin module verification via `checkModules()` in tests
