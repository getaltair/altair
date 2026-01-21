---
name: kotest
description: Kotest is a flexible and comprehensive testing project for Kotlin with multiplatform support
---

# Kotest Skill

Kotest is a flexible and comprehensive testing project for Kotlin with multiplatform support. It provides multiple testing styles, over 300 matchers, property-based testing, data-driven testing, and extensive extension support.

This skill was generated from official Kotest documentation covering versions 5.8.x, 5.9.x, and 6.0.

## When to Use This Skill

This skill should be triggered when:

### Writing Tests
- Creating unit tests for Kotlin applications
- Setting up test specs using different testing styles (FunSpec, StringSpec, DescribeSpec, BehaviorSpec, etc.)
- Writing nested or hierarchical tests
- Configuring test timeouts, tags, or parallelism

### Using Assertions
- Writing assertions with Kotest matchers (`shouldBe`, `shouldContain`, `shouldThrow`, etc.)
- Testing collections with inspectors (`forAll`, `forAtLeast`, `forNone`, etc.)
- Creating custom matchers for domain-specific testing
- Adding context to assertions with `withClue` and `asClue`

### Property-Based Testing
- Testing with randomly generated inputs using `checkAll` or `forAll`
- Creating custom generators (`Arb<T>` or `Exhaustive<T>`)
- Testing invariants across thousands of input combinations
- Configuring property test seeds for reproducibility

### Data-Driven Testing
- Testing the same logic with multiple input/output combinations using `withData`
- Creating parameterized tests
- Using table-driven testing patterns

### Non-Deterministic Testing
- Testing asynchronous or concurrent code with `eventually`
- Retrying flaky operations with `retry`
- Testing time-dependent code with `TestDispatcher` or `TestClock`

### Test Configuration
- Setting up lifecycle hooks (`beforeTest`, `afterTest`, `beforeSpec`, etc.)
- Configuring project-wide settings via `AbstractProjectConfig`
- Using tags to filter test execution
- Managing test ordering and isolation modes

### Kotest 6.0 Migration
- Understanding breaking changes from 5.x to 6.0
- Migrating extension registration patterns
- Adopting new concurrency features

## Key Concepts

### Testing Styles
Kotest offers 8 different testing styles. All styles support the same features; choose based on preference:

| Style | Description |
|-------|-------------|
| **FunSpec** | Simple `test("name") {}` syntax - recommended default |
| **StringSpec** | Minimal `"test name" {}` syntax |
| **ShouldSpec** | Uses `should` keyword |
| **DescribeSpec** | Ruby/Jest-style `describe/it` blocks |
| **BehaviorSpec** | BDD-style `given/when/then` |
| **WordSpec** | Natural language `should` nesting |
| **FreeSpec** | Arbitrary nesting with `-` operator |
| **FeatureSpec** | Cucumber-style `feature/scenario` |
| **ExpectSpec** | Uses `expect` keyword |

### Generators (Property Testing)
- **Arb (Arbitrary)**: Generates random values with edge cases
- **Exhaustive**: Generates all values from a finite set (enums, small ranges)

### Isolation Modes
- **SingleInstance**: One spec instance, all tests share state (default)
- **InstancePerLeaf**: New instance for each leaf test
- **InstancePerRoot**: New instance for each root test (new in 6.0)

## Quick Reference

### Basic Test Structure

**FunSpec - Recommended starter style** (from official docs):
```kotlin
class MyTests : FunSpec({
    test("String length should return the length of the string") {
        "sammy".length shouldBe 5
        "".length shouldBe 0
    }
})
```

**StringSpec - Minimal syntax** (from official docs):
```kotlin
class MyTests : StringSpec({
    "length should return size of string" {
        "hello".length shouldBe 5
    }
    "startsWith should test for a prefix" {
        "world" should startWith("wor")
    }
})
```

**DescribeSpec - Nested structure** (from official docs):
```kotlin
class MyTests : DescribeSpec({
    describe("String.length") {
        it("should return the length of the string") {
            "sammy".length shouldBe 5
        }
    }
})
```

