# Design: Migrate to Kotest

## Context

The Altair project has 43 test files using `kotlin.test` with basic assertions. Tests lack:
- Structured organization (flat `@Test` methods)
- Comprehensive input coverage (manual test cases only)
- Clear behavior documentation (implementation-focused names)
- Deterministic coroutine testing

This design establishes testing best practices using Kotest's full capabilities.

## Goals

- Tests serve as executable specifications
- Domain invariants validated through property-based testing
- Edge cases covered systematically through data-driven tests
- Clear separation of arrangement, action, and assertion
- Deterministic and reproducible test execution

## Non-Goals

- Preserving existing test structure
- Minimizing diff size
- Supporting multiple testing frameworks

## Decisions

### Decision 1: BehaviorSpec as Primary Style

**Choice**: Use BehaviorSpec for most tests, DescribeSpec for API/component organization

**Rationale**: BehaviorSpec enforces given/when/then structure, making tests self-documenting and ensuring proper separation of concerns. This matches how we think about system behavior.

**Pattern**:
```kotlin
class QuestValidationTest : BehaviorSpec({
    given("a quest with valid data") {
        val quest = createValidQuest()

        `when`("the quest is validated") {
            val result = quest.validate()

            then("validation succeeds") {
                result.shouldBeRight()
            }
        }
    }

    given("a quest with blank title") {
        `when`("attempting to create the quest") {
            then("construction fails with IllegalArgumentException") {
                shouldThrow<IllegalArgumentException> {
                    Quest(title = "", ...)
                }.message shouldContain "title"
            }
        }
    }
})
```

**Style Selection Guide**:

| Scenario | Style | Why |
|----------|-------|-----|
| Business logic with preconditions | BehaviorSpec | given/when/then maps to business rules |
| API with multiple methods | DescribeSpec | describe per method, it per behavior |
| Pure functions with properties | FunSpec + checkAll | Mathematical invariants |
| Enumerated edge cases | Any + withData | Systematic coverage |

### Decision 2: Property-Based Testing for Domain Models

**Choice**: All domain model validation uses property-based testing

**Rationale**: Manual test cases miss edge cases. Property tests generate thousands of inputs, finding bugs humans wouldn't anticipate.

**Pattern**:
```kotlin
class UserValidationTest : BehaviorSpec({
    given("user email validation") {
        `when`("email contains @ with valid domain") {
            then("email is accepted") {
                checkAll(Arb.email()) { email ->
                    shouldNotThrow<IllegalArgumentException> {
                        User(email = email, ...)
                    }
                }
            }
        }

        `when`("email is malformed") {
            then("construction fails") {
                checkAll(Arb.string().filter { !it.contains("@") || !it.contains(".") }) { invalid ->
                    shouldThrow<IllegalArgumentException> {
                        User(email = invalid, ...)
                    }
                }
            }
        }
    }
})
```

**Custom Generators**: Create `Arb` generators for domain types in a shared `TestGenerators.kt`:
```kotlin
object TestGenerators {
    fun Arb.Companion.ulid(): Arb<Ulid> = arbitrary { Ulid.generate() }

    fun Arb.Companion.quest(
        status: QuestStatus = QuestStatus.BACKLOG
    ): Arb<Quest> = arbitrary {
        Quest(
            id = Arb.ulid().bind(),
            userId = Arb.ulid().bind(),
            title = Arb.string(1..200).bind(),
            energyCost = Arb.int(1..5).bind(),
            status = status,
            ...
        )
    }

    fun Arb.Companion.validEmail(): Arb<String> =
        Arb.stringPattern("[a-z]{3,10}@[a-z]{3,10}\\.[a-z]{2,4}")
}
```

### Decision 3: Data-Driven Testing for Edge Cases

**Choice**: Use `withData` for systematic edge case coverage

**Rationale**: When testing boundary conditions or enumerated cases, data-driven tests ensure complete coverage with minimal duplication.

**Pattern**:
```kotlin
class QuestStatusTransitionTest : BehaviorSpec({
    given("quest status transitions") {
        data class Transition(
            val from: QuestStatus,
            val to: QuestStatus,
            val allowed: Boolean
        )

        withData(
            Transition(BACKLOG, ACTIVE, true),
            Transition(BACKLOG, COMPLETED, false),
            Transition(ACTIVE, COMPLETED, true),
            Transition(ACTIVE, ABANDONED, true),
            Transition(COMPLETED, ACTIVE, false),
            Transition(ABANDONED, ACTIVE, false),
        ) { (from, to, allowed) ->
            `when`("transitioning from $from to $to") {
                val quest = createQuest(status = from)
                val result = quest.transitionTo(to)

                then("transition ${if (allowed) "succeeds" else "fails"}") {
                    if (allowed) {
                        result.shouldBeRight()
                    } else {
                        result.shouldBeLeft()
                    }
                }
            }
        }
    }
})
```

### Decision 4: Kotest Assertions Exclusively

**Choice**: Use Kotest matchers for all assertions, including Arrow `Either` support

**Rationale**: Kotest matchers provide better error messages, compose well, and have Arrow integration.

**Standard Patterns**:
```kotlin
// Equality
actual shouldBe expected
actual shouldNotBe unexpected

// Collections
list shouldHaveSize 3
list shouldContainExactly listOf(a, b, c)
list.shouldContainAll(a, b)
list.shouldBeEmpty()

// Strings
string shouldStartWith "prefix"
string shouldMatch Regex("pattern")
string.shouldBeBlank()

// Exceptions
shouldThrow<IllegalArgumentException> { code() }
    .message shouldContain "expected text"

// Arrow Either
result.shouldBeRight()
result.shouldBeRight { it.id shouldBe expectedId }
result.shouldBeLeft()
result.shouldBeLeft { error -> error shouldBe NotFound }

// Soft assertions (multiple checks, all reported)
assertSoftly {
    user.name shouldBe "expected"
    user.email shouldContain "@"
    user.status shouldBe ACTIVE
}
```

