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
- No `runBlocking` outside of main function or tests
- Catch blocks must not swallow `CancellationException`
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
- Use `kotlinx-datetime` or `java.time`, never `java.util.Date`
- Use a logging framework, never `println()` for logging
- Concrete types behind interfaces in constructors (dependency inversion)

## Error Handling

- Sealed classes or sealed interfaces for error states
- `UiState` pattern: `Loading`, `Success(data)`, `Error(message)`
- Never swallow exceptions silently in catch blocks

## Testing

- Unit tests with JUnit 5 and Turbine for Flow testing
- Compose UI tests with `createComposeRule`
- Koin module verification via `checkModules()` in tests