**BehaviorSpec - BDD style** (from official docs):
```kotlin
class MyTests : BehaviorSpec({
    given("a user") {
        `when`("logging in with valid credentials") {
            then("access should be granted") {
                // test logic
            }
        }
    }
})
```

### Assertions and Matchers

**Basic assertions** (from official docs):
```kotlin
"substring".shouldContain("str")
user.email.shouldBeLowerCase()
myImageFile.shouldHaveExtension(".jpg")
cityMap.shouldContainKey("London")
```

**Adding context with clues** (from official docs):
```kotlin
withClue("Name should be present") {
    user.name shouldNotBe null
}

data class HttpResponse(val status: Int, val body: String)
val response = HttpResponse(200, "the content")
response.asClue {
    it.status shouldBe 200
    it.body shouldBe "the content"
}
```

**Exception testing** (from official docs):
```kotlin
shouldThrow<IllegalAccessException> {
    // code that should throw
}

val exception = shouldThrow<IllegalAccessException> {
    // code that should throw
}
exception.message should startWith("Something went wrong")

// For exact type matching (won't catch subtypes)
val exception = shouldThrowExactly<FileNotFoundException> {
    // test here
}

// For catching any throwable
val exception = shouldThrowAny {
    // test here
}
```

**Collection inspectors** (from official docs):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")

xs.forAtLeast(2) {
    it.shouldHaveMinLength(7)
}

xs.forNone {
    it.shouldContain("x")
    it.shouldStartWith("bb")
}

// Other inspectors: forAll, forOne, forExactly(n), forAtMost(n)
```

### Property-Based Testing

**Basic property test** (from official docs):
```kotlin
class PropertyExample : StringSpec({
    "String size" {
        checkAll<String, String> { a, b ->
            (a + b) shouldHaveLength a.length + b.length
        }
    }
})
```

**With explicit seed for reproducibility** (from official docs):
```kotlin
class PropertyExample : FreeSpec({
    "String size" {
        forAll<String, String>(PropTestConfig(seed = 127305235)) { a, b ->
            (a + b).length == a.length + b.length
        }
    }
})
```

**Custom arbitrary generator** (from official docs):
```kotlin
val sillyArb = arbitrary { rs: RandomSource ->
    rs.random.nextInt(3..6)
}

data class Person(val name: String, val age: Int)
val personArb = arbitrary {
    val name = Arb.string(10..12).bind()
    val age = Arb.int(21, 150).bind()
    Person(name, age)
}
```

**Exhaustive generator** (from official docs):
```kotlin
val singleDigitPrimes = listOf(2, 3, 5, 7).exhaustive()

class PropertyExample : FreeSpec({
    "testing single digit primes" {
        checkAll(singleDigitPrimes) { prime ->
            isPrime(prime) shouldBe true
            isPrime(prime * prime) shouldBe false
        }
    }
})
```

**Reflective Arb for complex types (JVM only)** (from official docs):
```kotlin
enum class Currency { USD, GBP, EUR }
class CurrencyAmount(val amount: Long, val currency: Currency)

context("Currencies converts to EUR") {
    checkAll(Arb.bind<CurrencyAmount>().filter { it.currency != EUR }) { currencyAmount ->
        val converted = currencyAmount.convertTo(EUR)
        converted.currency shouldBe EUR
    }
}
```

**Assumptions for filtering input combinations** (from official docs):
```kotlin
checkAll<String, String> { a, b ->
    withAssumptions(a != b) {
        levenshtein(a, b) shouldBeGreaterThan 0
    }
}
```

### Data-Driven Testing

**Using withData** (from official docs):
```kotlin
class DataTestExample : FreeSpec({
    "maximum of two numbers" {
        withData(
            Triple(1, 5, 5),
            Triple(1, 0, 1),
            Triple(0, 0, 0)
        ) { (a, b, max) ->
            Math.max(a, b) shouldBe max
        }
    }
})
```

### Non-Deterministic Testing

**Eventually - wait for condition** (from official docs):
```kotlin
eventually(5000) { // duration in millis
    userRepository.getById(1).name shouldBe "bob"
}

eventually({
    duration = 5000
    interval = 1000.fixed()
}) {
    userRepository.getById(1).name shouldBe "bob"
}