### Decision 5: Testcontainers with Kotest Extension

**Choice**: Use Kotest's `install` extension for container lifecycle

**Rationale**: Native integration with spec lifecycle, automatic cleanup, no manual `@BeforeAll`/`@AfterAll`.

**Pattern**:
```kotlin
class SurrealQuestRepositoryTest : BehaviorSpec({
    val container = install(ContainerExtension(SurrealDbTestContainer())) {
        // Configuration after start
    }

    lateinit var repository: SurrealQuestRepository

    beforeSpec {
        val client = SurrealDbClient(container.createNetworkConfig())
        client.connect()
        runMigrations(client)
        repository = SurrealQuestRepository(client, testUserId)
    }

    afterEach {
        // Clean up between tests
        repository.deleteAll()
    }

    given("an empty quest repository") {
        `when`("saving a new quest") {
            val quest = createQuest()
            val result = repository.save(quest)

            then("quest is persisted") {
                result.shouldBeRight()
                repository.findById(quest.id).shouldBeRight()
            }
        }
    }
})
```

### Decision 6: Project Configuration

**Choice**: Strict configuration with assertion mode enforcement

**Rationale**: Catch tests that don't actually assert anything, enforce timeouts, enable coroutine test scope.

**Configuration**:
```kotlin
// Each module's test source set
class ProjectConfig : AbstractProjectConfig() {
    // Fail tests without assertions
    override val assertionMode = AssertionMode.Error

    // Use test dispatcher for coroutines
    override val coroutineTestScope = true

    // Reasonable timeout
    override val timeout = 30.seconds

    // Property test configuration
    override val propertyTestIterations = 1000

    // Isolation for stateful tests
    override val isolationMode = IsolationMode.InstancePerLeaf
}
```

### Decision 7: Test Organization

**Choice**: Mirror source structure with descriptive package/class naming

**Structure**:
```
src/commonTest/kotlin/
└── com/getaltair/altair/
    ├── domain/
    │   ├── model/
    │   │   ├── UserValidationTest.kt      # BehaviorSpec
    │   │   ├── QuestValidationTest.kt     # BehaviorSpec
    │   │   └── EntitySerializationTest.kt # DescribeSpec
    │   └── types/
    │       ├── UlidPropertyTest.kt        # FunSpec + property tests
    │       └── ScheduleTest.kt            # BehaviorSpec
    ├── repository/
    │   └── QuestRepositoryTest.kt         # BehaviorSpec
    └── generators/
        └── TestGenerators.kt              # Shared Arb generators
```

**Naming Conventions**:
- Test class: `{Subject}Test.kt` or `{Subject}PropertyTest.kt`
- Given blocks: Describe the precondition state
- When blocks: Describe the action being tested
- Then blocks: Describe the expected outcome

### Decision 8: Multiplatform Considerations

**JVM (Desktop, Server)**:
- Use `kotest-runner-junit5` for IDE integration
- Full Kotest spec support

**iOS**:
- Use `kotest-framework-engine` (no JUnit dependency)
- Full Kotest spec support via native runner

**Android Unit Tests**:
- Use `kotest-runner-junit5`
- Full Kotest spec support

**Android Instrumented Tests**:
- Limited to `@Test` annotation (Android Test Runner constraint)
- Use Kotest assertions within test methods
- Cannot use Kotest spec styles

## Risks / Trade-offs

### Risk: Learning Curve

**Issue**: Team unfamiliar with BehaviorSpec, property testing.

**Mitigation**:
- Kotest skill provides comprehensive examples
- Start with simpler modules, iterate on patterns
- Document patterns in CLAUDE.md

### Risk: Property Test Flakiness

**Issue**: Random inputs may cause non-deterministic failures.

**Mitigation**:
- Log seeds on failure for reproduction
- Use `checkAll(PropTestConfig(seed = X))` when debugging
- CI captures and reports failing seeds

### Risk: Slower Test Execution

**Issue**: Property tests run 1000 iterations by default.

**Mitigation**:
- Acceptable trade-off for coverage
- Can reduce iterations for slow tests
- Parallel execution in CI

## Migration Plan

### Phase 1: Infrastructure

1. Update Gradle configurations with all Kotest dependencies
2. Create `ProjectConfig` in each module
3. Create shared `TestGenerators.kt` with domain type generators

### Phase 2: Shared Module (Priority: Domain Models)

1. Rewrite domain validation tests with property-based testing
2. Rewrite serialization tests with data-driven approach
3. Rewrite error handling tests with BehaviorSpec

### Phase 3: Server Module (Priority: Integration Tests)

1. Create Testcontainers Kotest extension
2. Rewrite repository tests with BehaviorSpec
3. Rewrite RPC tests with comprehensive scenarios

### Phase 4: ComposeApp Module

1. Rewrite navigation tests with BehaviorSpec
2. Rewrite component tests with DescribeSpec
3. Handle Android instrumented test limitations

### Phase 5: Verification

1. Run full test suite across all platforms
2. Verify property test seed logging works
3. Update documentation

## Open Questions

None - proceeding with best practices approach.

## References

- Kotest Skill: `.claude/skills/kotest/`
- BehaviorSpec: `.claude/skills/kotest/references/framework.md`
- Property Testing: `.claude/skills/kotest/references/proptest.md`
- Assertions: `.claude/skills/kotest/references/assertions.md`
- Extensions: `.claude/skills/kotest/references/extensions.md`
