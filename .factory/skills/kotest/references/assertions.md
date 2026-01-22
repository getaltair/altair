# Kotest - Assertions

**Pages:** 282

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the ideal scenario for writing tests, but for the times when it is required, Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to asset that no elements in a collection passed the assertions, we could do something like:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Retry | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/retry.html

**Contents:**
- Retry

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> {  return MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## Retry | Kotest

**URL:** https://kotest.io/docs/next/assertions/retry.html

**Contents:**
- Retry

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## Composed Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/composed-matchers.html

**Contents:**
- Composed Matchers

Composed matchers can be created for any type by composing one or more matchers. This allows to build up complex matchers from simpler ones. There are two logical operations, using which we can compose matchers: logical sum (Matcher.any) and logical product (Matcher.all).

Let's say we'd like to define a password Matcher, which will containADigit(), contain(Regex("[a-z]")) and contain(Regex("[A-Z]")). We can compose these matchers this way:

We can add extension function then:

So it can be invoked like this:

By analogy, we can build a composed matcher using Matcher.any. In this case, passwordMatcher will fail only if all matchers fail, otherwise it will pass.

Composed matchers can also be created for any class or interface by composing one or more other matchers along with the property to extract to test against.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we can add the extension variant too:

Then we invoke it this way:

**Examples:**

Example 1 (kotlin):
```kotlin
val passwordMatcher = Matcher.all(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

Example 2 (kotlin):
```kotlin
fun String.shouldBeStrongPassword() = this shouldBe passwordMatcher
```

Example 3 (kotlin):
```kotlin
"StrongPassword123".shouldBeStrongPassword()"WeakPassword".shouldBeStrongPassword() // would fail
```

Example 4 (kotlin):
```kotlin
val passwordMatcher = Matcher.any(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to asset that no elements in a collection passed the assertions, we could do something like:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

---

## JSON Schema Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/json/json-schema-matchers.html

**Contents:**
- JSON Schema Matchers
- Parsing Schema​
- Building Schema​
  - Array​
    - Length (minItems and maxItems)​
    - Uniqueness​
- Validating​
- Limitations​

A subset of JSON Schemas can be defined either by parsing a textual schema. Example:

or using Kotest's built-in DSL:

Arrays are used for ordered elements. In JSON, each element in an array may be of a different type.

The length of the array can be specified using the minItems and maxItems keywords. The value of each keyword must be a non-negative number and defaults are 0 and Int.MAX_VALUE

A schema can ensure that each of the items in an array is unique. Simply set the uniqueItems keyword to true.

Once a schema has been defined, you can validate String and kotlinx.serialization.JsonElement against it:

⚠️ Note that Kotest only supports a subset of JSON schema currently. Currently, missing support for:

**Examples:**

Example 1 (kotlin):
```kotlin
val parsedSchema = parseSchema(  """  {  "$id": "https://example.com/geographical-location.schema.json",  // will be ignored  "$schema": "https://json-schema.org/draft/2020-12/schema",       // will be ignored  "title": "Longitude and Latitude Values",                        // will be ignored  "description": "A geographical coordinate.",                     // will be ignored  "required": [ "latitude", "longitude" ],  "type": "object",  "properties": {    "latitude": {      "type": "number",      "minimum": -90,      "maximum": 90    },    "longitude": {      "type": "number",      "minimum": -180,      "maximum": 180    }  }}  """)
```

Example 2 (kotlin):
```kotlin
val addressSchema = jsonSchema {  obj {   // object is reserved, obj was chosen over jsonObject for brevity but could be changed ofc, or jsonObject could be added as alternative.    withProperty("street", required = true) { string() }    withProperty("zipCode", required = true) {      integer {        beEven() and beInRange(10000..99999)   // supports constructing a matcher that will be used to test values      }    }    additionalProperties = false   // triggers failure if other properties are defined in actual  }}val personSchema = jsonSchema {  obj {    withProperty("name", required = true) { string() }    withProperty("address") { addressSchema() } // Schemas can re-use other schemas 🎉  }}
```

Example 3 (kotlin):
```kotlin
val lengthBoundedSchema = jsonSchema {  array(minItems = 0, maxItems = 1) { number() }}
```

Example 4 (kotlin):
```kotlin
val uniqueArray = jsonSchema {  array(uniqueItems = true) { number() }}
```

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to asset that no elements in a collection passed the assertions, we could do something like:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

---

## Retry | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/retry.html

**Contents:**
- Retry

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-assertions-compiler:$version")
```

Example 2 (kotlin):
```kotlin
class CompilationTest : FreeSpec({    "shouldCompile test" {        val rawStringCodeSnippet = """            val aString: String = "A valid assignment"        """        val syntaxHighlightedSnippet = codeSnippet("""            val aString: String = "A valid assignment"        """)        rawStringCodeSnippet.shouldCompile()        syntaxHighlightedSnippet.shouldCompile()        File("SourceFile.kt").shouldCompile()    }    "shouldNotCompile test" {        val rawStringCodeSnippet = """            val anInteger: Int = "An invalid assignment"        """        val syntaxHighlightedSnippet = codeSnippet("""            val anInteger: Int = "An invalid assignment"        """)        rawStringCodeSnippet.shouldNotCompile()        syntaxHighlightedSnippet.shouldNotCompile()        File("SourceFile.kt").shouldNotCompile()        // check that a compilation error occurred for a specific reason        rawStringCodeSnippet.shouldNotCompile("expected 'Int', actual 'String'")        syntaxHighlightedSnippet.shouldNotCompile("expected 'Int', actual 'String'")        File("SourceFile.kt").shouldNotCompile("expected 'Int', actual 'String'")    }    @OptIn(ExperimentalCompilerApi::class)    "custom assertions on JvmCompilationResult" {        val codeSnippet = codeSnippet("""            fun foo() {                printDate(LocalDate.now())            }        """)        codeSnippet.compile {            exitCode shouldBe ExitCode.COMPILATION_ERROR            messages shouldContain "Unresolved reference 'LocalDate'"            messages shouldContain "Unresolved reference 'printDate'"        }    }    @OptIn(ExperimentalCompilerApi::class)    "custom compiler configuration" {        val compileConfig = CompileConfig {            compilerPluginRegistrars = listOf(MyCompilerPluginRegistrar())        }        val codeSnippet = compileConfig.codeSnippet("""            @MyAnnotation            fun hello() {}        """)        codeSnippet.shouldCompile()    }})
```

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

---

## Until | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> { value ->  MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/matchers

**Contents:**
- matchers
  - Kotest Matcher Modules​
  - Kotest External Matcher Modules​
  - Community Provided Matchers​

For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

These modules are hosted in the kotest organization but in separate repositories from the main kotest project. They are released on an independent cadence from the Kotest framework. They provide matchers for third party libraries.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

---

## Range Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/range-matchers.html

**Contents:**
- Range Matchers

This page describes the rich assertions (matchers) that are available for ClosedRange and OpenEndRange types.

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to asset that no elements in a collection passed the assertions, we could do something like:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

To use this library you need to add io.kotest.extensions:kotest-assertions-arrow to your build.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

The following module is needed: io.kotest.extensions:kotest-assertions-arrow which is versioned independently of the main Kotest project. Search maven central for latest version here.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Retry | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/retry.html

**Contents:**
- Retry

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## Until | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## Collection Matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/collection-matchers.html

**Contents:**
- Collection Matchers

This page describes the rich assertions (matchers) that are available for Collection, Iterable and Array types.

Also, see inspectors which are useful ways to test multiple elements in a collection.

---

## Android Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/android-matchers.html

**Contents:**
- Android Matchers

This page lists all current Android matchers in Kotest. These are additional to the default matchers and are specific to Android.

To use them, it's required to add an extra dependency to your project:

**Examples:**

Example 1 (kotlin):
```kotlin
implementation("io.kotest:kotest-assertions-android:VERSION")
```

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## Clues | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/clues.html

**Contents:**
- Clues

Clues only work if you are using the Kotest assertions library or Kotest test framework

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

And you would be able to see that you were populating the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self-explanatory:

For example, we can use withClue with a string message

Would give an error like this:

We can also use the asClue extension function to turn any object into the clue message.

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/eventually.html

**Contents:**
- Eventually
- API​
- Configuration Options​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 5.7, the non-deterministic testing functions have moved to the kotest-assertions-core module, and are available under the new package io.kotest.assertions.nondeterministic. The previous iterations of these functions are still available, but deprecated.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-determistic operation. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually function which solves the common use case of "I expect this code to pass after a short period of time".

Eventually works by periodically invoking a given lambda, ignoring specified exceptions, until the lambda passes, or a timeout is reached, or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized with regards to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration, using the Kotlin Duration type, followed by the code that should eventually pass without an exception being raised.

The second is by providing a config block. This method should be used when you need to set more options than just the duration. It also allows the config to be shared between multiple invocations of eventually.

The duration is the total amount of time to keep trying to pass the test. The interval allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Alternatively, rather than specifying the interval as a fixed number, we can pass in a function. This allows us to perform some kind of backoff, or anything else we need.

For example, to use a fibonacci increasing interval, starting with 100ms:

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test. We call these exceptions, the expected exceptions.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be ignored, or false if the exception should bubble out.

We can attach a listener, which will be invoked on each iteration, with the current iteration count and the exception that caused the iteration to fail. Note: The listener will not be fired on a successful invocation.

Sharing the configuration for eventually is a breeze with the eventuallyConfig builder. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5.seconds) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 1.seconds  interval = 100.milliseconds}eventually(config) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  interval = 250.milliseconds}
```

Example 4 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  intervalFn = 100.milliseconds.fibonacci()}
```

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid user1
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(bob) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/matchers.html

**Contents:**
- Matchers
  - Kotest Matcher Modules​
  - Community Provided Matchers​

A Matcher is the Kotest term for an assertion that performs a specific test. For example, a matcher may test that a double is greater than zero. Or it it may test that a file is not empty.

Kotest currently has approximately 400 matchers split across several modules. Most of these matchers are for standard library types. Others are project specific. Additionally, there are matchers provided by third party libraries.

Kotest matchers are framework agnostic. You can use them with the Kotest framework, or with any other framework, such as JUnit.

Matchers can be used in two styles:

Both styles are supported. The advantage of the extension function style is that the IDE can autocomplete for you, but some people may prefer the infix style as it is slightly cleaner.

Matchers can be negated by using shouldNot instead of should for the infix style. For example, a shouldNot startWith("boo"). For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

To add your library to this list, please open a PR.

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (bash):
```bash
testImplementation("io.kotest.extensions:kotest-assertions-compiler:${version}")
```

Example 2 (kotlin):
```kotlin
class CompilationTest: StringSpec() {        init {            "shouldCompile test" {                val codeSnippet = """ val aString: String = "A valid assignment" """.trimMargin()                codeSnippet.shouldCompile()                File("SourceFile.kt").shouldCompile()            }            "shouldNotCompile test" {                val codeSnippet = """ val aInteger: Int = "A invalid assignment" """.trimMargin()                codeSnippet.shouldNotCompile()                File("SourceFile.kt").shouldNotCompile()            }        }    }
```

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> { value ->  MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## Android Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/android-matchers.html

**Contents:**
- Android Matchers

This page lists all current Android matchers in Kotest. These are additional to the default matchers and are specific to Android.

To use them, it's required to add an extra dependency to your project:

**Examples:**

Example 1 (kotlin):
```kotlin
implementation("io.kotest:kotest-assertions-android:VERSION")
```

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

If you need to assert that no exception is thrown, then use shouldNotThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## JSON | Kotest

**URL:** https://kotest.io/docs/next/assertions/json/json-overview.html

**Contents:**
- JSON
- Basic matchers​
- Content-based matching​
- Schema validation​

To use these matchers add testImplementation("io.kotest:kotest-assertions-json:<version>") to your build.

There exist copies of all matchers that validate a File or a Path instead of a String for the JVM platform.

For more details, see here or follow matcher-specific links below

---

## Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/matchers.html

**Contents:**
- Matchers
  - Kotest Matcher Modules​
  - Kotest External Matcher Modules​
  - Community Provided Matchers​

A Matcher is the Kotest term for an assertion that performs a specific test. For example, a matcher may test that a double is greater than zero. Or it it may test that a file is not empty.

Kotest currently has approximately 325 matchers split across several modules. Most of these matchers are for standard library types. Others are project specific. Additionally, there are matchers provided by third party libraries.

Kotest matchers are framework agnostic. You can use them with the Kotest framework, or with any other framework. If you are happy with JUnit, you can still use the powerful matchers provided by the kotest assertion modules.

Matchers can be used in two styles:

Both styles are supported. The advantage of the extension function style is that the IDE can autocomplete for you, but some people may prefer the infix style as it is slightly cleaner.

Matchers can be negated by using shouldNot instead of should for the infix style. For example, a shouldNot startWith("boo"). For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

These modules are hosted in the kotest organization but in separate repositories from the main kotest project. They are released on an independent cadence from the Kotest framework. They provide matchers for third party libraries.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Continually | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/continually.html

**Contents:**
- Continually

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

The function passed to the continually block is executed every 10 milliseconds. We can specify the poll interval if we prefer:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds, 5.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the ideal scenario for writing tests, but for the times when it is required, Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Matching JSON content | Kotest

**URL:** https://kotest.io/docs/assertions/json/content-json-matchers.html

**Contents:**
- Matching JSON content
- shouldEqualJson​
  - compareJsonOptions​
    - Usage:​
    - Parameters​
- shouldEqualSpecifiedJson​
- shouldEqualSpecifiedJsonIgnoringOrder​
- shouldBeEmptyJsonArray​
- shouldBeEmptyJsonObject​
- shouldBeJsonArray​

This module is available for all targets.

json.shouldEqualJson(other) asserts that the left-hand side represents the same JSON structure as the right-hand side.

The matcher allows for different formatting, and for different order of keys.

For example, the following two JSON strings would be considered equal:

The inverse of this matcher is shouldNotEqualJson which will error if two JSON strings are considered equal.

shouldEqualJson supports an additional parameter of type CompareJsonOptions which supports the following flags to toggle behaviour of the JSON comparison:

Options can be specified inline, like:

Another option is to define a compare function which suits your desires, like:

Targets: Multiplatform

Alias for shouldEqualJson, with default options except FieldComparison which is set to FieldComparison.Lenient instead.

The inverse of this matcher is shouldNotEqualSpecifiedJson which will error if two JSON strings are considered equal.

Targets: Multiplatform

Alias for shouldEqualJson, with default options except

Targets: Multiplatform

json.shouldBeEmptyJsonArray() asserts that the JSON is an empty array ([]).

Targets: Multiplatform

json.shouldBeEmptyJsonObject() asserts that the JSON is an empty array ({}).

Targets: Multiplatform

json.shouldBeJsonArray() asserts that the JSON is an array.

The inverse of this matcher is shouldNotBeJsonArray which will error if the JSON string is an array.

Targets: Multiplatform

json.shouldBeJsonObject() asserts that the JSON is an object.

The inverse of this matcher is shouldNotBeJsonObject which will error if the JSON string is an object.

Targets: Multiplatform

json.shouldBeValidJson() asserts that the string is valid JSON.

The inverse of this matcher is shouldNotBeValidJson which will error if the string is valid JSON.

Targets: Multiplatform

json.shouldContainJsonKey("$.json.path") asserts that a JSON string contains the given JSON path.

The inverse of this matcher is shouldNotContainJsonKey which will error if a JSON string does contain the given JSON path.

str.shouldContainJsonKeyValue("$.json.path", value) asserts that a JSON string contains a JSON path with a specific value.

The inverse of this matcher is shouldNotContainJsonKeyValue which will error if a JSON string does contain the given value at the given JSON path.

json.shouldMatchJsonResource("/file.json") asserts that the JSON is equal to the existing test resource /file.json, ignoring properties' order and formatting.

**Examples:**

Example 1 (json):
```json
{  "name":     "sam",  "location": "chicago",  "age":      41}
```

Example 2 (json):
```json
{ "age": 41, "name": "sam", "location": "chicago" }
```

Example 3 (kotlin):
```kotlin
a.shouldEqualJson(b, compareJsonOptions { arrayOrder = ArrayOrder.Strict })
```

Example 4 (kotlin):
```kotlin
val myOptions = compareJsonOptions {  typeCoercion = TypeCoercion.Enabled  arrayOrder = ArrayOrder.Lenient}infix fun String.lenientShouldEqualJson(other: String) = this.shouldEqualJson(other, myOptions)"[1, 2]" lenientShouldEqualJson "[2, 1]" // This will pass
```

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid user1
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(a) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid user1
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(a) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> { value ->  MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## YAML | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/yaml-matchers.html

**Contents:**
- YAML
- Basic matchers​
- Content-based matching​

To use these matchers add testImplementation("io.kotest:kotest-assertions-yaml:<version>") to your build.

---

## Retry | Kotest

**URL:** https://kotest.io/docs/assertions/retry.html

**Contents:**
- Retry

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## Power Assert | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/power-assert

**Contents:**
- Power Assert
- How It Works​
- Setup​

Power Assert support was introduced in Kotest 6.0 that enhances assertion failure messages by providing detailed information about the values of each part of an expression when an assertion fails. This makes it easier to understand why an assertion failed without having to add additional debug statements.

When an assertion fails, Power Assert shows the values of each part of the expression in the error message, making it clear what went wrong. This is particularly useful for complex expressions with method calls or property access chains.

For example, consider this assertion:

Without Power Assert, the error message would simply be:

With Power Assert enabled, the error message becomes much more informative:

This detailed output shows the values of each part of the expression, making it immediately clear what's happening:

Power Assert is implemented as a Kotlin compiler plugin that's part of Kotlin 2.0+. To use it with Kotest 6.0:

**Examples:**

Example 1 (kotlin):
```kotlin
val hello = "Hello"val world = "world!"hello.substring(1, 3) shouldBe world.substring(1, 4)
```

Example 2 (yaml):
```yaml
expected:<"orl"> but was:<"el">
```

Example 3 (unknown):
```unknown
hello.substring(1, 3) shouldBe world.substring(1, 4)|     |                        |     ||     |                        |     orl|     |                        world!|     elHelloexpected:<"orl"> but was:<"el">
```

Example 4 (kotlin):
```kotlin
plugins {  kotlin("jvm") version "2.2.0"  id("org.jetbrains.kotlin.plugin.power-assert") version "2.2.0"}
```

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Continually | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/continually.html

**Contents:**
- Continually

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

The function passed to the continually block is executed every 10 milliseconds. We can specify the poll interval if we prefer:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds, 5.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> {  return MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## Matching By Field | Kotest

**URL:** https://kotest.io/docs/assertions/field-matching.html

**Contents:**
- Matching By Field
  - matchBigDecimalsIgnoringScale​
  - matchDoublesWithTolerance​
  - matchInstantsWithTolerance​
  - matchListsIgnoringOrder​
  - matchLocalDateTimesWithTolerance​
  - matchOffsetDateTimesWithTolerance​
  - matchStringsIgnoringCase​
  - matchZonedDateTimesWithTolerance​
- Building Your Own Override Matcher​

Whenever we want to match only some of the fields, excluding some other fields from comparison, we should use shouldBeEqualUsingFields:

Likewise, we can explicitly say which fields to match on, and all other fields will be excluded:

For nested classes, comparison goes recursively, as follows:

But we can explicitly stop recursive comparison. In the following example, we are comparing instances of Doctor class as a whole, not comparing their individual fields. So the difference in mainHospital.mainDoctor is detected, as opposed to detected differences in mainHospital.mainDoctor.name in the previous example:

Also we can provide custom matchers for fields. In the following example we are matching SimpleDataClass::name as case-insensitive strings:

Kotest provides the following override matchers:

Implement Assertable interface:

For instance, here is the implementation of matchListsIgnoringOrder:

We can use any of Kotest's should*** assertions.

**Examples:**

Example 1 (kotlin):
```kotlin
val expected = Thing(name = "apple", createdAt = Instant.now())   val actual = Thing(name = "apple", createdAt = Instant.now().plusMillis(42L))   actual shouldBeEqualUsingFields {      excludedProperties = setOf(Thing::createdAt)      expected   }
```

Example 2 (kotlin):
```kotlin
val expected = Thing(name = "apple", createdAt = Instant.now())   val actual = Thing(name = "apple", createdAt = Instant.now().plusMillis(42L))   actual shouldBeEqualUsingFields {      includedProperties = setOf(Thing::name)      expected   }
```

Example 3 (kotlin):
```kotlin
val doctor1 = Doctor("billy", 23, emptyList())         val doctor2 = Doctor("barry", 23, emptyList())         val city = City("test1", Hospital("test-hospital1", doctor1))         val city2 = City("test2", Hospital("test-hospital2", doctor2))         shouldThrowAny {            city.shouldBeEqualUsingFields {               city2            }         }.message shouldContain """Using fields: - mainHospital.mainDoctor.age - mainHospital.mainDoctor.name - mainHospital.name - nameFields that differ: - mainHospital.mainDoctor.name  =>  expected:<"barry"> but was:<"billy"> - mainHospital.name  =>  expected:<"test-hospital2"> but was:<"test-hospital1"> - name  =>  expected:<"test2"> but was:<"test1">"""
```

Example 4 (kotlin):
```kotlin
val doctor1 = Doctor("billy", 22, emptyList())         val doctor2 = Doctor("billy", 22, emptyList())         val city = City("test", Hospital("test-hospital", doctor1))         val city2 = City("test", Hospital("test-hospital", doctor2))         shouldFail {            city.shouldBeEqualUsingFields {               useDefaultShouldBeForFields = listOf(Doctor::class)               city2            }         }.message shouldContain """Using fields: - mainHospital.mainDoctor - mainHospital.name - nameFields that differ: - mainHospital.mainDoctor  =>
```

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## Until | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid user1
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(a) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid user1
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(a) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to assert that no elements in a collection passed the assertions, we could do something like:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

---

## Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/matchers.html

**Contents:**
- Matchers
  - Kotest Matcher Modules​
  - Kotest External Matcher Modules​
  - Community Provided Matchers​

A Matcher is the Kotest term for an assertion that performs a specific test. For example, a matcher may test that a double is greater than zero. Or it it may test that a file is not empty.

Kotest currently has approximately 325 matchers split across several modules. Most of these matchers are for standard library types. Others are project specific. Additionally, there are matchers provided by third party libraries.

Kotest matchers are framework agnostic. You can use them with the Kotest framework, or with any other framework. If you are happy with JUnit, you can still use the powerful matchers provided by the kotest assertion modules.

Matchers can be used in two styles:

Both styles are supported. The advantage of the extension function style is that the IDE can autocomplete for you, but some people may prefer the infix style as it is slightly cleaner.

Matchers can be negated by using shouldNot instead of should for the infix style. For example, a shouldNot startWith("boo"). For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

These modules are hosted in the kotest organization but in separate repositories from the main kotest project. They are released on an independent cadence from the Kotest framework. They provide matchers for third party libraries.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/eventually.html

**Contents:**
- Eventually
- API​
- Configuration Options​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 5.7, the non-deterministic testing functions have moved to the kotest-assertions-core module, and are available under the new package io.kotest.assertions.nondeterministic. The previous iterations of these functions are still available, but deprecated.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-determistic operation. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually function which solves the common use case of "I expect this code to pass after a short period of time".

Eventually works by periodically invoking a given lambda, ignoring specified exceptions, until the lambda passes, or a timeout is reached, or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized with regards to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration, using the Kotlin Duration type, followed by the code that should eventually pass without an exception being raised.

The second is by providing a config block. This method should be used when you need to set more options than just the duration. It also allows the config to be shared between multiple invocations of eventually.

The duration is the total amount of time to keep trying to pass the test. The interval allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Alternatively, rather than specifying the interval as a fixed number, we can pass in a function. This allows us to perform some kind of backoff, or anything else we need.

For example, to use a fibonacci increasing interval, starting with 100ms:

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test. We call these exceptions, the expected exceptions.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be ignored, or false if the exception should bubble out.

We can attach a listener, which will be invoked on each iteration, with the current iteration count and the exception that caused the iteration to fail. Note: The listener will not be fired on a successful invocation.

Sharing the configuration for eventually is a breeze with the eventuallyConfig builder. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5.seconds) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 1.seconds  interval = 100.milliseconds}eventually(config) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  interval = 250.milliseconds}
```

Example 4 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  intervalFn = 100.milliseconds.fibonacci()}
```

---

## Until | Kotest

**URL:** https://kotest.io/docs/next/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## Should Be | Kotest

**URL:** https://kotest.io/docs/next/assertions/shouldbe.html

**Contents:**
- Should Be
- Custom Eq Instances​

The main matcher or assertion in Kotest is the shouldBe matcher. This matcher is used to assert equality between an an actual and an expected value. The syntax is in the format actual shouldBe expected For example:

When two values do not compare equal, Kotest will print out a nice error message including an intellij <click to see difference> between the two values. For example:

Note, you can check two values are not equal using shouldNotBe.

The shouldBe matcher can be combined with power assert for greater effect.

Behind the scenes, Kotest uses the equals method but also adds extra logic to determine equality for some types where simple object equality isn't quite appropriate. For example, on the JVM, it is well known that Arrays with the same contents will not be considered equal when using the equals method. Another example is primitives of different types even with the same value.

This logic is encapsulated in the Eq typeclass which Kotest uses internally. It is also possible to define your own equality logic for types by implementing Eq and registering it with Kotest.

Let's show an example of creating a custom Eq instance for comparing Foo objects. Firstly, the definition of Foo.

Then we implement the Eq typeclass for whatever equality logic we want, returning an EqResult which is either Success or Failure.

Here are are saying that if one Foo contains the string hello and the other contains the string world then they are equal. To return a failure message we can use the AssertionErrorBuilder which is a helper to build the appropriate concrete AssertionError for whichever platform we are running on.

If we specify the expected and actual values to the error builder the <click to see difference> link will be automatically generated too.

Then we register it with Kotest, specifying the type that we want to use it for. Here we are using project config to set it up before any tests are run. We could do this at the spec level too, but bear in mind if you are running tests in parallel then the registration will be non-deterministic.

Finally, we can use our custom Eq instance in our tests by simply using shouldBe or shouldNotBe as normal.

Custom Eq instances are only selected if both sides of the call are the type specified when registered. Also the type must be exact, subclasses are not selected automatically and must also be registered

**Examples:**

Example 1 (kotlin):
```kotlin
val a = "samuel"val b = a.take(3)b shouldBe "sam"
```

Example 2 (jsx):
```jsx
Expected :worldActual   :hello<Click to see difference>
```

Example 3 (kotlin):
```kotlin
val a = "samuel"val b = a.take(3)b shouldNotBe "bob"
```

Example 4 (kotlin):
```kotlin
data class Foo(val value: String)
```

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (bash):
```bash
testImplementation("io.kotest.extensions:kotest-assertions-compiler:${version}")
```

Example 2 (kotlin):
```kotlin
class CompilationTest: StringSpec() {        init {            "shouldCompile test" {                val codeSnippet = """ val aString: String = "A valid assignment" """.trimMargin()                codeSnippet.shouldCompile()                File("SourceFile.kt").shouldCompile()            }            "shouldNotCompile test" {                val codeSnippet = """ val aInteger: Int = "A invalid assignment" """.trimMargin()                codeSnippet.shouldNotCompile()                File("SourceFile.kt").shouldNotCompile()            }        }    }
```

---

## Continually | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/continually.html

**Contents:**
- Continually

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

The function passed to the continually block is executed every 10 milliseconds. We can specify the poll interval if we prefer:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds, 5.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/next/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the ideal scenario for writing tests, but for the times when it is required, Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Retry | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/retry.html

**Contents:**
- Retry
- Retry ​

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## YAML | Kotest

**URL:** https://kotest.io/docs/next/assertions/yaml-matchers.html

**Contents:**
- YAML
- Basic matchers​
- Content-based matching​

To use these matchers add testImplementation("io.kotest:kotest-assertions-yaml:<version>") to your build.

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid user1
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(a) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

The following module is needed: io.kotest.extensions:kotest-assertions-arrow which is versioned independently of the main Kotest project. Search maven central for latest version here.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/next/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

The following module is needed: io.kotest.extensions:kotest-assertions-arrow which is versioned independently of the main Kotest project. Search maven central for latest version here.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/next/assertions/eventually.html

**Contents:**
- Eventually
- API​
- Configuration Options​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 5.7, the non-deterministic testing functions have moved to the kotest-assertions-core module, and are available under the new package io.kotest.assertions.nondeterministic. The previous iterations of these functions are still available, but deprecated.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-deterministic operation - kotest's parallelRunner implementation shows how to do that. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually function which solves the common use case of "I expect this code to pass after a short period of time".

Eventually works by periodically invoking a given lambda, ignoring specified exceptions, until the lambda passes, or a timeout is reached, or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized with regards to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration, using the Kotlin Duration type, followed by the code that should eventually pass without an exception being raised.

The second is by providing a config block. This method should be used when you need to set more options than just the duration. It also allows the config to be shared between multiple invocations of eventually.

The duration is the total amount of time to keep trying to pass the test. The interval allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Alternatively, rather than specifying the interval as a fixed number, we can pass in a function. This allows us to perform some kind of backoff, or anything else we need.

For example, to use a fibonacci increasing interval, starting with 100ms:

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test. We call these exceptions, the expected exceptions.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be ignored, or false if the exception should bubble out. If expectedExceptions is specified and the set is not empty, this function will be ignored.

We can attach a listener, which will be invoked on each iteration, with the current iteration count and the exception that caused the iteration to fail. Note: The listener will not be fired on a successful invocation.

Sharing the configuration for eventually is a breeze with the eventuallyConfig builder. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5.seconds) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 1.seconds  interval = 100.milliseconds}eventually(config) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  interval = 250.milliseconds}
```

Example 4 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  intervalFn = 100.milliseconds.fibonacci()}
```

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

Collections: also see inspectors which are useful ways to test multiple elements in a collection.

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid user1
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(a) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

The following module is needed: io.kotest.extensions:kotest-assertions-arrow which is versioned independently of the main Kotest project. Search maven central for latest version here.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> {  return MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid user1
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(a) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/matchers

**Contents:**
- matchers
  - Kotest Matcher Modules​
  - Kotest External Matcher Modules​
  - Community Provided Matchers​

For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

These modules are hosted in the kotest organization but in separate repositories from the main kotest project. They are released on an independent cadence from the Kotest framework. They provide matchers for third party libraries.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

To use this library you need to add io.kotest.extensions:kotest-assertions-arrow to your build.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## Collection Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/collection-matchers.html

**Contents:**
- Collection Matchers

This page describes the rich assertions (matchers) that are available for Collection, Iterable and Array types.

Also, see inspectors which are useful ways to test multiple elements in a collection.

---

## Composed Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/composed-matchers.html

**Contents:**
- Composed Matchers

Composed matchers can be created for any class or interface by composing one or more other matchers along with the property to extract to test against. This allows us to build up complicated matchers from simpler ones.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we could add the extension variant too:

Then we invoke like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class Person(  val name: String,  val age: Int,  val address: Address,)data class Address(  val city: String,  val street: String,  val buildingNumber: String,)
```

Example 2 (kotlin):
```kotlin
fun nameMatcher(name: String) = Matcher<String> {  MatcherResult(    value == name,    { "Name $value should be $name" },    { "Name $value should not be $name" }  )}fun ageMatcher(age: Int) = Matcher<Int> {  MatcherResult(    value == age,    { "Age $value should be $age" },    { "Age $value should not be $age" }  )}val addressMatcher = Matcher<Address> {  MatcherResult(    value == Address("Warsaw", "Test", "1/1"),    { "Address $value should be Test 1/1 Warsaw" },    { "Address $value should not be Test 1/1 Warsaw" }  )}
```

Example 3 (kotlin):
```kotlin
fun personMatcher(name: String, age: Int) = Matcher.compose(  nameMatcher(name) to Person::name,  ageMatcher(age) to Person::age,  addressMatcher to Person::address)
```

Example 4 (kotlin):
```kotlin
fun Person.shouldBePerson(name: String, age: Int) = this shouldBe personMatcher(name, age)
```

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the ideal scenario for writing tests, but for the times when it is required, Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/assertions/eventually.html

**Contents:**
- Eventually
- API​
- Configuration Options​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 5.7, the non-deterministic testing functions have moved to the kotest-assertions-core module, and are available under the new package io.kotest.assertions.nondeterministic. The previous iterations of these functions are still available, but deprecated.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-deterministic operation - kotest's parallelRunner implementation shows how to do that. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually function which solves the common use case of "I expect this code to pass after a short period of time".

Eventually works by periodically invoking a given lambda, ignoring specified exceptions, until the lambda passes, or a timeout is reached, or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized with regards to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration, using the Kotlin Duration type, followed by the code that should eventually pass without an exception being raised.

The second is by providing a config block. This method should be used when you need to set more options than just the duration. It also allows the config to be shared between multiple invocations of eventually.

The duration is the total amount of time to keep trying to pass the test. The interval allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Alternatively, rather than specifying the interval as a fixed number, we can pass in a function. This allows us to perform some kind of backoff, or anything else we need.

For example, to use a fibonacci increasing interval, starting with 100ms:

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test. We call these exceptions, the expected exceptions.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be ignored, or false if the exception should bubble out. If expectedExceptions is specified and the set is not empty, this function will be ignored.

We can attach a listener, which will be invoked on each iteration, with the current iteration count and the exception that caused the iteration to fail. Note: The listener will not be fired on a successful invocation.

Sharing the configuration for eventually is a breeze with the eventuallyConfig builder. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5.seconds) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 1.seconds  interval = 100.milliseconds}eventually(config) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  interval = 250.milliseconds}
```

Example 4 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  intervalFn = 100.milliseconds.fibonacci()}
```

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Composed Matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/composed-matchers.html

**Contents:**
- Composed Matchers

Composed matchers can be created for any type by composing one or more matchers. This allows to build up complex matchers from simpler ones. There are two logical operations, using which we can compose matchers: logical sum (Matcher.any) and logical product (Matcher.all).

Let's say we'd like to define a password Matcher, which will containADigit(), contain(Regex("[a-z]")) and contain(Regex("[A-Z]")). We can compose these matchers this way:

We can add extension function then:

So it can be invoked like this:

By analogy, we can build a composed matcher using Matcher.any. In this case, passwordMatcher will fail only if all matchers fail, otherwise it will pass.

Composed matchers can also be created for any class or interface by composing one or more other matchers along with the property to extract to test against.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we can add the extension variant too:

Then we invoke it this way:

**Examples:**

Example 1 (kotlin):
```kotlin
val passwordMatcher = Matcher.all(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

Example 2 (kotlin):
```kotlin
fun String.shouldBeStrongPassword() = this shouldBe passwordMatcher
```

Example 3 (kotlin):
```kotlin
"StrongPassword123".shouldBeStrongPassword()"WeakPassword".shouldBeStrongPassword() // would fail
```

Example 4 (kotlin):
```kotlin
val passwordMatcher = Matcher.any(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Until | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/matchers

**Contents:**
- matchers
  - Kotest Matcher Modules​
  - Kotest External Matcher Modules​
  - Community Provided Matchers​

For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

These modules are hosted in the kotest organization but in separate repositories from the main kotest project. They are released on an independent cadence from the Kotest framework. They provide matchers for third party libraries.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the preferred scenario for writing tests, but if you have no choice then Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Until | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

The following module is needed: io.kotest.extensions:kotest-assertions-arrow which is versioned independently of the main Kotest project. Search maven central for latest version here.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the preferred scenario for writing tests, but if you have no choice then Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Retry | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/retry.html

**Contents:**
- Retry

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## JSON | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/json/json-overview.html

**Contents:**
- JSON
- Basic matchers​
- Content-based matching​
- Schema validation​

To use these matchers add testImplementation("io.kotest:kotest-assertions-json:<version>") to your build.

For more details, see here or follow matcher-specific links below

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/matchers

**Contents:**
- matchers
  - Kotest Matcher Modules​
  - Kotest External Matcher Modules​
  - Community Provided Matchers​

For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

These modules are hosted in the kotest organization but in separate repositories from the main kotest project. They are released on an independent cadence from the Kotest framework. They provide matchers for third party libraries.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> { value ->  MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (bash):
```bash
testImplementation("io.kotest.extensions:kotest-assertions-compiler:${version}")
```

Example 2 (kotlin):
```kotlin
class CompilationTest: StringSpec() {        init {            "shouldCompile test" {                val codeSnippet = """ val aString: String = "A valid assignment" """.trimMargin()                codeSnippet.shouldCompile()                File("SourceFile.kt").shouldCompile()            }            "shouldNotCompile test" {                val codeSnippet = """ val aInteger: Int = "A invalid assignment" """.trimMargin()                codeSnippet.shouldNotCompile()                File("SourceFile.kt").shouldNotCompile()            }        }    }
```

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (bash):
```bash
testImplementation("io.kotest.extensions:kotest-assertions-compiler:${version}")
```

Example 2 (kotlin):
```kotlin
class CompilationTest: StringSpec() {        init {            "shouldCompile test" {                val codeSnippet = """ val aString: String = "A valid assignment" """.trimMargin()                codeSnippet.shouldCompile()                File("SourceFile.kt").shouldCompile()            }            "shouldNotCompile test" {                val codeSnippet = """ val aInteger: Int = "A invalid assignment" """.trimMargin()                codeSnippet.shouldNotCompile()                File("SourceFile.kt").shouldNotCompile()            }        }    }
```

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (bash):
```bash
testImplementation("io.kotest.extensions:kotest-assertions-compiler:${version}")
```

Example 2 (kotlin):
```kotlin
class CompilationTest: StringSpec() {        init {            "shouldCompile test" {                val codeSnippet = """ val aString: String = "A valid assignment" """.trimMargin()                codeSnippet.shouldCompile()                File("SourceFile.kt").shouldCompile()            }            "shouldNotCompile test" {                val codeSnippet = """ val aInteger: Int = "A invalid assignment" """.trimMargin()                codeSnippet.shouldNotCompile()                File("SourceFile.kt").shouldNotCompile()            }        }    }
```

---

## Should Be | Kotest

**URL:** https://kotest.io/docs/assertions/shouldbe.html

**Contents:**
- Should Be
- Custom Eq Instances​

The main matcher or assertion in Kotest is the shouldBe matcher. This matcher is used to assert equality between an an actual and an expected value. The syntax is in the format actual shouldBe expected For example:

When two values do not compare equal, Kotest will print out a nice error message including an intellij <click to see difference> between the two values. For example:

Note, you can check two values are not equal using shouldNotBe.

The shouldBe matcher can be combined with power assert for greater effect.

Behind the scenes, Kotest uses the equals method but also adds extra logic to determine equality for some types where simple object equality isn't quite appropriate. For example, on the JVM, it is well known that Arrays with the same contents will not be considered equal when using the equals method. Another example is primitives of different types even with the same value.

This logic is encapsulated in the Eq typeclass which Kotest uses internally. It is also possible to define your own equality logic for types by implementing Eq and registering it with Kotest.

Let's show an example of creating a custom Eq instance for comparing Foo objects. Firstly, the definition of Foo.

Then we implement the Eq typeclass for whatever equality logic we want, returning an EqResult which is either Success or Failure.

Here are are saying that if one Foo contains the string hello and the other contains the string world then they are equal. To return a failure message we can use the AssertionErrorBuilder which is a helper to build the appropriate concrete AssertionError for whichever platform we are running on.

If we specify the expected and actual values to the error builder the <click to see difference> link will be automatically generated too.

Then we register it with Kotest, specifying the type that we want to use it for. Here we are using project config to set it up before any tests are run. We could do this at the spec level too, but bear in mind if you are running tests in parallel then the registration will be non-deterministic.

Finally, we can use our custom Eq instance in our tests by simply using shouldBe or shouldNotBe as normal.

Custom Eq instances are only selected if both sides of the call are the type specified when registered. Also the type must be exact, subclasses are not selected automatically and must also be registered

**Examples:**

Example 1 (kotlin):
```kotlin
val a = "samuel"val b = a.take(3)b shouldBe "sam"
```

Example 2 (jsx):
```jsx
Expected :worldActual   :hello<Click to see difference>
```

Example 3 (kotlin):
```kotlin
val a = "samuel"val b = a.take(3)b shouldNotBe "bob"
```

Example 4 (kotlin):
```kotlin
data class Foo(val value: String)
```

---

## Collection Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/collection-matchers.html

**Contents:**
- Collection Matchers

This page describes the rich assertions (matchers) that are available for Collection, Iterable and Array types.

Also, see inspectors which are useful ways to test multiple elements in a collection.

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/next/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the preferred scenario for writing tests, but if you have no choice then Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Matching By Field | Kotest

**URL:** https://kotest.io/docs/next/assertions/field-matching.html

**Contents:**
- Matching By Field
  - matchBigDecimalsIgnoringScale​
  - matchDoublesWithTolerance​
  - matchInstantsWithTolerance​
  - matchListsIgnoringOrder​
  - matchLocalDateTimesWithTolerance​
  - matchOffsetDateTimesWithTolerance​
  - matchStringsIgnoringCase​
  - matchZonedDateTimesWithTolerance​
- Building Your Own Override Matcher​

Whenever we want to match only some of the fields, excluding some other fields from comparison, we should use shouldBeEqualUsingFields:

Likewise, we can explicitly say which fields to match on, and all other fields will be excluded:

For nested classes, comparison goes recursively, as follows:

But we can explicitly stop recursive comparison. In the following example, we are comparing instances of Doctor class as a whole, not comparing their individual fields. So the difference in mainHospital.mainDoctor is detected, as opposed to detected differences in mainHospital.mainDoctor.name in the previous example:

Also we can provide custom matchers for fields. In the following example we are matching SimpleDataClass::name as case-insensitive strings:

Kotest provides the following override matchers:

Implement Assertable interface:

For instance, here is the implementation of matchListsIgnoringOrder:

We can use any of Kotest's should*** assertions.

**Examples:**

Example 1 (kotlin):
```kotlin
val expected = Thing(name = "apple", createdAt = Instant.now())   val actual = Thing(name = "apple", createdAt = Instant.now().plusMillis(42L))   actual shouldBeEqualUsingFields {      excludedProperties = setOf(Thing::createdAt)      expected   }
```

Example 2 (kotlin):
```kotlin
val expected = Thing(name = "apple", createdAt = Instant.now())   val actual = Thing(name = "apple", createdAt = Instant.now().plusMillis(42L))   actual shouldBeEqualUsingFields {      includedProperties = setOf(Thing::name)      expected   }
```

Example 3 (kotlin):
```kotlin
val doctor1 = Doctor("billy", 23, emptyList())         val doctor2 = Doctor("barry", 23, emptyList())         val city = City("test1", Hospital("test-hospital1", doctor1))         val city2 = City("test2", Hospital("test-hospital2", doctor2))         shouldThrowAny {            city.shouldBeEqualUsingFields {               city2            }         }.message shouldContain """Using fields: - mainHospital.mainDoctor.age - mainHospital.mainDoctor.name - mainHospital.name - nameFields that differ: - mainHospital.mainDoctor.name  =>  expected:<"barry"> but was:<"billy"> - mainHospital.name  =>  expected:<"test-hospital2"> but was:<"test-hospital1"> - name  =>  expected:<"test2"> but was:<"test1">"""
```

Example 4 (kotlin):
```kotlin
val doctor1 = Doctor("billy", 22, emptyList())         val doctor2 = Doctor("billy", 22, emptyList())         val city = City("test", Hospital("test-hospital", doctor1))         val city2 = City("test", Hospital("test-hospital", doctor2))         shouldFail {            city.shouldBeEqualUsingFields {               useDefaultShouldBeForFields = listOf(Doctor::class)               city2            }         }.message shouldContain """Using fields: - mainHospital.mainDoctor - mainHospital.name - nameFields that differ: - mainHospital.mainDoctor  =>
```

---

## Json Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/json-matchers.html

**Contents:**
- Json Matchers
- shouldEqualJson​
  - compareJsonOptions​
    - Usage:​
    - Parameters​
  - shouldEqualSpecifiedJson​
- shouldContainJsonKey​
- shouldContainJsonKeyValue​
- shouldMatchJsonResource​
- Basic JSON Validation​

Kotest provides powerful JSON assertions in the kotest-assertions-json module. These allow flexible testing of json strings without the need to worry about formatting or ordering. They provide precise error messages when comparing json so that the error can be easily found in a large json structure.

This module is available for JVM and JS targets.

json.shouldEqualJson(other) asserts that the left-hand side represents the same JSON structure as the right-hand side.

The matcher allows for different formatting, and for different order of keys.

For example, the following two JSON strings would be considered equal:

The inverse of this matcher is shouldNotEqualJson which will error if two JSON strings are considered equal.

shouldEqualJson supports an additional parameter of type CompareJsonOptions which supports the following flags to toggle behaviour of the JSON comparison:

Options can be specified inline, like:

Another option is to define a compare function which suits your desires, like:

Alias for shouldEqualJson, with default options except FieldComparison which is set to FieldComparison.Lenient instead.

json.shouldContainJsonKey("$.json.path") asserts that a JSON string contains the given JSON path.

The inverse of this matcher is shouldNotContainJsonKey which will error if a JSON string does contain the given JSON path.

str.shouldContainJsonKeyValue("$.json.path", value) asserts that a JSON string contains a JSON path with a specific value.

The inverse of this matcher is shouldNotContainJsonKeyValue which will error if a JSON string does contain the given value at the given JSON path.

json.shouldMatchJsonResource("/file.json") asserts that the JSON is equal to the existing test reosource /file.json, ignoring properties' order and formatting.

There are a few matchers that simply validate that JSON is valid and optionally of a certain type.

shouldBeValidJson simply verifies that a given string parses to valid json. The inverse is shouldNotBeValidJson which will error if the string is valid json.

Targets: JVM Since: 5.2

shouldBeJsonObject asserts that a string is a valid JSON object. The inverse is shouldNotBeJsonObject which will error if the string is an object.

Targets: JVM Since: 5.2

shouldBeJsonArray asserts that a string is a valid JSON array. The inverse is shouldNotBeJsonArray which will error if the string is an array.

Targets: JVM Since: 5.2

**Examples:**

Example 1 (json):
```json
{   "name": "sam",   "location": "chicago",   "age" : 41}
```

Example 2 (json):
```json
{ "age" : 41, "name": "sam", "location": "chicago" }
```

Example 3 (kotlin):
```kotlin
a.shouldEqualJson(b, compareJsonOptions { arrayOrder = ArrayOrder.Strict })
```

Example 4 (kotlin):
```kotlin
val myOptions = compareJsonOptions {   typeCoercion = TypeCoercion.Enabled   arrayOrder = ArrayOrder.Lenient}infix fun String.lenientShouldEqualJson(other: String) = this.shouldEqualJson(other, myOptions)"[1, 2]" lenientShouldEqualJson "[2, 1]" // This will pass
```

---

## Partial Matches | Kotest

**URL:** https://kotest.io/docs/next/assertions/similarity.html

**Contents:**
- Partial Matches

If kotest fails to match a String or an instance of a data class, it may try to find something similar. For instance, in the following example two fields out of three match, so kotest considers sweetGreenApple to be 66.6% similar to sweetRedApple:

By default, kotest will only consider pairs of objects that have more than 50% matching fields. If needed, we can change similarityThresholdInPercent in configuration.

Likewise, if kotest does not detect an exact match, it may try to find a similar String. In the output, the matching part of String is indicated with plus signs:

By default, searching for similar strings is only enabled when both expected and actuals strings' lengthes are between 8 and 1024.

**Examples:**

Example 1 (kotlin):
```kotlin
listOf(sweetGreenApple, sweetGreenPear) shouldContain (sweetRedApple)(snip)PossibleMatches: expected: Fruit(name=apple, color=red, taste=sweet),  but was: Fruit(name=apple, color=green, taste=sweet),  The following fields did not match:    "color" expected: <"red">, but was: <"green">
```

Example 2 (kotlin):
```kotlin
listOf("sweet green apple", "sweet red plum") shouldContain ("sweet green pear")(snip)PossibleMatches:Match[0]: part of slice with indexes [0..11] matched actual[0..11]Line[0] ="sweet green apple"Match[0]= ++++++++++++-----
```

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> { value ->  MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (bash):
```bash
testImplementation("io.kotest.extensions:kotest-assertions-compiler:${version}")
```

Example 2 (kotlin):
```kotlin
class CompilationTest: StringSpec() {        init {            "shouldCompile test" {                val codeSnippet = """ val aString: String = "A valid assignment" """.trimMargin()                codeSnippet.shouldCompile()                File("SourceFile.kt").shouldCompile()            }            "shouldNotCompile test" {                val codeSnippet = """ val aInteger: Int = "A invalid assignment" """.trimMargin()                codeSnippet.shouldNotCompile()                File("SourceFile.kt").shouldNotCompile()            }        }    }
```

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Composed Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/composed-matchers.html

**Contents:**
- Composed Matchers

Composed matchers can be created for any type by composing one or more matchers. This allows to build up complex matchers from simpler ones. There are two logical operations, using which we can compose matchers: logical sum (Matcher.any) and logical product (Matcher.all).

Let's say we'd like to define a password Matcher, which will containADigit(), contain(Regex("[a-z]")) and contain(Regex("[A-Z]")). We can compose these matchers this way:

We can add extension function then:

So it can be invoked like this:

By analogy, we can build a composed matcher using Matcher.any. In this case, passwordMatcher will fail only if all matchers fail, otherwise it will pass.

Composed matchers can also be created for any class or interface by composing one or more other matchers along with the property to extract to test against.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we can add the extension variant too:

Then we invoke it this way:

**Examples:**

Example 1 (kotlin):
```kotlin
val passwordMatcher = Matcher.all(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

Example 2 (kotlin):
```kotlin
fun String.shouldBeStrongPassword() = this shouldBe passwordMatcher
```

Example 3 (kotlin):
```kotlin
"StrongPassword123".shouldBeStrongPassword()"WeakPassword".shouldBeStrongPassword() // would fail
```

Example 4 (kotlin):
```kotlin
val passwordMatcher = Matcher.any(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/eventually.html

**Contents:**
- Eventually
  - Examples​
    - Simple examples​
    - Exceptions​
    - Predicates​
    - Sharing configuration​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, if you were testing a IO operation, you might need to wait until the IO operation has flushed.

Sometimes you can do a Thread.sleep but this is isn't ideal as you need to set a sleep threshold high enough so that it won't expire prematurely on a slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Or you can roll a loop and sleep and retry and sleep and retry, but this is just boilerplate slowing you down.

Another common approach is to use countdown latches and this works fine if you are able to inject the latches in the appropriate places but it isn't always possible to have the code under test trigger a latch.

As an alternative, kotest provides the eventually function and the Eventually configuration which periodically test the code ignoring your specified exceptions and ensuring the result satisfies an optional predicate, until the timeout is eventually reached or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code.

Let's assume that we send a message to an asynchronous service. After the message is processed, a new row is inserted into user table.

We can check this behaviour with our eventually function.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test.

Let's assume that our example from before throws a UserNotFoundException while the user is not found in the database. It will eventually return the user when the message is processed by the system.

In this scenario, we can explicitly skip the exception that we expect to happen until the test passed, but any other exceptions would not be ignored. Note, this example is similar to the former, but if there was some other error, say a ConnectionException for example, this would cause the eventually block to immediately exit with a failure message.

In addition to verifying a test case eventually runs without throwing, we can also verify the result and treat a non-throwing result as failing.

Sharing the configuration for eventually is a breeze with the Eventually data class. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

Here we can see sharing of configuration can be useful to reduce duplicate code while allowing flexibility for things like custom logging per test suite for clear test logs.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("check if user repository has one row after message is sent") {      sendMessage()      eventually(5.seconds) {        userRepository.size() shouldBe 1      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("check if user repository has one row") {      eventually(5.seconds, UserNotFoundException::class.java) {        userRepository.findBy(1) shouldNotBe null      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({  "check that predicate eventually succeeds in time" {    var i = 0    eventually<Int>(25.seconds, predicate = { it == 5 }) {      delay(1.seconds)      i++    }  }})
```

Example 4 (kotlin):
```kotlin
val slow = EventuallyConfig<ServerResponse, ServerException>(5.minutes, interval = 25.milliseconds.fibonacci(), exceptionClass = ServerException::class)val fast = slow.copy(duration = 5.seconds)class FooTests : StringSpec({  val logger = logger("FooTests")  val fSlow = slow.copy(listener = { i, t -> logger.info("Current $i after {${t.times} attempts")})  "server eventually provides a result for /foo" {    eventually(fSlow) {      fooApi()    }  }})class BarTests : StringSpec({  val logger = logger("BarTests")  val bFast = fast.copy(listener = { i, t -> logger.info("Current $i after {${t.times} attempts")})  "server eventually provides a result for /bar" {    eventually(bFast) {      barApi()    }  }})
```

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

---

## Composed Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/composed-matchers.html

**Contents:**
- Composed Matchers

Composed matchers can be created for any class or interface by composing one or more other matchers along with the property to extract to test against. This allows us to build up complicated matchers from simpler ones.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we could add the extension variant too:

Then we invoke like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class Person(  val name: String,  val age: Int,  val address: Address,)data class Address(  val city: String,  val street: String,  val buildingNumber: String,)
```

Example 2 (kotlin):
```kotlin
fun nameMatcher(name: String) = Matcher<String> {  MatcherResult(    value == name,    { "Name $value should be $name" },    { "Name $value should not be $name" }  )}fun ageMatcher(age: Int) = Matcher<Int> {  MatcherResult(    value == age,    { "Age $value should be $age" },    { "Age $value should not be $age" }  )}val addressMatcher = Matcher<Address> {  MatcherResult(    value == Address("Warsaw", "Test", "1/1"),    { "Address $value should be Test 1/1 Warsaw" },    { "Address $value should not be Test 1/1 Warsaw" }  )}
```

Example 3 (kotlin):
```kotlin
fun personMatcher(name: String, age: Int) = Matcher.compose(  nameMatcher(name) to Person::name,  ageMatcher(age) to Person::age,  addressMatcher to Person::address)
```

Example 4 (kotlin):
```kotlin
fun Person.shouldBePerson(name: String, age: Int) = this shouldBe personMatcher(name, age)
```

---

## JSON | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/json/json-overview.html

**Contents:**
- JSON
- Basic matchers​
- Content-based matching​
- Schema validation​

For more details, see here or follow matcher-specific links below

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/eventually.html

**Contents:**
- Eventually
- API​
- Configuration Options​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 5.7, the non-deterministic testing functions have moved to the kotest-assertions-core module, and are available under the new package io.kotest.assertions.nondeterministic. The previous iterations of these functions are still available, but deprecated.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-determistic operation. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually function which solves the common use case of "I expect this code to pass after a short period of time".

Eventually works by periodically invoking a given lambda, ignoring specified exceptions, until the lambda passes, or a timeout is reached, or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized with regards to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration, using the Kotlin Duration type, followed by the code that should eventually pass without an exception being raised.

The second is by providing a config block. This method should be used when you need to set more options than just the duration. It also allows the config to be shared between multiple invocations of eventually.

The duration is the total amount of time to keep trying to pass the test. The interval allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Alternatively, rather than specifying the interval as a fixed number, we can pass in a function. This allows us to perform some kind of backoff, or anything else we need.

For example, to use a fibonacci increasing interval, starting with 100ms:

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test. We call these exceptions, the expected exceptions.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be ignored, or false if the exception should bubble out.

We can attach a listener, which will be invoked on each iteration, with the current iteration count and the exception that caused the iteration to fail. Note: The listener will not be fired on a successful invocation.

Sharing the configuration for eventually is a breeze with the eventuallyConfig builder. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5.seconds) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 1.seconds  interval = 100.milliseconds}eventually(config) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  interval = 250.milliseconds}
```

Example 4 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  intervalFn = 100.milliseconds.fibonacci()}
```

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## Matching By Field | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/field-matching.html

**Contents:**
- Matching By Field
  - matchBigDecimalsIgnoringScale​
  - matchDoublesWithTolerance​
  - matchInstantsWithTolerance​
  - matchListsIgnoringOrder​
  - matchLocalDateTimesWithTolerance​
  - matchOffsetDateTimesWithTolerance​
  - matchStringsIgnoringCase​
  - matchZonedDateTimesWithTolerance​
- Building Your Own Override Matcher​

Whenever we want to match only some of the fields, excluding some other fields from comparison, we should use shouldBeEqualUsingFields:

Likewise, we can explicitly say which fields to match on, and all other fields will be excluded:

For nested classes, comparison goes recursively, as follows:

But we can explicitly stop recursive comparison. In the following example, we are comparing instances of Doctor class as a whole, not comparing their individual fields. So the difference in mainHospital.mainDoctor is detected, as opposed to detected differences in mainHospital.mainDoctor.name in the previous example:

Also we can provide custom matchers for fields. In the following example we are matching SimpleDataClass::name as case-insensitive strings:

Kotest provides the following override matchers:

Implement Assertable interface:

For instance, here is the implementation of matchListsIgnoringOrder:

We can use any of Kotest's should*** assertions.

**Examples:**

Example 1 (kotlin):
```kotlin
val expected = Thing(name = "apple", createdAt = Instant.now())   val actual = Thing(name = "apple", createdAt = Instant.now().plusMillis(42L))   actual shouldBeEqualUsingFields {      excludedProperties = setOf(Thing::createdAt)      expected   }
```

Example 2 (kotlin):
```kotlin
val expected = Thing(name = "apple", createdAt = Instant.now())   val actual = Thing(name = "apple", createdAt = Instant.now().plusMillis(42L))   actual shouldBeEqualUsingFields {      includedProperties = setOf(Thing::name)      expected   }
```

Example 3 (kotlin):
```kotlin
val doctor1 = Doctor("billy", 23, emptyList())         val doctor2 = Doctor("barry", 23, emptyList())         val city = City("test1", Hospital("test-hospital1", doctor1))         val city2 = City("test2", Hospital("test-hospital2", doctor2))         shouldThrowAny {            city.shouldBeEqualUsingFields {               city2            }         }.message shouldContain """Using fields: - mainHospital.mainDoctor.age - mainHospital.mainDoctor.name - mainHospital.name - nameFields that differ: - mainHospital.mainDoctor.name  =>  expected:<"barry"> but was:<"billy"> - mainHospital.name  =>  expected:<"test-hospital2"> but was:<"test-hospital1"> - name  =>  expected:<"test2"> but was:<"test1">"""
```

Example 4 (kotlin):
```kotlin
val doctor1 = Doctor("billy", 22, emptyList())         val doctor2 = Doctor("billy", 22, emptyList())         val city = City("test", Hospital("test-hospital", doctor1))         val city2 = City("test", Hospital("test-hospital", doctor2))         shouldFail {            city.shouldBeEqualUsingFields {               useDefaultShouldBeForFields = listOf(Doctor::class)               city2            }         }.message shouldContain """Using fields: - mainHospital.mainDoctor - mainHospital.name - nameFields that differ: - mainHospital.mainDoctor  =>
```

---

## Composed Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/composed-matchers.html

**Contents:**
- Composed Matchers

Composed matchers can be created for any type by composing one or more matchers. This allows to build up complex matchers from simpler ones. There are two logical operations, using which we can compose matchers: logical sum (Matcher.any) and logical product (Matcher.all).

Let's say we'd like to define a password Matcher, which will containADigit(), contain(Regex("[a-z]")) and contain(Regex("[A-Z]")). We can compose these matchers this way:

We can add extension function then:

So it can be invoked like this:

By analogy, we can build a composed matcher using Matcher.any. In this case, passwordMatcher will fail only if all matchers fail, otherwise it will pass.

Composed matchers can also be created for any class or interface by composing one or more other matchers along with the property to extract to test against.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we can add the extension variant too:

Then we invoke it this way:

**Examples:**

Example 1 (kotlin):
```kotlin
val passwordMatcher = Matcher.all(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

Example 2 (kotlin):
```kotlin
fun String.shouldBeStrongPassword() = this shouldBe passwordMatcher
```

Example 3 (kotlin):
```kotlin
"StrongPassword123".shouldBeStrongPassword()"WeakPassword".shouldBeStrongPassword() // would fail
```

Example 4 (kotlin):
```kotlin
val passwordMatcher = Matcher.any(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

---

## Until | Kotest

**URL:** https://kotest.io/docs/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/next/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

Note: only Kotest's own assertions can be asserted softly. To be compatible with assertSoftly, assertions from other libraries must be wrapped in shouldNotThrowAny, which is described later in this section. If any other checks fail and throw an AssertionError, it will not respect assertSoftly and bubble up, erasing the results of previous assertions. This includes Kotest's own fail() function, so when the following code runs, we won't know if the first assertion foo shouldBe bar succeeded or failed:

Note, however, that failSoftly is compatible with assertSoftly, so the following code will report both failures:

Likewise, if mockk's verify(...) fails in the following example, the second assertion will not execute:

So if we want to invoke non-kotest assertions inside assertSoftly blocks, they need to be invoked via shouldPass. In the following example both verify and the second assertion can fail, and we shall get both errors accumulated:

Likewise, in the following example the failure of verify will not be ignored, it will be added along with the failure of the first assertion:

Note: by design, some of Kotest's own assertions are not compatible with assertSoftly, including:

But shouldThrowSoftly is compatible with assertSoftly.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

Example 3 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  fail("Something happened")}
```

Example 4 (kotlin):
```kotlin
assertSoftly {  2*2 shouldBe 5  failSoftly("Something happened")}
```

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

Note: only Kotest's own assertions can be asserted softly. To be compatible with assertSoftly, assertions from other libraries must be wrapped in shouldNotThrowAny, which is described later in this section. If any other checks fail and throw an AssertionError, it will not respect assertSoftly and bubble up, erasing the results of previous assertions. This includes Kotest's own fail() function, so when the following code runs, we won't know if the first assertion foo shouldBe bar succeeded or failed:

Likewise, if mockk's verify(...) fails in the following example, the second assertion will not execute:

So if we want to invoke non-kotest assertions inside assertSoftly blocks, they need to be invoked via shouldPass. In the following example both verify and the second assertion can fail, and we shall get both errors accumulated:

Likewise, in the following example the failure of verify will not be ignored, it will be added along with the failure of the first assertion:

Note: by design, some of Kotest's own assertions are not compatible with assertSoftly, including:

But shouldThrowSoftly and shouldNotThrowExactlyUnit are compatible with assertSoftly.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

Example 3 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  fail("Something happened")}
```

Example 4 (kotlin):
```kotlin
assertSoftly {  verify(exactly = 1) { myClass.myMethod(any()) }  foo shouldBe bar}
```

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/eventually.html

**Contents:**
- Eventually
  - Examples​
    - Simple examples​
    - Exceptions​
    - Predicates​
    - Sharing configuration​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, if you were testing a IO operation, you might need to wait until the IO operation has flushed.

Sometimes you can do a Thread.sleep but this is isn't ideal as you need to set a sleep threshold high enough so that it won't expire prematurely on a slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Or you can roll a loop and sleep and retry and sleep and retry, but this is just boilerplate slowing you down.

Another common approach is to use countdown latches and this works fine if you are able to inject the latches in the appropriate places but it isn't always possible to have the code under test trigger a latch.

As an alternative, kotest provides the eventually function and the Eventually configuration which periodically test the code ignoring your specified exceptions and ensuring the result satisfies an optional predicate, until the timeout is eventually reached or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code.

Let's assume that we send a message to an asynchronous service. After the message is processed, a new row is inserted into user table.

We can check this behaviour with our eventually function.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test.

Let's assume that our example from before throws a UserNotFoundException while the user is not found in the database. It will eventually return the user when the message is processed by the system.

In this scenario, we can explicitly skip the exception that we expect to happen until the test passed, but any other exceptions would not be ignored. Note, this example is similar to the former, but if there was some other error, say a ConnectionException for example, this would cause the eventually block to immediately exit with a failure message.

In addition to verifying a test case eventually runs without throwing, we can also verify the result and treat a non-throwing result as failing.

Sharing the configuration for eventually is a breeze with the Eventually data class. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

Here we can see sharing of configuration can be useful to reduce duplicate code while allowing flexibility for things like custom logging per test suite for clear test logs.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("check if user repository has one row after message is sent") {      sendMessage()      eventually(5.seconds) {        userRepository.size() shouldBe 1      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("check if user repository has one row") {      eventually(5.seconds, UserNotFoundException::class.java) {        userRepository.findBy(1) shouldNotBe null      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({  "check that predicate eventually succeeds in time" {    var i = 0    eventually<Int>(25.seconds, predicate = { it == 5 }) {      delay(1.seconds)      i++    }  }})
```

Example 4 (kotlin):
```kotlin
val slow = EventuallyConfig<ServerResponse, ServerException>(5.minutes, interval = 25.milliseconds.fibonacci(), exceptionClass = ServerException::class)val fast = slow.copy(duration = 5.seconds)class FooTests : StringSpec({  val logger = logger("FooTests")  val fSlow = slow.copy(listener = { i, t -> logger.info("Current $i after {${t.times} attempts")})  "server eventually provides a result for /foo" {    eventually(fSlow) {      fooApi()    }  }})class BarTests : StringSpec({  val logger = logger("BarTests")  val bFast = fast.copy(listener = { i, t -> logger.info("Current $i after {${t.times} attempts")})  "server eventually provides a result for /bar" {    eventually(bFast) {      barApi()    }  }})
```

---

## Continually | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/continually.html

**Contents:**
- Continually

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

The function passed to the continually block is executed every 10 milliseconds. We can specify the poll interval if we prefer:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds, 5.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-assertions-compiler:$version")
```

Example 2 (kotlin):
```kotlin
class CompilationTest : FreeSpec({    "shouldCompile test" {        val rawStringCodeSnippet = """            val aString: String = "A valid assignment"        """        val syntaxHighlightedSnippet = codeSnippet("""            val aString: String = "A valid assignment"        """)        rawStringCodeSnippet.shouldCompile()        syntaxHighlightedSnippet.shouldCompile()        File("SourceFile.kt").shouldCompile()    }    "shouldNotCompile test" {        val rawStringCodeSnippet = """            val anInteger: Int = "An invalid assignment"        """        val syntaxHighlightedSnippet = codeSnippet("""            val anInteger: Int = "An invalid assignment"        """)        rawStringCodeSnippet.shouldNotCompile()        syntaxHighlightedSnippet.shouldNotCompile()        File("SourceFile.kt").shouldNotCompile()        // check that a compilation error occurred for a specific reason        rawStringCodeSnippet.shouldNotCompile("expected 'Int', actual 'String'")        syntaxHighlightedSnippet.shouldNotCompile("expected 'Int', actual 'String'")        File("SourceFile.kt").shouldNotCompile("expected 'Int', actual 'String'")    }    @OptIn(ExperimentalCompilerApi::class)    "custom assertions on JvmCompilationResult" {        val codeSnippet = codeSnippet("""            fun foo() {                printDate(LocalDate.now())            }        """)        codeSnippet.compile {            exitCode shouldBe ExitCode.COMPILATION_ERROR            messages shouldContain "Unresolved reference 'LocalDate'"            messages shouldContain "Unresolved reference 'printDate'"        }    }    @OptIn(ExperimentalCompilerApi::class)    "custom compiler configuration" {        val compileConfig = CompileConfig {            compilerPluginRegistrars = listOf(MyCompilerPluginRegistrar())        }        val codeSnippet = compileConfig.codeSnippet("""            @MyAnnotation            fun hello() {}        """)        codeSnippet.shouldCompile()    }})
```

---

## Until | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## Composed Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/composed-matchers.html

**Contents:**
- Composed Matchers

Composed matchers can be created for any class or interface by composing one or more other matchers along with the property to extract to test against. This allows us to build up complicated matchers from simpler ones.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we could add the extension variant too:

Then we invoke like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class Person(  val name: String,  val age: Int,  val address: Address,)data class Address(  val city: String,  val street: String,  val buildingNumber: String,)
```

Example 2 (kotlin):
```kotlin
fun nameMatcher(name: String) = Matcher<String> {  MatcherResult(    value == name,    { "Name $value should be $name" },    { "Name $value should not be $name" }  )}fun ageMatcher(age: Int) = Matcher<Int> {  MatcherResult(    value == age,    { "Age $value should be $age" },    { "Age $value should not be $age" }  )}val addressMatcher = Matcher<Address> {  MatcherResult(    value == Address("Warsaw", "Test", "1/1"),    { "Address $value should be Test 1/1 Warsaw" },    { "Address $value should not be Test 1/1 Warsaw" }  )}
```

Example 3 (kotlin):
```kotlin
fun personMatcher(name: String, age: Int) = Matcher.compose(  nameMatcher(name) to Person::name,  ageMatcher(age) to Person::age,  addressMatcher to Person::address)
```

Example 4 (kotlin):
```kotlin
fun Person.shouldBePerson(name: String, age: Int) = this shouldBe personMatcher(name, age)
```

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the ideal scenario for writing tests, but for the times when it is required, Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Clues | Kotest

**URL:** https://kotest.io/docs/next/assertions/clues.html

**Contents:**
- Clues
- Nested clues​

Clues only work if you are using the Kotest assertions library

A rule of thumb is that a failing test should look like a good bug report. In other words, it should tell you what went wrong, and ideally why it went wrong.

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

In this case, it looks like the system populates the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

For example, we can use withClue with a string message

Would give an error like this:

The error message became much better, however, it is still not as good as it could be. For instance, it might be helpful to know the user's id to check the database.

We can use withClue to add the user's id to the error message:

We can also use the asClue extension function to turn any object into the clue message.

The message will be computed only in case the test fails, so it is safe to use it with expensive operations.

Test failures include a failed assertion, test name, clues, and stacktrace. Consider using them in such a way, so they answer both what has failed, and why it failed. It will make the tests easier to maintain, especially when it comes to reverse-engineering the intention of the test author.

Every time you see a code comment above an assertion, consider using asClue, or withClue instead. The comments are not visible in the test failures, especially on CI, while clues will be visible.

You can use domain objects as clues as well:

Kotest considers all () -> Any? clues as lazy clues, and would compute them and use .toString() on the resulting value instead of calling .toString() on the function itself. In most cases, it should do exactly what you need, however, if clue object implements () -> Any?, and you want using clue.toString(), then consider wrapping the clue manually as { clue.toString() }.asClue { ... }.

Clues can be nested, and they all will be visible in the failed assertion messages:

The failure might look like

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to assert that no elements in a collection passed the assertions, we could do something like:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (bash):
```bash
testImplementation("io.kotest.extensions:kotest-assertions-compiler:${version}")
```

Example 2 (kotlin):
```kotlin
class CompilationTest: StringSpec() {        init {            "shouldCompile test" {                val codeSnippet = """ val aString: String = "A valid assignment" """.trimMargin()                codeSnippet.shouldCompile()                File("SourceFile.kt").shouldCompile()            }            "shouldNotCompile test" {                val codeSnippet = """ val aInteger: Int = "A invalid assignment" """.trimMargin()                codeSnippet.shouldNotCompile()                File("SourceFile.kt").shouldNotCompile()            }        }    }
```

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/eventually.html

**Contents:**
- Eventually
- API​
- Configuration Options​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 5.7, the non-deterministic testing functions have moved to the kotest-assertions-core module, and are available under the new package io.kotest.assertions.nondeterministic. The previous iterations of these functions are still available, but deprecated.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-determistic operation. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually function which solves the common use case of "I expect this code to pass after a short period of time".

Eventually works by periodically invoking a given lambda, ignoring specified exceptions, until the lambda passes, or a timeout is reached, or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized with regards to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration, using the Kotlin Duration type, followed by the code that should eventually pass without an exception being raised.

The second is by providing a config block. This method should be used when you need to set more options than just the duration. It also allows the config to be shared between multiple invocations of eventually.

The duration is the total amount of time to keep trying to pass the test. The interval allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Alternatively, rather than specifying the interval as a fixed number, we can pass in a function. This allows us to perform some kind of backoff, or anything else we need.

For example, to use a fibonacci increasing interval, starting with 100ms:

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test. We call these exceptions, the expected exceptions.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be ignored, or false if the exception should bubble out. If expectedExceptions is specified and the set is not empty, this function will be ignored.

We can attach a listener, which will be invoked on each iteration, with the current iteration count and the exception that caused the iteration to fail. Note: The listener will not be fired on a successful invocation.

Sharing the configuration for eventually is a breeze with the eventuallyConfig builder. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5.seconds) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 1.seconds  interval = 100.milliseconds}eventually(config) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  interval = 250.milliseconds}
```

Example 4 (kotlin):
```kotlin
val config = eventuallyConfig {  duration = 5.seconds  intervalFn = 100.milliseconds.fibonacci()}
```

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## Collection Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/collection-matchers.html

**Contents:**
- Collection Matchers

This page describes the rich assertions (matchers) that are available for Collection, Iterable and Array types.

Also, see inspectors which are useful ways to test multiple elements in a collection.

---

## Continually | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/continually.html

**Contents:**
- Continually

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

The function passed to the continually block is executed every 10 milliseconds. We can specify the poll interval if we prefer:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds, 5.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> { value ->  MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the preferred scenario for writing tests, but if you have no choice then Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/next/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

The following module is needed: io.kotest.extensions:kotest-assertions-arrow which is versioned independently of the main Kotest project. Search maven central for latest version here.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to asset that no elements in a collection passed the assertions, we could do something like:

If you want to filter a collection to only elements that pass the assertions, you can use the filterMatching method:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

Example 3 (kotlin):
```kotlin
xs.filterMatching {  it.shouldContain("x")  it.shouldStartWith("bb")}.shouldBeEmpty()
```

---

## JSON | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/json/json-overview.html

**Contents:**
- JSON
- Basic matchers​
- Content-based matching​
- Schema validation​

To use these matchers add testImplementation("io.kotest:kotest-assertions-json:<version>") to your build.

For more details, see here or follow matcher-specific links below

---

## Continually | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/continually.html

**Contents:**
- Continually

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

The function passed to the continually block is executed every 10 milliseconds. We can specify the poll interval if we prefer:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds, 5.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/eventually.html

**Contents:**
- Eventually
  - Examples​
    - Simple examples​
    - Exceptions​
    - Predicates​
    - Sharing configuration​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, if you were testing a IO operation, you might need to wait until the IO operation has flushed.

Sometimes you can do a Thread.sleep but this is isn't ideal as you need to set a sleep threshold high enough so that it won't expire prematurely on a slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Or you can roll a loop and sleep and retry and sleep and retry, but this is just boilerplate slowing you down.

Another common approach is to use countdown latches and this works fine if you are able to inject the latches in the appropriate places but it isn't always possible to have the code under test trigger a latch.

As an alternative, kotest provides the eventually function and the Eventually configuration which periodically test the code ignoring your specified exceptions and ensuring the result satisfies an optional predicate, until the timeout is eventually reached or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code.

Let's assume that we send a message to an asynchronous service. After the message is processed, a new row is inserted into user table.

We can check this behaviour with our eventually function.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test.

Let's assume that our example from before throws a UserNotFoundException while the user is not found in the database. It will eventually return the user when the message is processed by the system.

In this scenario, we can explicitly skip the exception that we expect to happen until the test passed, but any other exceptions would not be ignored. Note, this example is similar to the former, but if there was some other error, say a ConnectionException for example, this would cause the eventually block to immediately exit with a failure message.

In addition to verifying a test case eventually runs without throwing, we can also verify the result and treat a non-throwing result as failing.

Sharing the configuration for eventually is a breeze with the Eventually data class. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

Here we can see sharing of configuration can be useful to reduce duplicate code while allowing flexibility for things like custom logging per test suite for clear test logs.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("check if user repository has one row after message is sent") {      sendMessage()      eventually(5.seconds) {        userRepository.size() shouldBe 1      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("check if user repository has one row") {      eventually(5.seconds, UserNotFoundException::class.java) {        userRepository.findBy(1) shouldNotBe null      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({  "check that predicate eventually succeeds in time" {    var i = 0    eventually<Int>(25.seconds, predicate = { it == 5 }) {      delay(1.seconds)      i++    }  }})
```

Example 4 (kotlin):
```kotlin
val slow = EventuallyConfig<ServerResponse, ServerException>(5.minutes, interval = 25.milliseconds.fibonacci(), exceptionClass = ServerException::class)val fast = slow.copy(duration = 5.seconds)class FooTests : StringSpec({  val logger = logger("FooTests")  val fSlow = slow.copy(listener = { i, t -> logger.info("Current $i after {${t.times} attempts")})  "server eventually provides a result for /foo" {    eventually(fSlow) {      fooApi()    }  }})class BarTests : StringSpec({  val logger = logger("BarTests")  val bFast = fast.copy(listener = { i, t -> logger.info("Current $i after {${t.times} attempts")})  "server eventually provides a result for /bar" {    eventually(bFast) {      barApi()    }  }})
```

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Clues | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/clues.html

**Contents:**
- Clues
- Nested clues​

Clues only work if you are using the Kotest assertions library

A rule of thumb is that a failing test should look like a good bug report. In other words, it should tell you what went wrong, and ideally why it went wrong.

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

In this case, it looks like the system populates the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

For example, we can use withClue with a string message

Would give an error like this:

The error message became much better, however, it is still not as good as it could be. For instance, it might be helpful to know the user's id to check the database.

We can use withClue to add the user's id to the error message:

We can also use the asClue extension function to turn any object into the clue message.

The message will be computed only in case the test fails, so it is safe to use it with expensive operations.

Test failures include a failed assertion, test name, clues, and stacktrace. Consider using them in such a way, so they answer both what has failed, and why it failed. It will make the tests easier to maintain, especially when it comes to reverse-engineering the intention of the test author.

Every time you see a code comment above an assertion, consider using asClue, or withClue instead. The comments are not visible in the test failures, especially on CI, while clues will be visible.

You can use domain objects as clues as well:

Kotest considers all () -> Any? clues as lazy clues, and would compute them and use .toString() on the resulting value instead of calling .toString() on the function itself. In most cases, it should do exactly what you need, however, if clue object implements () -> Any?, and you want using clue.toString(), then consider wrapping the clue manually as { clue.toString() }.asClue { ... }.

Clues can be nested, and they all will be visible in the failed assertion messages:

The failure might look like

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to asset that no elements in a collection passed the assertions, we could do something like:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid user1
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(a) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to asset that no elements in a collection passed the assertions, we could do something like:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

---

## Composed Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/composed-matchers.html

**Contents:**
- Composed Matchers

Composed matchers can be created for any type by composing one or more matchers. This allows to build up complex matchers from simpler ones. There are two logical operations, using which we can compose matchers: logical sum (Matcher.any) and logical product (Matcher.all).

Let's say we'd like to define a password Matcher, which will containADigit(), contain(Regex("[a-z]")) and contain(Regex("[A-Z]")). We can compose these matchers this way:

We can add extension function then:

So it can be invoked like this:

By analogy, we can build a composed matcher using Matcher.any. In this case, passwordMatcher will fail only if all matchers fail, otherwise it will pass.

Composed matchers can also be created for any class or interface by composing one or more other matchers along with the property to extract to test against.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we can add the extension variant too:

Then we invoke it this way:

**Examples:**

Example 1 (kotlin):
```kotlin
val passwordMatcher = Matcher.all(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

Example 2 (kotlin):
```kotlin
fun String.shouldBeStrongPassword() = this shouldBe passwordMatcher
```

Example 3 (kotlin):
```kotlin
"StrongPassword123".shouldBeStrongPassword()"WeakPassword".shouldBeStrongPassword() // would fail
```

Example 4 (kotlin):
```kotlin
val passwordMatcher = Matcher.any(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

---

## JSON | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/json/json-overview.html

**Contents:**
- JSON
- Basic matchers​
- Content-based matching​
- Schema validation​

To use these matchers add testImplementation("io.kotest:kotest-assertions-json:<version>") to your build.

There exist copies of all matchers that validate a File or a Path instead of a String for the JVM platform.

For more details, see here or follow matcher-specific links below

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/eventually.html

**Contents:**
- Eventually
  - Examples​
    - Simple examples​
    - Exceptions​
    - Predicates​
    - Sharing configuration​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, if you were testing a IO operation, you might need to wait until the IO operation has flushed.

Sometimes you can do a Thread.sleep but this is isn't ideal as you need to set a sleep threshold high enough so that it won't expire prematurely on a slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Or you can roll a loop and sleep and retry and sleep and retry, but this is just boilerplate slowing you down.

Another common approach is to use countdown latches and this works fine if you are able to inject the latches in the appropriate places but it isn't always possible to have the code under test trigger a latch.

As an alternative, kotest provides the eventually function and the Eventually configuration which periodically test the code ignoring your specified exceptions and ensuring the result satisfies an optional predicate, until the timeout is eventually reached or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code.

Let's assume that we send a message to an asynchronous service. After the message is processed, a new row is inserted into user table.

We can check this behaviour with our eventually function.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test.

Let's assume that our example from before throws a UserNotFoundException while the user is not found in the database. It will eventually return the user when the message is processed by the system.

In this scenario, we can explicitly skip the exception that we expect to happen until the test passed, but any other exceptions would not be ignored. Note, this example is similar to the former, but if there was some other error, say a ConnectionException for example, this would cause the eventually block to immediately exit with a failure message.

In addition to verifying a test case eventually runs without throwing, we can also verify the result and treat a non-throwing result as failing.

Sharing the configuration for eventually is a breeze with the Eventually data class. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

Here we can see sharing of configuration can be useful to reduce duplicate code while allowing flexibility for things like custom logging per test suite for clear test logs.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("check if user repository has one row after message is sent") {      sendMessage()      eventually(5.seconds) {        userRepository.size() shouldBe 1      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("check if user repository has one row") {      eventually(5.seconds, UserNotFoundException::class.java) {        userRepository.findBy(1) shouldNotBe null      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({  "check that predicate eventually succeeds in time" {    var i = 0    eventually<Int>(25.seconds, predicate = { it == 5 }) {      delay(1.seconds)      i++    }  }})
```

Example 4 (kotlin):
```kotlin
val slow = EventuallyConfig<ServerResponse, ServerException>(5.minutes, interval = 25.milliseconds.fibonacci(), exceptionClass = ServerException::class)val fast = slow.copy(duration = 5.seconds)class FooTests : StringSpec({  val logger = logger("FooTests")  val fSlow = slow.copy(listener = { i, t -> logger.info("Current $i after {${t.times} attempts")})  "server eventually provides a result for /foo" {    eventually(fSlow) {      fooApi()    }  }})class BarTests : StringSpec({  val logger = logger("BarTests")  val bFast = fast.copy(listener = { i, t -> logger.info("Current $i after {${t.times} attempts")})  "server eventually provides a result for /bar" {    eventually(bFast) {      barApi()    }  }})
```

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## JSON | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/json/json-overview.html

**Contents:**
- JSON
- Basic matchers​
- Content-based matching​
- Schema validation​

For more details, see here or follow matcher-specific links below

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## Retry | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/retry.html

**Contents:**
- Retry

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## Continually | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/continually.html

**Contents:**
- Continually

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

The function passed to the continually block is executed every 10 milliseconds. We can specify the poll interval if we prefer:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds, 5.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

---

## Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/matchers.html

**Contents:**
- Matchers
  - Kotest Matcher Modules​
  - Community Provided Matchers​

A Matcher is the Kotest term for an assertion that performs a specific test. For example, a matcher may test that a double is greater than zero. Or it it may test that a file is not empty.

Kotest currently has approximately 400 matchers split across several modules. Most of these matchers are for standard library types. Others are project specific. Additionally, there are matchers provided by third party libraries.

Kotest matchers are framework agnostic. You can use them with the Kotest framework, or with any other framework, such as JUnit.

Matchers can be used in two styles:

Both styles are supported. The advantage of the extension function style is that the IDE can autocomplete for you, but some people may prefer the infix style as it is slightly cleaner.

Matchers can be negated by using shouldNot instead of should for the infix style. For example, a shouldNot startWith("boo"). For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

To add your library to this list, please open a PR.

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to asset that no elements in a collection passed the assertions, we could do something like:

If you want to filter a collection to only elements that pass the assertions, you can use the filterMatching method:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

Example 3 (kotlin):
```kotlin
xs.filterMatching {  it.shouldContain("x")  it.shouldStartWith("bb")}.shouldBeEmpty()
```

---

## JSON | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/json/json-overview.html

**Contents:**
- JSON
- Basic matchers​
- Content-based matching​
- Schema validation​

For more details, see here or follow matcher-specific links below

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## Partial Matches | Kotest

**URL:** https://kotest.io/docs/assertions/similarity.html

**Contents:**
- Partial Matches

If kotest fails to match a String or an instance of a data class, it may try to find something similar. For instance, in the following example two fields out of three match, so kotest considers sweetGreenApple to be 66.6% similar to sweetRedApple:

By default, kotest will only consider pairs of objects that have more than 50% matching fields. If needed, we can change similarityThresholdInPercent in configuration.

Likewise, if kotest does not detect an exact match, it may try to find a similar String. In the output, the matching part of String is indicated with plus signs:

By default, searching for similar strings is only enabled when both expected and actuals strings' lengthes are between 8 and 1024.

**Examples:**

Example 1 (kotlin):
```kotlin
listOf(sweetGreenApple, sweetGreenPear) shouldContain (sweetRedApple)(snip)PossibleMatches: expected: Fruit(name=apple, color=red, taste=sweet),  but was: Fruit(name=apple, color=green, taste=sweet),  The following fields did not match:    "color" expected: <"red">, but was: <"green">
```

Example 2 (kotlin):
```kotlin
listOf("sweet green apple", "sweet red plum") shouldContain ("sweet green pear")(snip)PossibleMatches:Match[0]: part of slice with indexes [0..11] matched actual[0..11]Line[0] ="sweet green apple"Match[0]= ++++++++++++-----
```

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid alice
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(bob) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

Note: only Kotest's own assertions can be asserted softly. To be compatible with assertSoftly, assertions from other libraries must be wrapped in shouldNotThrowAny, which is described later in this section. If any other checks fail and throw an AssertionError, it will not respect assertSoftly and bubble up, erasing the results of previous assertions. This includes Kotest's own fail() function, so when the following code runs, we won't know if the first assertion foo shouldBe bar succeeded or failed:

Note, however, that failSoftly is compatible with assertSoftly, so the following code will report both failures:

Likewise, if mockk's verify(...) fails in the following example, the second assertion will not execute:

So if we want to invoke non-kotest assertions inside assertSoftly blocks, they need to be invoked via shouldPass. In the following example both verify and the second assertion can fail, and we shall get both errors accumulated:

Likewise, in the following example the failure of verify will not be ignored, it will be added along with the failure of the first assertion:

Note: by design, some of Kotest's own assertions are not compatible with assertSoftly, including:

But shouldThrowSoftly is compatible with assertSoftly.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

Example 3 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  fail("Something happened")}
```

Example 4 (kotlin):
```kotlin
assertSoftly {  2*2 shouldBe 5  failSoftly("Something happened")}
```

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Android Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/android-matchers.html

**Contents:**
- Android Matchers

This page lists all current Android matchers in Kotest. These are additional to the default matchers and are specific to Android.

To use them, it's required to add an extra dependency to your project:

**Examples:**

Example 1 (kotlin):
```kotlin
implementation("io.kotest:kotest-assertions-android:VERSION")
```

---

## Klock Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/klock-matchers.html

**Contents:**
- Klock Matchers

Matchers for the Klock library, provided by the kotest-assertions-klock module.

---

## Range Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/range-matchers.html

**Contents:**
- Range Matchers

This page describes the rich assertions (matchers) that are available for ClosedRange and OpenEndRange types.

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

If you need to assert that no exception is thrown, then use shouldNotThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## Until | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> { value ->  MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## Range Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/range-matchers.html

**Contents:**
- Range Matchers

This page describes the rich assertions (matchers) that are available for ClosedRange and OpenEndRange types.

---

## JSON | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/json/json-overview.html

**Contents:**
- JSON
- Basic matchers​
- Content-based matching​
- Schema validation​

For more details, see here or follow matcher-specific links below

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the preferred scenario for writing tests, but if you have no choice then Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Assertions | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/assertions.html

**Contents:**
- Assertions
- Multitude of Matchers​
- Clues​
- Inspectors​
- Custom Matchers​

Kotest is split into several subprojects which can be used independently. One of these subprojects is the comprehensive assertion / matchers support. These can be used with the Kotest test framework, or with another test framework like JUnit or Spock.

The core functionality of the assertion modules are functions that test state. Kotest calls these types of state assertion functions matchers. There are core matchers and matchers for third party libraries.

There are also many other utilities for writing tests, such as testing for exceptions, functions to help test non-determistic code, inspectors for collections, and soft assertions to group assertions.

For example, to assert that a variable has an expected value, we can use the shouldBe function.

There are general purpose matchers, such as shouldBe as well as matchers for many other specific scenarios, such as str.shouldHaveLength(10) for testing the length of a string, and file.shouldBeDirectory() which test that a particular file points to a directory. They come in both infix and regular variants.

Assertions can generally be chained, for example:

There are over 350 matchers spread across multiple modules. Read about all the matchers here.

Sometimes a failed assertion does not contain enough information to know exactly what went wrong.

If this failed, you would simply get:

Which isn't particularly helpful. We can add extra context to failure messages through the use of clues.

Inspectors allow us to test elements in a collection, and assert the quantity of elements that should be expected to pass (all, none, exactly k and so on). For example

Read about inspectors here

It is easy to add your own matchers by extending the Matcher<T> interface, where T is the type you wish to match against. Custom matchers can compose existing matchers or be completely standalone.

See a full worked example.

**Examples:**

Example 1 (kotlin):
```kotlin
name shouldBe "sam"
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")           .shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")           .shouldStartWith("https")
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## Konform Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/konform-matchers.html

**Contents:**
- Konform Matchers

Kotest provides various matchers for use with Konform. They can be used in your tests to assert that a given object is validated or fails validation.

To use these matchers add implementation 'io.kotest.extensions:kotest-assertions-konform:<version>' to your build. This module is available for both JVM and JS targets.

Let's start with a basic data class:

Then given a UserProfile validator like this:

We can test that instances pass validation like this:

And we can test that instances fail validation with specific error messages like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class UserProfile(   val fullName: String,   val age: Int?)
```

Example 2 (kotlin):
```kotlin
val validateUser = Validation<UserProfile> {  UserProfile::fullName {     minLength(4)     maxLength(100)  }  UserProfile::age ifPresent {     minimum(21)     maximum(99)  }}
```

Example 3 (kotlin):
```kotlin
val alice = UserProfile("Alice", 25)validateUser shouldBeValid user1
```

Example 4 (kotlin):
```kotlin
val bob = UserProfile("bob", 18)validateUser.shouldBeInvalid(a) {  it.shouldContainError(UserProfile::fullName, "must have at least 4 characters")  it.shouldContainError(UserProfile::age, "must be at least '21'")}
```

---

## Continually | Kotest

**URL:** https://kotest.io/docs/assertions/continually.html

**Contents:**
- Continually
- Listeners​

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

By default, the function passed to the continually block is executed every 25 milliseconds. We can explicitly set the poll interval. In the following example we set it to 50 milliseconds:

If we need to record successful executions of the block(), we can use a listener, as shown in the following example:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {     val config = continuallyConfig<Unit> {        duration = 60.seconds        interval = 50.milliseconds     }      continually(config) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 3 (kotlin):
```kotlin
var invoked = 0val executed = mutableMapOf<Int, Int>()val config = continuallyConfig<Int> {  duration = 500.milliseconds  listener = { index, value -> executed[index] = value }}val result = testContinually(config) {  invoked*2 shouldBe 2*invoked  invoked++ + 42}assertSoftly {  executed.keys.shouldHaveSize(20)  executed.forEach { (k, v) ->     v shouldBe k + 42  }}
```

---

## Inspectors | Kotest

**URL:** https://kotest.io/docs/next/assertions/inspectors.html

**Contents:**
- Inspectors

Inspectors allow us to test elements in a collection. They are extension functions for collections and arrays that test that all, none or some of the elements pass the given assertions. For example, to test that a list of names contains at least two elements which have a length of 7 or more, we can do this:

Similarly, if we wanted to asset that no elements in a collection passed the assertions, we could do something like:

If you want to filter a collection to only elements that pass the assertions, you can use the filterMatching method:

The full list of inspectors are:

**Examples:**

Example 1 (kotlin):
```kotlin
val xs = listOf("sam", "gareth", "timothy", "muhammad")xs.forAtLeast(2) {    it.shouldHaveMinLength(7)}
```

Example 2 (kotlin):
```kotlin
xs.forNone {  it.shouldContain("x")  it.shouldStartWith("bb")}
```

Example 3 (kotlin):
```kotlin
xs.filterMatching {  it.shouldContain("x")  it.shouldStartWith("bb")}.shouldBeEmpty()
```

---

## Composed Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/composed-matchers.html

**Contents:**
- Composed Matchers

Composed matchers can be created for any class or interface by composing one or more other matchers along with the property to extract to test against. This allows us to build up complicated matchers from simpler ones.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we could add the extension variant too:

Then we invoke like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class Person(  val name: String,  val age: Int,  val address: Address,)data class Address(  val city: String,  val street: String,  val buildingNumber: String,)
```

Example 2 (kotlin):
```kotlin
fun nameMatcher(name: String) = Matcher<String> {  MatcherResult(    value == name,    { "Name $value should be $name" },    { "Name $value should not be $name" }  )}fun ageMatcher(age: Int) = Matcher<Int> {  MatcherResult(    value == age,    { "Age $value should be $age" },    { "Age $value should not be $age" }  )}val addressMatcher = Matcher<Address> {  MatcherResult(    value == Address("Warsaw", "Test", "1/1"),    { "Address $value should be Test 1/1 Warsaw" },    { "Address $value should not be Test 1/1 Warsaw" }  )}
```

Example 3 (kotlin):
```kotlin
fun personMatcher(name: String, age: Int) = Matcher.compose(  nameMatcher(name) to Person::name,  ageMatcher(age) to Person::age,  addressMatcher to Person::address)
```

Example 4 (kotlin):
```kotlin
fun Person.shouldBePerson(name: String, age: Int) = this shouldBe personMatcher(name, age)
```

---

## Clues | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/clues.html

**Contents:**
- Clues
- Nested clues​

Clues only work if you are using the Kotest assertions library

A rule of thumb is that a failing test should look like a good bug report. In other words, it should tell you what went wrong, and ideally why it went wrong.

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

In this case, it looks like the system populates the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

For example, we can use withClue with a string message

Would give an error like this:

The error message became much better, however, it is still not as good as it could be. For instance, it might be helpful to know the user's id to check the database.

We can use withClue to add the user's id to the error message:

We can also use the asClue extension function to turn any object into the clue message.

The message will be computed only in case the test fails, so it is safe to use it with expensive operations.

Test failures include a failed assertion, test name, clues, and stacktrace. Consider using them in such a way, so they answer both what has failed, and why it failed. It will make the tests easier to maintain, especially when it comes to reverse-engineering the intention of the test author.

Every time you see a code comment above an assertion, consider using asClue, or withClue instead. The comments are not visible in the test failures, especially on CI, while clues will be visible.

You can use domain objects as clues as well:

Kotest considers all () -> Any? clues as lazy clues, and would compute them and use .toString() on the resulting value instead of calling .toString() on the function itself. In most cases, it should do exactly what you need, however, if clue object implements () -> Any?, and you want using clue.toString(), then consider wrapping the clue manually as { clue.toString() }.asClue { ... }.

Clues can be nested, and they all will be visible in the failed assertion messages:

The failure might look like

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Clues | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/clues.html

**Contents:**
- Clues

Clues only work if you are using the Kotest assertions library or Kotest test framework

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

And you would be able to see that you were populating the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

For example, we can use withClue with a string message

Would give an error like this:

We can also use the asClue extension function to turn any object into the clue message.

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Retry | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/retry.html

**Contents:**
- Retry

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/matchers.html

**Contents:**
- Matchers
  - Kotest Matcher Modules​
  - Community Provided Matchers​

A Matcher is the Kotest term for an assertion that performs a specific test. For example, a matcher may test that a double is greater than zero. Or it it may test that a file is not empty.

Kotest currently has approximately 400 matchers split across several modules. Most of these matchers are for standard library types. Others are project specific. Additionally, there are matchers provided by third party libraries.

Kotest matchers are framework agnostic. You can use them with the Kotest framework, or with any other framework, such as JUnit.

Matchers can be used in two styles:

Both styles are supported. The advantage of the extension function style is that the IDE can autocomplete for you, but some people may prefer the infix style as it is slightly cleaner.

Matchers can be negated by using shouldNot instead of should for the infix style. For example, a shouldNot startWith("boo"). For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

To add your library to this list, please open a PR.

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

The following module is needed: io.kotest.extensions:kotest-assertions-arrow which is versioned independently of the main Kotest project. Search maven central for latest version here.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## Clues | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/clues.html

**Contents:**
- Clues
- Nested clues​

Clues only work if you are using the Kotest assertions library

A rule of thumb is that a failing test should look like a good bug report. In other words, it should tell you what went wrong, and ideally why it went wrong.

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

In this case, it looks like the system populates the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

For example, we can use withClue with a string message

Would give an error like this:

The error message became much better, however, it is still not as good as it could be. For instance, it might be helpful to know the user's id to check the database.

We can use withClue to add the user's id to the error message:

We can also use the asClue extension function to turn any object into the clue message.

The message will be computed only in case the test fails, so it is safe to use it with expensive operations.

Test failures include a failed assertion, test name, clues, and stacktrace. Consider using them in such a way, so they answer both what has failed, and why it failed. It will make the tests easier to maintain, especially when it comes to reverse-engineering the intention of the test author.

Every time you see a code comment above an assertion, consider using asClue, or withClue instead. The comments are not visible in the test failures, especially on CI, while clues will be visible.

You can use domain objects as clues as well:

Kotest considers all () -> Any? clues as lazy clues, and would compute them and use .toString() on the resulting value instead of calling .toString() on the function itself. In most cases, it should do exactly what you need, however, if clue object implements () -> Any?, and you want using clue.toString(), then consider wrapping the clue manually as { clue.toString() }.asClue { ... }.

Clues can be nested, and they all will be visible in the failed assertion messages:

The failure might look like

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Clues | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/clues.html

**Contents:**
- Clues

Clues only work if you are using the Kotest assertions library or Kotest test framework

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

And you would be able to see that you were populating the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

For example, we can use withClue with a string message

Would give an error like this:

We can also use the asClue extension function to turn any object into the clue message.

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-assertions-compiler:$version")
```

Example 2 (kotlin):
```kotlin
class CompilationTest : FreeSpec({    "shouldCompile test" {        val rawStringCodeSnippet = """            val aString: String = "A valid assignment"        """        val syntaxHighlightedSnippet = codeSnippet("""            val aString: String = "A valid assignment"        """)        rawStringCodeSnippet.shouldCompile()        syntaxHighlightedSnippet.shouldCompile()        File("SourceFile.kt").shouldCompile()    }    "shouldNotCompile test" {        val rawStringCodeSnippet = """            val anInteger: Int = "An invalid assignment"        """        val syntaxHighlightedSnippet = codeSnippet("""            val anInteger: Int = "An invalid assignment"        """)        rawStringCodeSnippet.shouldNotCompile()        syntaxHighlightedSnippet.shouldNotCompile()        File("SourceFile.kt").shouldNotCompile()        // check that a compilation error occurred for a specific reason        rawStringCodeSnippet.shouldNotCompile("expected 'Int', actual 'String'")        syntaxHighlightedSnippet.shouldNotCompile("expected 'Int', actual 'String'")        File("SourceFile.kt").shouldNotCompile("expected 'Int', actual 'String'")    }    @OptIn(ExperimentalCompilerApi::class)    "custom assertions on JvmCompilationResult" {        val codeSnippet = codeSnippet("""            fun foo() {                printDate(LocalDate.now())            }        """)        codeSnippet.compile {            exitCode shouldBe ExitCode.COMPILATION_ERROR            messages shouldContain "Unresolved reference 'LocalDate'"            messages shouldContain "Unresolved reference 'printDate'"        }    }    @OptIn(ExperimentalCompilerApi::class)    "custom compiler configuration" {        val compileConfig = CompileConfig {            compilerPluginRegistrars = listOf(MyCompilerPluginRegistrar())        }        val codeSnippet = compileConfig.codeSnippet("""            @MyAnnotation            fun hello() {}        """)        codeSnippet.shouldCompile()    }})
```

---

## Power Assert | Kotest

**URL:** https://kotest.io/docs/next/assertions/power-assert

**Contents:**
- Power Assert
- How It Works​
- Setup​

Power Assert support was introduced in Kotest 6.0 that enhances assertion failure messages by providing detailed information about the values of each part of an expression when an assertion fails. This makes it easier to understand why an assertion failed without having to add additional debug statements.

When an assertion fails, Power Assert shows the values of each part of the expression in the error message, making it clear what went wrong. This is particularly useful for complex expressions with method calls or property access chains.

For example, consider this assertion:

Without Power Assert, the error message would simply be:

With Power Assert enabled, the error message becomes much more informative:

This detailed output shows the values of each part of the expression, making it immediately clear what's happening:

Power Assert is implemented as a Kotlin compiler plugin that's part of Kotlin 2.0+. To use it with Kotest 6.0:

**Examples:**

Example 1 (kotlin):
```kotlin
val hello = "Hello"val world = "world!"hello.substring(1, 3) shouldBe world.substring(1, 4)
```

Example 2 (yaml):
```yaml
expected:<"orl"> but was:<"el">
```

Example 3 (unknown):
```unknown
hello.substring(1, 3) shouldBe world.substring(1, 4)|     |                        |     ||     |                        |     orl|     |                        world!|     elHelloexpected:<"orl"> but was:<"el">
```

Example 4 (kotlin):
```kotlin
plugins {  kotlin("jvm") version "2.2.0"  id("org.jetbrains.kotlin.plugin.power-assert") version "2.2.0"}
```

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

---

## Clues | Kotest

**URL:** https://kotest.io/docs/assertions/clues.html

**Contents:**
- Clues
- Nested clues​

Clues only work if you are using the Kotest assertions library

A rule of thumb is that a failing test should look like a good bug report. In other words, it should tell you what went wrong, and ideally why it went wrong.

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

In this case, it looks like the system populates the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

For example, we can use withClue with a string message

Would give an error like this:

The error message became much better, however, it is still not as good as it could be. For instance, it might be helpful to know the user's id to check the database.

We can use withClue to add the user's id to the error message:

We can also use the asClue extension function to turn any object into the clue message.

The message will be computed only in case the test fails, so it is safe to use it with expensive operations.

Test failures include a failed assertion, test name, clues, and stacktrace. Consider using them in such a way, so they answer both what has failed, and why it failed. It will make the tests easier to maintain, especially when it comes to reverse-engineering the intention of the test author.

Every time you see a code comment above an assertion, consider using asClue, or withClue instead. The comments are not visible in the test failures, especially on CI, while clues will be visible.

You can use domain objects as clues as well:

Kotest considers all () -> Any? clues as lazy clues, and would compute them and use .toString() on the resulting value instead of calling .toString() on the function itself. In most cases, it should do exactly what you need, however, if clue object implements () -> Any?, and you want using clue.toString(), then consider wrapping the clue manually as { clue.toString() }.asClue { ... }.

Clues can be nested, and they all will be visible in the failed assertion messages:

The failure might look like

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Matchers | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/matchers.html

**Contents:**
- Matchers
  - Kotest Matcher Modules​
  - Kotest External Matcher Modules​
  - Community Provided Matchers​

A Matcher is the Kotest term for an assertion that performs a specific test. For example, a matcher may test that a double is greater than zero. Or it it may test that a file is not empty.

Kotest currently has approximately 325 matchers split across several modules. Most of these matchers are for standard library types. Others are project specific. Additionally, there are matchers provided by third party libraries.

Kotest matchers are framework agnostic. You can use them with the Kotest framework, or with any other framework. If you are happy with JUnit, you can still use the powerful matchers provided by the kotest assertion modules.

Matchers can be used in two styles:

Both styles are supported. The advantage of the extension function style is that the IDE can autocomplete for you, but some people may prefer the infix style as it is slightly cleaner.

Matchers can be negated by using shouldNot instead of should for the infix style. For example, a shouldNot startWith("boo"). For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

These modules are hosted in the kotest organization but in separate repositories from the main kotest project. They are released on an independent cadence from the Kotest framework. They provide matchers for third party libraries.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

---

## Until | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## Data Class Matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/data-class-matchers.html

**Contents:**
- Data Class Matchers

Matchers for data classes can be created by composing one or more other matchers along with the property to extract to test against. This allows us to build up complicated matchers from simpler ones.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we could add the extension variant too:

Then we invoke like this:

**Examples:**

Example 1 (kotlin):
```kotlin
data class Person(  val name: String,  val age: Int,  val address: Address,)data class Address(  val city: String,  val street: String,  val buildingNumber: String,)
```

Example 2 (kotlin):
```kotlin
fun nameMatcher(name: String) = Matcher<String> {  MatcherResult(    value == name,    { "Name $value should be $name" },    { "Name $value should not be $name" }  )}fun ageMatcher(age: Int) = Matcher<Int> {  MatcherResult(    value == age,    { "Age $value should be $age" },    { "Age $value should not be $age" }  )}val addressMatcher = Matcher<Address> {  MatcherResult(    value == Address("Warsaw", "Test", "1/1"),    { "Address $value should be Test 1/1 Warsaw" },    { "Address $value should not be Test 1/1 Warsaw" }  )}
```

Example 3 (kotlin):
```kotlin
fun personMatcher(name: String, age: Int) = Matcher.compose(  nameMatcher(name) to Person::name,  ageMatcher(age) to Person::age,  addressMatcher to Person::address)
```

Example 4 (kotlin):
```kotlin
fun Person.shouldBePerson(name: String, age: Int) = this shouldBe personMatcher(name, age)
```

---

## Partial Matches | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/similarity.html

**Contents:**
- Partial Matches

If kotest fails to match a String or an instance of a data class, it may try to find something similar. For instance, in the following example two fields out of three match, so kotest considers sweetGreenApple to be 66.6% similar to sweetRedApple:

By default, kotest will only consider pairs of objects that have more than 50% matching fields. If needed, we can change similarityThresholdInPercent in configuration.

Likewise, if kotest does not detect an exact match, it may try to find a similar String. In the output, the matching part of String is indicated with plus signs:

By default, searching for similar strings is only enabled when both expected and actuals strings' lengthes are between 8 and 1024.

**Examples:**

Example 1 (kotlin):
```kotlin
listOf(sweetGreenApple, sweetGreenPear) shouldContain (sweetRedApple)(snip)PossibleMatches: expected: Fruit(name=apple, color=red, taste=sweet),  but was: Fruit(name=apple, color=green, taste=sweet),  The following fields did not match:    "color" expected: <"red">, but was: <"green">
```

Example 2 (kotlin):
```kotlin
listOf("sweet green apple", "sweet red plum") shouldContain ("sweet green pear")(snip)PossibleMatches:Match[0]: part of slice with indexes [0..11] matched actual[0..11]Line[0] ="sweet green apple"Match[0]= ++++++++++++-----
```

---

## Range Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/range-matchers.html

**Contents:**
- Range Matchers

This page describes the rich assertions (matchers) that are available for ClosedRange and OpenEndRange types.

---

## Power Assert | Kotest

**URL:** https://kotest.io/docs/assertions/power-assert

**Contents:**
- Power Assert
- How It Works​
- Setup​

Power Assert support was introduced in Kotest 6.0 that enhances assertion failure messages by providing detailed information about the values of each part of an expression when an assertion fails. This makes it easier to understand why an assertion failed without having to add additional debug statements.

When an assertion fails, Power Assert shows the values of each part of the expression in the error message, making it clear what went wrong. This is particularly useful for complex expressions with method calls or property access chains.

For example, consider this assertion:

Without Power Assert, the error message would simply be:

With Power Assert enabled, the error message becomes much more informative:

This detailed output shows the values of each part of the expression, making it immediately clear what's happening:

Power Assert is implemented as a Kotlin compiler plugin that's part of Kotlin 2.0+. To use it with Kotest 6.0:

**Examples:**

Example 1 (kotlin):
```kotlin
val hello = "Hello"val world = "world!"hello.substring(1, 3) shouldBe world.substring(1, 4)
```

Example 2 (yaml):
```yaml
expected:<"orl"> but was:<"el">
```

Example 3 (unknown):
```unknown
hello.substring(1, 3) shouldBe world.substring(1, 4)|     |                        |     ||     |                        |     orl|     |                        world!|     elHelloexpected:<"orl"> but was:<"el">
```

Example 4 (kotlin):
```kotlin
plugins {  kotlin("jvm") version "2.2.0"  id("org.jetbrains.kotlin.plugin.power-assert") version "2.2.0"}
```

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/next/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

If you need to assert that no exception is thrown, then use shouldNotThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## Soft Assertions | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/soft-assertions.html

**Contents:**
- Soft Assertions

Normally, assertions like shouldBe throw an exception when they fail. But sometimes you want to perform multiple assertions in a test, and would like to see all of the assertions that failed. Kotest provides the assertSoftly function for this purpose.

If any assertions inside the block failed, the test will continue to run. All failures will be reported in a single exception at the end of the block.

Another version of assertSoftly takes a test target and lambda with test target as its receiver.

We can configure assert softly to be implicitly added to every test via project config.

**Examples:**

Example 1 (kotlin):
```kotlin
assertSoftly {  foo shouldBe bar  foo should contain(baz)}
```

Example 2 (kotlin):
```kotlin
assertSoftly(foo) {    shouldNotEndWith("b")    length shouldBe 3}
```

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

To use this library you need to add io.kotest.extensions:kotest-assertions-arrow to your build.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## Range Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/range-matchers.html

**Contents:**
- Range Matchers

This page describes the rich assertions (matchers) that are available for ClosedRange and OpenEndRange types.

---

## Compiler Matchers | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/compiler-matchers.html

**Contents:**
- Compiler Matchers

The kotest-assertions-compiler extension provides matchers to assert that given kotlin code snippet compiles or not. This extension is a wrapper over kotlin-compile-testing and provides following matchers

To add the compilation matcher, add the following dependency to your project

During checking of code snippet compilation the classpath of calling process is inherited, which means any dependencies which are available in calling process will also be available while compiling the code snippet.

Matchers that verify if a given piece of Kotlin code compiles or not

**Examples:**

Example 1 (bash):
```bash
testImplementation("io.kotest.extensions:kotest-assertions-compiler:${version}")
```

Example 2 (kotlin):
```kotlin
class CompilationTest: StringSpec() {        init {            "shouldCompile test" {                val codeSnippet = """ val aString: String = "A valid assignment" """.trimMargin()                codeSnippet.shouldCompile()                File("SourceFile.kt").shouldCompile()            }            "shouldNotCompile test" {                val codeSnippet = """ val aInteger: Int = "A invalid assignment" """.trimMargin()                codeSnippet.shouldNotCompile()                File("SourceFile.kt").shouldNotCompile()            }        }    }
```

---

## Exceptions | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/exceptions.html

**Contents:**
- Exceptions

To assert that a given block of code throws an exception, one can use the shouldThrow function. Eg,

You can also check the caught exception:

If you want to test that a specific type of exception is thrown, then use shouldThrowExactly<E>. For example, the following block would catch a FileNotFoundException but not a IOException even though FileNotFoundException extends from IOException.

If you simply want to test that any exception is thrown, regardles of type, then use shouldThrowAny.

**Examples:**

Example 1 (kotlin):
```kotlin
shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}
```

Example 2 (kotlin):
```kotlin
val exception = shouldThrow<IllegalAccessException> {  // code in here that you expect to throw an IllegalAccessException}exception.message should startWith("Something went wrong")
```

Example 3 (kotlin):
```kotlin
val exception = shouldThrowExactly<FileNotFoundException> {  // test here}
```

Example 4 (kotlin):
```kotlin
val exception = shouldThrowAny {  // test here can throw any type of Throwable!}
```

---

## Matchers | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/matchers.html

**Contents:**
- Matchers
  - Kotest Matcher Modules​
  - Kotest External Matcher Modules​
  - Community Provided Matchers​

A Matcher is the Kotest term for an assertion that performs a specific test. For example, a matcher may test that a double is greater than zero. Or it it may test that a file is not empty.

Kotest currently has approximately 325 matchers split across several modules. Most of these matchers are for standard library types. Others are project specific. Additionally, there are matchers provided by third party libraries.

Kotest matchers are framework agnostic. You can use them with the Kotest framework, or with any other framework. If you are happy with JUnit, you can still use the powerful matchers provided by the kotest assertion modules.

Matchers can be used in two styles:

Both styles are supported. The advantage of the extension function style is that the IDE can autocomplete for you, but some people may prefer the infix style as it is slightly cleaner.

Matchers can be negated by using shouldNot instead of should for the infix style. For example, a shouldNot startWith("boo"). For the extension function style, each function has an equivalent negated version, for example, a.shouldNotStartWith("boo").

These modules provide the core matcher experience. They are hosted in the main Kotest repo, and are released on the same cadence as the Kotest framework.

These modules are hosted in the kotest organization but in separate repositories from the main kotest project. They are released on an independent cadence from the Kotest framework. They provide matchers for third party libraries.

This is a list of projects that provide Kotest matchers. They are maintained outside of the Kotest organization.

---

## Clues | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/clues.html

**Contents:**
- Clues
- Nested clues​

Clues only work if you are using the Kotest assertions library

A rule of thumb is that a failing test should look like a good bug report. In other words, it should tell you what went wrong, and ideally why it went wrong.

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

In this case, it looks like the system populates the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

For example, we can use withClue with a string message

Would give an error like this:

The error message became much better, however, it is still not as good as it could be. For instance, it might be helpful to know the user's id to check the database.

We can use withClue to add the user's id to the error message:

We can also use the asClue extension function to turn any object into the clue message.

The message will be computed only in case the test fails, so it is safe to use it with expensive operations.

Test failures include a failed assertion, test name, clues, and stacktrace. Consider using them in such a way, so they answer both what has failed, and why it failed. It will make the tests easier to maintain, especially when it comes to reverse-engineering the intention of the test author.

Every time you see a code comment above an assertion, consider using asClue, or withClue instead. The comments are not visible in the test failures, especially on CI, while clues will be visible.

You can use domain objects as clues as well:

Kotest considers all () -> Any? clues as lazy clues, and would compute them and use .toString() on the resulting value instead of calling .toString() on the function itself. In most cases, it should do exactly what you need, however, if clue object implements () -> Any?, and you want using clue.toString(), then consider wrapping the clue manually as { clue.toString() }.asClue { ... }.

Clues can be nested, and they all will be visible in the failed assertion messages:

The failure might look like

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/next/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

Collections: also see inspectors which are useful ways to test multiple elements in a collection.

---

## YAML | Kotest

**URL:** https://kotest.io/docs/assertions/yaml-matchers.html

**Contents:**
- YAML
- Basic matchers​
- Content-based matching​

To use these matchers add testImplementation("io.kotest:kotest-assertions-yaml:<version>") to your build.

---

## Clues | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/clues.html

**Contents:**
- Clues
- Nested clues​

Clues only work if you are using the Kotest assertions library

A rule of thumb is that a failing test should look like a good bug report. In other words, it should tell you what went wrong, and ideally why it went wrong.

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

In this case, it looks like the system populates the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

For example, we can use withClue with a string message

Would give an error like this:

The error message became much better, however, it is still not as good as it could be. For instance, it might be helpful to know the user's id to check the database.

We can use withClue to add the user's id to the error message:

We can also use the asClue extension function to turn any object into the clue message.

The message will be computed only in case the test fails, so it is safe to use it with expensive operations.

Test failures include a failed assertion, test name, clues, and stacktrace. Consider using them in such a way, so they answer both what has failed, and why it failed. It will make the tests easier to maintain, especially when it comes to reverse-engineering the intention of the test author.

Every time you see a code comment above an assertion, consider using asClue, or withClue instead. The comments are not visible in the test failures, especially on CI, while clues will be visible.

You can use domain objects as clues as well:

Kotest considers all () -> Any? clues as lazy clues, and would compute them and use .toString() on the resulting value instead of calling .toString() on the function itself. In most cases, it should do exactly what you need, however, if clue object implements () -> Any?, and you want using clue.toString(), then consider wrapping the clue manually as { clue.toString() }.asClue { ... }.

Clues can be nested, and they all will be visible in the failed assertion messages:

The failure might look like

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Kotlinx Datetime Matchers | Kotest

**URL:** https://kotest.io/docs/5.4.x/assertions/kotlinx-datetime-matchers.html

**Contents:**
- Kotlinx Datetime Matchers

Matchers for the Kotlinx Datetime library are provided by the kotest-assertions-kotlinx-time module.

---

## Assertion Mode | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/assertion-mode.html

**Contents:**
- Assertion Mode

If you are using Kotest framework alongside Kotest assertions, you can ask Kotest to fail the build, or output a warning to stderr, if a test is executed that does not execute an assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside a spec. For example.

Running this test will output something like:

If we want to set this globally, we can do so in project config or via the system property kotest.framework.assertion.mode.

Assertion mode only works for Kotest assertions and not other assertion libraries.

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {   init {      assertions = AssertionMode.Error      test("this test has no assertions") {         val name = "sam"         name.length == 3 // this isn't actually testing anything      }   }}
```

Example 2 (unknown):
```unknown
Test 'this test has no assertions' did not invoke any assertions
```

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Custom Matchers | Kotest

**URL:** https://kotest.io/docs/5.6.x/assertions/custom-matchers.html

**Contents:**
- Custom Matchers
- Extension Variants​

It is easy to define your own matchers in Kotest.

Simply extend the Matcher<T> interface, where T is the type you wish to match against. The Matcher interface specifies one method, test which returns an instance of MatcherResult.

This MatcherResult type defines three methods - a boolean to indicate if the test passed or failed, and two failure messages.

The first failure message is the message to the user if the matcher predicate failed. Usually you can include some details of the expected value and the actual value and how they differed. The second failure message is the message to the user if the matcher predicate evaluated true in negated mode. Here you usually indicate that you expected the predicate to fail.

The difference in those two messages will be clearer with an example. Let's consider writing a length matcher for strings, to assert that a string has a required length. We will want our syntax to be something like str.shouldHaveLength(8).

Then the first message should be something like "string had length 15 but we expected length 8". The second message would need to be something like "string should not have length 8"

First we build out our matcher type:

Notice that we wrap the error messages in a function call so we don't evaluate if not needed. This is important for error messages that take some time to generate.

This matcher can then be passed to the should and shouldNot infix functions as follows:

Usually, we want to define extension functions which invoke the matcher function for you and return the original value for chaining. This is how Kotest structures the built in matchers, and Kotest adopts a shouldXYZ naming strategy. For example:

Then we can invoke these like:

**Examples:**

Example 1 (kotlin):
```kotlin
interface Matcher<in T> {  fun test(value: T): MatcherResult}
```

Example 2 (kotlin):
```kotlin
interface MatcherResult {  fun passed(): Boolean  fun failureMessage(): String  fun negatedFailureMessage(): String}
```

Example 3 (kotlin):
```kotlin
fun haveLength(length: Int) = Matcher<String> { value ->  MatcherResult(    value.length == length,    { "string had length ${value.length} but we expected length $length" },    { "string should not have length $length" },  )}
```

Example 4 (kotlin):
```kotlin
"hello foo" should haveLength(9)"hello bar" shouldNot haveLength(3)
```

---

## Jsoup Matchers | Kotest

**URL:** https://kotest.io/docs/6.0/assertions/jsoup-matchers.html

**Contents:**
- Jsoup Matchers

This page lists all current matchers in the KotlinTest jsoup matchers extension library. To use this library you need to add kotlintest-assertions-jsoup to your build.

---

## Until | Kotest

**URL:** https://kotest.io/docs/5.2.x/assertions/until.html

**Contents:**
- Until
  - Duration​
  - Interval​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, you might want to test that a message has been received by a broker. You could setup a time limit, and repeatedly poll until the message was received, but this would block the thread. Plus you would have to write the loop code, adding boilerplate.

As an alternative, kotest provides the until function which will periodically execute a function until either that function returns true, or the given duration expires.

Until is the predicate equivalent of eventually.

Let's say we have a function that polls a broker, and returns a list of messages. We want to test that when we send a message the message is picked up by the broker within 5 seconds.

By default, the predicate is checked every second. We can specify an interval which controls the delay between invocations. Here is the same example again, this time with a more aggressive fixed 250 millisecond interval.

We can also specify a fibonacci interval, if we want to increase the delay after each failure.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds) {        broker.poll().size > 0      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 250.milliseconds.fixed()) {        broker.poll().size > 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  private val broker = createBrokerClient()  init {    should("broker should receive a message") {      sendMessage()      until(5.seconds, 100.milliseconds.fibonacci()) {        broker.poll().size > 0      }    }  }}
```

---

## Collection Matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/collection-matchers.html

**Contents:**
- Collection Matchers

This page describes the rich assertions (matchers) that are available for Collection, Iterable and Array types.

Also, see inspectors which are useful ways to test multiple elements in a collection.

---

## Retry | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/retry.html

**Contents:**
- Retry

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## Continually | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/continually.html

**Contents:**
- Continually

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

The function passed to the continually block is executed every 10 milliseconds. We can specify the poll interval if we prefer:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds, 5.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

---

## Arrow | Kotest

**URL:** https://kotest.io/docs/5.9.x/assertions/arrow.html

**Contents:**
- Arrow

This page lists all current matchers in the Kotest arrow matchers extension library.

The following module is needed: io.kotest.extensions:kotest-assertions-arrow which is versioned independently of the main Kotest project. Search maven central for latest version here.

In the case io.arrow-kt:arrow-core:arrow-version is not in your classpath, please add it. To prevent Unresolved Reference errors.

---

## JSON | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/json/json-overview.html

**Contents:**
- JSON
- Basic matchers​
- Content-based matching​
- Schema validation​

For more details, see here or follow matcher-specific links below

---

## Non-deterministic Testing | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/non-deterministic-testing.html

**Contents:**
- Non-deterministic Testing

Sometimes you have to work with code that is non-deterministic in nature. This is not the preferred scenario for writing tests, but if you have no choice then Kotest provides several functions that help writing tests where the happy path can take a variable amount of time to pass successfully.

---

## Composed Matchers | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/composed-matchers.html

**Contents:**
- Composed Matchers

Composed matchers can be created for any type by composing one or more matchers. This allows to build up complex matchers from simpler ones. There are two logical operations, using which we can compose matchers: logical sum (Matcher.any) and logical product (Matcher.all).

Let's say we'd like to define a password Matcher, which will containADigit(), contain(Regex("[a-z]")) and contain(Regex("[A-Z]")). We can compose these matchers this way:

We can add extension function then:

So it can be invoked like this:

By analogy, we can build a composed matcher using Matcher.any. In this case, passwordMatcher will fail only if all matchers fail, otherwise it will pass.

Composed matchers can also be created for any class or interface by composing one or more other matchers along with the property to extract to test against.

For example, say we had the following structures:

And our goal is to have a Person matcher that checks for people in Warsaw. We can define matchers for each of those components like this:

Now we can simply combine these together to make a John in Warsaw matcher. Notice that we specify the property to extract to pass to each matcher in turn.

And we can add the extension variant too:

Then we invoke it this way:

**Examples:**

Example 1 (kotlin):
```kotlin
val passwordMatcher = Matcher.all(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

Example 2 (kotlin):
```kotlin
fun String.shouldBeStrongPassword() = this shouldBe passwordMatcher
```

Example 3 (kotlin):
```kotlin
"StrongPassword123".shouldBeStrongPassword()"WeakPassword".shouldBeStrongPassword() // would fail
```

Example 4 (kotlin):
```kotlin
val passwordMatcher = Matcher.any(   containADigit(), contain(Regex("[a-z]")), contain(Regex("[A-Z]")))
```

---

## Ktor Matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/ktor-matchers.html

**Contents:**
- Ktor Matchers
  - Test Application Response​
  - HttpResponse​

Code is kept on a separate repository and on a different group: io.kotest.extensions.

implementation("io.kotest.extensions:kotest-assertions-ktor:version")

implementation "io.kotest.extensions:kotest-assertions-ktor:version"

Matchers for Ktor are provided by the kotest-assertions-ktor module.

The following matchers are used when testing via the ktor server testkit.

The following matchers can be used against responses from the ktor http client.

---

## Continually | Kotest

**URL:** https://kotest.io/docs/5.8.x/assertions/continually.html

**Contents:**
- Continually

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

The function passed to the continually block is executed every 10 milliseconds. We can specify the poll interval if we prefer:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds, 5.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

Collections: also see inspectors which are useful ways to test multiple elements in a collection.

---

## Clues | Kotest

**URL:** https://kotest.io/docs/5.5.x/assertions/clues.html

**Contents:**
- Clues
- Nested clues​

Clues only work if you are using the Kotest assertions library or Kotest test framework

A rule of thumb is that a failing test should look like a good bug report. In other words, it should tell you what went wrong, and ideally why it went wrong.

Sometimes a failed assertion contains enough information in the error message to know what went wrong.

Might give an error like:

In this case, it looks like the system populates the username field with an email address.

But let's say you had a test like this:

If this failed, you would simply get:

Which isn't particularly helpful. This is where withClue comes into play.

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

For example, we can use withClue with a string message

Would give an error like this:

The error message became much better, however, it is still not as good as it could be. For instance, it might be helpful to know the user's id to check the database.

We can use withClue to add the user's id to the error message:

We can also use the asClue extension function to turn any object into the clue message.

The message will be computed only in case the test fails, so it is safe to use it with expensive operations.

Test failures include a failed assertion, test name, clues, and stacktrace. Consider using them in such a way, so they answer both what has failed, and why it failed. It will make the tests easier to maintain, especially when it comes to reverse-engineering the intention of the test author.

Every time you see a code comment above an assertion, consider using asClue, or withClue instead. The comments are not visible in the test failures, especially on CI, while clues will be visible.

You can use domain objects as clues as well:

Kotest considers all () -> Any? clues as lazy clues, and would compute them and use .toString() on the resulting value instead of calling .toString() on the function itself. In most cases, it should do exactly what you need, however, if clue object implements () -> Any?, and you want using clue.toString(), then consider wrapping the clue manually as { clue.toString() }.asClue { ... }.

Clues can be nested, and they all will be visible in the failed assertion messages:

The failure might look like

**Examples:**

Example 1 (kotlin):
```kotlin
username shouldBe "sksamuel"
```

Example 2 (yaml):
```yaml
expected: "sksamuel" but was: "sam@myemailaddress.com"
```

Example 3 (kotlin):
```kotlin
user.name shouldNotBe null
```

Example 4 (typescript):
```typescript
<null> should not equal <null>
```

---

## Core Matchers | Kotest

**URL:** https://kotest.io/docs/5.7.x/assertions/core-matchers.html

**Contents:**
- Core Matchers

Matchers provided by the kotest-assertions-core module.

---

## JSON | Kotest

**URL:** https://kotest.io/docs/assertions/json/json-overview.html

**Contents:**
- JSON
- Basic matchers​
- Content-based matching​
- Schema validation​

To use these matchers add testImplementation("io.kotest:kotest-assertions-json:<version>") to your build.

There exist copies of all matchers that validate a File or a Path instead of a String for the JVM platform.

For more details, see here or follow matcher-specific links below

---

## Retry | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/retry.html

**Contents:**
- Retry
- Retry ​

Retry is similar to eventually, but rather than attempt a block of code for a period of time, it attempts a block of code a maximum number of times. We still provide a timeout period to avoid the loop running for ever.

Additional options include the delay between runs, a multiplier to use exponential delays, and an exception class if we only want to repeat for certain exceptions and fail for others.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("retry up to 4 times") {      retry(4, 10.minutes) {      }    }  }}
```

---

## Continually | Kotest

**URL:** https://kotest.io/docs/next/assertions/continually.html

**Contents:**
- Continually
- Listeners​

As the dual of eventually, continually allows you to assert that a block of code succeeds, and continues to succeed, for a period of time. For example you may want to check that a http connection is kept alive for 60 seconds after the last packet has been received. You could sleep for 60 seconds, and then check, but if the connection was terminated after 5 seconds, your test will sit idle for a further 55 seconds before then failing. Better to fail fast.

By default, the function passed to the continually block is executed every 25 milliseconds. We can explicitly set the poll interval. In the following example we set it to 50 milliseconds:

If we need to record successful executions of the block(), we can use a listener, as shown in the following example:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("pass for 60 seconds") {      continually(60.seconds) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests: ShouldSpec() {  init {    should("pass for 60 seconds") {     val config = continuallyConfig<Unit> {        duration = 60.seconds        interval = 50.milliseconds     }      continually(config) {        // code here that should succeed and continue to succeed for 60 seconds      }    }  }}
```

Example 3 (kotlin):
```kotlin
var invoked = 0val executed = mutableMapOf<Int, Int>()val config = continuallyConfig<Int> {  duration = 500.milliseconds  listener = { index, value -> executed[index] = value }}val result = testContinually(config) {  invoked*2 shouldBe 2*invoked  invoked++ + 42}assertSoftly {  executed.keys.shouldHaveSize(20)  executed.forEach { (k, v) ->     v shouldBe k + 42  }}
```

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.3.x/assertions/eventually.html

**Contents:**
- Eventually
  - Examples​
    - Simple examples​
    - Exceptions​
    - Predicates​
    - Sharing configuration​

When testing non-deterministic code, a common use case is "I expect this code to pass after a short period of time".

For example, if you were testing a IO operation, you might need to wait until the IO operation has flushed.

Sometimes you can do a Thread.sleep but this is isn't ideal as you need to set a sleep threshold high enough so that it won't expire prematurely on a slow machine. Plus it means that your test will sit around waiting on the timeout even if the code completes quickly on a fast machine.

Or you can roll a loop and sleep and retry and sleep and retry, but this is just boilerplate slowing you down.

Another common approach is to use countdown latches and this works fine if you are able to inject the latches in the appropriate places but it isn't always possible to have the code under test trigger a latch.

As an alternative, kotest provides the eventually function and the Eventually configuration which periodically test the code ignoring your specified exceptions and ensuring the result satisfies an optional predicate, until the timeout is eventually reached or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code.

Let's assume that we send a message to an asynchronous service. After the message is processed, a new row is inserted into user table.

We can check this behaviour with our eventually function.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test.

Let's assume that our example from before throws a UserNotFoundException while the user is not found in the database. It will eventually return the user when the message is processed by the system.

In this scenario, we can explicitly skip the exception that we expect to happen until the test passed, but any other exceptions would not be ignored. Note, this example is similar to the former, but if there was some other error, say a ConnectionException for example, this would cause the eventually block to immediately exit with a failure message.

In addition to verifying a test case eventually runs without throwing, we can also verify the result and treat a non-throwing result as failing.

Sharing the configuration for eventually is a breeze with the Eventually data class. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

Here we can see sharing of configuration can be useful to reduce duplicate code while allowing flexibility for things like custom logging per test suite for clear test logs.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("check if user repository has one row after message is sent") {      sendMessage()      eventually(5.seconds) {        userRepository.size() shouldBe 1      }    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("check if user repository has one row") {      eventually(5.seconds, UserNotFoundException::class.java) {        userRepository.findBy(1) shouldNotBe null      }    }  }}
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({  "check that predicate eventually succeeds in time" {    var i = 0    eventually<Int>(25.seconds, predicate = { it == 5 }) {      delay(1.seconds)      i++    }  }})
```

Example 4 (kotlin):
```kotlin
val slow = EventuallyConfig<ServerResponse, ServerException>(5.minutes, interval = 25.milliseconds.fibonacci(), exceptionClass = ServerException::class)val fast = slow.copy(duration = 5.seconds)class FooTests : StringSpec({  val logger = logger("FooTests")  val fSlow = slow.copy(listener = { i, t -> logger.info("Current $i after {${t.times} attempts")})  "server eventually provides a result for /foo" {    eventually(fSlow) {      fooApi()    }  }})class BarTests : StringSpec({  val logger = logger("BarTests")  val bFast = fast.copy(listener = { i, t -> logger.info("Current $i after {${t.times} attempts")})  "server eventually provides a result for /bar" {    eventually(bFast) {      barApi()    }  }})
```

---