eventually({
    duration = 8000
    retries = 10
    suppressExceptions = setOf(UserNotFoundException::class)
}) {
    userRepository.getById(1).name shouldNotBe "bob"
}
```

**Retry - attempt N times** (from official docs):
```kotlin
class MyTests : ShouldSpec() {
    init {
        should("retry up to 4 times") {
            retry(4, 10.minutes) {
                // test logic
            }
        }
    }
}
```

### Test Configuration

**Test config options** (from official docs):
```kotlin
class MySpec : DescribeSpec({
    describe("should use config").config(
        timeout = 2.seconds,
        invocations = 10,
        tags = setOf(Database, Linux)
    ) {
        // test here
    }
})
```

**Lifecycle hooks** (from official docs):
```kotlin
class TestSpec : WordSpec({
    beforeTest {
        println("Starting a test $it")
    }
    afterTest { (test, result) ->
        println("Finished spec with result $result")
    }
    "this test" should {
        "be alive" {
            println("Johnny5 is alive!")
        }
    }
})
```

**Tags for filtering** (from official docs):
```kotlin
object Linux : Tag()
object Windows : Tag()

class MyTest : StringSpec() {
    init {
        "should run on Windows".config(tags = setOf(Windows)) {
            // ...
        }
        "should run on Linux".config(tags = setOf(Linux)) {
            // ...
        }
    }
}

// Run with: gradle test -Dkotest.tags="Linux & !Database"
```

**Auto-closing resources** (from official docs):
```kotlin
class StringSpecExample : StringSpec() {
    val reader = autoClose(StringReader("xyz"))

    init {
        "your test case" {
            // use resource reader here
        }
    }
}
```

**Test coroutine dispatcher** (from official docs):
```kotlin
class TestDispatcherTest : FunSpec() {
    init {
        test("advance time").config(coroutineTestScope = true) {
            val duration = 1.days
            launch {
                delay(duration.inWholeMilliseconds)
            }
            // Move clock forward - delay completes immediately
            testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)
        }
    }
}
```

**Assertion mode** (from official docs):
```kotlin
class MySpec : FunSpec() {
    init {
        assertions = AssertionMode.Error
        test("this test has no assertions") {
            val name = "sam"
            name.length == 3 // Warning: this isn't actually testing anything
        }
    }
}
```

### Custom Matchers

**Creating a custom matcher** (from official docs):
```kotlin
fun haveLength(length: Int) = Matcher<String> {
    MatcherResult(
        value.length == length,
        { "string had length ${value.length} but we expected length $length" },
        { "string should not have length $length" },
    )
}

// Usage
"hello foo" should haveLength(9)
"hello bar" shouldNot haveLength(3)
```

**Composed matchers** (from official docs):
```kotlin
val passwordMatcher = Matcher.all(
    containADigit(),
    contain(Regex("[a-z]")),
    contain(Regex("[A-Z]"))
)

fun String.shouldBeStrongPassword() = this shouldBe passwordMatcher

// Usage
"StrongPassword123".shouldBeStrongPassword()
```

### Test Factories

**Reusable test factories** (from official docs):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {
    name should {
        "increase size as elements are added" {
            empty.size() shouldBe 0
            val plus1 = empty.add(1)
            plus1.size() shouldBe 1
        }
        "contain an element after it is added" {
            empty.contains(1) shouldBe false
            empty.add(1).contains(1) shouldBe true
        }
    }
}

class IndexedSeqTestSuite : WordSpec({
    include(indexedSeqTests("vector", Vector()))
    include(indexedSeqTests("list", List()))
})
```

### Project Configuration

**AbstractProjectConfig** (from official docs):
```kotlin
class ProjectConfig : AbstractProjectConfig() {
    override var coroutineTestScope = true
    override val specExecutionOrder = SpecExecutionOrder.Annotated

    // Kotest 6.0 style
    override val extensions = listOf(
        MyExtension(),
        AnotherExtension()
    )
}
```

## Kotest 6.0 Changes

### Key Changes from 5.x
- **Minimum JDK 11** and **Kotlin 2.2** required
- **Extension registration**: Now uses `val extensions` instead of `fun extensions()`
- **Classpath scanning removed**: Extensions must be explicitly registered
- **withData merged into core**: No separate `kotest-framework-data` dependency needed
- **New isolation mode**: `InstancePerRoot` added
- **Enhanced concurrency**: New `specConcurrencyMode` and `testConcurrencyMode` settings
- **Power Assert support**: Integration with Kotlin 2.2's Power Assert

### Extension Registration in 6.0

```kotlin
// In project config
object ProjectConfig : AbstractProjectConfig() {
    override val extensions = listOf(
        MyExtension(),
        AnotherExtension()
    )
}

// Or using annotation
@ApplyExtension(MyExtension::class)
class MySpec : FunSpec() {
    // tests here
}
```

### Deprecated Isolation Modes
`InstancePerLeaf` and `InstancePerTest` are deprecated - use `InstancePerRoot` instead.

## Reference Files

This skill includes comprehensive documentation in `references/`:

| File | Description | Confidence |
|------|-------------|------------|
| **framework.md** | Core framework documentation: testing styles, lifecycle hooks, test configuration, coroutines, ordering, tags | High |
| **assertions.md** | Assertions and matchers: core matchers, inspectors, custom matchers, JSON matchers, non-deterministic testing | High |
| **proptest.md** | Property-based testing: generators, seeds, assumptions, Arrow integration | High |
| **extensions.md** | Extensions documentation: built-in extensions, creating custom extensions | High |
| **6.0.md** | Kotest 6.0 specific features and migration guide | High |
| **5.9.x.md** | Version 5.9.x documentation and changelog | Medium |
| **5.8.x.md** | Version 5.8.x documentation and changelog | Medium |
| **next.md** | Upcoming/development version documentation | Medium |
| **other.md** | Additional documentation and miscellaneous topics | Medium |

## Working with This Skill

### For Beginners
1. Start with **framework.md** to understand testing styles
2. Choose a style (FunSpec is recommended for most cases)
3. Learn basic matchers from **assertions.md**
4. Reference the quick examples above for common patterns

### For Intermediate Users
1. Explore property-based testing in **proptest.md**
2. Learn data-driven testing with `withData`
3. Set up lifecycle hooks for setup/teardown
4. Use tags for test filtering

### For Advanced Users
1. Create custom matchers and generators
2. Use test factories for reusable test suites
3. Configure project-wide settings via `AbstractProjectConfig`
4. Explore coroutine testing with `TestDispatcher`

### For Kotest 6.0 Migration
1. Review **6.0.md** for breaking changes
2. Update extension registration patterns
3. Migrate to `InstancePerRoot` if using deprecated isolation modes
4. Take advantage of new concurrency features

## Dependencies

### Gradle (Kotlin DSL)
```kotlin
dependencies {
    // Core framework
    testImplementation("io.kotest:kotest-runner-junit5:$version")

    // Assertions (optional, but recommended)
    testImplementation("io.kotest:kotest-assertions-core:$version")

    // Property testing (optional)
    testImplementation("io.kotest:kotest-property:$version")
}

tasks.withType<Test>().configureEach {
    useJUnitPlatform()
}
```

### Maven
```xml
<dependency>
    <groupId>io.kotest</groupId>
    <artifactId>kotest-runner-junit5-jvm</artifactId>
    <version>${kotest.version}</version>
    <scope>test</scope>
</dependency>
```

## Resources

### Official Links
- Documentation: https://kotest.io/docs/
- GitHub: https://github.com/kotest/kotest
- Examples: https://github.com/kotest/kotest-examples

### references/
Organized documentation extracted from official sources containing:
- Detailed explanations and API documentation
- Code examples with language annotations
- Links to original documentation
- Version-specific information

## Notes

- This skill was generated from official Kotest documentation
- Reference files preserve structure and examples from source docs
- Code examples include proper Kotlin syntax highlighting
- Quick reference patterns are extracted from common usage examples
- When sources differ between versions, prefer 6.0 documentation for new projects

## Updating

To refresh this skill with updated documentation:
1. Re-run the scraper with the same configuration
2. The skill will be rebuilt with the latest information
