# Kotest - Framework

**Pages:** 254

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/concurrency/eventually.html

**Contents:**
- Eventually
- API​
- Configuration​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Predicates​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 4.6, a new experimental module has been added which contains improved utilities for testing concurrent, asynchronous, or non-deterministic code. This module is kotest-framework-concurrency and is intended as a long term replacement for the previous module. The previous utilities are still available as part of the core framework.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine, and even when complete, the thread will stay blocked until the timeout has expired.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-determistic operation. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually utility which solves the common use case of "I expect this code to pass after a short period of time".

Eventually does this by periodically invoking a given lambda until the timeout is eventually reached or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized in regardless to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration in either milliseconds (or using the Kotlin Duration type) followed by the code that should eventually pass without an exception being raised.

The second is by providing a configuration block before the test code. This method should be used when you need to set more options than just the duration.

The duration is the total amount of time to keep trying to pass the test. The interval however allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be handled, or false if the exception should bubble out.

In addition to verifying a test case eventually runs without throwing an exception, we can also verify that the return value of the test is as expected - and if not, consider that iteration a failure and try again.

For example, here we continue to append "x" to a string until the result of the previous iteration is equal to "xxx".

We can attach a listener, which will be invoked on each iteration, with the state of that iteration. The state object contains the last exception, last value, iteration count and so on.

Sharing the configuration for eventually is a breeze with the EventuallyConfig data class. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

Here we can see sharing of configuration can be useful to reduce duplicate code while allowing flexibility for things like custom logging per test suite for clear test logs.

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5000) { // duration in millis  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
eventually({  duration = 5000  interval = 1000.fixed()}) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
eventually({  duration = 5000  initialDelay = 1000}) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 4 (kotlin):
```kotlin
eventually({  duration = 8000  retries = 10  suppressExceptions = setOf(UserNotFoundException::class)}) {  userRepository.getById(1).name shouldNotBe "bob"}
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class StringSpecExample : StringSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable coroutineTestScope in test config:

Inside this test, can you retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting coroutineTestScope to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(coroutineTestScope = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(coroutineTestScope = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      coroutineTestScope = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var coroutineTestScope = true}
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/6.0/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 8 different styles of test layout. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks. context can also be used as an alias for describe.

Tests can be disabled using the xcontext, xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use context, given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Context, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xcontext, xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec({    should("return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : ShouldSpec({    context("String.length") {        should("return the length of the string") {            "sammy".length shouldBe 5            "".length shouldBe 0        }    }})
```

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/concurrency/eventually.html

**Contents:**
- Eventually
- API​
- Configuration​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Predicates​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 4.6, a new experimental module has been added which contains improved utilities for testing concurrent, asynchronous, or non-deterministic code. This module is kotest-framework-concurrency and is intended as a long term replacement for the previous module. The previous utilities are still available as part of the core framework.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine, and even when complete, the thread will stay blocked until the timeout has expired.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-determistic operation. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually utility which solves the common use case of "I expect this code to pass after a short period of time".

Eventually does this by periodically invoking a given lambda until the timeout is eventually reached or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized in regardless to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration in either milliseconds (or using the Kotlin Duration type) followed by the code that should eventually pass without an exception being raised.

The second is by providing a configuration block before the test code. This method should be used when you need to set more options than just the duration.

The duration is the total amount of time to keep trying to pass the test. The interval however allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be handled, or false if the exception should bubble out.

In addition to verifying a test case eventually runs without throwing an exception, we can also verify that the return value of the test is as expected - and if not, consider that iteration a failure and try again.

For example, here we continue to append "x" to a string until the result of the previous iteration is equal to "xxx".

We can attach a listener, which will be invoked on each iteration, with the state of that iteration. The state object contains the last exception, last value, iteration count and so on.

Sharing the configuration for eventually is a breeze with the EventuallyConfig data class. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

Here we can see sharing of configuration can be useful to reduce duplicate code while allowing flexibility for things like custom logging per test suite for clear test logs.

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5000) { // duration in millis  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
eventually({  duration = 5000  interval = 1000.fixed()}) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
eventually({  duration = 5000  initialDelay = 1000}) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 4 (kotlin):
```kotlin
eventually({  duration = 8000  retries = 10  suppressExceptions = setOf(UserNotFoundException::class)}) {  userRepository.getById(1).name shouldNotBe "bob"}
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Marking Tests​
- Running with Tags​
- Tag Expression Operators​
- Tagging All Tests​
- Tagging a Spec​
  - Inheriting tags​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

Test cases can then be marked with tags using the config function:

Then by invoking the test runner with a system property of kotest.tags you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Please pay attention to the use of upper case and lower case! If two tag objects have the same simple name (in different name spaces) they are treated as the same tag.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle like this:

Tags can also be included/excluded in runtime (for example, if you're running a project configuration instead of properties) through the RuntimeTagExtension:

Operators (in descending order of precedence)

You can add a tag to all tests in a spec using the tags function in the spec itself. For example:

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

There are two annotations you can add to a spec class itself - @Tags and @RequiresTag - which accept one or more tag names as their arguments.

The first tag - @Tags - will be applied to all tests in the class, however this will only stop a spec from being instantiated if we can guarantee that no tests would be executed (because a tag is being explicitly excluded).

Consider the following example:

The second tag - @RequiresTag - only checks that all the referenced tags are present and if not, will skip the spec.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use these annotations you pass the tag string name, not the tag itself. This is due to Kotlin annotations only allow "primitive" arguments

By default, the @Tags annotation will only be considered on the immediate Spec which it was applied to. However, a Spec can also inherit tags from superclasses and superinterfaces. To enable this, toggle tagInheritance = true in your project config

Special attention is needed in your gradle configuration

To use System Properties (-Dx=y), your gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows: Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
import io.kotest.specs.StringSpecclass MyTest : StringSpec() {  init {    "should run on Windows".config(tags = setOf(Windows)) {      // ...    }    "should run on Linux".config(tags = setOf(Linux)) {      // ...    }    "should run on Windows and Linux".config(tags = setOf(Windows, Linux)) {      // ...    }  }}
```

Example 4 (unknown):
```unknown
gradle test -Dkotest.tags="Linux & !Database"
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/6.0/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Marking Tests​
- Running with Tags​
- Tag Expression Operators​
- Tagging All Tests​
- Tagging a Spec​
  - Inheriting tags​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

Test cases can then be marked with tags using the config function:

Then by invoking the test runner with a system property of kotest.tags you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Please pay attention to the use of upper case and lower case! If two tag objects have the same simple name (in different name spaces) they are treated as the same tag.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle like this:

Tags can also be included/excluded in runtime (for example, if you're running a project configuration instead of properties) through the RuntimeTagExtension:

Operators (in descending order of precedence)

You can add a tag to all tests in a spec using the tags function in the spec itself. For example:

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

There are two annotations you can add to a spec class itself - @Tags and @RequiresTag - which accept one or more tag names as their arguments.

The first tag - @Tags - will be applied to all tests in the class, however this will only stop a spec from being instantiated if we can guarantee that no tests would be executed (because a tag is being explicitly excluded).

Consider the following example:

The second tag - @RequiresTag - only checks that all the referenced tags are present and if not, will skip the spec.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use these annotations you pass the tag string name, not the tag itself. This is due to Kotlin annotations only allow "primitive" arguments

By default, the @Tags annotation will only be considered on the immediate Spec which it was applied to. However, a Spec can also inherit tags from superclasses and superinterfaces. To enable this, toggle tagInheritance = true in your project config

Special attention is needed in your gradle configuration

To use System Properties (-Dx=y), your gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows: Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
class MyTest : FreeSpec() {  init {    "should run on Windows".config(tags = setOf(Windows)) {      // ...    }    "should run on Linux".config(tags = setOf(Linux)) {      // ...    }    "should run on Windows and Linux".config(tags = setOf(Windows, Linux)) {      // ...    }  }}
```

Example 4 (unknown):
```unknown
gradle test -Dkotest.tags="Linux & !Database"
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a default TestCaseConfig for all test cases of a Spec:

Overriding the defaultTestCaseConfig function:

Or via assignment to the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10, threads = 2) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec : StringSpec() {  override fun defaultTestCaseConfig() = TestCaseConfig(invocations = 3)  init {    // your test cases ...  }}
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/next/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listeners for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
    - DSL Methods​
    - DSL methods with functions​
    - Overriding callback functions in a Spec​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. For more advanced cases, such as writing distributable plugins or re-usable hooks, one can use extensions.

At the end of this section is a list of the available hooks and when they are executed.

There are several ways to use hooks in Kotest:

The first and simplest, is to use the DSL methods available inside a Spec which create and register a TestListener for you. For example, we can invoke beforeTest or afterTest (and others) directly alongside our tests.

Behind the scenes, these DSL methods will create an instance of TestListener, overriding the appropriate functions, and ensuring that this test listener is registered to run.

You can use afterProject as a DSL method which will create an instance of ProjectListener, but there is no beforeProject because by the time the framework is at this stage of detecting a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The BeforeTest type used on the function definition is an alias to suspend (TestCase) -> Unit to keep things simple. There are aliases for the types of each of the callbacks.

The second, related, method is to override the callback functions in the Spec. This is essentially just a variation on the first method.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : WordSpec({  beforeTest {    println("Starting a test $it")  }  afterTest { (test, result) ->    println("Finished spec with result $result")  }  "this test" should {    "be alive" {      println("Johnny5 is alive!")    }  }})
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : WordSpec({   // used once   beforeTest(startTest)   "this test" should {      "be alive" {         println("Johnny5 is alive!")      }   }})class OtherSpec : WordSpec({   // used twice   beforeTest(startTest)   "this test" should {      "fail" {         fail("boom")      }   }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : WordSpec() {    override fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" should {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/next/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​
  - Random Seed​
  - Custom Ordering​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is usually fine as the order is perhaps not important to most test suites, but if you require control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from the JVM classpath or the order they appear in JavaScript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with the lowest values executed first. Any specs without such an annotation are considered "last" (Max integer). This option only works on the JVM. Specs with the same order value are executed in the order they are discovered.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and BazTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, WazTest will execute last, as it has no annotation.

When using the Random spec execution order, you can set a seed to ensure that the same order is always used if required.

You can also order specs yourself by implementing the SpecExecutionOrderExtension interface and registering it with the project config. If such an extension is registered, the specExecutionOrder property will be ignored and the extension will be used instead.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() {}@Order(0)class BarTest : FunSpec() {}@Order(1)class BazTest : FunSpec() {}class WazTest : FunSpec() {}
```

Example 3 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override val randomOrderSeed = ...}
```

Example 4 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override val extensions = listOf(MySpecExecutionOrderExtension())}
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is often sufficient, but if we need control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from JVM classpath discovery, or the order they appear in javascript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are explicitly executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with lowest values executed first. Any specs without such an annotation are considered "last". This option only works on the JVM. Any ties will be broken arbitrarily.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and FarTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, BooTest will execute last, as it has no annotation.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig: AbstractProjectConfig() {    override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() { }@Order(0)class BarTest: FunSpec() {}@Order(1)class FarTest : FunSpec() { }class BooTest : FunSpec() {}
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/6.0/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration or the system property kotest.framework.testname.display.full.path.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, and a timeout for each test or for all tests. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : StringSpec({   "length should return size of string" {      "hello".length shouldBe 5   }   "startsWith should test for a prefix" {      "world" should startWith("wor")   }})
```

Example 2 (kotlin):
```kotlin
class StringSpecExample : StringSpec({   "maximum of two numbers" {      forAll(         row(1, 5, 5),         row(1, 0, 1),         row(0, 0, 0)      ) { a, b, max ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : StringSpec({   "should use config".config(timeout = 2.seconds, invocations = 10, threads = 2, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Fake Functions | Kotest

**URL:** https://kotest.io/docs/next/framework/fakery.html

**Contents:**
- Fake Functions

In functional programming, our dependencies are less likely to be instances of concrete classes and more likely to be functions. Whenever we are unit testing something with functional dependencies, it's usually easier to just pass another function rather than mock that dependency. Consider, for example, the following implementation:

Traditionally, we would mock HasAnswer and pass that mock to MyService:

However, we can also just pass a lambda, which is so very much simpler:

If we want this test-double function to return different values and/or throw exceptions, kotest has simple helper functions which make these tasks easier, such as:

This fake function can be used in unit tests as follows:

Should we need a fake function that sometimes returns a value and sometimes throws an exception,it can easily be done as follows:

As this function implements HasAnswer interface, we can use it as a dependency in our unit tests as well.

**Examples:**

Example 1 (kotlin):
```kotlin
fun interface HasAnswer {   fun answer(question: String): Int}class AnsweringService: HasAnswer {   override fun answer(question: String): Int { TODO() }}class MyService(private val hasAnswer: HasAnswer) {   fun respond(question: String): Int = hasAnswer.answer(question)}
```

Example 2 (kotlin):
```kotlin
val mockHasAnswer = run {  val ret = mockk<HasAnswer>()  every { ret.answer(any()) } returns 42  ret}val myService = MyService(mockHasAnswer)// tests here
```

Example 3 (kotlin):
```kotlin
val myService = MyService(hasAnswer = { 42 })// tests to follow
```

Example 4 (kotlin):
```kotlin
val fakeFunction = sequenceOf("yes", "no", "maybe").toFunction() fakeFunction.next() shouldBe "yes" fakeFunction.next() shouldBe "no" fakeFunction.next() shouldBe "maybe"
```

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/6.0/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerRoot​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - Config​
  - System Property​

The isolation mode InstancePerRoot is only available in Kotest 6.0 and later, and InstancePerTest and InstancePerLeaf are now deprecated due to undefined behavior in edge cases.

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are four values: SingleInstance, InstancePerRoot, InstancePerLeaf, and InstancePerTest. Note that InstancePerLeaf and InstancePerTest are deprecated in favor of InstancePerRoot.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed four times as the same instance is used for all tests.

The InstancePerRoot isolation mode creates a new instance of the Spec class for every top level (root) test case. Each root test is executed in its own associated instance.

This mode is recommended when you want to isolate your tests but still maintain a clean structure.

In this example, the tests a, b and c will all print the same UUID, but test d will print a different UUID because it is executed in a new instance as it is a top level (aka root) test case.

This mode is deprecated due to undefined behavior on edge cases. It is recommended to use InstancePerRoot instead.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

This mode is deprecated due to undefined behavior on edge cases. It is recommended to use InstancePerRoot instead.

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({  isolationMode = IsolationMode.SingleInstance  // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({  val id = UUID.randomUUID()  "a" should {    println(id)    "b" {      println(id)    }    "c" {      println(id)    }  }  "d" should {    println(id)  }})
```

Example 4 (kotlin):
```kotlin
class InstancePerRootExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerRoot  val id = UUID.randomUUID()  init {    "a" should {      println(id)      "b" {        println(id)      }      "c" {        println(id)      }    }    "d" should {      println(id)    }  }}
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Conditional tests with X Methods | Kotest

**URL:** https://kotest.io/docs/framework/conditional/conditional-tests-with-x-methods.html

**Contents:**
- Conditional tests with X Methods

An idea that is popular with Javascript testing frameworks is to allow the test keywords to be prefixed with x to disable those tests, or to be prefixed with f to focus only those tests.

This is similar to using the bang or focus characters in the test name.

Using DescribeSpec as an example, we can replace describe with xdescribe to disable that test:

Similarly, we could add the prefix to a nested test by replacing it with xit:

And if we wanted to focus to one or more tests, we can replace describe with fdescribe or it with fit:

The focus flag does not work if placed on nested tests due to the fact that nested tests are only discovered once the parent test has executed. So there would be no way for the test engine to know that a nested test has the f prefix without first executing all the parents.

If you just want to run a single test, you can of course just run that from intelliJ directly using the green arrow. However sometimes you want to run a subset of tests, or you want to run all tests except a few. This is when focus and disabling can be useful.

See which specs support this, and the syntax required on the specs styles guide.

**Examples:**

Example 1 (kotlin):
```kotlin
class XMethodsExample : DescribeSpec({  xdescribe("this block and it's children are now disabled") {    it("will not run") {      // disabled test    }  }})
```

Example 2 (kotlin):
```kotlin
class XMethodsExample : DescribeSpec({  describe("this block is enabled") {    xit("will not run") {      // disabled test    }    it("will run") {      // enabled test    }  }})
```

Example 3 (kotlin):
```kotlin
class XMethodsExample : DescribeSpec({  fdescribe("this block is focused") {    // tests  }  describe("this block will not run because it is not focused") {    // tests  }})
```

---

## Conditional tests with Gradle | Kotest

**URL:** https://kotest.io/docs/framework/conditional/conditional-tests-with-gradle.html

**Contents:**
- Conditional tests with Gradle
  - Gradle Test Filtering​
  - Kotest Specific Test Filtering​

Kotest supports multiple ways to filter tests from the command line using Gradle.

When running Kotest via the JUnit Platform runner through Gradle, Kotest supports the standard Gradle syntax for test filtering. You can enable filtering either in the build script or via the --tests command-line option.

For example, in the build script:

Or via the command line:

gradle test --tests 'com.sksamuel.some.package.*'

gradle test --tests '*IntegrationTest'

gradle test --tests 'com.sksamuel.some.package.MyTestClass.some test name'

See full Gradle documentation here.

Because Gradle's test support is class.method based, when filtering to individual tests, we can specify nested tests by using the -- delimiter between test names. For example, --tests 'com.mypackage.MySpec.test -- nested test'. Note the delimiter has a space around the double dashes and remember to escape the test path with single quotes.

For multiplatform testing, Kotest offers its own flag which is provided via an environment variable. This flag support wildcards via * and matches either tests or specs using the same syntax as the Gradle format.

This example would execute all tests in the com.somepackage (and nested) packages by setting the KOTEST_INCLUDE_PATTERN environment variable:

KOTEST_INCLUDE_PATTERN='com.somepackage.*' gradle test

It's best to enclose the value in single quotes rather than double quotes to avoid your shell performing globbing on any * characters.

**Examples:**

Example 1 (go):
```go
tasks.test {  filter {    //include all tests from package    includeTestsMatching("com.somepackage.*")  }}
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 8 different styles for test definitions. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks. context can also be used as an alias for describe.

Tests can be disabled using the xcontext, xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use context, given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Context, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xcontext, xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec({    should("return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : ShouldSpec({    context("String.length") {        should("return the length of the string") {            "sammy".length shouldBe 5            "".length shouldBe 0        }    }})
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration or the system property kotest.framework.testname.display.full.path.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- String Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 10 different styles of test layout. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

StringSpec reduces the syntax to the absolute minimum. Just write a string followed by a lambda expression with your test code.

Adding config to the test.

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks.

Tests can be disabled using the xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

If you are migrating from JUnit then AnnotationSpec is a spec that uses annotations like JUnit 4/5. Just add the @Test annotation to any function defined in the spec class.

You can also add annotations to execute something before tests/specs and after tests/specs, similarly to JUnit's

If you want to ignore a test, use @Ignore.

Although this spec doesn't offer much advantage over using JUnit, it allows you to migrate existing tests relatively easily, as you typically just need to adjust imports.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string" {        "hello".length shouldBe 5    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string".config(enabled = false, invocations = 3) {        "hello".length shouldBe 5    }})
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/next/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features, except for fakery which allows to build test doubles. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Setup | Kotest

**URL:** https://kotest.io/docs/framework/project-setup.html

**Contents:**
- Setup
- Re-running tests​

The Kotest test framework is supported on all targets, including JVM, JavaScript, Native and Wasm. To enable Kotest for multiple platforms, follow the steps for the platform you are targeting as detailed in the following tabs.

The KMP support in Kotest 6.0 has changed from the previous versions. There is no longer a compiler plugin but a simplified setup. Please see the rest of this page for details on how to configure Kotest for KMP in Kotest 6.0 and later.

When running the Gradle test task, Gradle will cache the output and report no tests executed if no source code has changed. See the section on rerunning tests for details on how to disable this behaviour.

Kotest provides an IntelliJ plugin for enhanced UX, including the ability to run individual tests, tool windows to show test layouts, and jump to source.

A working project with JVM support can be found here: https://github.com/kotest/kotest-examples

Kotest on the JVM builds atop of the JUnit Platform project which is widely supported in the JVM ecosystem.

To use the JUnit Platform support, first configure Gradle to use JUnit platform support:

Andd then add the following dependency to your build:

And then execute the test task in gradle, or run tests directly from the IDE.

For enhanced support for jump-to-source and re-running failed tests from the test results tree, add the Kotest Gradle plugin to to your build.

A working JS project can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest or jsTest source set:

Tests can be placed in either commonTest or jsTest. Run your tests using the jsTest gradle task.

The JS test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to JS code.

A working WasmJS project can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest or wasmJsTest source set:

Tests can be placed in either commonTest or wasmJsTest. Run your tests using the wasmJsTest gradle task.

The WasmJS test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to Wasm code.

A working native project with Linux, Windows and MacOS targets configured, with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest, nativeTest or platform specific sourceset:

Tests can be placed in either commonTest or a specific native sourceset. Run your tests using the standard test tasks, for example linuxX86Test.

The native test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to native code.

Currently, only Unit tests are supported in Kotest. The following steps enable Kotest to be used for unit tests - where the Android framework is not needed or is mocked - and that usually reside in the src/test folder of your module.

Kotest on Android uses the JUnit Platform gradle plugin. This requires configuring the android test options block in your build file and then adding the Kotest junit5 runner dependency.

A working Android project with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples

A working multiplatform project with JVM, JS and native targets, and unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest source set:

Tests can be placed in either commonTest or a platform specific directory such as jsTest or macosX64Test etc. Run your tests using the gradle check task, or a platform specific test task such as macosX64Test

The JS, Wasm and native test engines are feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to non-JVM platforms.

By default, Gradle's incremental build will skip running tests if no source code has changed, marking the task as UP-TO-DATE. This can be inconvenient during debugging.

To force your tests to run every time, you can temporarily add the following configuration to your build.gradle.kts file:

Quick Alternative: For a single re-run without modifying build files, you can use the --rerun flag from the command line:

**Examples:**

Example 1 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {   useJUnitPlatform()}
```

Example 2 (kotlin):
```kotlin
dependencies {   testImplementation("io.kotest:kotest-runner-junit5:<kotest-version>")}
```

Example 3 (kotlin):
```kotlin
plugins {   id("io.kotest").version("<kotest-version>")}
```

Example 4 (kotlin):
```kotlin
plugins {   id("io.kotest").version("<kotest-version>")   id("com.google.devtools.ksp").version("<ksp-version>")}
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/next/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, test timeouts, and a host of other options. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({   test("length should return size of string") {      "hello".length shouldBe 5   }   test("startsWith should test for a prefix") {      "hello world" should startWith("hello")   }})
```

Example 2 (kotlin):
```kotlin
class DataTestExample : FreeSpec({   "maximum of two numbers" {      withData(         Triple(1, 5, 5),         Triple(1, 0, 1),         Triple(0, 0, 0)      ) { (a, b, max) ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : DescribeSpec({   describe("should use config").config(timeout = 2.seconds, invocations = 10, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/next/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration or the system property kotest.framework.testname.display.full.path.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Marking Tests​
- Running with Tags​
- Tag Expression Operators​
- Tagging All Tests​
- Tagging a Spec​
  - Inheriting tags​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

Test cases can then be marked with tags using the config function:

Then by invoking the test runner with a system property of kotest.tags you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Please pay attention to the use of upper case and lower case! If two tag objects have the same simple name (in different name spaces) they are treated as the same tag.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle like this:

Tags can also be included/excluded in runtime (for example, if you're running a project configuration instead of properties) through the RuntimeTagExtension:

Operators (in descending order of precedence)

You can add a tag to all tests in a spec using the tags function in the spec itself. For example:

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

There are two annotations you can add to a spec class itself - @Tags and @RequiresTag - which accept one or more tag names as their arguments.

The first tag - @Tags - will be applied to all tests in the class, however this will only stop a spec from being instantiated if we can guarantee that no tests would be executed (because a tag is being explicitly excluded).

Consider the following example:

The second tag - @RequiresTag - only checks that all the referenced tags are present and if not, will skip the spec.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use these annotations you pass the tag string name, not the tag itself. This is due to Kotlin annotations only allow "primitive" arguments

By default, the @Tags annotation will only be considered on the immediate Spec which it was applied to. However, a Spec can also inherit tags from superclasses and superinterfaces. To enable this, toggle tagInheritance = true in your project config

Special attention is needed in your gradle configuration

To use System Properties (-Dx=y), your gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows: Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
import io.kotest.specs.StringSpecclass MyTest : StringSpec() {  init {    "should run on Windows".config(tags = setOf(Windows)) {      // ...    }    "should run on Linux".config(tags = setOf(Linux)) {      // ...    }    "should run on Windows and Linux".config(tags = setOf(Windows, Linux)) {      // ...    }  }}
```

Example 4 (unknown):
```unknown
gradle test -Dkotest.tags="Linux & !Database"
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/6.0/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
    - DSL Methods​
    - DSL methods with functions​
    - Overriding callback functions in a Spec​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. For more advanced cases, such as writing distributable plugins or re-usable hooks, one can use extensions.

At the end of this section is a list of the available hooks and when they are executed.

There are several ways to use hooks in Kotest:

The first and simplest, is to use the DSL methods available inside a Spec which create and register a TestListener for you. For example, we can invoke beforeTest or afterTest (and others) directly alongside our tests.

Behind the scenes, these DSL methods will create an instance of TestListener, overriding the appropriate functions, and ensuring that this test listener is registered to run.

You can use afterProject as a DSL method which will create an instance of ProjectListener, but there is no beforeProject because by the time the framework is at this stage of detecting a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The BeforeTest type used on the function definition is an alias to suspend (TestCase) -> Unit to keep things simple. There are aliases for the types of each of the callbacks.

The second, related, method is to override the callback functions in the Spec. This is essentially just a variation on the first method.

To understand all callbacks correctly it's important to have a good understanding of possible TestType values:

Notice that as far as beforeAny and beforeTest are just another name for the same functionality, beforeEach is different. Each of beforeAny and beforeTest will be invoked before any TestType.Test, whereas beforeEach will be invoked before both TestType.Container and TestType.Test. The same applies to afterAny, afterTest and afterEach.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : WordSpec({  beforeTest {    println("Starting a test $it")  }  afterTest { (test, result) ->    println("Finished spec with result $result")  }  "this test" should {    "be alive" {      println("Johnny5 is alive!")    }  }})
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : WordSpec({   // used once   beforeTest(startTest)   "this test" should {      "be alive" {         println("Johnny5 is alive!")      }   }})class OtherSpec : WordSpec({   // used twice   beforeTest(startTest)   "this test" should {      "fail" {         fail("boom")      }   }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : WordSpec() {    override suspend fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" should {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function that contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are defined using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a superclass that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Tests must be defined inside an init {} block or a class body lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Test Ordering | Kotest

**URL:** https://kotest.io/docs/framework/test-ordering.html

**Contents:**
- Test Ordering
  - Sequential Ordering​
  - Random Ordering​
  - Lexicographic Ordering​

When defining multiple tests in a Spec, there's a certain order to how Kotest will execute them.

By default, a sequential order is used (order that the tests are defined in the file), but it's also possible to configure them to be executed in a random order or lexicographic order.

This setting can be configured in either each Spec individually (if you wish fine-grained control) or in ProjectConfig by overriding the testCaseOrder function (if you wish for project wide defaults). If both exist, the Spec's configuration will have priority.

Nested tests will always run in sequential order regardless of test ordering setting.

Root tests are dispatched in the order they are defined in the spec file.

Root tests are dispatched in a random order.

Root tests are dispatched in a lexicographic order.

**Examples:**

Example 1 (kotlin):
```kotlin
class SequentialSpec : FreeSpec() {    override fun testCaseOrder(): TestCaseOrder? = TestCaseOrder.Sequential    init {      "foo" {        // I run first as I'm defined first      }      "bar" {        // I run second as I'm defined second      }    }}
```

Example 2 (kotlin):
```kotlin
class RandomSpec : FreeSpec() {    override fun testCaseOrder(): TestCaseOrder? = TestCaseOrder.Random    init {      "foo" {        // This test may run first or second      }      "bar" {        // This test may run first or second      }    }}
```

Example 3 (kotlin):
```kotlin
class LexicographicSpec : FreeSpec() {    override fun testCaseOrder(): TestCaseOrder? = TestCaseOrder.Lexicographic    init {      "foo" {        // I run second as bar < foo      }      "bar" {        // I run first as bar < foo      }    }}
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listeners for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/project-config.html

**Contents:**
- Project Level Config
- Runtime Detection​
- Parallelism​
- Assertion Mode​
- Global Assert Softly​
- Duplicate Test Name Handling​
- Fail On Ignored Tests​
- Ordering​
  - Test Ordering​
  - Spec Ordering​

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project level configuration can be used by creating an object or class that extends from AbstractProjectConfig.

Any configuration set at the Spec level or directly on a test will override the config specified at the project level.

Some configuration options available in KotestProjectConfig include parallelism of tests, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions.

At runtime, Kotest will scan for classes that extend AbstractProjectConfig and instantiate them, using any configuration values defined in those classes.

You can create more than one config class in different modules, and any on the current classpath will be detected and configs merged. This is effective for allowing common config to be placed into a root module. In the case of clashes, one value will be arbitrarily picked, so it is not recommended adding competing settings to different configs.

If you have a large project, then you may wish to disable the auto scanning for these config classes if it is incurring a significant startup cost. You can do this by setting a system property or environment variable kotest.framework.classpath.scanning.config.disable to true.

Once auto scanning is disabled, if you wish to still use project config, you can specify a well known class name which Kotest will reflectively instantiate. The system property or environment variable to use is kotest.framework.config.fqn.

For example, setting:

Will disable runtime scanning, and look for a class com.wibble.KotestConfig. The class must still inherit AbstractProjectConfig.

Another related setting is kotest.framework.classpath.scanning.autoscan.disable which can also be set to false for speed. With auto scan disabled, Kotest will not scan the classpath looking for for @AutoScan annotated extensions.

System properties set in your gradle file won't be picked up by the intellij plugin if you have that installed. Instead, look to specify the properties inside a kotest.properties file. Full details here.

You can ask Kotest to run specs in parallel to take advantage of modern cpus with several cores by setting the parallelism level (default is 1). Tests inside a spec are always executed sequentially.

To do this, override parallelism inside your config and set it to a value higher than 1. The number set is the number of concurrently executing specs. For example.

An alternative way to enable this is the system property kotest.framework.parallelism which will always (if defined) take priority over the value here.

Some tests may not play nice in parallel, so you can opt out individual specs and force them to be executed in isolation by using the @DoNotParallelize annotation on the spec.

This is only available on the JVM target.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to opt-in to the assertion mode when enabled.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

By default, Kotest will rename a test if it has the same name as another test in the same scope. It will append _1, _2 and so on to the test name. This is useful for automatically generated tests.

You can change this behavior globally by setting duplicateTestNameMode to either DuplicateTestNameMode.Error or DuplicateTestNameMode.Warn.

Error will fail the test suite on a repeated name, and warn will rename but output a warning.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

Kotest supports ordering both specs and tests independently.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

Test names can be adjusted in several ways.

Test names case can be controlled by changing the value of testNameCase.

By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

Another using test name option is testNameAppendTags which, when set to true, will include any applicable tags in the test name. For example, if a test foo was defined in a spec with the tags linux and spark then the test name would be adjusted to be foo [linux, spark]

This setting can also be set using a system property or environment variable kotest.framework.testname.append.tags to true.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is my test case. By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

**Examples:**

Example 1 (unknown):
```unknown
kotest.framework.classpath.scanning.config.disable=truekotest.framework.config.fqn=com.wibble.KotestConfig
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val parallelism = 3}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val assertionMode = AssertionMode.Error}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val globalAssertSoftly = true}
```

---

## Project Timeout | Kotest

**URL:** https://kotest.io/docs/framework/timeouts/project-timeouts.html

**Contents:**
- Project Timeout

Kotest supports a project level timeout. This timeout applies to all tests in a module and includes the setup/teardown time of every spec/test in the module.

To enable this, we can use ProjectConfig.

In the above example, we have specified a project timeout of 10 minutes. All specs and tests must complete within that 10 minute period or the build will fail.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override val projectTimeout: Duration = 10.minutes}
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, and a timeout for each test or for all tests. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : StringSpec({   "length should return size of string" {      "hello".length shouldBe 5   }   "startsWith should test for a prefix" {      "world" should startWith("wor")   }})
```

Example 2 (kotlin):
```kotlin
class StringSpecExample : StringSpec({   "maximum of two numbers" {      forAll(         row(1, 5, 5),         row(1, 0, 1),         row(0, 0, 0)      ) { a, b, max ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : StringSpec({   "should use config".config(timeout = 2.seconds, invocations = 10, threads = 2, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/6.0/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/project-config.html

**Contents:**
- Project Level Config
- Runtime Detection​
- Parallelism​
- Assertion Mode​
- Global Assert Softly​
- Duplicate Test Name Handling​
- Fail On Ignored Tests​
- Ordering​
  - Test Ordering​
  - Spec Ordering​

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project level configuration can be used by creating an object or class that extends from AbstractProjectConfig.

Any configuration set at the Spec level or directly on a test will override the config specified at the project level.

Some configuration options available in KotestProjectConfig include parallelism of tests, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions.

At runtime, Kotest will scan for classes that extend AbstractProjectConfig and instantiate them, using any configuration values defined in those classes.

You can create more than one config class in different modules, and any on the current classpath will be detected and configs merged. This is effective for allowing common config to be placed into a root module. In the case of clashes, one value will be arbitrarily picked, so it is not recommended adding competing settings to different configs.

If you have a large project, then you may wish to disable the auto scanning for these config classes if it is incurring a significant startup cost. You can do this by setting a system property or environment variable kotest.framework.classpath.scanning.config.disable to true.

Once auto scanning is disabled, if you wish to still use project config, you can specify a well known class name which Kotest will reflectively instantiate. The system property or environment variable to use is kotest.framework.config.fqn.

For example, setting:

Will disable runtime scanning, and look for a class com.wibble.KotestConfig. The class must still inherit AbstractProjectConfig.

Another related setting is kotest.framework.classpath.scanning.autoscan.disable which can also be set to false for speed. With auto scan disabled, Kotest will not scan the classpath looking for for @AutoScan annotated extensions.

System properties set in your gradle file won't be picked up by the intellij plugin if you have that installed. Instead, look to specify the properties inside a kotest.properties file. Full details here.

You can ask Kotest to run specs in parallel to take advantage of modern cpus with several cores by setting the parallelism level (default is 1). Tests inside a spec are always executed sequentially.

To do this, override parallelism inside your config and set it to a value higher than 1. The number set is the number of concurrently executing specs. For example.

An alternative way to enable this is the system property kotest.framework.parallelism which will always (if defined) take priority over the value here.

Some tests may not play nice in parallel, so you can opt out individual specs and force them to be executed in isolation by using the @DoNotParallelize annotation on the spec.

This is only available on the JVM target.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to opt-in to the assertion mode when enabled.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

By default, Kotest will rename a test if it has the same name as another test in the same scope. It will append _1, _2 and so on to the test name. This is useful for automatically generated tests.

You can change this behavior globally by setting duplicateTestNameMode to either DuplicateTestNameMode.Error or DuplicateTestNameMode.Warn.

Error will fail the test suite on a repeated name, and warn will rename but output a warning.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

Kotest supports ordering both specs and tests independently.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

Test names can be adjusted in several ways.

Test names case can be controlled by changing the value of testNameCase.

By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

Another using test name option is testNameAppendTags which, when set to true, will include any applicable tags in the test name. For example, if a test foo was defined in a spec with the tags linux and spark then the test name would be adjusted to be foo [linux, spark]

This setting can also be set using a system property or environment variable kotest.framework.testname.append.tags to true.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is my test case. By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

**Examples:**

Example 1 (unknown):
```unknown
kotest.framework.classpath.scanning.config.disable=truekotest.framework.config.fqn=com.wibble.KotestConfig
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val parallelism = 3}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val assertionMode = AssertionMode.Error}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val globalAssertSoftly = true}
```

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/project-config.html

**Contents:**
- Project Level Config
- Parallelism​
- Assertion Mode​
- Global Assert Softly​
- Fail On Ignored Tests​
- Test Ordering​
- Spec Ordering​
- Test name case​
- Test name whitespace​

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project level configuration can be used by creating an object or class that extends from AbstractProjectConfig. At runtime, Kotest will scan for classes that extend this abstract class and instantiate them, reading any configuration defined there.

You can create more than one config class in different modules, and any on the current classpath will be detected and configs merged. This is effective for allowing common config to be placed into a root module. In the case of clashes, one value will be arbitrarily picked, so it is not recommended adding competing settings to different configs.

If your project specifies more than one project config, they will be merged, but the resolution of conflicting values is unspecified. It is advised that separate configs do not specify the same settings

Any configuration set at the Spec level or directly on a test will override the config specified at the project level.

Some configuration options available in KotestProjectConfig include parallelism of tests, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions.

You can ask Kotest to run specs in parallel to take advantage of modern cpus with several cores by setting the parallelism level (default is 1). Tests inside a spec are always executed sequentially.

To do this, override parallelism inside your config and set it to a value higher than 1. The number set is the number of concurrently executing specs. For example.

An alternative way to enable this is the system property kotest.framework.parallelism which will always (if defined) take priority over the value here.

Some tests may not play nice in parallel, so you can opt out individual specs and force them to be executed in isolation by using the @DoNotParallelize annotation on the spec.

This is only available on the JVM target.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to opt-in to the assertion mode when enabled.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

The case of the test names can be controlled by changing the value of testNameCase. By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is my test case. By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

**Examples:**

Example 1 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val parallelism = 3}
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val assertionMode = AssertionMode.Error}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val globalAssertSoftly = true}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val failOnIgnoredTests = true}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/next/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​
  - Native Support​
  - Callbacks​
  - WithXXX Variants​

Prior to kotest 6.0, data-driven-testing was a separate module. Starting from kotest 6.0, data-driven-testing is included in the core framework so there is no kotest-framework-datatest to be added. Please remove that from your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

If you are using data testing on kotlin-native platforms, and you only have data tests (ie, zero manual tests) then see the section on Native Support.

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withXXX function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<9> but was:<41>

In that previous example, we wrapped the withContexts call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

If you are using data testing on kotlin-native platforms, and you only have data tests (ie, zero manual tests) then you must instruct the Kotlin gradle plugin to not fail the build because no tests are discovered. This happens because data tests are generated at runtime by Kotest, the kotlin-native test discovery mechanism does not see any tests at compile time. Again, this only matters if you are using data tests exclusively.

If you wish to have before / after callbacks in data-driven tests, then you can use the standard beforeTest / afterTest support. Every test created using data-driven testing acts the same way as a regular test, so all standard callbacks work as if you had written all the test by hand.

Kotest provides a variety of withXXX functions to support different input types, and they change per spec style.

Each spec style has its own set of withXXX functions, and the standard withData which points to an appropriate variant for that spec style.

Combinations per spec style are listed below:

Examples of how these are used can be found in these kotest tests

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withContexts(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withContexts(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a default TestCaseConfig for all test cases of a Spec:

Overriding the defaultTestCaseConfig function:

Or via assignment to the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10, threads = 2) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec : StringSpec() {  override fun defaultTestCaseConfig() = TestCaseConfig(invocations = 3)  init {    // your test cases ...  }}
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a default TestCaseConfig for all test cases of a Spec:

Overriding the defaultTestCaseConfig function:

Or via assignment to the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10, threads = 2) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec : StringSpec() {  override fun defaultTestCaseConfig() = TestCaseConfig(invocations = 3)  init {    // your test cases ...  }}
```

---

## Concurrency | Kotest

**URL:** https://kotest.io/docs/6.0/framework/concurrency6.html

**Contents:**
- Concurrency
- Concurrency Modes​
  - Spec Concurrency Mode​
  - Test Concurrency Mode​
    - Project-wide configuration​
    - Package-level configuration​
    - Spec-level configuration​
- Examples​
  - Example: Running tests within a spec concurrently​
  - Example: Limited concurrency for tests​

This document describes the new concurrency features introduced in Kotest 6.0. If you are using an earlier version of Kotest, please refer to the previous concurrency documentation.

Concurrency is at the heart of Kotlin, with compiler support for continuations (suspend functions), enabling the powerful coroutines library, in addition to the standard Java concurrency tools.

So it is expected that a Kotlin test framework should offer full support for executing tests concurrently, whether that is through traditional blocking calls or suspendable functions.

Kotest offers the following features:

These features are orthogonal but complimentary.

By default, Kotest will execute each test case sequentially using Dispatchers.Default. This means if a test suspends or blocks, the whole test suite will suspend or block until that test resumes.

This is the safest default to use, since it places no burden or expectation on the user to write thread-safe tests. For example, tests can share state or use instance fields which are not thread safe. It won't subject your tests to race conditions or require you to know Java's memory model. Specs can use before and after methods confidently knowing they won't interfere with each other.

However, some users will want to run tests concurrently to reduce the total execution time of their test suite. This is especially true when testing code that suspends or blocks - the performance gains from allowing tests to run concurrently can be significant.

The concurrency modes described below are only available on the JVM platform. On other platforms, tests will always run sequentially.

Kotest provides two types of concurrency modes:

Spec concurrency mode determines whether multiple specs can be executed at the same time. There are three options:

You can configure the spec concurrency mode in your project config:

Or for limited concurrency:

Test concurrency mode determines whether multiple root tests within a spec can be executed at the same time. Note that nested tests (tests defined within other tests) are not affected by this setting; they will always run sequentially.

There are three options:

You can configure the test concurrency mode at different levels:

This will apply for all specs and tests in the project unless overridden at a lower level.

Package-level configuration allows you to set the test execution mode for all specs in a specific package, and is only available on the JVM platform.

You can configure test concurrency mode for a specific spec in two ways:

Kotest allows you to customize the coroutine dispatcher used for executing specs and tests through the CoroutineDispatcherFactory feature. This gives you fine-grained control over the execution context of your tests.

The CoroutineDispatcherFactory interface provides methods to switch the CoroutineDispatcher used for:

The CoroutineDispatcherFactory interface has two main methods:

When a CoroutineDispatcherFactory is configured, Kotest will use it to determine which dispatcher to use when executing specs and tests.

You can configure a CoroutineDispatcherFactory at different levels:

Kotest provides a built-in implementation called ThreadPerSpecCoroutineContextFactory that creates a dedicated thread per spec.

You can create your own custom implementation to suit your specific needs:

The coroutineDispatcherFactory feature is useful for:

When working with blocking code in tests, you may encounter issues with timeouts not working as expected. This is because coroutine timeouts are cooperative by nature, meaning they rely on the coroutine to yield control back to the scheduler.

To address this issue, Kotest provides a blockingTest mode that can be enabled on a per-test basis:

When blockingTest is set to true:

The blockingTest mode is only necessary when you're using blocking calls in your tests. For tests that use suspending functions, the regular timeout mechanism works fine without needing to enable this mode.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyProjectConfig : AbstractProjectConfig() {    override val specExecutionMode = SpecExecutionMode.Concurrent}
```

Example 2 (kotlin):
```kotlin
class MyProjectConfig : AbstractProjectConfig() {    override val specExecutionMode = SpecExecutionMode.LimitedConcurrency(4) // Run up to 4 specs concurrently}
```

Example 3 (kotlin):
```kotlin
class MyProjectConfig : AbstractProjectConfig() {    override val testExecutionMode = TestExecutionMode.Concurrent}
```

Example 4 (kotlin):
```kotlin
class MyPackageConfig : AbstractPackageConfig() {    override val testExecutionMode = TestExecutionMode.Concurrent}
```

---

## Setup | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/project-setup.html

**Contents:**
- Setup

The Kotest test framework is supported on JVM, Javascript and Native. To enable Kotest for multiple platforms, combine the steps for the individual platforms as detailed in the following tabs.

Kotest on the JVM uses the JUnit Platform gradle plugin. For Gradle 4.6 and higher this is as simple as adding useJUnitPlatform() inside the tasks with type Test and then adding the Kotest junit5 runner dependency.

If you are using Gradle + Groovy then:

Or if you are using Gradle + Kotlin then:

And then the dependency:

A working multiplatform project with JVM, native and Javascript all configured, with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples-multiplatform

Add the Kotest multiplatform gradle plugin to your build.

Add the engine dependency to your commonTest dependencies block:

Only the new IR compiler backend for Kotlin/JS is supported. If you are compiling JS with the legacy compiler backend then you will not be able to use Kotest for testing.

Write your tests using FunSpec, ShouldSpec or StringSpec. Tests can be placed in either commonTest or jsTest source sets. Run your tests using the gradle check command.

The Javascript test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to javascript code.

Tests for Javascript cannot nest tests. This is due to the underlying Javascript test runners (such as Mocha or Karma) not supporting promises in parent tests, which is incompatible with coroutines and in Kotest every test scope is a coroutine. This is why the supported specs are limited to FunSpec, ShouldSpec and StringSpec.

The IntelliJ Kotest plugin does not support running common, native or JS tests directly from the IDE using the green run icons. Only execution via gradle is supported.

A working multiplatform project with JVM, native and Javascript all configured, with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples-multiplatform

Add the Kotest multiplatform gradle plugin to your build.

Add the engine dependency to your commonTest dependencies block:

Tests can be placed in either commonTest or a specific native sourceset. Run your tests using the gradle check command.

The native test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to native code.

The IntelliJ Kotest plugin does not support running common, native or JS tests from the IDE. You will need to use the gradle check task.

For maven you must configure the surefire plugin for junit tests.

And then add the Kotest JUnit5 runner to your dependencies section.

Currently, only JVM tests are officially supported in Kotest. We are open to suggestions on how to support UI tests.

The following steps enable Kotest to be used for unit and integration tests, where the Android framework is not needed or is mocked that usually reside in the src/test folder of your module.

Kotest on Android uses the JUnit Platform gradle plugin. This requires configuring the android test options block in your build file and then adding the Kotest junit5 runner dependency.

To configure the test framework for both JS and JVM, you just combine copy the steps for JVM and JS.

**Examples:**

Example 1 (unknown):
```unknown
test {   useJUnitPlatform()}
```

Example 2 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {   useJUnitPlatform()}
```

Example 3 (bash):
```bash
testImplementation 'io.kotest:kotest-runner-junit5:$version'
```

Example 4 (kotlin):
```kotlin
plugins {  id("io.kotest.multiplatform") version "5.0.2"}
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class StringSpecExample : StringSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a default TestCaseConfig for all test cases of a Spec:

Overriding the defaultTestCaseConfig function:

Or via assignment to the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10, threads = 2) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec : StringSpec() {  override fun defaultTestCaseConfig() = TestCaseConfig(invocations = 3)  init {    // your test cases ...  }}
```

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Marking Tests​
- Running with Tags​
- Tag Expression Operators​
- Tagging All Tests​
- Tagging a Spec​
  - Inheriting tags​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

Test cases can then be marked with tags using the config function:

Then by invoking the test runner with a system property of kotest.tags you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Please pay attention to the use of upper case and lower case! If two tag objects have the same simple name (in different name spaces) they are treated as the same tag.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle like this:

Tags can also be included/excluded in runtime (for example, if you're running a project configuration instead of properties) through the RuntimeTagExtension:

Operators (in descending order of precedence)

You can add a tag to all tests in a spec using the tags function in the spec itself. For example:

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

There are two annotations you can add to a spec class itself - @Tags and @RequiresTag - which accept one or more tag names as their arguments.

The first tag - @Tags - will be applied to all tests in the class, however this will only stop a spec from being instantiated if we can guarantee that no tests would be executed (because a tag is being explicitly excluded).

Consider the following example:

The second tag - @RequiresTag - only checks that all the referenced tags are present and if not, will skip the spec.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use these annotations you pass the tag string name, not the tag itself. This is due to Kotlin annotations only allow "primitive" arguments

By default, the @Tags annotation will only be considered on the immediate Spec which it was applied to. However, a Spec can also inherit tags from superclasses and superinterfaces. To enable this, toggle tagInheritance = true in your project config

Special attention is needed in your gradle configuration

To use System Properties (-Dx=y), your gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows: Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
import io.kotest.specs.StringSpecclass MyTest : StringSpec() {  init {    "should run on Windows".config(tags = setOf(Windows)) {      // ...    }    "should run on Linux".config(tags = setOf(Linux)) {      // ...    }    "should run on Windows and Linux".config(tags = setOf(Windows, Linux)) {      // ...    }  }}
```

Example 4 (unknown):
```unknown
gradle test -Dkotest.tags="Linux & !Database"
```

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - System Property​
  - Config​

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are three values: SingleInstance, InstancePerLeaf, and InstancePerTest.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed three times as the same instance is used for all tests.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({ isolationMode = IsolationMode.SingleInstance // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({   val id = UUID.randomUUID()   "a" should {      println(id)      "b" {         println(id)      }      "c" {         println(id)      }   }})
```

Example 4 (kotlin):
```kotlin
class InstancePerTestExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerTest  init {    "a" should {      println("Hello")      "b" {        println("From")      }      "c" {        println("Sam")      }    }  }}
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is often sufficient, but if we need control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from JVM classpath discovery, or the order they appear in javascript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are explicitly executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with lowest values executed first. Any specs without such an annotation are considered "last". This option only works on the JVM. Any ties will be broken arbitrarily.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and FarTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, BooTest will execute last, as it has no annotation.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig: AbstractProjectConfig() {    override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() { }@Order(0)class BarTest: FunSpec() {}@Order(1)class FarTest : FunSpec() { }class BooTest : FunSpec() {}
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listeners for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/6.0/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​
  - Native Support​
  - Callbacks​

Prior to kotest 6.0, data-driven-testing was a separate module. Starting from kotest 6.0, data-driven-testing is included in the core framework so there is no kotest-framework-datatest to be added. Please remove that from your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

If you are using data testing on kotlin-native platforms, and you only have data tests (ie, zero manual tests) then see the section on Native Support.

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withData function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<9> but was:<41>

In that previous example, we wrapped the withData call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

If you are using data testing on kotlin-native platforms, and you only have data tests (ie, zero manual tests) then you must instruct the Kotlin gradle plugin to not fail the build because no tests are discovered. This happens because data tests are generated at runtime by Kotest, the kotlin-native test discovery mechanism does not see any tests at compile time. Again, this only matters if you are using data tests exclusively.

If you wish to have before / after callbacks in data-driven tests, then you can use the standard beforeTest / afterTest support. Every test created using data-driven testing acts the same way as a regular test, so all standard callbacks work as if you had written all the test by hand.

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withData(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withData(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/project-config.html

**Contents:**
- Project Level Config
- Runtime Detection​
- Parallelism​
- Assertion Mode​
- Global Assert Softly​
- Duplicate Test Name Handling​
- Fail On Ignored Tests​
- Ordering​
  - Test Ordering​
  - Spec Ordering​

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project level configuration can be used by creating a class that extends from AbstractProjectConfig. Note: On the JVM and JS platforms, an object is also supported if you prefer to a class.

Any configuration set at the Spec level or directly on a test will override the config specified at the project level.

Some configuration options available in KotestProjectConfig include parallelism of tests, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions.

At runtime, Kotest will scan for classes that extend AbstractProjectConfig and instantiate them, using any configuration values defined in those classes.

You can create more than one config class in different modules, and any on the current classpath will be detected and configs merged. This is effective for allowing common config to be placed into a root module. In the case of clashes, one value will be arbitrarily picked, so it is not recommended adding competing settings to different configs.

If you have a large project, then you may wish to disable the auto scanning for these config classes if it is incurring a significant startup cost. You can do this by setting a system property or environment variable kotest.framework.classpath.scanning.config.disable to true.

Once auto scanning is disabled, if you wish to still use project config, you can specify a well known class name which Kotest will reflectively instantiate. The system property or environment variable to use is kotest.framework.config.fqn.

For example, setting:

Will disable runtime scanning, and look for a class com.wibble.KotestConfig. The class must still inherit AbstractProjectConfig.

Another related setting is kotest.framework.classpath.scanning.autoscan.disable which can also be set to false for speed. With auto scan disabled, Kotest will not scan the classpath looking for for @AutoScan annotated extensions.

System properties set in your gradle file won't be picked up by the intellij plugin if you have that installed. Instead, look to specify the properties inside a kotest.properties file. Full details here.

You can ask Kotest to run specs in parallel to take advantage of modern cpus with several cores by setting the parallelism level (default is 1). Tests inside a spec are always executed sequentially.

To do this, override parallelism inside your config and set it to a value higher than 1. The number set is the number of concurrently executing specs. For example.

An alternative way to enable this is the system property kotest.framework.parallelism which will always (if defined) take priority over the value here.

Some tests may not play nice in parallel, so you can opt out individual specs and force them to be executed in isolation by using the @DoNotParallelize annotation on the spec.

This is only available on the JVM target.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to opt-in to the assertion mode when enabled.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

By default, Kotest will rename a test if it has the same name as another test in the same scope. It will append _1, _2 and so on to the test name. This is useful for automatically generated tests.

You can change this behavior globally by setting duplicateTestNameMode to either DuplicateTestNameMode.Error or DuplicateTestNameMode.Warn.

Error will fail the test suite on a repeated name, and warn will rename but output a warning.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

Kotest supports ordering both specs and tests independently.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

Test names can be adjusted in several ways.

Test names case can be controlled by changing the value of testNameCase.

By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

Another using test name option is testNameAppendTags which, when set to true, will include any applicable tags in the test name. For example, if a test foo was defined in a spec with the tags linux and spark then the test name would be adjusted to be foo [linux, spark]

This setting can also be set using a system property or environment variable kotest.framework.testname.append.tags to true.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is my test case. By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

**Examples:**

Example 1 (unknown):
```unknown
kotest.framework.classpath.scanning.config.disable=truekotest.framework.config.fqn=com.wibble.KotestConfig
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val parallelism = 3}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val assertionMode = AssertionMode.Error}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val globalAssertSoftly = true}
```

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Jacoco | Kotest

**URL:** https://kotest.io/docs/framework/integrations/jacoco.html

**Contents:**
- Jacoco

Kotest integrates with Jacoco for code coverage in the standard gradle way. You can read gradle installation instructions here.

Now when you run test, the Jacoco report files should be generated in $buildDir/reports/jacoco.

You may need to apply the jacoco plugin to each submodule if you have a multi module project.

**Examples:**

Example 1 (kotlin):
```kotlin
plugins {   ...   jacoco   ...}
```

Example 2 (kotlin):
```kotlin
jacoco {    toolVersion = "0.8.7"    reportsDirectory = layout.buildDirectory.dir('customJacocoReportDir') // optional}
```

Example 3 (kotlin):
```kotlin
tasks.jacocoTestReport {    dependsOn(tasks.test)    reports {        xml.required.set(true)    }}
```

Example 4 (kotlin):
```kotlin
tasks.test {  ...  finalizedBy(tasks.jacocoTestReport)}
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listeners for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Marking Tests​
- Running with Tags​
- Tag Expression Operators​
- Tagging All Tests​
- Tagging a Spec​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

Test cases can then be marked with tags using the config function:

Then by invoking the test runner with a system property of kotest.tags you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Please pay attention to the use of upper case and lower case! If two tag objects have the same simple name (in different name spaces) they are treated as the same tag.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle like this:

Tags can also be included/excluded in runtime (for example, if you're running a project configuration instead of properties) through the RuntimeTagExtension:

Operators (in descending order of precedence)

You can add a tag to all tests in a spec using the tags function in the spec itself. For example:

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

There are two annotations you can add to a spec class itself - @Tags and @RequiresTag - which accept one or more tag names as their arguments.

The first tag - @Tags - will be applied to all tests in the class, however this will only stop a spec from being instantiated if we can guarantee that no tests would be executed (because a tag is being explicitly excluded).

Consider the following example:

The second tag - @RequiresTag - only checks that all the referenced tags are present and if not, will skip the spec.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use these annotations you pass the tag string name, not the tag itself. This is due to Kotlin annotations only allow "primitive" arguments

Special attention is needed in your gradle configuration

To use System Properties (-Dx=y), your gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows: Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
import io.kotest.specs.StringSpecclass MyTest : StringSpec() {  init {    "should run on Windows".config(tags = setOf(Windows)) {      // ...    }    "should run on Linux".config(tags = setOf(Linux)) {      // ...    }    "should run on Windows and Linux".config(tags = setOf(Windows, Linux)) {      // ...    }  }}
```

Example 4 (unknown):
```unknown
gradle test -Dkotest.tags="Linux & !Database"
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function TestContext -> Unit which contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are not defined manually, but instead using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a class that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Note that tests must be defined inside an init {} block or an init lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Coroutine Debugging | Kotest

**URL:** https://kotest.io/docs/framework/coroutines/coroutine-debugging.html

**Contents:**
- Coroutine Debugging
  - Spec level config​
  - Project wide config​

kotlinx-coroutines-debug is a module that provides debugging capabilities for coroutines on the JVM. When enabled, a debug agent is installed by ByteBuddy and captures information on coroutines as they are created, started, suspended and resumed.

Kotest provides the ability to enable debugging per test. We can do this by enabling coroutineDebugProbes in test config.

Once enabled, any coroutines launched inside the test will be included in a "coroutine dump" after the test completes, or as soon as an exception is thrown.

The coroutine dump will look something like:

Coroutine debugging can be enabled for all tests in a spec by overriding the coroutineDebugProbes setting inside a spec:

Coroutine debugging can be enabled for all tests in a project by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class CoroutineDebugging : FunSpec() {   init {      test("foo").config(coroutineDebugProbes = true) {         someMethodThatLaunchesACoroutine() // launches a new coroutine      }   }}
```

Example 2 (swift):
```swift
Coroutines dump 2021/11/27 22:17:43Coroutine DeferredCoroutine{Active}@71f1906, state: CREATED    (Coroutine creation stacktrace)    at kotlin.coroutines.intrinsics.IntrinsicsKt__IntrinsicsJvmKt.createCoroutineUnintercepted(IntrinsicsJvm.kt:122)    at kotlinx.coroutines.intrinsics.CancellableKt.startCoroutineCancellable(Cancellable.kt:30)    at kotlinx.coroutines.BuildersKt__Builders_commonKt.async$default(Builders.common.kt:82)    at kotlinx.coroutines.BuildersKt.async$default(Unknown Source)    at com.sksamuel.kotest.engine.coroutines.Wibble$1.invokeSuspend(CoroutineDebugTest.kt:37)    at com.sksamuel.kotest.engine.coroutines.Wibble$1.invoke(CoroutineDebugTest.kt)
```

Example 3 (kotlin):
```kotlin
class CoroutineDebugging : FunSpec() {  init {    coroutineDebugProbes = true    test("foo") {      // debugging enabled here    }    test("bar") {      // debugging enabled here    }  }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override val coroutineDebugProbes = true}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​
  - Callbacks​

Before data-driven-testing can be used, you need to add the module kotest-framework-datatest to your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withData function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<9> but was:<41>

In that previous example, we wrapped the withData call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

If you wish to have before / after callbacks in data-driven tests, then you can use the standard beforeTest / afterTest support. Every test created using data-driven testing acts the same way as a regular test, so all standard callbacks work as if you had written all the test by hand.

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withData(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withData(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- String Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 10 different styles of test layout. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

StringSpec reduces the syntax to the absolute minimum. Just write a string followed by a lambda expression with your test code.

Adding config to the test.

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks.

Tests can be disabled using the xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use context, given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Context, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xcontext, xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

If you are migrating from JUnit then AnnotationSpec is a spec that uses annotations like JUnit 4/5. Just add the @Test annotation to any function defined in the spec class.

You can also add annotations to execute something before tests/specs and after tests/specs, similarly to JUnit's

If you want to ignore a test, use @Ignore.

Although this spec doesn't offer much advantage over using JUnit, it allows you to migrate existing tests relatively easily, as you typically just need to adjust imports.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string" {        "hello".length shouldBe 5    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string".config(enabled = false, invocations = 3) {        "hello".length shouldBe 5    }})
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a default TestCaseConfig for all test cases of a Spec:

Overriding the defaultTestCaseConfig function:

Or via assignment to the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10, threads = 2) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec : StringSpec() {  override fun defaultTestCaseConfig() = TestCaseConfig(invocations = 3)  init {    // your test cases ...  }}
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
    - DSL Methods​
    - DSL methods with functions​
    - Overriding callback functions in a Spec​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. For more advanced cases, such as writing distributable plugins or re-usable hooks, one can use extensions.

At the end of this section is a list of the available hooks and when they are executed.

There are several ways to use hooks in Kotest:

The first and simplest, is to use the DSL methods available inside a Spec which create and register a TestListener for you. For example, we can invoke beforeTest or afterTest (and others) directly alongside our tests.

Behind the scenes, these DSL methods will create an instance of TestListener, overriding the appropriate functions, and ensuring that this test listener is registered to run.

You can use afterProject as a DSL method which will create an instance of ProjectListener, but there is no beforeProject because by the time the framework is at this stage of detecting a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The BeforeTest type used on the function definition is an alias to suspend (TestCase) -> Unit to keep things simple. There are aliases for the types of each of the callbacks.

The second, related, method is to override the callback functions in the Spec. This is essentially just a variation on the first method.

To understand all callbacks correctly it's important to have a good understanding of possible TestType values:

Notice that as far as beforeAny and beforeTest are just another name for the same functionality, beforeEach is different. Each of beforeAny and beforeTest will be invoked before both TestType.Container and TestType.Test, whereas beforeEach will be invoked before any TestType.Test. The same applies to afterAny, afterTest and afterEach.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : WordSpec({  beforeTest {    println("Starting a test $it")  }  afterTest { (test, result) ->    println("Finished spec with result $result")  }  "this test" should {    "be alive" {      println("Johnny5 is alive!")    }  }})
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : WordSpec({   // used once   beforeTest(startTest)   "this test" should {      "be alive" {         println("Johnny5 is alive!")      }   }})class OtherSpec : WordSpec({   // used twice   beforeTest(startTest)   "this test" should {      "fail" {         fail("boom")      }   }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : WordSpec() {    override suspend fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" should {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​

Before data-driven-testing can be used, you need to add the module kotest-framework-datatest to your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withData function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<true> but was:<false>

In that previous example, we wrapped the withData call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withData(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withData(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Conditional Evaluation | Kotest

**URL:** https://kotest.io/docs/next/framework/conditional-evaluation.html

**Contents:**
- Conditional Evaluation

There are several ways to disable tests. Some of these are hardcoded in your test, others are evaluated at runtime.

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - System Property​
  - Config​

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are three values: SingleInstance, InstancePerLeaf, and InstancePerTest.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed three times as the same instance is used for all tests.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({ isolationMode = IsolationMode.SingleInstance // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({   val id = UUID.randomUUID()   "a" should {      println(id)      "b" {         println(id)      }      "c" {         println(id)      }   }})
```

Example 4 (kotlin):
```kotlin
class InstancePerTestExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerTest  init {    "a" should {      println("Hello")      "b" {        println("From")      }      "c" {        println("Sam")      }    }  }}
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class StringSpecExample : StringSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Fake Functions | Kotest

**URL:** https://kotest.io/docs/framework/fakery.html

**Contents:**
- Fake Functions

In functional programming, our dependencies are less likely to be instances of concrete classes and more likely to be functions. Whenever we are unit testing something with functional dependencies, it's usually easier to just pass another function rather than mock that dependency. Consider, for example, the following implementation:

Traditionally, we would mock HasAnswer and pass that mock to MyService:

However, we can also just pass a lambda, which is so very much simpler:

If we want this test-double function to return different values and/or throw exceptions, kotest has simple helper functions which make these tasks easier, such as:

This fake function can be used in unit tests as follows:

Should we need a fake function that sometimes returns a value and sometimes throws an exception,it can easily be done as follows:

As this function implements HasAnswer interface, we can use it as a dependency in our unit tests as well.

**Examples:**

Example 1 (kotlin):
```kotlin
fun interface HasAnswer {   fun answer(question: String): Int}class AnsweringService: HasAnswer {   override fun answer(question: String): Int { TODO() }}class MyService(private val hasAnswer: HasAnswer) {   fun respond(question: String): Int = hasAnswer.answer(question)}
```

Example 2 (kotlin):
```kotlin
val mockHasAnswer = run {  val ret = mockk<HasAnswer>()  every { ret.answer(any()) } returns 42  ret}val myService = MyService(mockHasAnswer)// tests here
```

Example 3 (kotlin):
```kotlin
val myService = MyService(hasAnswer = { 42 })// tests to follow
```

Example 4 (kotlin):
```kotlin
val fakeFunction = sequenceOf("yes", "no", "maybe").toFunction() fakeFunction.next() shouldBe "yes" fakeFunction.next() shouldBe "no" fakeFunction.next() shouldBe "maybe"
```

---

## Package Level Config | Kotest

**URL:** https://kotest.io/docs/next/framework/package-level-config.html

**Contents:**
- Package Level Config
- Introduction​
- Basic Usage​
- Configuration Resolution Order​
- Available Configuration Options​
- Examples​
  - Example: Setting Timeouts and Retries​
  - Example: Configuring Test Execution Mode​
  - Example: Adding Extensions for a Package​
- Package Hierarchy​

This page describes how to use package-level configuration to share configuration across multiple specs in the same package.

Package-level configuration was introduced in Kotest 6.0. If you are using an earlier version, please upgrade to take advantage of this feature.

Package-level configuration is a JVM only feature.

When writing tests, you often need to apply the same configuration to multiple test files in the same package. Instead of repeating the same configuration for each spec, or setting it at the global project level, you can use package-level configuration to define a shared configuration that applies to all specs in a specific package and its sub-packages.

Package-level configuration works by creating a class named PackageConfig that extends AbstractPackageConfig in the package where you want to apply the configuration.

To set a default configuration for all specs in a package, create a class named PackageConfig that extends AbstractPackageConfig in the target package:

With this configuration:

This configuration will also apply to all sub-packages (e.g., com.example.mypackage.subpackage).

Kotest uses the following order to resolve configuration values:

This means that more specific configurations will override more general ones. For example, if you set a timeout at both the test level and in a package-level config, the test-level timeout will be used.

AbstractPackageConfig supports the following configuration options:

When you have package-level configurations at different levels of your package hierarchy, the configuration closest to the spec's package takes precedence.

For example, if you have:

And your test is in com.example.api.v1.UserTest, then:

Kotest automatically detects classes named PackageConfig that extend AbstractPackageConfig at runtime. The detection happens when a test is executed, and Kotest looks for package configs in the package of the test and all parent packages.

For performance reasons, package configs are cached after they are first loaded, so changes to a package config class will only take effect after restarting the test run.

**Examples:**

Example 1 (kotlin):
```kotlin
// In package: com.example.mypackageclass PackageConfig : AbstractPackageConfig() {  override val timeout = 5.seconds  override val invocations = 2  override val failfast = true}
```

Example 2 (kotlin):
```kotlin
// In package: com.example.api.testsclass PackageConfig : AbstractPackageConfig() {  // All API tests might need longer timeouts and retries  override val timeout = 30.seconds  override val retries = 3}
```

Example 3 (kotlin):
```kotlin
// In package: com.example.unit.testsclass PackageConfig : AbstractPackageConfig() {  // Run all unit tests concurrently for faster execution  override val testExecutionMode = TestExecutionMode.Concurrent}
```

Example 4 (kotlin):
```kotlin
// In package: com.example.database.testsclass PackageConfig : AbstractPackageConfig() {  // Add a database container for all database tests  override val extensions = listOf(    ContainerExtension(PostgreSQLContainer<Nothing>().withDatabaseName("testdb"))  )}
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/next/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​
  - Random Seed​
  - Custom Ordering​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is usually fine as the order is perhaps not important to most test suites, but if you require control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from the JVM classpath or the order they appear in JavaScript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with the lowest values executed first. Any specs without such an annotation are considered "last" (Max integer). This option only works on the JVM. Specs with the same order value are executed in the order they are discovered.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and BazTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, WazTest will execute last, as it has no annotation.

When using the Random spec execution order, you can set a seed to ensure that the same order is always used if required.

You can also order specs yourself by implementing the SpecExecutionOrderExtension interface and registering it with the project config. If such an extension is registered, the specExecutionOrder property will be ignored and the extension will be used instead.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() {}@Order(0)class BarTest : FunSpec() {}@Order(1)class BazTest : FunSpec() {}class WazTest : FunSpec() {}
```

Example 3 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override val randomOrderSeed = ...}
```

Example 4 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override val extensions = listOf(MySpecExecutionOrderExtension())}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, and a timeout for each test or for all tests. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : StringSpec({   "length should return size of string" {      "hello".length shouldBe 5   }   "startsWith should test for a prefix" {      "world" should startWith("wor")   }})
```

Example 2 (kotlin):
```kotlin
class StringSpecExample : StringSpec({   "maximum of two numbers" {      forAll(         row(1, 5, 5),         row(1, 0, 1),         row(0, 0, 0)      ) { a, b, max ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : StringSpec({   "should use config".config(timeout = 2.seconds, invocations = 10, threads = 2, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable coroutineTestScope in test config:

Inside this test, can you retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting coroutineTestScope to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(coroutineTestScope = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(coroutineTestScope = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      coroutineTestScope = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var coroutineTestScope = true}
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/6.0/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class StringSpecExample : StringSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/next/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listener for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

include is only supported at the top level of a spec

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/next/framework/project-config.html

**Contents:**
- Project Level Config
- Setup​
- Examples​
  - Assertion Mode​
  - Global Assert Softly​
  - Timeouts​
  - Duplicate Test Name Handling​
  - Fail On Ignored Tests​
  - Ordering​
    - Test Ordering​

This document describes project-level configuration in Kotest 6.0. If you were using project-level configuration in Kotest 5.x, note that the location of the project config instance must now be specified, otherwise it will not be picked up by the framework.

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project wide configuration can be used by creating a class that extends from AbstractProjectConfig. On the JVM and JS platforms, an object is also supported if you prefer using an object to a class.

Any configuration set at the spec level or directly on a test will override config specified at the project level. Some configuration options are only available at the project level because they change how the test engine runs the entire test suite (eg spec concurrency settings).

Some configuration options available in AbstractProjectConfig include assertions modes, timeouts, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions and so on.

On the JVM, Kotest will inspect the classpath for a class with a specified name and package that extends AbstractProjectConfig. By default, this class should be named io.kotest.provided.ProjectConfig and stored in the file src/test/kotlin/io/kotest/provided/ProjectConfig.kt. If you don't want to place your class in that particular package, you can specify a different name using the system property kotest.framework.config.fqn.

For example, in gradle, you would configure something like this:

On native and JS platforms, the config class can be located anywhere but must still extend AbstractProjectConfig.

You should only create a single project config class, otherwise the behavior is undefined. If you want to have different configurations per package, see package level config.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to be aware of the assertion detection framework that Kotest provides.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

You can set a default timeout for all tests in your project by setting the timeout property in your project config.

By default, Kotest will rename a test if it has the same name as another test in the same scope. It will append _1, _2 and so on to the test name. This is useful for automatically generated tests.

You can change this behavior globally by setting duplicateTestNameMode to either DuplicateTestNameMode.Error or DuplicateTestNameMode.Warn.

Error will fail the test suite on a repeated name, and warn will rename but output a warning.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

Kotest supports ordering both specs and tests independently.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

Test names can be adjusted in several ways.

Test names case can be controlled by changing the value of testNameCase.

By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

Another using test name option is testNameAppendTags which, when set to true, will include any applicable tags in the test name. For example, if a test foo was defined in a spec with the tags linux and spark then the test name would be adjusted to be foo [linux, spark]

This setting can also be set using a system property or environment variable kotest.framework.testname.append.tags to true.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is _ _ _ my test case (note: the underscores are added for emphasis). By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

You can specify a custom coroutine dispatcher factory to control how coroutines are executed in your tests.

For more details on this feature, see the concurrency documentation.

**Examples:**

Example 1 (kotlin):
```kotlin
tests.task {  useJunitPlatform()  systemProperty("kotest.framework.config.fqn", "com.sksamuel.mypackage.WibbleConfig")}
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {  override val assertionMode = AssertionMode.Error}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {  override val globalAssertSoftly = true}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {  override val timeout = 5.seconds}
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/next/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function that contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are defined using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a superclass that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Tests must be defined inside an init {} block or a class body lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/project-config.html

**Contents:**
- Project Level Config
- Runtime Detection​
- Parallelism​
- Assertion Mode​
- Global Assert Softly​
- Duplicate Test Name Handling​
- Fail On Ignored Tests​
- Ordering​
  - Test Ordering​
  - Spec Ordering​

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project level configuration can be used by creating an object or class that extends from AbstractProjectConfig.

Any configuration set at the Spec level or directly on a test will override the config specified at the project level.

Some configuration options available in KotestProjectConfig include parallelism of tests, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions.

At runtime, Kotest will scan for classes that extend AbstractProjectConfig and instantiate them, using any configuration values defined in those classes.

You can create more than one config class in different modules, and any on the current classpath will be detected and configs merged. This is effective for allowing common config to be placed into a root module. In the case of clashes, one value will be arbitrarily picked, so it is not recommended adding competing settings to different configs.

If you have a large project, then you may wish to disable the auto scanning for these config classes if it is incurring a significant startup cost. You can do this by setting a system property or environment variable kotest.framework.classpath.scanning.config.disable to true.

Once auto scanning is disabled, if you wish to still use project config, you can specify a well known class name which Kotest will reflectively instantiate. The system property or environment variable to use is kotest.framework.config.fqn.

For example, setting:

Will disable runtime scanning, and look for a class com.wibble.KotestConfig. The class must still inherit AbstractProjectConfig.

You can ask Kotest to run specs in parallel to take advantage of modern cpus with several cores by setting the parallelism level (default is 1). Tests inside a spec are always executed sequentially.

To do this, override parallelism inside your config and set it to a value higher than 1. The number set is the number of concurrently executing specs. For example.

An alternative way to enable this is the system property kotest.framework.parallelism which will always (if defined) take priority over the value here.

Some tests may not play nice in parallel, so you can opt out individual specs and force them to be executed in isolation by using the @DoNotParallelize annotation on the spec.

This is only available on the JVM target.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to opt-in to the assertion mode when enabled.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

By default, Kotest will rename a test if it has the same name as another test in the same scope. It will append _1, _2 and so on to the test name. This is useful for automatically generated tests.

You can change this behavior globally by setting duplicateTestNameMode to either DuplicateTestNameMode.Error or DuplicateTestNameMode.Warn.

Error will fail the test suite on a repeated name, and warn will rename but output a warning.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

Kotest supports ordering both specs and tests independently.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

Test names can be adjusted in several ways.

Test names case can be controlled by changing the value of testNameCase.

By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

Another using test name option is testNameAppendTags which, when set to true, will include any applicable tags in the test name. For example, if a test foo was defined in a spec with the tags linux and spark then the test name would be adjusted to be foo [linux, spark]

This setting can also be set using a system property or environment variable kotest.framework.testname.append.tags to true.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is my test case. By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

**Examples:**

Example 1 (unknown):
```unknown
kotest.framework.classpath.scanning.config.disable=truekotest.framework.config.fqn=com.wibble.KotestConfig
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val parallelism = 3}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val assertionMode = AssertionMode.Error}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val globalAssertSoftly = true}
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function TestContext -> Unit which contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are not defined manually, but instead using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a class that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Note that tests must be defined inside an init {} block or an init lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/next/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
- DSL Methods​
  - Lambda Type-aliases​
- Method Overrides​
- Available Hooks​
  - Test Lifecycle Hooks​
  - Spec Lifecycle Hooks​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. At the end of this section is a list of the available hooks and when they are executed.

For more advanced cases, such as writing distributable plugins, re-usable hooks, or for events that take place outside a spec (such as project-started or project-finished) take a look at extensions.

There are generally two ways to define these hooks in Kotest that are functionally equivalent but different in style. Pick whichever you and your team prefer.

The first is to use the DSL methods available inside a Spec that accept a lambda for the hook logic. For example, we can invoke beforeTest or afterTest (or others) directly alongside our tests.

You can use afterProject as a DSL method but there is no equivalent beforeProject, because by the time the framework is at the stage of executing a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The beforeTest hook accepts a function of type suspend (TestCase) -> Unit. There are typealiases for each of the function signatures to keep your code simple.

For example, to create a re-usable beforeTest lambda:

The second way to create hooks is to override the appropriate function in the Spec. For example, to add a before-test hook, we can override the beforeTest function:

Kotest provides callbacks for various test and spec events.

To understand all callbacks correctly it's important to have a good understanding of the two possible TestType values:

Notice that before-each and before-container are constrained to a particular test-type (leaf or container), whereas before-any will be invoked for both. The same applies to after-each, after-container and after-any.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : FreeSpec() {  init {    beforeTest {      println("Starting a test $it")    }    afterTest { (test, result) ->      println("Finished spec with result $result")    }    "this test" - {      "be alive" {        println("Johnny5 is alive!")      }    }  }}
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : FreeSpec({   // used once   beforeTest(startTest)   "test1" { }})class OtherSpec : FreeSpec({   // used again   beforeTest(startTest)   "test2" { }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : FreeSpec() {    override suspend fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" - {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- String Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 10 different styles of test layout. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

StringSpec reduces the syntax to the absolute minimum. Just write a string followed by a lambda expression with your test code.

Adding config to the test.

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks.

Tests can be disabled using the xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

If you are migrating from JUnit then AnnotationSpec is a spec that uses annotations like JUnit 4/5. Just add the @Test annotation to any function defined in the spec class.

You can also add annotations to execute something before tests/specs and after tests/specs, similarly to JUnit's

If you want to ignore a test, use @Ignore.

Although this spec doesn't offer much advantage over using JUnit, it allows you to migrate existing tests relatively easily, as you typically just need to adjust imports.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string" {        "hello".length shouldBe 5    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string".config(enabled = false, invocations = 3) {        "hello".length shouldBe 5    }})
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable coroutineTestScope in test config:

Inside this test, can you retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting coroutineTestScope to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(coroutineTestScope = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(coroutineTestScope = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      coroutineTestScope = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var coroutineTestScope = true}
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/next/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class AutoCloseExample : FreeSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, and a timeout for each test or for all tests. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : StringSpec({   "length should return size of string" {      "hello".length shouldBe 5   }   "startsWith should test for a prefix" {      "world" should startWith("wor")   }})
```

Example 2 (kotlin):
```kotlin
class StringSpecExample : StringSpec({   "maximum of two numbers" {      forAll(         row(1, 5, 5),         row(1, 0, 1),         row(0, 0, 0)      ) { a, b, max ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : StringSpec({   "should use config".config(timeout = 2.seconds, invocations = 10, threads = 2, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is often sufficient, but if we need control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from JVM classpath discovery, or the order they appear in javascript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are explicitly executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with lowest values executed first. Any specs without such an annotation are considered "last". This option only works on the JVM. Any ties will be broken arbitrarily.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and FarTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, BooTest will execute last, as it has no annotation.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig: AbstractProjectConfig() {    override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() { }@Order(0)class BarTest: FunSpec() {}@Order(1)class FarTest : FunSpec() { }class BooTest : FunSpec() {}
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## Blocking Tests | Kotest

**URL:** https://kotest.io/docs/framework/timeouts/blocking-tests.html

**Contents:**
- Blocking Tests

When specifying timeouts in tests, Kotest uses the withTimeout coroutine functions that the Kotlin coroutine library provides. These timeouts are co-operative in nature, and a timeout is detected when a coroutine suspends, resumes, or calls yield.

However when executing blocking code, the thread will be blocked and so the coperative approach will not work. In this scenario we must revert to interrupting the thread using Thread.interrupt or something similar. In order for this interruption to work safely, we must execute the test on a dedicated thread.

Therefore, it is up to the user to signify to Kotest that they want a particular test to execute on a dedicated thread that can be safely used for interruption. We do this by enabling the blockingTest flag in test config.

In the above example, the first test requires the blockingTest flag because it uses a thread blocking operation. The second test does not because it uses a suspendable operation.

This feature is only available on the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyBlockingTest : FunSpec() {  init {    test("interrupt me!").config(blockingTest = true, timeout = 10.seconds) {       Thread.sleep(100000000)    }    test("uses suspension").config(timeout = 10.seconds) {      delay(100000000)    }  }}
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## Fake Functions | Kotest

**URL:** https://kotest.io/docs/6.0/framework/fakery.html

**Contents:**
- Fake Functions

In functional programming, our dependencies are less likely to be instances of concrete classes and more likely to be functions. Whenever we are unit testing something with functional dependencies, it's usually easier to just pass another function rather than mock that dependency. Consider, for example, the following implementation:

Traditionally, we would mock HasAnswer and pass that mock to MyService:

However, we can also just pass a lambda, which is so very much simpler:

If we want this test-double function to return different values and/or throw exceptions, kotest has simple helper functions which make these tasks easier, such as:

This fake function can be used in unit tests as follows:

Should we need a fake function that sometimes returns a value and sometimes throws an exception,it can easily be done as follows:

As this function implements HasAnswer interface, we can use it as a dependency in our unit tests as well.

**Examples:**

Example 1 (kotlin):
```kotlin
fun interface HasAnswer {   fun answer(question: String): Int}class AnsweringService: HasAnswer {   override fun answer(question: String): Int { TODO() }}class MyService(private val hasAnswer: HasAnswer) {   fun respond(question: String): Int = hasAnswer.answer(question)}
```

Example 2 (kotlin):
```kotlin
val mockHasAnswer = run {  val ret = mockk<HasAnswer>()  every { ret.answer(any()) } returns 42  ret}val myService = MyService(mockHasAnswer)// tests here
```

Example 3 (kotlin):
```kotlin
val myService = MyService(hasAnswer = { 42 })// tests to follow
```

Example 4 (kotlin):
```kotlin
val fakeFunction = sequenceOf("yes", "no", "maybe").toFunction() fakeFunction.next() shouldBe "yes" fakeFunction.next() shouldBe "no" fakeFunction.next() shouldBe "maybe"
```

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​
  - Native Support​
  - Callbacks​
  - WithXXX Variants​

Prior to kotest 6.0, data-driven-testing was a separate module. Starting from kotest 6.0, data-driven-testing is included in the core framework so there is no kotest-framework-datatest to be added. Please remove that from your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

If you are using data testing on kotlin-native platforms, and you only have data tests (ie, zero manual tests) then see the section on Native Support.

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withXXX function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<9> but was:<41>

In that previous example, we wrapped the withContexts call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

If you are using data testing on kotlin-native platforms, and you only have data tests (ie, zero manual tests) then you must instruct the Kotlin gradle plugin to not fail the build because no tests are discovered. This happens because data tests are generated at runtime by Kotest, the kotlin-native test discovery mechanism does not see any tests at compile time. Again, this only matters if you are using data tests exclusively.

If you wish to have before / after callbacks in data-driven tests, then you can use the standard beforeTest / afterTest support. Every test created using data-driven testing acts the same way as a regular test, so all standard callbacks work as if you had written all the test by hand.

Kotest provides a variety of withXXX functions to support different input types, and they change per spec style.

Each spec style has its own set of withXXX functions, and the standard withData which points to an appropriate variant for that spec style.

Combinations per spec style are listed below:

Examples of how these are used can be found in these kotest tests

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withContexts(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withContexts(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
    - DSL Methods​
    - DSL methods with functions​
    - Overriding callback functions in a Spec​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. For more advanced cases, such as writing distributable plugins or re-usable hooks, one can use extensions.

At the end of this section is a list of the available hooks and when they are executed.

There are several ways to use hooks in Kotest:

The first and simplest, is to use the DSL methods available inside a Spec which create and register a TestListener for you. For example, we can invoke beforeTest or afterTest (and others) directly alongside our tests.

Behind the scenes, these DSL methods will create an instance of TestListener, overriding the appropriate functions, and ensuring that this test listener is registered to run.

You can use afterProject as a DSL method which will create an instance of ProjectListener, but there is no beforeProject because by the time the framework is at this stage of detecting a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The BeforeTest type used on the function definition is an alias to suspend (TestCase) -> Unit to keep things simple. There are aliases for the types of each of the callbacks.

The second, related, method is to override the callback functions in the Spec. This is essentially just a variation on the first method.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : WordSpec({  beforeTest {    println("Starting a test $it")  }  afterTest { (test, result) ->    println("Finished spec with result $result")  }  "this test" should {    "be alive" {      println("Johnny5 is alive!")    }  }})
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : WordSpec({   // used once   beforeTest(startTest)   "this test" should {      "be alive" {         println("Johnny5 is alive!")      }   }})class OtherSpec : WordSpec({   // used twice   beforeTest(startTest)   "this test" should {      "fail" {         fail("boom")      }   }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : WordSpec() {    override fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" should {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class StringSpecExample : StringSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/6.0/framework/project-config.html

**Contents:**
- Project Level Config
- Setup​
- Examples​
  - Assertion Mode​
  - Global Assert Softly​
  - Timeouts​
  - Duplicate Test Name Handling​
  - Fail On Ignored Tests​
  - Ordering​
    - Test Ordering​

This document describes project-level configuration in Kotest 6.0. If you were using project-level configuration in Kotest 5.x, note that the location of the project config instance must now be specified, otherwise it will not be picked up by the framework.

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project wide configuration can be used by creating a class that extends from AbstractProjectConfig. On the JVM and JS platforms, an object is also supported if you prefer using an object to a class.

Any configuration set at the spec level or directly on a test will override config specified at the project level. Some configuration options are only available at the project level because they change how the test engine runs the entire test suite (eg spec concurrency settings).

Some configuration options available in AbstractProjectConfig include assertions modes, timeouts, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions and so on.

On the JVM, Kotest will inspect the classpath for a class with a specified name and package that extends AbstractProjectConfig. By default, this class should be named io.kotest.provided.ProjectConfig. If you don't want to place your class in that particular package, you can specify a different name using the system property kotest.framework.config.fqn.

For example, in gradle, you would configure something like this:

On native and JS platforms, the config class can be located anywhere but must still extend AbstractProjectConfig.

You should only create a single project config class, otherwise the behavior is undefined. If you want to have different configurations per package, see package level config.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to be aware of the assertion detection framework that Kotest provides.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

You can set a default timeout for all tests in your project by setting the timeout property in your project config.

By default, Kotest will rename a test if it has the same name as another test in the same scope. It will append _1, _2 and so on to the test name. This is useful for automatically generated tests.

You can change this behavior globally by setting duplicateTestNameMode to either DuplicateTestNameMode.Error or DuplicateTestNameMode.Warn.

Error will fail the test suite on a repeated name, and warn will rename but output a warning.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

Kotest supports ordering both specs and tests independently.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

Test names can be adjusted in several ways.

Test names case can be controlled by changing the value of testNameCase.

By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

Another using test name option is testNameAppendTags which, when set to true, will include any applicable tags in the test name. For example, if a test foo was defined in a spec with the tags linux and spark then the test name would be adjusted to be foo [linux, spark]

This setting can also be set using a system property or environment variable kotest.framework.testname.append.tags to true.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is _ _ _ my test case (note: the underscores are added for emphasis). By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

You can specify a custom coroutine dispatcher factory to control how coroutines are executed in your tests.

For more details on this feature, see the concurrency documentation.

**Examples:**

Example 1 (kotlin):
```kotlin
tests.task {  useJunitPlatform()  systemProperty("kotest.framework.config.fqn", "com.sksamuel.mypackage.WibbleConfig")}
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {  override val assertionMode = AssertionMode.Error}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {  override val globalAssertSoftly = true}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {  override val timeout = 5.seconds}
```

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## data_driven_testing_4.2.0 | Kotest

**URL:** https://kotest.io/docs/framework/datatesting/data_driven_testing_4.2.0

**Contents:**
- data_driven_testing_4.2.0

To test your code with different parameter combinations, you can use a table of values as input for your test cases. This is called data driven testing also known as table driven testing.

Invoke the forAll or forNone function, passing in one or more row objects, where each row object contains the values to be used be a single invocation of the test. After the forAll or forNone function, setup your actual test function to accept the values of each row as parameters.

The row object accepts any set of types, and the type checker will ensure your types are consistent with the parameter types in the test function.

In the above example, the root and square parameters are automatically inferred to be integers.

If there is an error for any particular input row, then the test will fail and KotlinTest will automatically match up each input to the corresponding parameter names. For example, if we change the previous example to include the row row(5,55) then the test will be marked as a failure with the following error message.

Table testing can be used within any spec. Here is an example using StringSpec.

It may be desirable to have each row of data parameters as an individual test. To generating such individual tests follow a similar pattern for each spec style. An example in the FreeSpec is below.

Produces 4 tests and 2 parent descriptions:

**Examples:**

Example 1 (kotlin):
```kotlin
"square roots" {  forAll(      row(2, 4),      row(3, 9),      row(4, 16),      row(5, 25)  ) { root, square ->    root * root shouldBe square  }}
```

Example 2 (json):
```json
Test failed for (root, 5), (square, 55) with error expected: 55 but was: 25
```

Example 3 (kotlin):
```kotlin
class StringSpecExample : StringSpec({  "string concat" {    forAll(      row("a", "b", "c", "abc"),      row("hel", "lo wo", "rld", "hello world"),      row("", "z", "", "z")    ) { a, b, c, d ->      a + b + c shouldBe d    }  }})
```

Example 4 (kotlin):
```kotlin
class IntegerMathSpec : FreeSpec({    "Addition" - {        listOf(            row("1 + 0", 1) { 1 + 0 },            row("1 + 1", 2) { 1 + 1 }        ).map { (description: String, expected: Int, math: () -> Int) ->            description {                math() shouldBe expected            }        }    }    // ...    "Complex Math" - {        listOf(            row("8/2(2+2)", 16) { 8 / 2 * (2 + 2) },            row("5/5 + 1*1 + 3-2", 3) { 5 / 5 + 1 * 1 + 3 - 2 }        ).map { (description: String, expected: Int, math: () -> Int) ->            description {                math() shouldBe expected            }        }    }})
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable coroutineTestScope in test config:

Inside this test, can you retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting coroutineTestScope to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(coroutineTestScope = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(coroutineTestScope = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      coroutineTestScope = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var testCoroutineDispatcher = true}
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Nested Data Tests | Kotest

**URL:** https://kotest.io/docs/framework/datatesting/nested-tests.html

**Contents:**
- Nested Data Tests

Kotest's data testing is extremely flexible and allows to unlimited nesting of data test constructs. Each extra nest will create another layer of nesting in the test output providing the cartesian join of all inputs. Please note that while StringSpec supports withXXX, this spec does not support Nested Data Tests.

For example, in the following code snippet, we have two layers of nesting.

This would give output in intellij like:

And then here is the same example, but this time with a custom test name on the second level:

**Examples:**

Example 1 (kotlin):
```kotlin
context("each service should support all http methods") {  val services = listOf(    "http://internal.foo",    "http://internal.bar",    "http://public.baz",  )  val methods = listOf("GET", "POST", "PUT")   withContexts(services) { service ->     withTests(methods) { method ->       // test service against method     }   }}
```

Example 2 (kotlin):
```kotlin
context("each service should support all http methods") {    val services = listOf(       "http://internal.foo",       "http://internal.bar",       "http://public.baz",    )    val methods = listOf("GET", "POST", "PUT")  withContexts(services) { service ->    withTests<String>({ "should support HTTP $it" }, methods) { method ->          // test service against method       }    }}
```

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/project-config.html

**Contents:**
- Project Level Config
- Runtime Detection​
- Parallelism​
- Assertion Mode​
- Global Assert Softly​
- Duplicate Test Name Handling​
- Fail On Ignored Tests​
- Ordering​
  - Test Ordering​
  - Spec Ordering​

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project level configuration can be used by creating an object or class that extends from AbstractProjectConfig.

Any configuration set at the Spec level or directly on a test will override the config specified at the project level.

Some configuration options available in KotestProjectConfig include parallelism of tests, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions.

At runtime, Kotest will scan for classes that extend AbstractProjectConfig and instantiate them, using any configuration values defined in those classes.

You can create more than one config class in different modules, and any on the current classpath will be detected and configs merged. This is effective for allowing common config to be placed into a root module. In the case of clashes, one value will be arbitrarily picked, so it is not recommended adding competing settings to different configs.

If you have a large project, then you may wish to disable the auto scanning for these config classes if it is incurring a significant startup cost. You can do this by setting a system property or environment variable kotest.framework.classpath.scanning.config.disable to true.

Once auto scanning is disabled, if you wish to still use project config, you can specify a well known class name which Kotest will reflectively instantiate. The system property or environment variable to use is kotest.framework.config.fqn.

For example, setting:

Will disable runtime scanning, and look for a class com.wibble.KotestConfig. The class must still inherit AbstractProjectConfig.

Another related setting is kotest.framework.classpath.scanning.autoscan.disable which can also be set to false for speed. With auto scan disabled, Kotest will not scan the classpath looking for for @AutoScan annotated extensions.

System properties set in your gradle file won't be picked up by the intellij plugin if you have that installed. Instead, look to specify the properties inside a kotest.properties file. Full details here.

You can ask Kotest to run specs in parallel to take advantage of modern cpus with several cores by setting the parallelism level (default is 1). Tests inside a spec are always executed sequentially.

To do this, override parallelism inside your config and set it to a value higher than 1. The number set is the number of concurrently executing specs. For example.

An alternative way to enable this is the system property kotest.framework.parallelism which will always (if defined) take priority over the value here.

Some tests may not play nice in parallel, so you can opt out individual specs and force them to be executed in isolation by using the @DoNotParallelize annotation on the spec.

This is only available on the JVM target.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to opt-in to the assertion mode when enabled.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

By default, Kotest will rename a test if it has the same name as another test in the same scope. It will append _1, _2 and so on to the test name. This is useful for automatically generated tests.

You can change this behavior globally by setting duplicateTestNameMode to either DuplicateTestNameMode.Error or DuplicateTestNameMode.Warn.

Error will fail the test suite on a repeated name, and warn will rename but output a warning.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

Kotest supports ordering both specs and tests independently.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

Test names can be adjusted in several ways.

Test names case can be controlled by changing the value of testNameCase.

By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

Another using test name option is testNameAppendTags which, when set to true, will include any applicable tags in the test name. For example, if a test foo was defined in a spec with the tags linux and spark then the test name would be adjusted to be foo [linux, spark]

This setting can also be set using a system property or environment variable kotest.framework.testname.append.tags to true.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is my test case. By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

**Examples:**

Example 1 (unknown):
```unknown
kotest.framework.classpath.scanning.config.disable=truekotest.framework.config.fqn=com.wibble.KotestConfig
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val parallelism = 3}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val assertionMode = AssertionMode.Error}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val globalAssertSoftly = true}
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/next/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, and a timeout for each test or for all tests. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : StringSpec({   "length should return size of string" {      "hello".length shouldBe 5   }   "startsWith should test for a prefix" {      "world" should startWith("wor")   }})
```

Example 2 (kotlin):
```kotlin
class StringSpecExample : StringSpec({   "maximum of two numbers" {      forAll(         row(1, 5, 5),         row(1, 0, 1),         row(0, 0, 0)      ) { a, b, max ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : StringSpec({   "should use config".config(timeout = 2.seconds, invocations = 10, threads = 2, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Conditional Evaluation | Kotest

**URL:** https://kotest.io/docs/framework/conditional-evaluation.html

**Contents:**
- Conditional Evaluation

There are several ways to disable tests. Some of these are hardcoded in your test, others are evaluated at runtime.

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/project-config.html

**Contents:**
- Project Level Config
- Parallelism​
- Assertion Mode​
- Global Assert Softly​
- Fail On Ignored Tests​
- Test Ordering​
- Spec Ordering​
- Test name case​
- Test name whitespace​

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project level configuration can be used by creating an object or class that extends from AbstractProjectConfig. At runtime, Kotest will scan for classes that extend this abstract class and instantiate them, reading any configuration defined there.

You can create more than one config class in different modules, and any on the current classpath will be detected and configs merged. This is effective for allowing common config to be placed into a root module. In the case of clashes, one value will be arbitrarily picked, so it is not recommended adding competing settings to different configs.

If your project specifies more than one project config, they will be merged, but the resolution of conflicting values is unspecified. It is advised that separate configs do not specify the same settings

Any configuration set at the Spec level or directly on a test will override the config specified at the project level.

Some configuration options available in KotestProjectConfig include parallelism of tests, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions.

You can ask Kotest to run specs in parallel to take advantage of modern cpus with several cores by setting the parallelism level (default is 1). Tests inside a spec are always executed sequentially.

To do this, override parallelism inside your config and set it to a value higher than 1. The number set is the number of concurrently executing specs. For example.

An alternative way to enable this is the system property kotest.framework.parallelism which will always (if defined) take priority over the value here.

Some tests may not play nice in parallel, so you can opt out individual specs and force them to be executed in isolation by using the @DoNotParallelize annotation on the spec.

This is only available on the JVM target.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to opt-in to the assertion mode when enabled.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

The case of the test names can be controlled by changing the value of testNameCase. By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is my test case. By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

**Examples:**

Example 1 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val parallelism = 3}
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val assertionMode = AssertionMode.Error}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val globalAssertSoftly = true}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val failOnIgnoredTests = true}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/6.0/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, and a timeout for each test or for all tests. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({  test("length should return size of string") {    "hello".length shouldBe 5  }  test("startsWith should test for a prefix") {    "hello world" should startWith("hello")  }})
```

Example 2 (kotlin):
```kotlin
class DataTestExample : FreeSpec({   "maximum of two numbers" {      withData(         Triple(1, 5, 5),         Triple(1, 0, 1),         Triple(0, 0, 0)      ) { (a, b, max) ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : DescribeSpec({   describe("should use config").config(timeout = 2.seconds, invocations = 10, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- String Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 10 different styles of test layout. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

StringSpec reduces the syntax to the absolute minimum. Just write a string followed by a lambda expression with your test code.

Adding config to the test.

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks.

Tests can be disabled using the xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

If you are migrating from JUnit then AnnotationSpec is a spec that uses annotations like JUnit 4/5. Just add the @Test annotation to any function defined in the spec class.

You can also add annotations to execute something before tests/specs and after tests/specs, similarly to JUnit's

If you want to ignore a test, use @Ignore.

Although this spec doesn't offer much advantage over using JUnit, it allows you to migrate existing tests relatively easily, as you typically just need to adjust imports.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string" {        "hello".length shouldBe 5    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string".config(enabled = false, invocations = 3) {        "hello".length shouldBe 5    }})
```

---

## Concurrency | Kotest

**URL:** https://kotest.io/docs/framework/concurrency.html

**Contents:**
- Concurrency
- Concurrency Mode​

This document describes the concurrency features introduced in Kotest 5.0 that were marked experimental and have been changed for Kotest 6.0. If you are using Kotest 6.0, please see new concurrency documentation.

Concurrency is at the heart of Kotlin, with compiler support for continuations (suspend functions), enabling the powerful coroutines library, in addition to the standard Java concurrency tools.

So it is expected that a Kotlin test framework should offer full support for executing tests concurrently, whether that is through traditional blocking calls or suspendable functions.

Kotest offers the following features:

These two features are orthogonal but complimentary.

By default, Kotest will execute each test case sequentially using a single thread. This means if a test inside a spec suspends or blocks, the whole test run will suspend or block until that test case resumes.

This is the safest default to use, since it places no burden or expectation on the user to write thread-safe tests. For example, tests can share state or use instance fields which are not thread safe. It won't subject your tests to race conditions or require you to know Java's memory model. Specs can use before and after methods confidently knowing they won't interfere with each other.

However, it is understandable that many users will want to run tests concurrently to reduce the total execution time of their test suite. This is especially true when testing code that suspends or blocks - the performance gains from allowing tests to run concurrently can be significant.

Kotest offers the ability to take advantage of multiple cores. When running in a multi-core environment, more than one spec could be executing in parallel.

Kotest supports this through the parallelism configuration setting or the kotest.framework.parallelism system property.

By default, the value is set to 1 so that the test engine would use a single thread for the entire test run. When we set this flag to a value greater than 1, multiple threads will be created for executing tests.

For example, setting this to K will (subject to caveats around blocking tests) allow up to K tests to be executing in parallel.

This setting has no effect on Javascript tests.

!!! note "Thread stickiness" When using multiple threads, all the tests of a particular spec (and the associated lifecycle callbacks) are guaranteed to be executed in the same thread. In other words, different threads are only used across different specs.

!!! tip "Blocking calls" Setting this value higher than the number of cores offers a benefit if you are testing code that is using blocking calls and you are unable to move the calls onto another dispatcher.

!!! note Setting parallelism > 1 automatically enables Spec concurrency mode unless another concurrency mode is set explicitly.

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, and a timeout for each test or for all tests. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : StringSpec({   "length should return size of string" {      "hello".length shouldBe 5   }   "startsWith should test for a prefix" {      "world" should startWith("wor")   }})
```

Example 2 (kotlin):
```kotlin
class StringSpecExample : StringSpec({   "maximum of two numbers" {      forAll(         row(1, 5, 5),         row(1, 0, 1),         row(0, 0, 0)      ) { a, b, max ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : StringSpec({   "should use config".config(timeout = 2.seconds, invocations = 10, threads = 2, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Focus and Bang | Kotest

**URL:** https://kotest.io/docs/framework/conditional/conditional-tests-with-focus-and-bang.html

**Contents:**
- Focus and Bang
- Focus​
- Bang​

Kotest allows us to quickly disable or focus a subset of tests via test name prefixes.

Kotest supports isolating top level tests by preceding the test name with f:. Then only those tests (and any subtests defined inside that scope) will be executed, with the rest being skipped.

For example, in the following snippet only the middle test will be executed.

The focus on a parent allows nested tests to execute:

The focus flag does not work if placed on nested tests due to the fact that nested tests are only discovered once the parent test has executed. So there would be no way for the test engine to know that a nested test has the f: prefix without first executing all the parents.

The opposite of focus is to prefix a test with an exclamation mark ! and then that test (and any subtests defined inside that scope) will be skipped. In the next example we’ve disabled only the first test by adding the “!” prefix.

If you just want to run a single test, you can of course just run that from intelliJ directly using the green arrow. However sometimes you want to run a subset of tests, or you want to run all tests except a few. This is when focus and disabling can be useful.

If you want to disable the use of ! (and allow it to be used as the first character in enabled test names) then set the system property kotest.bang.disable to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class FocusExample : FreeSpec({    "test 1" {     // this will be skipped    }    "f:test 2" {     // this will be executed    }    "test 3" {     // this will be skipped    }})
```

Example 2 (kotlin):
```kotlin
class FocusExample : FunSpec({   context("test 1") {      // this will be skipped      test("foo") {         // this will be skipped      }   }   context("f:test 2") {      // this will be executed      test("foo") {         // this will be executed      }   }   context("test 3") {      // this will be skipped      test("foo") {         // this will be skipped      }    }})
```

Example 3 (kotlin):
```kotlin
class BangExample : FreeSpec({  "!test 1" {    // this will be ignored  }  "test 2" {    // this will run  }  "test 3" {    // this will run too  }})
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is often sufficient, but if we need control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from JVM classpath discovery, or the order they appear in javascript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are explicitly executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with lowest values executed first. Any specs without such an annotation are considered "last". This option only works on the JVM. Any ties will be broken arbitrarily.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and FarTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, BooTest will execute last, as it has no annotation.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig: AbstractProjectConfig() {    override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() { }@Order(0)class BarTest: FunSpec() {}@Order(1)class FarTest : FunSpec() { }class BooTest : FunSpec() {}
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable coroutineTestScope in test config:

Inside this test, can you retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting coroutineTestScope to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(coroutineTestScope = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(coroutineTestScope = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      coroutineTestScope = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var coroutineTestScope = true}
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function TestContext -> Unit which contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are not defined manually, but instead using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a class that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Note that tests must be defined inside an init {} block or an init lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function TestContext -> Unit which contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are not defined manually, but instead using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a class that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Note that tests must be defined inside an init {} block or an init lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable testCoroutineDispatcher in test config:

Inside this test, can you retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting testCoroutineDispatcher to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(testCoroutineDispatcher = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(testCoroutineDispatcher = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      testCoroutineDispatcher = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var testCoroutineDispatcher = true}
```

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - System Property​
  - Config​

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are three values: SingleInstance, InstancePerLeaf, and InstancePerTest.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed three times as the same instance is used for all tests.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({ isolationMode = IsolationMode.SingleInstance // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({   val id = UUID.randomUUID()   "a" should {      println(id)      "b" {         println(id)      }      "c" {         println(id)      }   }})
```

Example 4 (kotlin):
```kotlin
class InstancePerTestExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerTest  init {    "a" should {      println("Hello")      "b" {        println("From")      }      "c" {        println("Sam")      }    }  }}
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable coroutineTestScope in test config:

Inside this test, can you retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting coroutineTestScope to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(coroutineTestScope = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(coroutineTestScope = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      coroutineTestScope = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var coroutineTestScope = true}
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Marking Tests​
- Running with Tags​
- Tag Expression Operators​
- Tagging All Tests​
- Tagging a Spec​
  - Inheriting tags​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

Test cases can then be marked with tags using the config function:

Then by invoking the test runner with a system property of kotest.tags you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Please pay attention to the use of upper case and lower case! If two tag objects have the same simple name (in different name spaces) they are treated as the same tag.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle like this:

Tags can also be included/excluded in runtime (for example, if you're running a project configuration instead of properties) through the RuntimeTagExtension:

Operators (in descending order of precedence)

You can add a tag to all tests in a spec using the tags function in the spec itself. For example:

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

There are two annotations you can add to a spec class itself - @Tags and @RequiresTag - which accept one or more tag names as their arguments.

The first tag - @Tags - will be applied to all tests in the class, however this will only stop a spec from being instantiated if we can guarantee that no tests would be executed (because a tag is being explicitly excluded).

Consider the following example:

The second tag - @RequiresTag - only checks that all the referenced tags are present and if not, will skip the spec.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use these annotations you pass the tag string name, not the tag itself. This is due to Kotlin annotations only allow "primitive" arguments

By default, the @Tags annotation will only be considered on the immediate Spec which it was applied to. However, a Spec can also inherit tags from superclasses and superinterfaces. To enable this, toggle tagInheritance = true in your project config

Special attention is needed in your gradle configuration

To use System Properties (-Dx=y), your gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows: Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
import io.kotest.specs.StringSpecclass MyTest : StringSpec() {  init {    "should run on Windows".config(tags = setOf(Windows)) {      // ...    }    "should run on Linux".config(tags = setOf(Linux)) {      // ...    }    "should run on Windows and Linux".config(tags = setOf(Windows, Linux)) {      // ...    }  }}
```

Example 4 (unknown):
```unknown
gradle test -Dkotest.tags="Linux & !Database"
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/6.0/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is often sufficient, but if we need control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from JVM classpath discovery, or the order they appear in javascript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are explicitly executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with lowest values executed first. Any specs without such an annotation are considered "last". This option only works on the JVM. Any ties will be broken arbitrarily.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and FarTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, BooTest will execute last, as it has no annotation.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig: AbstractProjectConfig() {    override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() { }@Order(0)class BarTest: FunSpec() {}@Order(1)class FarTest : FunSpec() { }class BooTest : FunSpec() {}
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/next/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 8 different styles for test definitions. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks. context can also be used as an alias for describe.

Tests can be disabled using the xcontext, xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use context, given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Context, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xcontext, xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : ShouldSpec({    should("return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : ShouldSpec({    context("String.length") {        should("return the length of the string") {            "sammy".length shouldBe 5            "".length shouldBe 0        }    }})
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Extension Examples | Kotest

**URL:** https://kotest.io/docs/framework/extensions/extension-examples.html

**Contents:**
- Extension Examples
- System Out Listener​
- Timer Listener​

This page contains some simple examples of how to write extensions.

A real example of an extension, is the NoSystemOutListener which comes with Kotest. This extension throws an error if any output is written to standard out.

Another example would be if we wanted to log the time taken for each test case. We can do this by using the beforeTest and afterTest functions as follows:

Then we can register it in a single spec like so:

Or we could register it project wide:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestSpec : DescribeSpec({  extensions(NoSystemOutListener)  describe("All these tests should not write to standard out") {    it("silence in the court") {      println("boom") // failure    }  }})
```

Example 2 (kotlin):
```kotlin
object TimerListener : BeforeTestListener, AfterTestListener {  var started = 0L  override fun beforeTest(testCase: TestCase): Unit {    started = System.currentTimeMillis()  }  override fun afterTest(testCase: TestCase, result: TestResult): Unit {    println("Duration of ${testCase.descriptor} = " + (System.currentTimeMillis() - started))  }}
```

Example 3 (kotlin):
```kotlin
class MyTestClass : FunSpec({  extensions(TimerListener)  // tests here})
```

Example 4 (kotlin):
```kotlin
object MyConfig : AbstractProjectConfig() {    override fun extensions(): List<Extension> = listOf(TimerListener)}
```

---

## Setup | Kotest

**URL:** https://kotest.io/docs/next/framework/project-setup.html

**Contents:**
- Setup
- Re-running tests​

The Kotest test framework is supported on all targets, including JVM, JavaScript, Native and Wasm. To enable Kotest for multiple platforms, follow the steps for the platform you are targeting as detailed in the following tabs.

The KMP support in Kotest 6.0 has changed from the previous versions. There is no longer a compiler plugin but a simplified setup. Please see the rest of this page for details on how to configure Kotest for KMP in Kotest 6.0 and later.

When running the Gradle test task, Gradle will cache the output and report no tests executed if no source code has changed. See the section on rerunning tests for details on how to disable this behaviour.

Kotest provides an IntelliJ plugin for enhanced UX, including the ability to run individual tests, tool windows to show test layouts, and jump to source.

A working project with JVM support can be found here: https://github.com/kotest/kotest-examples

Kotest on the JVM builds atop of the JUnit Platform project which is widely supported in the JVM ecosystem.

To use the JUnit Platform support, first configure Gradle to use JUnit platform support:

Andd then add the following dependency to your build:

And then execute the test task in gradle, or run tests directly from the IDE.

For enhanced support for jump-to-source and re-running failed tests from the test results tree, add the Kotest Gradle plugin to to your build.

A working JS project can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest or jsTest source set:

Tests can be placed in either commonTest or jsTest. Run your tests using the jsTest gradle task.

The JS test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to JS code.

A working WasmJS project can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest or wasmJsTest source set:

Tests can be placed in either commonTest or wasmJsTest. Run your tests using the wasmJsTest gradle task.

The WasmJS test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to Wasm code.

A working native project with Linux, Windows and MacOS targets configured, with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest, nativeTest or platform specific sourceset:

Tests can be placed in either commonTest or a specific native sourceset. Run your tests using the standard test tasks, for example linuxX86Test.

The native test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to native code.

Currently, only Unit tests are supported in Kotest. The following steps enable Kotest to be used for unit tests - where the Android framework is not needed or is mocked - and that usually reside in the src/test folder of your module.

Kotest on Android uses the JUnit Platform gradle plugin. This requires configuring the android test options block in your build file and then adding the Kotest junit5 runner dependency.

A working Android project with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples

A working multiplatform project with JVM, JS and native targets, and unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest source set:

Tests can be placed in either commonTest or a platform specific directory such as jsTest or macosX64Test etc. Run your tests using the gradle check task, or a platform specific test task such as macosX64Test

The JS, Wasm and native test engines are feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to non-JVM platforms.

By default, Gradle's incremental build will skip running tests if no source code has changed, marking the task as UP-TO-DATE. This can be inconvenient during debugging.

To force your tests to run every time, you can temporarily add the following configuration to your build.gradle.kts file:

Quick Alternative: For a single re-run without modifying build files, you can use the --rerun flag from the command line:

**Examples:**

Example 1 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {   useJUnitPlatform()}
```

Example 2 (kotlin):
```kotlin
dependencies {   testImplementation("io.kotest:kotest-runner-junit5:<kotest-version>")}
```

Example 3 (kotlin):
```kotlin
plugins {   id("io.kotest").version("<kotest-version>")}
```

Example 4 (kotlin):
```kotlin
plugins {   id("io.kotest").version("<kotest-version>")   id("com.google.devtools.ksp").version("<ksp-version>")}
```

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/next/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - System Property​
  - Config​

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are three values: SingleInstance, InstancePerLeaf, and InstancePerTest.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed three times as the same instance is used for all tests.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({ isolationMode = IsolationMode.SingleInstance // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({   val id = UUID.randomUUID()   "a" should {      println(id)      "b" {         println(id)      }      "c" {         println(id)      }   }})
```

Example 4 (kotlin):
```kotlin
class InstancePerTestExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerTest  init {    "a" should {      println("Hello")      "b" {        println("From")      }      "c" {        println("Sam")      }    }  }}
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class AutoCloseExample : FreeSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Marking Tests​
- Running with Tags​
- Tag Expression Operators​
- Tagging All Tests​
- Tagging a Spec​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

Test cases can then be marked with tags using the config function:

Then by invoking the test runner with a system property of kotest.tags you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Please pay attention to the use of upper case and lower case! If two tag objects have the same simple name (in different name spaces) they are treated as the same tag.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle like this:

Tags can also be included/excluded in runtime (for example, if you're running a project configuration instead of properties) through the RuntimeTagExtension:

Operators (in descending order of precedence)

You can add a tag to all tests in a spec using the tags function in the spec itself. For example:

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

There are two annotations you can add to a spec class itself - @Tags and @RequiresTag - which accept one or more tag names as their arguments.

The first tag - @Tags - will be applied to all tests in the class, however this will only stop a spec from being instantiated if we can guarantee that no tests would be executed (because a tag is being explicitly excluded).

Consider the following example:

The second tag - @RequiresTag - only checks that all the referenced tags are present and if not, will skip the spec.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use these annotations you pass the tag string name, not the tag itself. This is due to Kotlin annotations only allow "primitive" arguments

Special attention is needed in your gradle configuration

To use System Properties (-Dx=y), your gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows: Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
import io.kotest.specs.StringSpecclass MyTest : StringSpec() {  init {    "should run on Windows".config(tags = setOf(Windows)) {      // ...    }    "should run on Linux".config(tags = setOf(Linux)) {      // ...    }    "should run on Windows and Linux".config(tags = setOf(Windows, Linux)) {      // ...    }  }}
```

Example 4 (unknown):
```unknown
gradle test -Dkotest.tags="Linux & !Database"
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/6.0/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class AutoCloseExample : FreeSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/6.0/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function TestContext -> Unit which contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are not defined manually, but instead using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a class that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Note that tests must be defined inside an init {} block or a class body lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
- DSL Methods​
  - Lambda Type-aliases​
- Method Overrides​
- Available Hooks​
  - Test Lifecycle Hooks​
  - Spec Lifecycle Hooks​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. At the end of this section is a list of the available hooks and when they are executed.

For more advanced cases, such as writing distributable plugins, re-usable hooks, or for events that take place outside a spec (such as project-started or project-finished) take a look at extensions.

There are generally two ways to define these hooks in Kotest that are functionally equivalent but different in style. Pick whichever you and your team prefer.

The first is to use the DSL methods available inside a Spec that accept a lambda for the hook logic. For example, we can invoke beforeTest or afterTest (or others) directly alongside our tests.

You can use afterProject as a DSL method but there is no equivalent beforeProject, because by the time the framework is at the stage of executing a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The beforeTest hook accepts a function of type suspend (TestCase) -> Unit. There are typealiases for each of the function signatures to keep your code simple.

For example, to create a re-usable beforeTest lambda:

The second way to create hooks is to override the appropriate function in the Spec. For example, to add a before-test hook, we can override the beforeTest function:

Kotest provides callbacks for various test and spec events.

To understand all callbacks correctly it's important to have a good understanding of the two possible TestType values:

Notice that before-each and before-container are constrained to a particular test-type (leaf or container), whereas before-any will be invoked for both. The same applies to after-each, after-container and after-any.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : FreeSpec() {  init {    beforeTest {      println("Starting a test $it")    }    afterTest { (test, result) ->      println("Finished spec with result $result")    }    "this test" - {      "be alive" {        println("Johnny5 is alive!")      }    }  }}
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : FreeSpec({   // used once   beforeTest(startTest)   "test1" { }})class OtherSpec : FreeSpec({   // used again   beforeTest(startTest)   "test2" { }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : FreeSpec() {    override suspend fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" - {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listeners for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a default TestCaseConfig for all test cases of a Spec:

Overriding the defaultTestCaseConfig function:

Or via assignment to the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10, threads = 2) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec : StringSpec() {  override fun defaultTestCaseConfig() = TestCaseConfig(invocations = 3)  init {    // your test cases ...  }}
```

---

## Concurrency | Kotest

**URL:** https://kotest.io/docs/framework/concurrency6.html

**Contents:**
- Concurrency
- Concurrency Modes​
  - Spec Concurrency Mode​
  - Test Concurrency Mode​
    - Project-wide configuration​
    - Package-level configuration​
    - Spec-level configuration​
- Isolation​
- Examples​
  - Example: Running tests within a spec concurrently​

This document describes the new concurrency features introduced in Kotest 6.0. If you are using an earlier version of Kotest, please refer to the previous concurrency documentation.

Concurrency is at the heart of Kotlin, with compiler support for continuations (suspend functions), enabling the powerful coroutines library, in addition to the standard Java concurrency tools.

So it is expected that a Kotlin test framework should offer full support for executing tests concurrently, whether that is through traditional blocking calls or suspendable functions.

Kotest offers the following features:

These features are orthogonal but complimentary.

By default, Kotest will execute each test case sequentially using Dispatchers.Default. This means if a test suspends or blocks, the whole test suite will suspend or block until that test resumes.

This is the safest default to use, since it places no burden or expectation on the user to write thread-safe tests. For example, tests can share state or use instance fields which are not thread safe. It won't subject your tests to race conditions or require you to know Java's memory model. Specs can use before and after methods confidently knowing they won't interfere with each other.

However, some users will want to run tests concurrently to reduce the total execution time of their test suite. This is especially true when testing code that suspends or blocks - the performance gains from allowing tests to run concurrently can be significant.

The concurrency modes described below are only available on the JVM platform. On other platforms, tests will always run sequentially.

Kotest provides two types of concurrency modes:

Spec concurrency mode determines whether multiple specs can be executed at the same time. There are three options:

You can configure the spec concurrency mode in your project config:

Or for limited concurrency:

Test concurrency mode determines whether multiple root tests within a spec can be executed at the same time. Note that nested tests (tests defined within other tests) are not affected by this setting; they will always run sequentially.

There are three options:

You can configure the test concurrency mode at different levels:

This will apply for all specs and tests in the project unless overridden at a lower level.

Package-level configuration allows you to set the test execution mode for all specs in a specific package, and is only available on the JVM platform.

You can configure test concurrency mode for a specific spec in two ways:

Sometimes you may have some tests which are not safe to run concurrently with other tests (maybe they mutate some external state or something along those lines), even if the rest of the test suite is still safe to run concurrently.

To support this, Kotest allows bifurcating specs into two contexts - those which can run concurrently with other specs, and those which should run sequentially - in isolation.

To mark a spec as being isolated, add the @Isolate annotation to the spec class:

By default, all specs are not isolated and are eligible to run concurrently if concurrency modes are enabled as per the above docs. Isolated specs will execute first sequentially, before the non isolated specs run together.

In Kotest 6.1 the project config object has a setting concurrencyOrder which can be used to control if isolated specs should be executed first or last.

Kotest allows you to customize the coroutine dispatcher used for executing specs and tests through the CoroutineDispatcherFactory feature. This gives you fine-grained control over the execution context of your tests.

The CoroutineDispatcherFactory interface provides methods to switch the CoroutineDispatcher used for:

The CoroutineDispatcherFactory interface has two main methods:

When a CoroutineDispatcherFactory is configured, Kotest will use it to determine which dispatcher to use when executing specs and tests.

You can configure a CoroutineDispatcherFactory at different levels:

Kotest provides a built-in implementation called ThreadPerSpecCoroutineContextFactory that creates a dedicated thread per spec.

You can create your own custom implementation to suit your specific needs:

The coroutineDispatcherFactory feature is useful for:

When working with blocking code in tests, you may encounter issues with timeouts not working as expected. This is because coroutine timeouts are cooperative by nature, meaning they rely on the coroutine to yield control back to the scheduler.

To address this issue, Kotest provides a blockingTest mode that can be enabled on a per-test basis:

When blockingTest is set to true:

The blockingTest mode is only necessary when you're using blocking calls in your tests. For tests that use suspending functions, the regular timeout mechanism works fine without needing to enable this mode.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyProjectConfig : AbstractProjectConfig() {    override val specExecutionMode = SpecExecutionMode.Concurrent}
```

Example 2 (kotlin):
```kotlin
class MyProjectConfig : AbstractProjectConfig() {    override val specExecutionMode = SpecExecutionMode.LimitedConcurrency(4) // Run up to 4 specs concurrently}
```

Example 3 (kotlin):
```kotlin
class MyProjectConfig : AbstractProjectConfig() {    override val testExecutionMode = TestExecutionMode.Concurrent}
```

Example 4 (kotlin):
```kotlin
class MyPackageConfig : AbstractPackageConfig() {    override val testExecutionMode = TestExecutionMode.Concurrent}
```

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
    - DSL Methods​
    - DSL methods with functions​
    - Overriding callback functions in a Spec​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. For more advanced cases, such as writing distributable plugins or re-usable hooks, one can use extensions.

At the end of this section is a list of the available hooks and when they are executed.

There are several ways to use hooks in Kotest:

The first and simplest, is to use the DSL methods available inside a Spec which create and register a TestListener for you. For example, we can invoke beforeTest or afterTest (and others) directly alongside our tests.

Behind the scenes, these DSL methods will create an instance of TestListener, overriding the appropriate functions, and ensuring that this test listener is registered to run.

You can use afterProject as a DSL method which will create an instance of ProjectListener, but there is no beforeProject because by the time the framework is at this stage of detecting a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The BeforeTest type used on the function definition is an alias to suspend (TestCase) -> Unit to keep things simple. There are aliases for the types of each of the callbacks.

The second, related, method is to override the callback functions in the Spec. This is essentially just a variation on the first method.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : WordSpec({  beforeTest {    println("Starting a test $it")  }  afterTest { (test, result) ->    println("Finished spec with result $result")  }  "this test" should {    "be alive" {      println("Johnny5 is alive!")    }  }})
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : WordSpec({   // used once   beforeTest(startTest)   "this test" should {      "be alive" {         println("Johnny5 is alive!")      }   }})class OtherSpec : WordSpec({   // used twice   beforeTest(startTest)   "this test" should {      "fail" {         fail("boom")      }   }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : WordSpec() {    override fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" should {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Concurrency | Kotest

**URL:** https://kotest.io/docs/next/framework/concurrency6.html

**Contents:**
- Concurrency
- Concurrency Modes​
  - Spec Concurrency Mode​
  - Test Concurrency Mode​
    - Project-wide configuration​
    - Package-level configuration​
    - Spec-level configuration​
- Isolation​
- Examples​
  - Example: Running tests within a spec concurrently​

This document describes the new concurrency features introduced in Kotest 6.0. If you are using an earlier version of Kotest, please refer to the previous concurrency documentation.

Concurrency is at the heart of Kotlin, with compiler support for continuations (suspend functions), enabling the powerful coroutines library, in addition to the standard Java concurrency tools.

So it is expected that a Kotlin test framework should offer full support for executing tests concurrently, whether that is through traditional blocking calls or suspendable functions.

Kotest offers the following features:

These features are orthogonal but complimentary.

By default, Kotest will execute each test case sequentially using Dispatchers.Default. This means if a test suspends or blocks, the whole test suite will suspend or block until that test resumes.

This is the safest default to use, since it places no burden or expectation on the user to write thread-safe tests. For example, tests can share state or use instance fields which are not thread safe. It won't subject your tests to race conditions or require you to know Java's memory model. Specs can use before and after methods confidently knowing they won't interfere with each other.

However, some users will want to run tests concurrently to reduce the total execution time of their test suite. This is especially true when testing code that suspends or blocks - the performance gains from allowing tests to run concurrently can be significant.

The concurrency modes described below are only available on the JVM platform. On other platforms, tests will always run sequentially.

Kotest provides two types of concurrency modes:

Spec concurrency mode determines whether multiple specs can be executed at the same time. There are three options:

You can configure the spec concurrency mode in your project config:

Or for limited concurrency:

Test concurrency mode determines whether multiple root tests within a spec can be executed at the same time. Note that nested tests (tests defined within other tests) are not affected by this setting; they will always run sequentially.

There are three options:

You can configure the test concurrency mode at different levels:

This will apply for all specs and tests in the project unless overridden at a lower level.

Package-level configuration allows you to set the test execution mode for all specs in a specific package, and is only available on the JVM platform.

You can configure test concurrency mode for a specific spec in two ways:

Sometimes you may have some tests which are not safe to run concurrently with other tests (maybe they mutate some external state or something along those lines), even if the rest of the test suite is still safe to run concurrently.

To support this, Kotest allows bifurcating specs into two contexts - those which can run concurrently with other specs, and those which should run sequentially - in isolation.

To mark a spec as being isolated, add the @Isolate annotation to the spec class:

By default, all specs are not isolated and are eligible to run concurrently if concurrency modes are enabled as per the above docs. Isolated specs will execute first sequentially, before the non isolated specs run together.

In Kotest 6.1 the project config object has a setting concurrencyOrder which can be used to control if isolated specs should be executed first or last.

Kotest allows you to customize the coroutine dispatcher used for executing specs and tests through the CoroutineDispatcherFactory feature. This gives you fine-grained control over the execution context of your tests.

The CoroutineDispatcherFactory interface provides methods to switch the CoroutineDispatcher used for:

The CoroutineDispatcherFactory interface has two main methods:

When a CoroutineDispatcherFactory is configured, Kotest will use it to determine which dispatcher to use when executing specs and tests.

You can configure a CoroutineDispatcherFactory at different levels:

Kotest provides a built-in implementation called ThreadPerSpecCoroutineContextFactory that creates a dedicated thread per spec.

You can create your own custom implementation to suit your specific needs:

The coroutineDispatcherFactory feature is useful for:

When working with blocking code in tests, you may encounter issues with timeouts not working as expected. This is because coroutine timeouts are cooperative by nature, meaning they rely on the coroutine to yield control back to the scheduler.

To address this issue, Kotest provides a blockingTest mode that can be enabled on a per-test basis:

When blockingTest is set to true:

The blockingTest mode is only necessary when you're using blocking calls in your tests. For tests that use suspending functions, the regular timeout mechanism works fine without needing to enable this mode.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyProjectConfig : AbstractProjectConfig() {    override val specExecutionMode = SpecExecutionMode.Concurrent}
```

Example 2 (kotlin):
```kotlin
class MyProjectConfig : AbstractProjectConfig() {    override val specExecutionMode = SpecExecutionMode.LimitedConcurrency(4) // Run up to 4 specs concurrently}
```

Example 3 (kotlin):
```kotlin
class MyProjectConfig : AbstractProjectConfig() {    override val testExecutionMode = TestExecutionMode.Concurrent}
```

Example 4 (kotlin):
```kotlin
class MyPackageConfig : AbstractPackageConfig() {    override val testExecutionMode = TestExecutionMode.Concurrent}
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- String Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 10 different styles of test layout. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

StringSpec reduces the syntax to the absolute minimum. Just write a string followed by a lambda expression with your test code.

Adding config to the test.

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks.

Tests can be disabled using the xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

If you are migrating from JUnit then AnnotationSpec is a spec that uses annotations like JUnit 4/5. Just add the @Test annotation to any function defined in the spec class.

You can also add annotations to execute something before tests/specs and after tests/specs, similarly to JUnit's

If you want to ignore a test, use @Ignore.

Although this spec doesn't offer much advantage over using JUnit, it allows you to migrate existing tests relatively easily, as you typically just need to adjust imports.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string" {        "hello".length shouldBe 5    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string".config(enabled = false, invocations = 3) {        "hello".length shouldBe 5    }})
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/6.0/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a DefaultTestConfig which will be used as the fallback for all test cases in a spec, unless overridden at the test level.

Set the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    defaultTestConfig = DefaultTestConfig(enabled = true, invocations = 3)    test("this test would run 3 times") {      // ...    }    test("this test would run 1 time because it is overriden at the test level").config(invocations = 1) {      // ...    }  }}
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/6.0/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable coroutineTestScope in test config:

Inside this test, can you retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting coroutineTestScope to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(coroutineTestScope = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(coroutineTestScope = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      coroutineTestScope = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var coroutineTestScope = true}
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is often sufficient, but if we need control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from JVM classpath discovery, or the order they appear in javascript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are explicitly executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with lowest values executed first. Any specs without such an annotation are considered "last". This option only works on the JVM. Any ties will be broken arbitrarily.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and FarTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, BooTest will execute last, as it has no annotation.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig: AbstractProjectConfig() {    override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() { }@Order(0)class BarTest: FunSpec() {}@Order(1)class FarTest : FunSpec() { }class BooTest : FunSpec() {}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​

Before data-driven-testing can be used, you need to add the module kotest-framework-datatest to your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withData function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<9> but was:<41>

In that previous example, we wrapped the withData call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withData(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withData(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable coroutineTestScope in test config:

Inside this test, you can retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting coroutineTestScope to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(coroutineTestScope = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(coroutineTestScope = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      coroutineTestScope = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var testCoroutineDispatcher = true}
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function TestContext -> Unit which contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are not defined manually, but instead using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a class that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Note that tests must be defined inside an init {} block or an init lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/concurrency/eventually.html

**Contents:**
- Eventually
- API​
- Configuration​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Predicates​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 4.6, a new experimental module has been added which contains improved utilities for testing concurrent, asynchronous, or non-deterministic code. This module is kotest-framework-concurrency and is intended as a long term replacement for the previous module. The previous utilities are still available as part of the core framework.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine, and even when complete, the thread will stay blocked until the timeout has expired.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-determistic operation. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually utility which solves the common use case of "I expect this code to pass after a short period of time".

Eventually does this by periodically invoking a given lambda until the timeout is eventually reached or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized in regardless to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration in either milliseconds (or using the Kotlin Duration type) followed by the code that should eventually pass without an exception being raised.

The second is by providing a configuration block before the test code. This method should be used when you need to set more options than just the duration.

The duration is the total amount of time to keep trying to pass the test. The interval however allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be handled, or false if the exception should bubble out.

In addition to verifying a test case eventually runs without throwing an exception, we can also verify that the return value of the test is as expected - and if not, consider that iteration a failure and try again.

For example, here we continue to append "x" to a string until the result of the previous iteration is equal to "xxx".

We can attach a listener, which will be invoked on each iteration, with the state of that iteration. The state object contains the last exception, last value, iteration count and so on.

Sharing the configuration for eventually is a breeze with the EventuallyConfig data class. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

Here we can see sharing of configuration can be useful to reduce duplicate code while allowing flexibility for things like custom logging per test suite for clear test logs.

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5000) { // duration in millis  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
eventually({  duration = 5000  interval = 1000.fixed()}) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
eventually({  duration = 5000  initialDelay = 1000}) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 4 (kotlin):
```kotlin
eventually({  duration = 8000  retries = 10  suppressExceptions = setOf(UserNotFoundException::class)}) {  userRepository.getById(1).name shouldNotBe "bob"}
```

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/project-config.html

**Contents:**
- Project Level Config
- Runtime Detection​
- Parallelism​
- Assertion Mode​
- Global Assert Softly​
- Duplicate Test Name Handling​
- Fail On Ignored Tests​
- Ordering​
  - Test Ordering​
  - Spec Ordering​

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project level configuration can be used by creating an object or class that extends from AbstractProjectConfig.

Any configuration set at the Spec level or directly on a test will override the config specified at the project level.

Some configuration options available in KotestProjectConfig include parallelism of tests, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions.

At runtime, Kotest will scan for classes that extend AbstractProjectConfig and instantiate them, using any configuration values defined in those classes.

You can create more than one config class in different modules, and any on the current classpath will be detected and configs merged. This is effective for allowing common config to be placed into a root module. In the case of clashes, one value will be arbitrarily picked, so it is not recommended adding competing settings to different configs.

If you have a large project, then you may wish to disable the auto scanning for these config classes if it is incurring a significant startup cost. You can do this by setting a system property or environment variable kotest.framework.classpath.scanning.config.disable to true.

Once auto scanning is disabled, if you wish to still use project config, you can specify a well known class name which Kotest will reflectively instantiate. The system property or environment variable to use is kotest.framework.config.fqn.

For example, setting:

Will disable runtime scanning, and look for a class com.wibble.KotestConfig. The class must still inherit AbstractProjectConfig.

Another related setting is kotest.framework.classpath.scanning.autoscan.disable which can also be set to false for speed. With auto scan disabled, Kotest will not scan the classpath looking for for @AutoScan annotated extensions.

System properties set in your gradle file won't be picked up by the intellij plugin if you have that installed. Instead, look to specify the properties inside a kotest.properties file. Full details here.

You can ask Kotest to run specs in parallel to take advantage of modern cpus with several cores by setting the parallelism level (default is 1). Tests inside a spec are always executed sequentially.

To do this, override parallelism inside your config and set it to a value higher than 1. The number set is the number of concurrently executing specs. For example.

An alternative way to enable this is the system property kotest.framework.parallelism which will always (if defined) take priority over the value here.

Some tests may not play nice in parallel, so you can opt out individual specs and force them to be executed in isolation by using the @DoNotParallelize annotation on the spec.

This is only available on the JVM target.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to opt-in to the assertion mode when enabled.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

By default, Kotest will rename a test if it has the same name as another test in the same scope. It will append _1, _2 and so on to the test name. This is useful for automatically generated tests.

You can change this behavior globally by setting duplicateTestNameMode to either DuplicateTestNameMode.Error or DuplicateTestNameMode.Warn.

Error will fail the test suite on a repeated name, and warn will rename but output a warning.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

Kotest supports ordering both specs and tests independently.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

Test names can be adjusted in several ways.

Test names case can be controlled by changing the value of testNameCase.

By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

Another using test name option is testNameAppendTags which, when set to true, will include any applicable tags in the test name. For example, if a test foo was defined in a spec with the tags linux and spark then the test name would be adjusted to be foo [linux, spark]

This setting can also be set using a system property or environment variable kotest.framework.testname.append.tags to true.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is my test case. By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

**Examples:**

Example 1 (unknown):
```unknown
kotest.framework.classpath.scanning.config.disable=truekotest.framework.config.fqn=com.wibble.KotestConfig
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val parallelism = 3}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val assertionMode = AssertionMode.Error}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {    override val globalAssertSoftly = true}
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is often sufficient, but if we need control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from JVM classpath discovery, or the order they appear in javascript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are explicitly executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with lowest values executed first. Any specs without such an annotation are considered "last". This option only works on the JVM. Any ties will be broken arbitrarily.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and FarTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, BooTest will execute last, as it has no annotation.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig: AbstractProjectConfig() {    override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() { }@Order(0)class BarTest: FunSpec() {}@Order(1)class FarTest : FunSpec() { }class BooTest : FunSpec() {}
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/6.0/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Fail On Empty Test Suite | Kotest

**URL:** https://kotest.io/docs/framework/fail-on-empty-test-suite.html

**Contents:**
- Fail On Empty Test Suite

To ensure that a project always executes at least one test, you can enable failOnEmptyTestSuite in project config.

If this is set to true and a module has no tests executed then the build will fail.

A module is considered empty if no tests are executed regardless of whether tests are defined. This is useful to catch scenarios were tests are being filtered out erroneously, such as by environment specific settings.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override val failOnEmptyTestSuite = true}
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listener for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Config Dump | Kotest

**URL:** https://kotest.io/docs/framework/config-dump.html

**Contents:**
- Config Dump

Kotest can optionally print the configuration that will be used for the test run when the test engine starts up. To do this, set a system property or environment variable, with the name kotest.framework.dump.config and the value true.

For example, with gradle, you set the system property inside the test task configuration block.

When activated, you should find output like the following in standard out:

**Examples:**

Example 1 (kotlin):
```kotlin
test {  systemProperty "kotest.framework.dump.config", "true"}
```

Example 2 (kotlin):
```kotlin
~~~ Kotest Configuration ~~~-> Parallelization factor: 1-> Concurrent specs: null-> Global concurrent tests: 1-> Dispatcher affinity: true-> Coroutine debug probe: false-> Spec execution order: Lexicographic-> Default test execution order: Sequential-> Default test timeout: 600000ms-> Default test invocation timeout: 600000ms-> Default isolation mode: SingleInstance-> Global soft assertions: false-> Write spec failure file: false-> Fail on ignored tests: false-> Fail on empty test suite: false-> Duplicate test name mode: Warn-> Remove test name whitespace: false-> Append tags to test names: false-> Extensions  - io.kotest.engine.extensions.SystemPropertyTagExtension
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a DefaultTestConfig which will be used as the fallback for all test cases in a spec, unless overridden at the test level.

Set the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    defaultTestConfig = DefaultTestConfig(enabled = true, invocations = 3)    test("this test would run 3 times") {      // ...    }    test("this test would run 1 time because it is overriden at the test level").config(invocations = 1) {      // ...    }  }}
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Package Level Config | Kotest

**URL:** https://kotest.io/docs/framework/package-level-config.html

**Contents:**
- Package Level Config
- Introduction​
- Basic Usage​
- Configuration Resolution Order​
- Available Configuration Options​
- Examples​
  - Example: Setting Timeouts and Retries​
  - Example: Configuring Test Execution Mode​
  - Example: Adding Extensions for a Package​
- Package Hierarchy​

This page describes how to use package-level configuration to share configuration across multiple specs in the same package.

Package-level configuration was introduced in Kotest 6.0. If you are using an earlier version, please upgrade to take advantage of this feature.

Package-level configuration is a JVM only feature.

When writing tests, you often need to apply the same configuration to multiple test files in the same package. Instead of repeating the same configuration for each spec, or setting it at the global project level, you can use package-level configuration to define a shared configuration that applies to all specs in a specific package and its sub-packages.

Package-level configuration works by creating a class named PackageConfig that extends AbstractPackageConfig in the package where you want to apply the configuration.

To set a default configuration for all specs in a package, create a class named PackageConfig that extends AbstractPackageConfig in the target package:

With this configuration:

This configuration will also apply to all sub-packages (e.g., com.example.mypackage.subpackage).

Kotest uses the following order to resolve configuration values:

This means that more specific configurations will override more general ones. For example, if you set a timeout at both the test level and in a package-level config, the test-level timeout will be used.

AbstractPackageConfig supports the following configuration options:

When you have package-level configurations at different levels of your package hierarchy, the configuration closest to the spec's package takes precedence.

For example, if you have:

And your test is in com.example.api.v1.UserTest, then:

Kotest automatically detects classes named PackageConfig that extend AbstractPackageConfig at runtime. The detection happens when a test is executed, and Kotest looks for package configs in the package of the test and all parent packages.

For performance reasons, package configs are cached after they are first loaded, so changes to a package config class will only take effect after restarting the test run.

**Examples:**

Example 1 (kotlin):
```kotlin
// In package: com.example.mypackageclass PackageConfig : AbstractPackageConfig() {  override val timeout = 5.seconds  override val invocations = 2  override val failfast = true}
```

Example 2 (kotlin):
```kotlin
// In package: com.example.api.testsclass PackageConfig : AbstractPackageConfig() {  // All API tests might need longer timeouts and retries  override val timeout = 30.seconds  override val retries = 3}
```

Example 3 (kotlin):
```kotlin
// In package: com.example.unit.testsclass PackageConfig : AbstractPackageConfig() {  // Run all unit tests concurrently for faster execution  override val testExecutionMode = TestExecutionMode.Concurrent}
```

Example 4 (kotlin):
```kotlin
// In package: com.example.database.testsclass PackageConfig : AbstractPackageConfig() {  // Add a database container for all database tests  override val extensions = listOf(    ContainerExtension(PostgreSQLContainer<Nothing>().withDatabaseName("testdb"))  )}
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- String Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 10 different styles of test layout. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

StringSpec reduces the syntax to the absolute minimum. Just write a string followed by a lambda expression with your test code.

Adding config to the test.

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks.

Tests can be disabled using the xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

If you are migrating from JUnit then AnnotationSpec is a spec that uses annotations like JUnit 4/5. Just add the @Test annotation to any function defined in the spec class.

You can also add annotations to execute something before tests/specs and after tests/specs, similarly to JUnit's

If you want to ignore a test, use @Ignore.

Although this spec doesn't offer much advantage over using JUnit, it allows you to migrate existing tests relatively easily, as you typically just need to adjust imports.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string" {        "hello".length shouldBe 5    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string".config(enabled = false, invocations = 3) {        "hello".length shouldBe 5    }})
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/6.0/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listeners for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class StringSpecExample : StringSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is often sufficient, but if we need control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from JVM classpath discovery, or the order they appear in javascript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are explicitly executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with lowest values executed first. Any specs without such an annotation are considered "last". This option only works on the JVM. Any ties will be broken arbitrarily.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and FarTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, BooTest will execute last, as it has no annotation.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig: AbstractProjectConfig() {    override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() { }@Order(0)class BarTest: FunSpec() {}@Order(1)class FarTest : FunSpec() { }class BooTest : FunSpec() {}
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Spec Ordering | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/spec-ordering.html

**Contents:**
- Spec Ordering
  - Annotated Example​

By default, the ordering of Spec classes is not defined. This means they are essentially random, in whatever order the discovery mechanism finds them.

This is often sufficient, but if we need control over the execution order of specs, we can do this by specifying the order in project config.

There are several options.

Undefined - This is the default. The order of specs is undefined and will execute in the order they are discovered at runtime. Eg either from JVM classpath discovery, or the order they appear in javascript files.

Lexicographic - Specs are ordered lexicographically.

Random - Specs are explicitly executed in a random order.

Annotated - Specs are ordered using the @Order annotation added at the class level, with lowest values executed first. Any specs without such an annotation are considered "last". This option only works on the JVM. Any ties will be broken arbitrarily.

Given the following specs annotated with @Order.

BarTest will be executed first, as it has the lowest order value. FooTest and FarTest will be executed next, as they have the next lowest order values, although their values are both 1 so the order between them is undefined. Finally, BooTest will execute last, as it has no annotation.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig: AbstractProjectConfig() {    override val specExecutionOrder = ...}
```

Example 2 (kotlin):
```kotlin
@Order(1)class FooTest : FunSpec() { }@Order(0)class BarTest: FunSpec() {}@Order(1)class FarTest : FunSpec() { }class BooTest : FunSpec() {}
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
    - DSL Methods​
    - DSL methods with functions​
    - Overriding callback functions in a Spec​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. For more advanced cases, such as writing distributable plugins or re-usable hooks, one can use extensions.

At the end of this section is a list of the available hooks and when they are executed.

There are several ways to use hooks in Kotest:

The first and simplest, is to use the DSL methods available inside a Spec which create and register a TestListener for you. For example, we can invoke beforeTest or afterTest (and others) directly alongside our tests.

Behind the scenes, these DSL methods will create an instance of TestListener, overriding the appropriate functions, and ensuring that this test listener is registered to run.

You can use afterProject as a DSL method which will create an instance of ProjectListener, but there is no beforeProject because by the time the framework is at this stage of detecting a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The BeforeTest type used on the function definition is an alias to suspend (TestCase) -> Unit to keep things simple. There are aliases for the types of each of the callbacks.

The second, related, method is to override the callback functions in the Spec. This is essentially just a variation on the first method.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : WordSpec({  beforeTest {    println("Starting a test $it")  }  afterTest { (test, result) ->    println("Finished spec with result $result")  }  "this test" should {    "be alive" {      println("Johnny5 is alive!")    }  }})
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : WordSpec({   // used once   beforeTest(startTest)   "this test" should {      "be alive" {         println("Johnny5 is alive!")      }   }})class OtherSpec : WordSpec({   // used twice   beforeTest(startTest)   "this test" should {      "fail" {         fail("boom")      }   }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : WordSpec() {    override fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" should {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function TestContext -> Unit which contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are not defined manually, but instead using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a class that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Note that tests must be defined inside an init {} block or an init lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Advanced Extensions | Kotest

**URL:** https://kotest.io/docs/framework/extensions/advanced-extensions.html

**Contents:**
- Advanced Extensions

This table lists more advanced extensions that can be used to hook into the Engine itself to:

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​
  - Callbacks​

Before data-driven-testing can be used, you need to add the module kotest-framework-datatest to your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withData function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<9> but was:<41>

In that previous example, we wrapped the withData call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

If you wish to have before / after callbacks in data-driven tests, then you can use the standard beforeTest / afterTest support. Every test created using data-driven testing acts the same way as a regular test, so all standard callbacks work as if you had written all the test by hand.

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withData(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withData(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features, except for fakery which allows to build test doubles. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Marking Tests​
- Running with Tags​
- Tag Expression Operators​
- Tagging All Tests​
- Tagging a Spec​
  - Inheriting tags​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

Test cases can then be marked with tags using the config function:

Then by invoking the test runner with a system property of kotest.tags you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Please pay attention to the use of upper case and lower case! If two tag objects have the same simple name (in different name spaces) they are treated as the same tag.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle like this:

Tags can also be included/excluded in runtime (for example, if you're running a project configuration instead of properties) through the RuntimeTagExtension:

Operators (in descending order of precedence)

You can add a tag to all tests in a spec using the tags function in the spec itself. For example:

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

There are two annotations you can add to a spec class itself - @Tags and @RequiresTag - which accept one or more tag names as their arguments.

The first tag - @Tags - will be applied to all tests in the class, however this will only stop a spec from being instantiated if we can guarantee that no tests would be executed (because a tag is being explicitly excluded).

Consider the following example:

The second tag - @RequiresTag - only checks that all the referenced tags are present and if not, will skip the spec.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use these annotations you pass the tag string name, not the tag itself. This is due to Kotlin annotations only allow "primitive" arguments

By default, the @Tags annotation will only be considered on the immediate Spec which it was applied to. However, a Spec can also inherit tags from superclasses and superinterfaces. To enable this, toggle tagInheritance = true in your project config

Special attention is needed in your gradle configuration

To use System Properties (-Dx=y), your gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows: Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
import io.kotest.specs.StringSpecclass MyTest : StringSpec() {  init {    "should run on Windows".config(tags = setOf(Windows)) {      // ...    }    "should run on Linux".config(tags = setOf(Linux)) {      // ...    }    "should run on Windows and Linux".config(tags = setOf(Windows, Linux)) {      // ...    }  }}
```

Example 4 (unknown):
```unknown
gradle test -Dkotest.tags="Linux & !Database"
```

---

## Setup | Kotest

**URL:** https://kotest.io/docs/6.0/framework/project-setup.html

**Contents:**
- Setup
- Re-running tests​

The Kotest test framework is supported on JVM, Javascript, Native and Wasm. To enable Kotest for multiple platforms, follow the steps for the platform you are targeting as detailed in the following tabs.

The KMP support in Kotest 6.0 has changed from the previous versions. There is no longer a compiler plugin but a simplified setup. Please see the rest of this page for details on how to configure Kotest for KMP in Kotest 6.0 and later.

When running the Gradle test task, Gradle will cache the output and report no tests executed if no source code has changed. See the section on rerunning tests for details on how to disable this behaviour.

A working project with JVM support can be found here: https://github.com/kotest/kotest-examples

Kotest on the JVM has two ways for running tests. One uses the Kotest gradle plugin, which provides detailed test output in the console, and a rich experience in Intellij (in conjuction with the Intellij Kotest plugin). The other option uses the JUnit Platform gradle plugin which is ubiquitous in the JVM ecosystem but lacks some features of the Kotest gradle plugin.

To use the Kotest gradle plugin, add the following to your build.gradle.kts file:

Add the following dependency to your build:

And then execute the jvmKotest task in gradle, or run tests directly from the IDE.

To use the JUnit Platform plugin, add the following to your build.gradle.kts file:

Add the following dependency to your build:

And then execute the test task in gradle, or run tests directly from the IDE.

A working JS project can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest or jsTest source set:

Tests can be placed in either commonTest or jsTest. Run your tests using the jsTest gradle task.

The JS test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to JS code.

A working WasmJS project can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest or wasmJsTest source set:

Tests can be placed in either commonTest or wasmJsTest. Run your tests using the wasmJsTest gradle task.

The WasmJS test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to Wasm code.

A working native project with linux, windows and macos configured, with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest, nativeTest or platform specific sourceset:

Tests can be placed in either commonTest or a specific native sourceset. Run your tests using the standard test tasks, for example linuxX86Test.

The native test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to native code.

Currently, only Unit tests are supported in Kotest. The following steps enable Kotest to be used for unit tests, where the Android framework is not needed or is mocked that usually reside in the src/test folder of your module.

Kotest on Android uses the JUnit Platform gradle plugin. This requires configuring the android test options block in your build file and then adding the Kotest junit5 runner dependency.

A working Android project with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples

To configure the test framework for multiplatform, combie the steps for JVM, JS and Native as detailed in the previous tabs.

A working multiplatform project with JVM, JS and native targets, and unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples

Add the Kotest gradle plugin and Google KSP plugin to to your build.

Add the kotest-framework-engine dependency to your commonTest source set:

Tests can be placed in either commonTest or a platform specific directory such as jsTest or macosX64Test etc. Run your tests using the gradle check task, or a platform specific test task such as macosX64Test

The JS, Wasm and native test engines are feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to non-JVM platforms.

By default, Gradle's incremental build will skip running tests if no source code has changed, marking the task as UP-TO-DATE. This can be inconvenient during debugging.

To force your tests to run every time, you can temporarily add the following configuration to your build.gradle.kts file:

Quick Alternative: For a single re-run without modifying build files, you can use the --rerun flag from the command line:

**Examples:**

Example 1 (kotlin):
```kotlin
plugins {   id("io.kotest") version "<kotest-version>"}
```

Example 2 (kotlin):
```kotlin
dependencies {   testImplementation("io.kotest:kotest-framework-engine:$version")}
```

Example 3 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {   useJUnitPlatform()}
```

Example 4 (kotlin):
```kotlin
dependencies {   testImplementation("io.kotest:kotest-runner-junit5:$version")}
```

---

## Shared Test Config | Kotest

**URL:** https://kotest.io/docs/framework/sharedtestconfig.html

**Contents:**
- Shared Test Config
- Introduction​
- Basic Usage​
- Available Configuration Options​
- Examples​
  - Example: Setting Retry Configuration​
  - Example: Setting Timeouts and Invocations​
  - Example: Using Tags and Assertion Mode​
- Overriding Default Configuration​

This page describes how to use DefaultTestConfig to share test configuration across multiple test cases in your specs.

This feature is available in Kotest 6.0 and later.

When writing tests, you often need to apply the same configuration to multiple test files. Instead of repeating the same configuration for each test, you can use DefaultTestConfig to define a shared configuration that applies to all tests in a spec.

DefaultTestConfig is a data class that allows for the configuration of test cases to be easily shared.

Each of the configuration values can be overridden on a per-test basis, but if you wish to have a common set of defaults that are shared across several tests, then you can create an instance of this class and declare it in each of the specs that you wish to share the configuration.

To set a default configuration for all tests in a spec, assign a DefaultTestConfig instance to the defaultTestConfig property in your spec:

DefaultTestConfig supports the following configuration options:

Individual tests can override any part of the default configuration using the .config() method:

The order of lookups is as follows:

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {  init {    defaultTestConfig = DefaultTestConfig(      timeout = 2.seconds,      invocations = 3,      threads = 2    )    // All tests in this spec will use the above configuration by default    test("test with default config") {      // This test will run 3 times with a timeout of 2 seconds    }    // You can still override the default config for specific tests    test("test with custom config").config(timeout = 5.seconds) {      // This test will run 3 times (from default config) with a timeout of 5 seconds    }  }}
```

Example 2 (kotlin):
```kotlin
class RetrySpec : DescribeSpec() {  init {    defaultTestConfig = DefaultTestConfig(retries = 5, retryDelay = 20.milliseconds)    describe("a flaky test") {      // This test will be retried up to 5 times with a 20ms delay between retries      it("should eventually pass") {        // Test logic here      }    }  }}
```

Example 3 (kotlin):
```kotlin
class PerformanceSpec : FreeSpec() {  init {    defaultTestConfig = DefaultTestConfig(      timeout = 1.minutes,      invocations = 10,      invocationTimeout = 5.seconds    )    "performance test" {      // This test will run 10 times, with each invocation having a 5 second timeout      // The entire test has a 1 minute timeout    }  }}
```

Example 4 (kotlin):
```kotlin
class IntegrationSpec : FunSpec() {  init {    defaultTestConfig = DefaultTestConfig(      tags = setOf(Tags.Integration, Tags.Slow),      assertionMode = AssertionMode.Error    )    test("database connection") {      // This test will be tagged as Integration and Slow      // Assertions will throw errors instead of exceptions    }  }}
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- String Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 10 different styles of test layout. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

StringSpec reduces the syntax to the absolute minimum. Just write a string followed by a lambda expression with your test code.

Adding config to the test.

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks.

Tests can be disabled using the xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

If you are migrating from JUnit then AnnotationSpec is a spec that uses annotations like JUnit 4/5. Just add the @Test annotation to any function defined in the spec class.

You can also add annotations to execute something before tests/specs and after tests/specs, similarly to JUnit's

If you want to ignore a test, use @Ignore.

Although this spec doesn't offer much advantage over using JUnit, it allows you to migrate existing tests relatively easily, as you typically just need to adjust imports.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string" {        "hello".length shouldBe 5    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string".config(enabled = false, invocations = 3) {        "hello".length shouldBe 5    }})
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, and a timeout for each test or for all tests. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : StringSpec({   "length should return size of string" {      "hello".length shouldBe 5   }   "startsWith should test for a prefix" {      "world" should startWith("wor")   }})
```

Example 2 (kotlin):
```kotlin
class StringSpecExample : StringSpec({   "maximum of two numbers" {      forAll(         row(1, 5, 5),         row(1, 0, 1),         row(0, 0, 0)      ) { a, b, max ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : StringSpec({   "should use config".config(timeout = 2.seconds, invocations = 10, threads = 2, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a default TestCaseConfig for all test cases of a Spec:

Overriding the defaultTestCaseConfig function:

Or via assignment to the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10, threads = 2) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec : StringSpec() {  override fun defaultTestCaseConfig() = TestCaseConfig(invocations = 3)  init {    // your test cases ...  }}
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function TestContext -> Unit which contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are not defined manually, but instead using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a class that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Note that tests must be defined inside an init {} block or an init lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/concurrency/eventually.html

**Contents:**
- Eventually
- API​
- Configuration​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Predicates​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 4.6, a new experimental module has been added which contains improved utilities for testing concurrent, asynchronous, or non-deterministic code. This module is kotest-framework-concurrency and is intended as a long term replacement for the previous module. The previous utilities are still available as part of the core framework.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine, and even when complete, the thread will stay blocked until the timeout has expired.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-determistic operation. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually utility which solves the common use case of "I expect this code to pass after a short period of time".

Eventually does this by periodically invoking a given lambda until the timeout is eventually reached or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized in regardless to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration in either milliseconds (or using the Kotlin Duration type) followed by the code that should eventually pass without an exception being raised.

The second is by providing a configuration block before the test code. This method should be used when you need to set more options than just the duration.

The duration is the total amount of time to keep trying to pass the test. The interval however allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be handled, or false if the exception should bubble out.

In addition to verifying a test case eventually runs without throwing an exception, we can also verify that the return value of the test is as expected - and if not, consider that iteration a failure and try again.

For example, here we continue to append "x" to a string until the result of the previous iteration is equal to "xxx".

We can attach a listener, which will be invoked on each iteration, with the state of that iteration. The state object contains the last exception, last value, iteration count and so on.

Sharing the configuration for eventually is a breeze with the EventuallyConfig data class. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

Here we can see sharing of configuration can be useful to reduce duplicate code while allowing flexibility for things like custom logging per test suite for clear test logs.

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5000) { // duration in millis  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
eventually({  duration = 5000  interval = 1000.fixed()}) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
eventually({  duration = 5000  initialDelay = 1000}) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 4 (kotlin):
```kotlin
eventually({  duration = 8000  retries = 10  suppressExceptions = setOf(UserNotFoundException::class)}) {  userRepository.getById(1).name shouldNotBe "bob"}
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​
  - Callbacks​
  - Custom case naming​

Before data-driven-testing can be used, you need to add the module kotest-framework-datatest to your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withData function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<9> but was:<41>

In that previous example, we wrapped the withData call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

If you wish to have before / after callbacks in data-driven tests, then you can use the standard beforeTest / afterTest support. Every test created using data-driven testing acts the same way as a regular test, so all standard callbacks work as if you had written all the test by hand.

If you wish or need to have data subtests named in a custom way, the easiest way to achieve it is by passing a Map to withData. The key will be used as the subtest name, and the value will be used as the actual it for the test.

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withData(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withData(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Setup | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/project-setup.html

**Contents:**
- Setup

The Kotest test framework is supported on JVM, Javascript and Native. To enable Kotest for multiple platforms, combine the steps for the individual platforms as detailed in the following tabs.

Kotest on the JVM uses the JUnit Platform gradle plugin. For Gradle 4.6 and higher this is as simple as adding useJUnitPlatform() inside the tasks with type Test and then adding the Kotest junit5 runner dependency.

If you are using Gradle + Groovy then:

Or if you are using Gradle + Kotlin then:

And then the dependency:

A working multiplatform project with JVM, native and Javascript all configured, with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples-multiplatform

Add the Kotest multiplatform gradle plugin to your build.

Add the engine dependency to your commonTest dependencies block:

Only the new IR compiler backend for Kotlin/JS is supported. If you are compiling JS with the legacy compiler backend then you will not be able to use Kotest for testing.

Write your tests using FunSpec, ShouldSpec or StringSpec. Tests can be placed in either commonTest or jsTest source sets. Run your tests using the gradle check command.

The Javascript test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to javascript code.

Tests for Javascript cannot nest tests. This is due to the underlying Javascript test runners (such as Mocha or Karma) not supporting promises in parent tests, which is incompatible with coroutines and in Kotest every test scope is a coroutine. This is why the supported specs are limited to FunSpec, ShouldSpec and StringSpec.

The IntelliJ Kotest plugin does not support running common, native or JS tests directly from the IDE using the green run icons. Only execution via gradle is supported.

A working multiplatform project with JVM, native and Javascript all configured, with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples-multiplatform

Add the Kotest multiplatform gradle plugin to your build.

Add the engine dependency to your commonTest dependencies block:

Tests can be placed in either commonTest or a specific native sourceset. Run your tests using the gradle check command.

The native test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to native code.

The IntelliJ Kotest plugin does not support running common, native or JS tests from the IDE. You will need to use the gradle check task.

For maven you must configure the surefire plugin for junit tests.

And then add the Kotest JUnit5 runner to your dependencies section.

Kotest is a multiplatform project. If you are unfamiliar with this, then Kotlin compiles to different targets - JVM, JS, Native, iOS and so on. Since version 5.9.0, Kotest includes a workaround so that the kotest-runner-junit5 module can be used directly, but for older versions you need to explicitly depend on the modules that end with -JVM, such as kotest-property-jvm_

Currently, only JVM tests are officially supported in Kotest. We are open to suggestions on how to support UI tests.

The following steps enable Kotest to be used for unit and integration tests, where the Android framework is not needed or is mocked that usually reside in the src/test folder of your module.

Kotest on Android uses the JUnit Platform gradle plugin. This requires configuring the android test options block in your build file and then adding the Kotest junit5 runner dependency.

To configure the test framework for both JS and JVM, you just combine copy the steps for JVM and JS.

**Examples:**

Example 1 (unknown):
```unknown
test {   useJUnitPlatform()}
```

Example 2 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {   useJUnitPlatform()}
```

Example 3 (bash):
```bash
testImplementation 'io.kotest:kotest-runner-junit5:$version'
```

Example 4 (kotlin):
```kotlin
plugins {  id("io.kotest.multiplatform") version "5.0.2"}
```

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - System Property​
  - Config​

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are three values: SingleInstance, InstancePerLeaf, and InstancePerTest.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed three times as the same instance is used for all tests.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({ isolationMode = IsolationMode.SingleInstance // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({   val id = UUID.randomUUID()   "a" should {      println(id)      "b" {         println(id)      }      "c" {         println(id)      }   }})
```

Example 4 (kotlin):
```kotlin
class InstancePerTestExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerTest  init {    "a" should {      println("Hello")      "b" {        println("From")      }      "c" {        println("Sam")      }    }  }}
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​

Before data-driven-testing can be used, you need to add the module kotest-framework-datatest to your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withData function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<9> but was:<41>

In that previous example, we wrapped the withData call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withData(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withData(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - System Property​
  - Config​

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are three values: SingleInstance, InstancePerLeaf, and InstancePerTest.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed three times as the same instance is used for all tests.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({ isolationMode = IsolationMode.SingleInstance // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({   val id = UUID.randomUUID()   "a" should {      println(id)      "b" {         println(id)      }      "c" {         println(id)      }   }})
```

Example 4 (kotlin):
```kotlin
class InstancePerTestExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerTest  init {    "a" should {      println("Hello")      "b" {        println("From")      }      "c" {        println("Sam")      }    }  }}
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Reproduce Race Conditions | Kotest

**URL:** https://kotest.io/docs/framework/race_conditions.html

**Contents:**
- Reproduce Race Conditions

A simple tool to reproduce some common race conditions such as deadlocks in automated tests. Whenever multiple coroutines or threads mutate shared state, there is a possibility of race conditions. In many common cases this tool allows to reproduce them easily.

If we are continuously running these two functions in parallel, eventually there should be deadlocks, but we don't know when exactly. With the help of runInParallel we can reliably reproduce the deadlock every time we run the following code:

Let's discuss a few more advanced scenarios where reproducing race conditions comes very handy.

Without concurrency, this code will always run correctly. Let us reproduce concurrency as follows:

For another example, suppose that we need to reproduce a deadlock between two threads that are trying to modify two Postgres tables in different order.

A brute force approach would be to run this scenario many times, hoping that eventually we shall reproduce the deadlock. Eventually this should work, but we shall have to spend some time setting up the test, and we might have to wait until it does reproduce.

This is a textbook scenario of a deadlock, and it is reliably reproduced every time we run this code. All the busywork of setting up threads and synchronizing them is handled by runInParallel.

**Examples:**

Example 1 (kotlin):
```kotlin
runInParallel({ runner: ParallelRunner ->    lockResourceA()    runner.await()    lockResourceB()  },    { runner: ParallelRunner ->      lockResourceB()      runner.await()      lockResourceA()    }  )
```

Example 2 (kotlin):
```kotlin
if(canRunTask()) {    runTask()}
```

Example 3 (kotlin):
```kotlin
private data class Box(val maxCapacity: Int) {      private val items = mutableListOf<String>()      fun addItem(item: String) = items.add(item)      fun hasCapacity() = items.size < maxCapacity      fun items() = items.toList()   }(snip)"two tasks share one mutable state, both make the same decision at the same time" {  val box = Box(maxCapacity = 2)  box.addItem("apple")  runInParallel({ runner: ParallelRunner ->    val hasCapacity = box.hasCapacity()    runner.await()    if(hasCapacity) {      box.addItem("banana")    }  },    { runner: ParallelRunner ->      val hasCapacity = box.hasCapacity()      runner.await()      if(hasCapacity) {        box.addItem("orange")      }    }  )  // capacity is exceeded as a result of race condition  box.items() shouldContainExactlyInAnyOrder listOf("apple", "banana", "orange")}
```

Example 4 (kotlin):
```kotlin
// Prerequisites:executeSql(  "DROP TABLE IF EXISTS test0",  "DROP TABLE IF EXISTS test1",  "SELECT 1 AS id, 'green' AS color INTO test0",  "SELECT 1 AS id, 'yellow' AS color INTO test1",)// reproduce a deadlockvar successCount = 0var thrownExceptions = mutableListOf<Throwable>()runInParallel(  { runner ->    try {      executeSql(jdbi, "UPDATE test0 SET color = 'blue' WHERE id = 1")      jdbi.useTransaction<Exception> { handle ->        handle.execute("UPDATE test0 SET color = 'blue' WHERE id = 1")        runner.await() // wait for the other thread to do its thing        handle.execute("UPDATE test1 SET color = 'purple' WHERE id = 1")        successCount++      }    } catch (ex: Throwable) {      thrownExceptions.add(ex)    }  },  { runner ->    try {      jdbi.useTransaction<Exception> { handle ->        handle.execute("UPDATE test1 SET color = 'blue' WHERE id = 1")        runner.await() // wait for the other thread to do its thing        handle.execute("UPDATE test0 SET color = 'purple' WHERE id = 1")        successCount++      }    } catch (ex: Throwable) {      thrownExceptions.add(ex)    }  })successCount shouldBe 1thrownExceptions shouldHaveSize 1isDeadlock(thrownExceptions[0]) shouldBe true
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/next/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable coroutineTestScope in test config:

Inside this test, can you retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting coroutineTestScope to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(coroutineTestScope = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(coroutineTestScope = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      coroutineTestScope = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var coroutineTestScope = true}
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Marking Tests​
- Running with Tags​
- Tag Expression Operators​
- Tagging All Tests​
- Tagging a Spec​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

Test cases can then be marked with tags using the config function:

Then by invoking the test runner with a system property of kotest.tags you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Please pay attention to the use of upper case and lower case! If two tag objects have the same simple name (in different name spaces) they are treated as the same tag.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle like this:

Tags can also be included/excluded in runtime (for example, if you're running a project configuration instead of properties) through the RuntimeTagExtension:

Operators (in descending order of precedence)

You can add a tag to all tests in a spec using the tags function in the spec itself. For example:

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

There are two annotations you can add to a spec class itself - @Tags and @RequiresTag - which accept one or more tag names as their arguments.

The first tag - @Tags - will be applied to all tests in the class, however this will only stop a spec from being instantiated if we can guarantee that no tests would be executed (because a tag is being explicitly excluded).

Consider the following example:

The second tag - @RequiresTag - only checks that all the referenced tags are present and if not, will skip the spec.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use these annotations you pass the tag string name, not the tag itself. This is due to Kotlin annotations only allow "primitive" arguments

Special attention is needed in your gradle configuration

To use System Properties (-Dx=y), your gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows: Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
import io.kotest.specs.StringSpecclass MyTest : StringSpec() {  init {    "should run on Windows".config(tags = setOf(Windows)) {      // ...    }    "should run on Linux".config(tags = setOf(Linux)) {      // ...    }    "should run on Windows and Linux".config(tags = setOf(Windows, Linux)) {      // ...    }  }}
```

Example 4 (unknown):
```unknown
gradle test -Dkotest.tags="Linux & !Database"
```

---

## Test Output | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/test_output.html

**Contents:**
- Test Output

If you are running Kotest via Gradle's Junit Platform support, and if you are using a nested spec style, you will notice that only the leaf test name is included in output and test reports. This is a limitation of gradle which is designed around class.method test frameworks.

Until such time that Gradle improves their test integration so that tests can be arbitrarily nested, Kotest offers a workaround by allowing you to specify displayFullTestPath in project configuration.

When this setting is enabled, the test names will be the concatenation of the entire test path. So a test like this:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests: DescribeSpec({  describe("describe 1"){    it("test 1"){}    it("test 2"){}  }})
```

Example 2 (unknown):
```unknown
MyTests. describe 1 - test 1MyTests. describe 1 - test 2
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listeners for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/next/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Adding tags to tests​
  - Tagging Tests​
  - Tagging Specs​
  - @RequiresTag​
- Executing with Tags​
  - Tag Expression Operators​
  - Inheriting tags​
- Examples​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

If two tag objects have the same simple name (even in different packages) they are treated as the same tag.

Tests can be tagged at various levels. Firstly, test cases themselves can have tags added via test config. Note that any nested tags inherit tags from their parents.

Secondly, you can add tags at the spec level, either through the tags function in the spec itself, or through the @Tags annotation. Any tags added this way are applied to all tests implicitly.

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags. Therefore, do not rely on this if you want to avoid instantiating classes completely, and instead see @RequiresTag.

Any tags added via @Tags do not stop the spec from being instantiated, as the engine needs to check for any tags added via code. If you want to avoid a spec from being instantiated completely, use @RequiresTag.

Finally, you can use the @RequiresTag annotation. This only checks that all the referenced tags are present and if not, will skip the spec entirely. This is an important distinction, because with the other annotation - @Tags - the spec will still need to be instantiated in order to check for any tags added via the DSL. This can be counter-intuitive.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use annotations you pass the string name of the tag, not the tag instance itself.

By invoking the test runner with a system property of kotest.tags or an environment variable of KOTEST_TAGS you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Eg, a tag created as val mytag = NamedTag("A") would use the tag name A.

Please pay attention to the use of upper case and lower case! Tags are case-sensitive.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle using the environment variable (since Kotest 6.1.0).

Or using the system property from the command line.

Or by specifying the system property inside your build script.

Prefer the environment variable as it works on all platforms, unless you are on an earlier version of Kotest (prior to 6.1), or need to specify the tags in your Gradle build script rather than at the command line.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

Operators (in descending order of precedence)

By default, the @Tags annotation will only be considered on the immediate Spec which it was applied to. However, a Spec can also inherit tags from superclasses and superinterfaces. To enable this, toggle tagInheritance = true in your project config

Consider the following example:

Special attention is needed in your Gradle configuration when using system properties

To use System Properties (-Dx=y), Gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows : Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec() {  init {    test("should run on Windows").config(tags = setOf(Windows)) {      // ...    }    test("should run on Linux").config(tags = setOf(Linux)) {      // ...    }    context("should run on Windows and Linux").config(tags = setOf(Windows, Linux)) {      test("and nested tests") { // implicity has windows and linux tags added      }    }  }}
```

Example 4 (kotlin):
```kotlin
@Tags("Foo") // applied to all tests in this specprivate class TaggedSpec : ExpectSpec() {  init {    tags(Windows) // applied to all tests in this spec    expect("should run on Windows") {      // ...    }  }}
```

---

## Test Coroutine Dispatcher | Kotest

**URL:** https://kotest.io/docs/framework/coroutines/test-coroutine-dispatcher.html

**Contents:**
- Test Coroutine Dispatcher

A TestDispatcher is a special CoroutineDispatcher provided by the kotlinx-coroutines-test module that allows developers to control its virtual clock and skip delays.

A TestDispatcher supports the following operations:

To use a TestDispatcher for a test, you can enable coroutineTestScope in test config:

Inside this test, can you retrieve a handle to the scheduler through the extension val testCoroutineScheduler. Using this scheduler, you can then manipulate the time:

You can enable a test dispatcher for all tests in a spec by setting coroutineTestScope to true at the spec level:

Finally, you can enable test dispatchers for all tests in a module by using ProjectConfig:

**Examples:**

Example 1 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      test("foo").config(coroutineTestScope = true) {         // this test will run with a test dispatcher      }   }}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.test.testCoroutineSchedulerclass TestDispatcherTest : FunSpec() {   init {      test("advance time").config(coroutineTestScope = true) {        val duration = 1.days        // launch a coroutine that would normally sleep for 1 day        launch {          delay(duration.inWholeMilliseconds)        }        // move the clock on and the delay in the above coroutine will finish immediately.        testCoroutineScheduler.advanceTimeBy(duration.inWholeMilliseconds)        val currentTime = testCoroutineScheduler.currentTime      }   }}
```

Example 3 (kotlin):
```kotlin
class TestDispatcherTest : FunSpec() {   init {      coroutineTestScope = true      test("this test uses a test dispatcher") {      }      test("and so does this test!") {      }   }}
```

Example 4 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override var coroutineTestScope = true}
```

---

## Mocking and Kotest | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/integrations/mocking.html

**Contents:**
- Mocking and Kotest
  - Option 1 - setup mocks before tests​
  - Option 2 - reset mocks after tests​
  - Positioning the listeners​
  - Option 3 - Tweak the IsolationMode​

Kotest itself has no mock features. However, you can plug-in your favourite mocking library with ease!

Let's take for example mockk:

This example works as expected, but what if we add more tests that use that mockk?

The above snippet will cause an exception!

2 matching calls found, but needs at least 1 and at most 1 calls

This will happen because the mocks are not restarted between invocations. By default, Kotest isolates tests by creating a single instance of the spec for all the tests to run.

This leads to mocks being reused. But how can we fix this?

As for any function that is executed inside the Spec definition, you can place listeners at the end

Depending on the usage, playing with the IsolationMode for a given Spec might be a good option as well. Head over to isolation mode documentation if you want to understand it better.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 2 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    test("Saves to repository") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }    test("Saves to repository as well") {        every { repository.save(any()) } just Runs        target.save(MyDataClass("a"))        verify(exactly = 1) { repository.save(MyDataClass("a")) }    }})
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec({    lateinit var repository: MyRepository    lateinit var target: MyService    beforeTest {        repository = mockk()        target = MyService(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

Example 4 (kotlin):
```kotlin
class MyTest : FunSpec({    val repository = mockk<MyRepository>()    val target = MyService(repository)    afterTest {        clearMocks(repository)    }    test("Saves to repository") {        // ...    }    test("Saves to repository as well") {        // ...    }})
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a default TestCaseConfig for all test cases of a Spec:

Overriding the defaultTestCaseConfig function:

Or via assignment to the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10, threads = 2) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec : StringSpec() {  override fun defaultTestCaseConfig() = TestCaseConfig(invocations = 3)  init {    // your test cases ...  }}
```

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - System Property​
  - Config​

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are three values: SingleInstance, InstancePerLeaf, and InstancePerTest.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed three times as the same instance is used for all tests.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({ isolationMode = IsolationMode.SingleInstance // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({   val id = UUID.randomUUID()   "a" should {      println(id)      "b" {         println(id)      }      "c" {         println(id)      }   }})
```

Example 4 (kotlin):
```kotlin
class InstancePerTestExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerTest  init {    "a" should {      println("Hello")      "b" {        println("From")      }      "c" {        println("Sam")      }    }  }}
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listeners for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerRoot​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - Config​
  - System Property​

The isolation mode InstancePerRoot is only available in Kotest 6.0 and later, and InstancePerTest and InstancePerLeaf are now deprecated due to undefined behavior in edge cases.

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are four values: SingleInstance, InstancePerRoot, InstancePerLeaf, and InstancePerTest. Note that InstancePerLeaf and InstancePerTest are deprecated in favor of InstancePerRoot.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed four times as the same instance is used for all tests.

The InstancePerRoot isolation mode creates a new instance of the Spec class for every top level (root) test case. Each root test is executed in its own associated instance.

This mode is recommended when you want to isolate your tests but still maintain a clean structure.

In this example, the tests a, b and c will all print the same UUID, but test d will print a different UUID because it is executed in a new instance as it is a top level (aka root) test case.

This mode is deprecated due to undefined behavior on edge cases. It is recommended to use InstancePerRoot instead.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

This mode is deprecated due to undefined behavior on edge cases. It is recommended to use InstancePerRoot instead.

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({  isolationMode = IsolationMode.SingleInstance  // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({  val id = UUID.randomUUID()  "a" should {    println(id)    "b" {      println(id)    }    "c" {      println(id)    }  }  "d" should {    println(id)  }})
```

Example 4 (kotlin):
```kotlin
class InstancePerRootExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerRoot  val id = UUID.randomUUID()  init {    "a" should {      println(id)      "b" {        println(id)      }      "c" {        println(id)      }    }    "d" should {      println(id)    }  }}
```

---

## Writing Tests | Kotest

**URL:** https://kotest.io/docs/5.4.x/framework/writing-tests.html

**Contents:**
- Writing Tests
  - Nested Tests​
  - Dynamic Tests​
  - Lifecycle Callbacks​

By using the language features available in Kotlin, Kotest is able to provide a more powerful and yet simple approach to defining tests. Gone are the days when tests need to be methods defined in a Java file.

In Kotest a test is essentially just a function TestContext -> Unit which contains your test logic. Any assert statements (matchers in Kotest nomenclature) invoked in this function that throw an exception will be intercepted by the framework and used to mark that test as failed or success.

Test functions are not defined manually, but instead using the Kotest DSL, which provides several ways in which these functions can be created and nested. The DSL is accessed by creating a class that extends from a class that implements a particular testing style.

For example, using the Fun Spec style, we create test functions using the test keyword, providing a name, and the actual test function.

Note that tests must be defined inside an init {} block or an init lambda as in the previous example.

Most styles offer the ability to nest tests. The actual syntax varies from style to style, but is essentially just a different keyword used for the outer tests.

For example, in Describe Spec, the outer tests are created using the describe function and inner tests using the it function. JavaScript and Ruby developers will instantly recognize this style as it is commonly used in testing frameworks for those languages.

In Kotest nomenclature, tests that can contain other tests are called test containers and tests that are terminal or leaf nodes are called test cases. Both can contain test logic and assertions.

Since tests are just functions, they are evaluated at runtime.

This approach offers a huge advantage - tests can be dynamically created. Unlike traditional JVM test frameworks, where tests are always methods and therefore declared at compile time, Kotest can add tests conditionally at runtime.

For example, we could add tests based on elements in a list.

This would result in three tests being created at runtime. It would be the equivalent to writing:

Kotest provides several callbacks which are invoked at various points during a test's lifecycle. These callbacks are useful for resetting state, setting up and tearing down resources that a test might use, and so on.

As mentioned earlier, test functions in Kotest are labelled either test containers or test cases, in addition to the containing class being labelled a spec. We can register callbacks that are invoked before or after any test function, container, test case, or a spec itself.

To register a callback, we just pass a function to one of the callback methods.

For example, we can add a callback before and after any test case using a function literal:

Note that the order of the callbacks in the file is not important. For example, an afterEach block can be placed first in the class if you so desired.

If we want to extract common code, we can create a named function and re-use it for multiple files. For example, say we wanted to reset a database before every test in more than one file, we could do this:

For details of all callbacks and when they are invoked, see here and here.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyFirstTestClass : FunSpec({   test("my first test") {      1 + 2 shouldBe 3   }})
```

Example 2 (kotlin):
```kotlin
class NestedTestExamples : DescribeSpec({   describe("an outer test") {      it("an inner test") {        1 + 2 shouldBe 3      }      it("an inner test too!") {        3 + 4 shouldBe 7      }   }})
```

Example 3 (kotlin):
```kotlin
class DynamicTests : FunSpec({    listOf(      "sam",      "pam",      "tim",    ).forEach {       test("$it should be a three letter name") {           it.shouldHaveLength(3)       }    }})
```

Example 4 (kotlin):
```kotlin
class DynamicTests : FunSpec({   test("sam should be a three letter name") {      "sam".shouldHaveLength(3)   }   test("pam should be a three letter name") {      "pam".shouldHaveLength(3)   }   test("tim should be a three letter name") {     "tim".shouldHaveLength(3)   }})
```

---

## Temporary Files | Kotest

**URL:** https://kotest.io/docs/framework/temporary-files

**Contents:**
- Temporary Files
- Temporary Directories​

Sometimes it is required for a test to create a file and delete it after test, deleting it manually may lead to flaky test.

For example, you may be using a temporary file during a test. If the test passes successfully, your clean up code will execute and the file will be deleted. But in case the assertion fails or another error occurs, which may have caused the file to not be deleted, you will get a stale file which might affect the test on the next run (file cannot be overwritten exception and so on).

Kotest provides a function tempfile() which you can use in your Spec to create a temporary file for your tests, and which Kotest will take the responsibility of cleaning up after running all tests in the Spec. This way your tests does not have to worry about deleting the temporary file.

Similar to temp files, we can create a temp dir using tempdir().

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec({   val file = tempfile()   test("a temporary file dependent test") {      //...   }})
```

Example 2 (kotlin):
```kotlin
class MySpec : FunSpec({   val dir = tempdir()   test("a temporary dir dependent test") {      //...   }})
```

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/next/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerRoot​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - Config​
  - System Property​

The isolation mode InstancePerRoot is only available in Kotest 6.0 and later, and InstancePerTest and InstancePerLeaf are now deprecated due to undefined behavior in edge cases.

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are four values: SingleInstance, InstancePerRoot, InstancePerLeaf, and InstancePerTest. Note that InstancePerLeaf and InstancePerTest are deprecated in favor of InstancePerRoot.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed four times as the same instance is used for all tests.

The InstancePerRoot isolation mode creates a new instance of the Spec class for every top level (root) test case. Each root test is executed in its own associated instance.

This mode is recommended when you want to isolate your tests but still maintain a clean structure.

In this example, the tests a, b and c will all print the same UUID, but test d will print a different UUID because it is executed in a new instance as it is a top level (aka root) test case.

This mode is deprecated due to undefined behavior on edge cases. It is recommended to use InstancePerRoot instead.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

This mode is deprecated due to undefined behavior on edge cases. It is recommended to use InstancePerRoot instead.

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({  isolationMode = IsolationMode.SingleInstance  // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({  val id = UUID.randomUUID()  "a" should {    println(id)    "b" {      println(id)    }    "c" {      println(id)    }  }  "d" should {    println(id)  }})
```

Example 4 (kotlin):
```kotlin
class InstancePerRootExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerRoot  val id = UUID.randomUUID()  init {    "a" should {      println(id)      "b" {        println(id)      }      "c" {        println(id)      }    }    "d" should {      println(id)    }  }}
```

---

## Simple Extensions | Kotest

**URL:** https://kotest.io/docs/framework/extensions/simple-extensions.html

**Contents:**
- Simple Extensions

This table lists the most basic extensions, that cover test and spec lifecycle events, and are mostly equivalent to lifecycle hooks. For more advanced extensions that can be used to modify the way the Engine runs, see advanced extensions.

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/5.7.x/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class StringSpecExample : StringSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
    - DSL Methods​
    - DSL methods with functions​
    - Overriding callback functions in a Spec​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. For more advanced cases, such as writing distributable plugins or re-usable hooks, one can use extensions.

At the end of this section is a list of the available hooks and when they are executed.

There are several ways to use hooks in Kotest:

The first and simplest, is to use the DSL methods available inside a Spec which create and register a TestListener for you. For example, we can invoke beforeTest or afterTest (and others) directly alongside our tests.

Behind the scenes, these DSL methods will create an instance of TestListener, overriding the appropriate functions, and ensuring that this test listener is registered to run.

You can use afterProject as a DSL method which will create an instance of ProjectListener, but there is no beforeProject because by the time the framework is at this stage of detecting a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The BeforeTest type used on the function definition is an alias to suspend (TestCase) -> Unit to keep things simple. There are aliases for the types of each of the callbacks.

The second, related, method is to override the callback functions in the Spec. This is essentially just a variation on the first method.

To understand all callbacks correctly it's important to have a good understanding of possible TestType values:

Notice that as far as beforeAny and beforeTest are just another name for the same functionality, beforeEach is different. Each of beforeAny and beforeTest will be invoked before both TestType.Container and TestType.Test, whereas beforeEach will be invoked before any TestType.Test. The same applies to afterAny, afterTest and afterEach.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : WordSpec({  beforeTest {    println("Starting a test $it")  }  afterTest { (test, result) ->    println("Finished spec with result $result")  }  "this test" should {    "be alive" {      println("Johnny5 is alive!")    }  }})
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : WordSpec({   // used once   beforeTest(startTest)   "this test" should {      "be alive" {         println("Johnny5 is alive!")      }   }})class OtherSpec : WordSpec({   // used twice   beforeTest(startTest)   "this test" should {      "fail" {         fail("boom")      }   }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : WordSpec() {    override fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" should {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Mountables | Kotest

**URL:** https://kotest.io/docs/framework/extensions/mountables.html

**Contents:**
- Mountables
- Creating a Mountable​
- Example​

Mountables are a special type of extension that can be installed in a spec, returning what is called a materialized value to the caller. This allows these extensions to return control objects which differ from the extension itself. The mountable can be customized as it is created through an optional configuration block.

Mountables are installed using the install function at the top level of a spec.

For example, we could imagine a Kafka extension that returns a Kafka client once installed.

Here kafka would be the materialized value and in this example, it could be a Kafka consumer connected to the embedded Kafka instance. The configuration block is used to configure the extension, and EmbeddedKafka is the extension itself.

Mountables can of course return the extension itself if they don't need a separate object, or return Unit.

Another example of a mountable is the JDBC Test Container extension, which returns a JDBC connection pool as the materialized value. This makes using test containers in Kotest very convenient.

Implement the MountableExtension interface and additionally implement any other lifecycle methods you need. This is another powerful feature of mountables, as it allows your extension to hook into other lifecycle events. For example, your mountable instance can itself implement AfterSpec and then the afterSpec method of the mountable will be called after the spec has finished running.

The interface has two type parameters: the config type and the materialized value type. The latter is the type that is returned to the caller, and as mentioned previously, it can be the same as the extension itself. The config type is passed as a receiver object to the configuration block. So this is where you define values or methods you want callers to be able to assign or invoke.

One drawback is that the configuration block is not suspendable, due to the fact that the init block is not itself suspendable. To work around this, you can use runBlocking { } inside your implementation.

Lets create a mountable that installs a H2 embedded database. The materialized value will be a connection to the database instance. We will allow the user to configure some details of the database in a configuration block. We will also implement AfterSpec so that the database is closed after the spec has finished running.

First we create the configuration class that contains our customization options.

Now we make the mountable extension, also implementing AfterSpec so we can close the connection after the spec has finished running. Notice that we use the mount function to perform the installation logic, and this function returns the materialized value - in this case, a JDBC connection to the database.

**Examples:**

Example 1 (swift):
```swift
class MyExampleTest : FunSpec() {  init {    val kafka = install(EmbeddedKafka) {      port = 9092    }  }}
```

Example 2 (kotlin):
```kotlin
class H2Config {   var databaseName = "test"}
```

Example 3 (kotlin):
```kotlin
class H2DatabaseMountableExtension() : MountableExtension<H2Config, Connection>, AfterSpecListener {   var conn: Connection? = null   override fun mount(configure: H2Config.() -> Unit): Connection {      val config = H2Config()      config.configure()      conn = DriverManager.getConnection("jdbc:h2:~/${config.databaseName}")      return conn!!   }   override suspend fun afterSpec(spec: Spec) {      conn?.close()   }}
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/6.0/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## Conditional tests with annotations | Kotest

**URL:** https://kotest.io/docs/framework/conditional/spec-annotations-conditional-evaluation.html

**Contents:**
- Conditional tests with annotations
  - @Ignored​
  - @EnabledIf​

If we wish to completely disable a Spec and all tests included in the spec, we can do this using annotations.

An advantage to this approach, instead of disabling each test one by one, is that the spec will not be instantiated. If a spec has expensive resource setup/teardown, then that time can be avoided by this approach.

These annotations are only available for the JVM target.

If we wish to simply disable a spec completely, then we can use the @Ignored annotation.

If we want to disable a spec dependent on the execution of a function, then we can use @EnabledIf.

This annotation accepts a class that implements EnabledCondition, and that class is instantiated and invoked to determine if a spec is enabled. Note that implementations must have a zero args constructor.

For example, we may wish to only execute tests containing the name "Linux" when run on a Linux machine.

Then we can apply this condition to one or more specs:

**Examples:**

Example 1 (kotlin):
```kotlin
@Ignoredclass IgnoredSpec : FunSpec() {  init {    error("boom") // spec will not be created so this error will not happen  }}
```

Example 2 (kotlin):
```kotlin
class LinuxOnlyCondition : Condition {  override fun evaluate(kclass: KClass<out Spec>): Boolean = when {    kclass.simpleName?.contains("Linux") == true -> IS_OS_LINUX    else -> true // non Linux tests always run  }}
```

Example 3 (kotlin):
```kotlin
@EnabledIf(LinuxOnlyCondition::class)class MyLinuxTest1 : FunSpec() {  // tests here}@EnabledIf(LinuxOnlyCondition::class)class MyLinuxTest2 : DescribeSpec() {  // tests here}@EnabledIf(LinuxOnlyCondition::class)class MyWindowsTests : DescribeSpec() {  // tests here}
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Test Factories | Kotest

**URL:** https://kotest.io/docs/framework/test-factories.html

**Contents:**
- Test Factories
- Overview​
- Listeners​

Sometimes we may wish to write a set of generic tests and then reuse them for specific inputs. In Kotest we can do this via test factories which create tests that can be included into one or more specs.

Say we wanted to build our own collections library. A slightly trite example, but one that serves the documentation purpose well.

We could create an interface IndexedSeq which has two implementations, List and Vector.

If we wanted to test our List implementation, we could do this:

Now, if we wanted to test Vector we have to copy n paste the test. As we add more implementations and more tests, the likelihood is our test suite will become fragmented and out of sync.

We can address this by creating a test factory, which accepts an IndexedSeq as a parameter.

To create a test factory, we use a builder function such as funSpec, wordSpec and so on. A builder function exists for each of the spec styles.

So, to convert our previous tests to a test factory, we simply do the following:

And then to use this, we must include it one or more times into a spec (or several specs).

You can include any style factory into any style spec. For example, a fun spec factory can be included into a string spec class.

A test class can include several different types of factory, as well as inline tests as normal. For example:

Each included test appears in the test output and reports as if it was individually defined.

Tests from factories are included in the order they are defined in the spec class.

Test factories support the usual before and after test callbacks. Any callback added to a factory, will in turn be added to the spec or specs where the factory is included.

However, only those tests generated by that factory will have the callback applied. This means you can create stand alone factories with their own lifecycle methods and be assured they won't clash with lifecycle methods defined in other factories or specs themselves.

After executing the test suite, the following would be printed:

And as you can see, the beforeTest block added to factory1 only applies to those tests defined in that factory, and not in the tests defined in the spec it was added to.

**Examples:**

Example 1 (kotlin):
```kotlin
interface IndexedSeq<T> {    // returns the size of t    fun size(): Int    // returns a new seq with t added    fun add(t: T): IndexedSeq<T>    // returns true if this seq contains t    fun contains(t: T): Boolean}
```

Example 2 (kotlin):
```kotlin
class ListTest : WordSpec({   val empty = List<Int>()   "List" should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }})
```

Example 3 (kotlin):
```kotlin
fun <T> indexedSeqTests(name: String, empty: IndexedSeq<T>) = wordSpec {   name should {      "increase size as elements are added" {         empty.size() shouldBe 0         val plus1 = empty.add(1)         plus1.size() shouldBe 1         val plus2 = plus1.add(2)         plus2.size() shouldBe 2      }      "contain an element after it is added" {         empty.contains(1) shouldBe false         empty.add(1).contains(1) shouldBe true         empty.add(1).contains(2) shouldBe false      }   }}
```

Example 4 (kotlin):
```kotlin
class IndexedSeqTestSuite : WordSpec({   include(indexedSeqTests("vector"), Vector())   include(indexedSeqTests("list"), List())})
```

---

## Closing resources automatically | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/autoclose.html

**Contents:**
- Closing resources automatically

You can let Kotest close resources automatically after all tests have been run:

Resources that should be closed this way must implement java.lang.AutoCloseable. Closing is performed in reversed order of declaration after the return of the last spec interceptor.

**Examples:**

Example 1 (kotlin):
```kotlin
class StringSpecExample : StringSpec() {  val reader = autoClose(StringReader("xyz"))  init {    "your test case" {      // use resource reader here    }  }}
```

---

## Data Test Names | Kotest

**URL:** https://kotest.io/docs/framework/datatesting/custom-test-names.html

**Contents:**
- Data Test Names
  - Stable Names​
  - Using a map​
  - Test Name Function​
  - WithDataTestName​

By default, the name of each test is simply the toString() on the input row. This typically works well for data classes on the JVM but requires the input rows to be stable.

However, we can specify how the test names are generated if we are not using stable data classes, or if we are executing on a non-JVM target, or simply wish to customize.

When generating tests, Kotest needs a stable test name over the course of the test suite execution. The test name is used as the basis of an identifier that points to a test when notifying gradle or intellij of a test status. If the name is not stable, then the id can change, leading to errors where tests don't appear, or look like they didn't complete.

Kotest will only use the toString() of the input class if it thinks the input class has a stable toString() value otherwise it will use the class name.

You can force Kotest to use the toString() for test names by annotating your type with @IsStableType. Then the toString() will be used regardless.

Alternatively, you can completely customize the display name of the test.

Kotest allows specifying test names by passing a map into the withXXX function, where the key is the test name, and the value is the input value for that row.

Or we can pass a function to withXXX which takes the row as input and return the test name. Depending on how generous the Kotlin type inference is feeling, you may need to specify the type parameter to the withXXX function.

The output from this example is now slightly clearer:

Another alternative is to implement the WithDataTestName interface. When provided, the toString() will not be used, instead the dataTestName() function from that interface will be invoked for each row.

**Examples:**

Example 1 (kotlin):
```kotlin
context("Pythag triples tests") {  withContexts(    mapOf(      "3, 4, 5" to PythagTriple(3, 4, 5),      "6, 8, 10" to PythagTriple(6, 8, 10),      "8, 15, 17" to PythagTriple(8, 15, 17),      "7, 24, 25" to PythagTriple(7, 24, 25)    )  ) { (a, b, c) ->    a * a + b * b shouldBe c * c  }}
```

Example 2 (kotlin):
```kotlin
context("Pythag triples tests") {  withContexts<PythagTriple>(    nameFn = { "${it.a}__${it.b}__${it.c}" },    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    a * a + b * b shouldBe c * c  }}
```

Example 3 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int) : WithDataTestName {  override fun dataTestName() = "wibble $a, $b, $c wobble"}
```

---

## Conditional tests with enabled flags | Kotest

**URL:** https://kotest.io/docs/6.0/framework/conditional/enabled-config-flag.html

**Contents:**
- Conditional tests with enabled flags
  - Enabled​
  - Enabled if​
  - Enabled or Reason If​

Kotest supports disabling tests by setting a configuration flag on a test. These configuration flags are very similar: enabled, enabledIf, and enabledOrReasonIf.

You can disable a test case simply by setting the config parameter enabled to false. If you're looking for something like JUnit's @Ignore, this is for you.

You can use the same mechanism to run tests only under certain conditions. For example you could run certain tests only on Linux systems using SystemUtils.IS_OS_LINUX from Apache Commons Lang.

If you want to use a function that is evaluated each time the test is invoked, then you can use enabledIf. This function has the signature (TestCase) -> Boolean, so as you can see, you have access to the test at runtime when evaluating if a test should be enabled or disabled.

For example, if we wanted to disable all tests that begin with the word "danger", but only when executing on Fridays, then we could do this:

There is a third variant of the enabled flag, called enabledOrReasonIf which allows you to return a reason for the test being disabled. This variant has the signature (TestCase) -> Enabled, where Enabled is a type that can contain a skip reason. This reason string is passed through to the test reports.

For example, we can re-write the earlier 'danger' example like this:

**Examples:**

Example 1 (kotlin):
```kotlin
"should do something".config(enabled = false) {  // test here}
```

Example 2 (kotlin):
```kotlin
"should do something".config(enabled = IS_OS_LINUX) {  // test here}
```

Example 3 (kotlin):
```kotlin
val disableDangerOnFridays: EnabledIf = { !(it.name.testName.startsWith("danger") && isFriday()) }"danger Will Robinson".config(enabledIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledIf = disableDangerOnFridays) { // test here}
```

Example 4 (kotlin):
```kotlin
val disableDangerOnFridays: (TestCase) -> Enabled = {   if (it.name.testName.startsWith("danger") && isFriday())      Enabled.disabled("It's a friday, and we don't like danger!")   else      Enabled.enabled}"danger Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) {  // test here}"safe Will Robinson".config(enabledOrReasonIf = disableDangerOnFridays) { // test here}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​
  - Callbacks​

Before data-driven-testing can be used, you need to add the module kotest-framework-datatest to your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withData function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<9> but was:<41>

In that previous example, we wrapped the withData call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

If you wish to have before / after callbacks in data-driven tests, then you can use the standard beforeTest / afterTest support. Every test created using data-driven testing acts the same way as a regular test, so all standard callbacks work as if you had written all the test by hand.

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withData(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withData(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/5.2.x/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
    - DSL Methods​
    - DSL methods with functions​
    - Overriding callback functions in a Spec​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. For more advanced cases, such as writing distributable plugins or re-usable hooks, one can use extensions.

At the end of this section is a list of the available hooks and when they are executed.

There are several ways to use hooks in Kotest:

The first and simplest, is to use the DSL methods available inside a Spec which create and register a TestListener for you. For example, we can invoke beforeTest or afterTest (and others) directly alongside our tests.

Behind the scenes, these DSL methods will create an instance of TestListener, overriding the appropriate functions, and ensuring that this test listener is registered to run.

You can use afterProject as a DSL method which will create an instance of ProjectListener, but there is no beforeProject because by the time the framework is at this stage of detecting a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The BeforeTest type used on the function definition is an alias to suspend (TestCase) -> Unit to keep things simple. There are aliases for the types of each of the callbacks.

The second, related, method is to override the callback functions in the Spec. This is essentially just a variation on the first method.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : WordSpec({  beforeTest {    println("Starting a test $it")  }  afterTest { (test, result) ->    println("Finished spec with result $result")  }  "this test" should {    "be alive" {      println("Johnny5 is alive!")    }  }})
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : WordSpec({   // used once   beforeTest(startTest)   "this test" should {      "be alive" {         println("Johnny5 is alive!")      }   }})class OtherSpec : WordSpec({   // used twice   beforeTest(startTest)   "this test" should {      "fail" {         fail("boom")      }   }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : WordSpec() {    override fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" should {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Lifecycle hooks | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/lifecycle-hooks.html

**Contents:**
- Lifecycle hooks
    - DSL Methods​
    - DSL methods with functions​
    - Overriding callback functions in a Spec​

It is extremely common in tests to want to perform some action before and after a test, or before and after all tests in the same file. It is in these lifecycle hooks that you would perform any setup/teardown logic required for a test.

Kotest provides a rich assortment of hooks that can be defined directly inside a spec. For more advanced cases, such as writing distributable plugins or re-usable hooks, one can use extensions.

At the end of this section is a list of the available hooks and when they are executed.

There are several ways to use hooks in Kotest:

The first and simplest, is to use the DSL methods available inside a Spec which create and register a TestListener for you. For example, we can invoke beforeTest or afterTest (and others) directly alongside our tests.

Behind the scenes, these DSL methods will create an instance of TestListener, overriding the appropriate functions, and ensuring that this test listener is registered to run.

You can use afterProject as a DSL method which will create an instance of ProjectListener, but there is no beforeProject because by the time the framework is at this stage of detecting a spec, the project has already started!

Since these DSL methods accept functions, we can pull out logic to a function and re-use it in several places. The BeforeTest type used on the function definition is an alias to suspend (TestCase) -> Unit to keep things simple. There are aliases for the types of each of the callbacks.

The second, related, method is to override the callback functions in the Spec. This is essentially just a variation on the first method.

**Examples:**

Example 1 (kotlin):
```kotlin
class TestSpec : WordSpec({  beforeTest {    println("Starting a test $it")  }  afterTest { (test, result) ->    println("Finished spec with result $result")  }  "this test" should {    "be alive" {      println("Johnny5 is alive!")    }  }})
```

Example 2 (kotlin):
```kotlin
val startTest: BeforeTest = {   println("Starting a test $it")}class TestSpec : WordSpec({   // used once   beforeTest(startTest)   "this test" should {      "be alive" {         println("Johnny5 is alive!")      }   }})class OtherSpec : WordSpec({   // used twice   beforeTest(startTest)   "this test" should {      "fail" {         fail("boom")      }   }})
```

Example 3 (kotlin):
```kotlin
class TestSpec : WordSpec() {    override fun beforeTest(testCase: TestCase) {        println("Starting a test $testCase")    }    init {        "this test" should {            "be alive" {                println("Johnny5 is alive!")            }        }    }}
```

---

## Project Level Config | Kotest

**URL:** https://kotest.io/docs/framework/project-config.html

**Contents:**
- Project Level Config
- Setup​
- Examples​
  - Assertion Mode​
  - Global Assert Softly​
  - Timeouts​
  - Duplicate Test Name Handling​
  - Fail On Ignored Tests​
  - Ordering​
    - Test Ordering​

This document describes project-level configuration in Kotest 6.0. If you were using project-level configuration in Kotest 5.x, note that the location of the project config instance must now be specified, otherwise it will not be picked up by the framework.

Kotest is flexible and has many ways to configure tests, such as configuring the order of tests inside a spec, or how test classes are created. Sometimes you may want to set this at a global level and for that you need to use project-level-config.

Project wide configuration can be used by creating a class that extends from AbstractProjectConfig. On the JVM and JS platforms, an object is also supported if you prefer using an object to a class.

Any configuration set at the spec level or directly on a test will override config specified at the project level. Some configuration options are only available at the project level because they change how the test engine runs the entire test suite (eg spec concurrency settings).

Some configuration options available in AbstractProjectConfig include assertions modes, timeouts, failing specs with ignored tests, global AssertSoftly, and reusable listeners or extensions and so on.

On the JVM, Kotest will inspect the classpath for a class with a specified name and package that extends AbstractProjectConfig. By default, this class should be named io.kotest.provided.ProjectConfig and stored in the file src/test/kotlin/io/kotest/provided/ProjectConfig.kt. If you don't want to place your class in that particular package, you can specify a different name using the system property kotest.framework.config.fqn.

For example, in gradle, you would configure something like this:

On native and JS platforms, the config class can be located anywhere but must still extend AbstractProjectConfig.

You should only create a single project config class, otherwise the behavior is undefined. If you want to have different configurations per package, see package level config.

You can ask Kotest to fail the build, or warn in std err, if a test is executed that does not use a Kotest assertion.

To do this, set assertionMode to AssertionMode.Error or AssertionMode.Warn inside your config. For example. An alternative way to enable this is the system property kotest.framework.assertion.mode which will always (if defined) take priority over the value here.

Assertion mode only works for Kotest assertions and not other assertion libraries. This is because the assertions need to be aware of the assertion detection framework that Kotest provides.

Assert softly is very useful to batch up errors into a single failure. If we want to enable this for every test automatically, we can do this in a config. An alternative way to enable this is by setting system property kotest.framework.assertion.globalassertsoftly to true which will always (if defined) take priority over the value here.

You can set a default timeout for all tests in your project by setting the timeout property in your project config.

By default, Kotest will rename a test if it has the same name as another test in the same scope. It will append _1, _2 and so on to the test name. This is useful for automatically generated tests.

You can change this behavior globally by setting duplicateTestNameMode to either DuplicateTestNameMode.Error or DuplicateTestNameMode.Warn.

Error will fail the test suite on a repeated name, and warn will rename but output a warning.

You may wish to consider an ignored test as a failure. To enable this feature, set failOnIgnoredTests to true inside your project config. For example.

Kotest supports ordering both specs and tests independently.

When running multiple tests from a Spec, there's a certain order on how to execute them.

By default, a sequential order is used (the order that tests are defined in the spec), but this can be changed. For available options see test ordering.

By default, the ordering of Spec classes is not defined. This is often sufficient, when we have no preference, but if we need control over the execution order of specs, we can use spec ordering.

Test names can be adjusted in several ways.

Test names case can be controlled by changing the value of testNameCase.

By default, the value is TestNameCase.AsIs which makes no change.

By setting the value to TestNameCase.Lowercase a test's name will be lowercase in output.

If you are using a spec that adds in prefixes to the test names (should as WordSpec or BehaviorSpec) then the values TestNameCase.Sentence and TestNameCase.InitialLowercase can be useful.

Another using test name option is testNameAppendTags which, when set to true, will include any applicable tags in the test name. For example, if a test foo was defined in a spec with the tags linux and spark then the test name would be adjusted to be foo [linux, spark]

This setting can also be set using a system property or environment variable kotest.framework.testname.append.tags to true.

If you define test names over several lines then removeTestNameWhitespace can be useful. Take this example:

Then the test name in output will be this is _ _ _ my test case (note: the underscores are added for emphasis). By setting removeTestNameWhitespace to true, then this name will be trimmed to this is my test case.

An alternative way to enable this is by setting system property kotest.framework.testname.multiline to true which will always (if defined) take priority over the value here.

You can specify a custom coroutine dispatcher factory to control how coroutines are executed in your tests.

For more details on this feature, see the concurrency documentation.

**Examples:**

Example 1 (kotlin):
```kotlin
tests.task {  useJunitPlatform()  systemProperty("kotest.framework.config.fqn", "com.sksamuel.mypackage.WibbleConfig")}
```

Example 2 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {  override val assertionMode = AssertionMode.Error}
```

Example 3 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {  override val globalAssertSoftly = true}
```

Example 4 (kotlin):
```kotlin
object KotestProjectConfig : AbstractProjectConfig() {  override val timeout = 5.seconds}
```

---

## Introduction to Extensions | Kotest

**URL:** https://kotest.io/docs/5.9.x/framework/extensions/extensions-introduction.html

**Contents:**
- Introduction to Extensions
  - How to use​

Extensions are reusable lifecycle hooks. In fact, lifecycle hooks are themselves represented internally as instances of extensions. In the past, Kotest used the term listeners for simple interfaces and extension for more advanced interfaces, however there is no distinction between the two and the terms can be used interchangeably.

The basic usage is to create an implementation of the required extension interface and register it with a test, a spec, or project wide in ProjectConfig.

For example, here we create a before and after spec listener, and register it with a spec.

Any extensions registered inside a Spec will be used for all tests in that spec (including test factories and nested tests).

To run an extension for every spec in the entire project you can either mark the listener with @AutoScan, or you can register the listener via project config.

An example of @AutoScan on a project listener:

Some extensions can only be registered at the project level. For example, registering a BeforeProjectListener inside a spec will have no effect, since the project has already started by the time that extension would be encountered!

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestListener : BeforeSpecListener, AfterSpecListener {   override suspend fun beforeSpec(spec:Spec) {      // power up kafka   }   override suspend fun afterSpec(spec: Spec) {      // shutdown kafka   }}class TestSpec : WordSpec({    extension(MyTestListener())    // tests here})
```

Example 2 (kotlin):
```kotlin
@AutoScanobject MyProjectListener : BeforeProjectListener, AfterProjectListener {  override suspend fun beforeProject() {    println("Project starting")  }  override suspend fun afterProject() {    println("Project complete")  }}
```

---

## Grouping Tests with Tags | Kotest

**URL:** https://kotest.io/docs/framework/tags.html

**Contents:**
- Grouping Tests with Tags
- Adding tags to tests​
  - Tagging Tests​
  - Tagging Specs​
  - @RequiresTag​
- Executing with Tags​
  - Tag Expression Operators​
  - Inheriting tags​
- Examples​
- Gradle​

Sometimes you don't want to run all tests and Kotest provides tags to be able to determine which tests are executed at runtime. Tags are objects inheriting from io.kotest.core.Tag.

For example, to group tests by operating system you could define the following tags:

Alternatively, tags can be defined using the NamedTag class. When using this class, observe the following rules:

If two tag objects have the same simple name (even in different packages) they are treated as the same tag.

Tests can be tagged at various levels. Firstly, test cases themselves can have tags added via test config. Note that any nested tags inherit tags from their parents.

Secondly, you can add tags at the spec level, either through the tags function in the spec itself, or through the @Tags annotation. Any tags added this way are applied to all tests implicitly.

When tagging tests in this way, the spec class will still need to be instantiated in order to examine the tags on each test, because the test itself may define further tags. Therefore, do not rely on this if you want to avoid instantiating classes completely, and instead see @RequiresTag.

Any tags added via @Tags do not stop the spec from being instantiated, as the engine needs to check for any tags added via code. If you want to avoid a spec from being instantiated completely, use @RequiresTag.

Finally, you can use the @RequiresTag annotation. This only checks that all the referenced tags are present and if not, will skip the spec entirely. This is an important distinction, because with the other annotation - @Tags - the spec will still need to be instantiated in order to check for any tags added via the DSL. This can be counter-intuitive.

For example, the following spec would be skipped and not instantiated unless the Linux and Mysql tags were specified at runtime.

Note that when you use annotations you pass the string name of the tag, not the tag instance itself.

By invoking the test runner with a system property of kotest.tags or an environment variable of KOTEST_TAGS you can control which tests are run. The expression to be passed in is a simple boolean expression using boolean operators: &, |, !, with parenthesis for association.

For example, Tag1 & (Tag2 | Tag3)

Provide the simple names of tag object (without package) when you run the tests. Eg, a tag created as val mytag = NamedTag("A") would use the tag name A.

Please pay attention to the use of upper case and lower case! Tags are case-sensitive.

Example: To run only test tagged with Linux, but not tagged with Database, you would invoke Gradle using the environment variable (since Kotest 6.1.0).

Or using the system property from the command line.

Or by specifying the system property inside your build script.

Prefer the environment variable as it works on all platforms, unless you are on an earlier version of Kotest (prior to 6.1), or need to specify the tags in your Gradle build script rather than at the command line.

If no root tests are active at runtime, the beforeSpec and afterSpec callbacks will not be invoked.

Operators (in descending order of precedence)

By default, the @Tags annotation will only be considered on the immediate Spec which it was applied to. However, a Spec can also inherit tags from superclasses and superinterfaces. To enable this, toggle tagInheritance = true in your project config

Consider the following example:

Special attention is needed in your Gradle configuration when using system properties

To use System Properties (-Dx=y), Gradle must be configured to propagate them to the test executors, and an extra configuration must be added to your tests:

This will guarantee that the system property is correctly read by the JVM.

**Examples:**

Example 1 (kotlin):
```kotlin
object Linux : Tag()object Windows : Tag()
```

Example 2 (kotlin):
```kotlin
val tag = NamedTag("Linux")
```

Example 3 (kotlin):
```kotlin
class MyTest : FunSpec() {  init {    test("should run on Windows").config(tags = setOf(Windows)) {      // ...    }    test("should run on Linux").config(tags = setOf(Linux)) {      // ...    }    context("should run on Windows and Linux").config(tags = setOf(Windows, Linux)) {      test("and nested tests") { // implicity has windows and linux tags added      }    }  }}
```

Example 4 (kotlin):
```kotlin
@Tags("Foo") // applied to all tests in this specprivate class TaggedSpec : ExpectSpec() {  init {    tags(Windows) // applied to all tests in this spec    expect("should run on Windows") {      // ...    }  }}
```

---

## Testing Styles | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/testing-styles.html

**Contents:**
- Testing Styles
- Fun Spec​
- String Spec​
- Should Spec​
- Describe Spec​
- Behavior Spec​
- Word Spec​
- Free Spec​
- Feature Spec​
- Expect Spec​

Kotest offers 10 different styles of test layout. Some are inspired from other popular test frameworks to make you feel right at home. Others were created just for Kotest.

To use Kotest, create a class file that extends one of the test styles. Then inside an init { } block, create your test cases. The following table contains the test styles you can pick from along with examples.

There are no functional differences between the styles. All allow the same types of configuration — threads, tags, etc — it is simply a matter of preference how you structure your tests.

Some teams prefer to mandate usage of a single style, others mix and match. There is no right or wrong - do whatever feels right for your team.

FunSpec allows you to create tests by invoking a function called test with a string argument to describe the test, and then the test itself as a lambda. If in doubt, this is the style to use.

Tests can be disabled using the xcontext and xtest variants (in addition to the usual ways)

StringSpec reduces the syntax to the absolute minimum. Just write a string followed by a lambda expression with your test code.

Adding config to the test.

ShouldSpec is similar to fun spec, but uses the keyword should instead of test.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xshould variants (in addition to the usual ways)

DescribeSpec offers a style familiar to those from a Ruby or Javascript background, as this testing style uses describe / it keywords. Tests must be nested in one or more describe blocks.

Tests can be disabled using the xdescribe and xit variants (in addition to the usual ways)

Popular with people who like to write tests in the BDD style, BehaviorSpec allows you to use given, when, then.

Because when is a keyword in Kotlin, we must enclose it with backticks. Alternatively, there are title case versions available if you don't like the use of backticks, eg, Given, When, Then.

You can also use the And keyword in Given and When to add an extra depth to it:

Note: Then scope doesn't have an and scope due to a Gradle bug. For more information, see #594

Tests can be disabled using the xgiven, xwhen, and xthen variants (in addition to the usual ways)

WordSpec uses the keyword should and uses that to nest tests after a context string.

It also supports the keyword When allowing to add another level of nesting. Note, since when is a keyword in Kotlin, we must use backticks or the uppercase variant.

FreeSpec allows you to nest arbitrary levels of depth using the keyword - (minus) for outer tests, and just the test name for the final test:

The innermost test must not use the - (minus) keyword after the test name.

FeatureSpec allows you to use feature and scenario, which will be familiar to those who have used cucumber. Although not intended to be exactly the same as cucumber, the keywords mimic the style.

Tests can be disabled using the xfeature and xscenario variants (in addition to the usual ways)

ExpectSpec is similar to FunSpec and ShouldSpec but uses the expect keyword.

Tests can be nested in one or more context blocks as well:

Tests can be disabled using the xcontext and xexpect variants (in addition to the usual ways)

If you are migrating from JUnit then AnnotationSpec is a spec that uses annotations like JUnit 4/5. Just add the @Test annotation to any function defined in the spec class.

You can also add annotations to execute something before tests/specs and after tests/specs, similarly to JUnit's

If you want to ignore a test, use @Ignore.

Although this spec doesn't offer much advantage over using JUnit, it allows you to migrate existing tests relatively easily, as you typically just need to adjust imports.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({    test("String length should return the length of the string") {        "sammy".length shouldBe 5        "".length shouldBe 0    }})
```

Example 2 (kotlin):
```kotlin
class MyTests : FunSpec({    context("this outer block is enabled") {        xtest("this test is disabled") {            // test here        }    }    xcontext("this block is disabled") {        test("disabled by inheritance from the parent") {            // test here        }    }})
```

Example 3 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string" {        "hello".length shouldBe 5    }})
```

Example 4 (kotlin):
```kotlin
class MyTests : StringSpec({    "strings.length should return size of string".config(enabled = false, invocations = 3) {        "hello".length shouldBe 5    }})
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Test Case Config | Kotest

**URL:** https://kotest.io/docs/next/framework/testcaseconfig.html

**Contents:**
- Test Case Config

Each test can be configured with various parameters. After the test name, invoke the config function passing in the parameters you wish to set. The available parameters are:

An example of setting config on a test:

You can also specify a DefaultTestConfig which will be used as the fallback for all test cases in a spec, unless overridden at the test level.

Set the defaultTestConfig val:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : ShouldSpec() {  init {    should("return the length of the string").config(invocations = 10) {      "sammy".length shouldBe 5      "".length shouldBe 0    }  }}
```

Example 2 (kotlin):
```kotlin
class MyTests : WordSpec() {  init {    "String.length" should {      "return the length of the string".config(timeout = 2.seconds) {        "sammy".length shouldBe 5        "".length shouldBe 0      }    }  }}
```

Example 3 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    test("FunSpec should support config syntax").config(tags = setOf(Database, Linux)) {      // ...    }  }}
```

Example 4 (kotlin):
```kotlin
class FunSpecTest : FunSpec() {  init {    defaultTestConfig = DefaultTestConfig(enabled = true, invocations = 3)    test("this test would run 3 times") {      // ...    }    test("this test would run 1 time because it is overriden at the test level").config(invocations = 1) {      // ...    }  }}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, and a timeout for each test or for all tests. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : StringSpec({   "length should return size of string" {      "hello".length shouldBe 5   }   "startsWith should test for a prefix" {      "world" should startWith("wor")   }})
```

Example 2 (kotlin):
```kotlin
class StringSpecExample : StringSpec({   "maximum of two numbers" {      forAll(         row(1, 5, 5),         row(1, 0, 1),         row(0, 0, 0)      ) { a, b, max ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : StringSpec({   "should use config".config(timeout = 2.seconds, invocations = 10, threads = 2, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Isolation Modes | Kotest

**URL:** https://kotest.io/docs/5.6.x/framework/isolation-mode.html

**Contents:**
- Isolation Modes
- Single Instance​
- InstancePerTest​
- InstancePerLeaf​
- Global Isolation Mode​
  - System Property​
  - Config​

All specs allow you to control how the test engine creates instances of Specs for test cases. This behavior is called the isolation mode and is controlled by an enum IsolationMode. There are three values: SingleInstance, InstancePerLeaf, and InstancePerTest.

If you want tests to be executed inside fresh instances of the spec - to allow for state shared between tests to be reset - you can change the isolation mode.

This can be done by using the DSL such as:

Or if you prefer function overrides, you can override fun isolationMode(): IsolationMode:

The default in Kotest is Single Instance which is the same as ScalaTest (the inspiration for this framework), Jest, Jasmine, and other Javascript frameworks, but different to JUnit.

The default isolation mode is SingleInstance whereby one instance of the Spec class is created and then each test case is executed in turn until all tests have completed.

For example, in the following spec, the same id would be printed three times as the same instance is used for all tests.

The next mode is IsolationMode.InstancePerTest where a new spec will be created for every test case, including inner contexts. In other words, outer contexts will execute as a "stand alone" test in their own instance of the spec. An example should make this clear.

Do you see how we've overridden the isolationMode function here.

When this is executed, the following will be printed:

This is because the outer context (test "a") will be executed first. Then it will be executed again for test "b", and then again for test "c". Each time in a clean instance of the Spec class. This is very useful when we want to re-use variables.

Another example will show how the variables are reset.

This time, the output will be:

The next mode is IsolationMode.InstancePerLeaf where a new spec will be created for every leaf test case - so excluding inner contexts. In other words, inner contexts are only executed as part of the "path" to an outer test. An example should make this clear.

When this is executed, the following will be printed:

This is because the outer context - test "a" - will be executed first, followed by test "b" in the same instance. Then a new spec will be created, and test "a" again executed, followed by test "c".

Another example will show how the variables are reset.

This time, the output will be:

Rather than setting the isolation mode in every spec, we can set it globally in project config or via a system property.

To set the global isolation mode at the command line, use the system property kotest.framework.isolation.mode with one of the values:

The values are case sensitive.

See the docs on setting up project wide config, and then add the isolation mode you want to be the default. For example:

Setting an isolation mode in a Spec will always override the project wide setting.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTestClass : WordSpec({ isolationMode = IsolationMode.SingleInstance // tests here})
```

Example 2 (kotlin):
```kotlin
class MyTestClass : WordSpec() {  override fun isolationMode() = IsolationMode.SingleInstance  init {    // tests here  }}
```

Example 3 (kotlin):
```kotlin
class SingleInstanceExample : WordSpec({   val id = UUID.randomUUID()   "a" should {      println(id)      "b" {         println(id)      }      "c" {         println(id)      }   }})
```

Example 4 (kotlin):
```kotlin
class InstancePerTestExample : WordSpec() {  override fun isolationMode(): IsolationMode = IsolationMode.InstancePerTest  init {    "a" should {      println("Hello")      "b" {        println("From")      }      "c" {        println("Sam")      }    }  }}
```

---

## Eventually | Kotest

**URL:** https://kotest.io/docs/5.3.x/framework/concurrency/eventually.html

**Contents:**
- Eventually
- API​
- Configuration​
  - Durations and Intervals​
  - Initial Delay​
  - Retries​
  - Specifying the exceptions to trap​
  - Predicates​
  - Listeners​
  - Sharing configuration​

Starting with Kotest 4.6, a new experimental module has been added which contains improved utilities for testing concurrent, asynchronous, or non-deterministic code. This module is kotest-framework-concurrency and is intended as a long term replacement for the previous module. The previous utilities are still available as part of the core framework.

Testing non-deterministic code can be hard. You might need to juggle threads, timeouts, race conditions, and the unpredictability of when events are happening.

For example, if you were testing that an asynchronous file write was completed successfully, you need to wait until the write operation has completed and flushed to disk.

Some common approaches to these problems are:

Using callbacks which are invoked once the operation has completed. The callback can be then used to assert that the state of the system is as we expect. But not all operations provide callback functionality.

Block the thread using Thread.sleep or suspend a function using delay, waiting for the operation to complete. The sleep threshold needs to be set high enough to be sure the operations will have completed on a fast or slow machine, and even when complete, the thread will stay blocked until the timeout has expired.

Use a loop with a sleep and retry and a sleep and retry, but then you need to write boilerplate to track number of iterations, handle certain exceptions and fail on others, ensure the total time taken has not exceeded the max and so on.

Use countdown latches and block threads until the latches are released by the non-determistic operation. This can work well if you are able to inject the latches in the appropriate places, but just like callbacks, it isn't always possible to have the code to be tested integrate with a latch.

As an alternative to the above solutions, kotest provides the eventually utility which solves the common use case of "I expect this code to pass after a short period of time".

Eventually does this by periodically invoking a given lambda until the timeout is eventually reached or too many iterations have passed. This is flexible and is perfect for testing nondeterministic code. Eventually can be customized in regardless to the types of exceptions to handle, how the lambda is considered a success or failure, with a listener, and so on.

There are two ways to use eventually. The first is simply providing a duration in either milliseconds (or using the Kotlin Duration type) followed by the code that should eventually pass without an exception being raised.

The second is by providing a configuration block before the test code. This method should be used when you need to set more options than just the duration.

The duration is the total amount of time to keep trying to pass the test. The interval however allows us to specify how often the test should be attempted. So if we set duration to 5 seconds, and interval to 250 millis, then the test would be attempted at most 5000 / 250 = 20 times.

Usually eventually starts executing the test block immediately, but we can add an initial delay before the first iteration using initialDelay, such as:

In addition to bounding the number of invocations by time, we can do so by iteration count. In the following example we retry the operation 10 times, or until 8 seconds has expired.

By default, eventually will ignore any AssertionError that is thrown inside the function (note, that means it won't catch Error). If you want to be more specific, you can tell eventually to ignore specific exceptions and any others will immediately fail the test.

For example, when testing that a user should exist in the database, a UserNotFoundException might be thrown if the user does not exist. We know that eventually that user will exist. But if an IOException is thrown, we don't want to keep retrying as this indicates a larger issue than simply timing.

We can do this by specifying that UserNotFoundException is an exception to suppress.

As an alternative to passing in a set of exceptions, we can provide a function which is invoked, passing in the throw exception. This function should return true if the exception should be handled, or false if the exception should bubble out.

In addition to verifying a test case eventually runs without throwing an exception, we can also verify that the return value of the test is as expected - and if not, consider that iteration a failure and try again.

For example, here we continue to append "x" to a string until the result of the previous iteration is equal to "xxx".

We can attach a listener, which will be invoked on each iteration, with the state of that iteration. The state object contains the last exception, last value, iteration count and so on.

Sharing the configuration for eventually is a breeze with the EventuallyConfig data class. Suppose you have classified the operations in your system to "slow" and "fast" operations. Instead of remembering which timing values were for slow and fast we can set up some objects to share between tests and customize them per suite. This is also a perfect time to show off the listener capabilities of eventually which give you insight into the current value of the result of your producer and the state of iterations!

Here we can see sharing of configuration can be useful to reduce duplicate code while allowing flexibility for things like custom logging per test suite for clear test logs.

**Examples:**

Example 1 (kotlin):
```kotlin
eventually(5000) { // duration in millis  userRepository.getById(1).name shouldBe "bob"}
```

Example 2 (kotlin):
```kotlin
eventually({  duration = 5000  interval = 1000.fixed()}) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 3 (kotlin):
```kotlin
eventually({  duration = 5000  initialDelay = 1000}) {  userRepository.getById(1).name shouldBe "bob"}
```

Example 4 (kotlin):
```kotlin
eventually({  duration = 8000  retries = 10  suppressExceptions = setOf(UserNotFoundException::class)}) {  userRepository.getById(1).name shouldNotBe "bob"}
```

---

## Shared Test Config | Kotest

**URL:** https://kotest.io/docs/next/framework/sharedtestconfig.html

**Contents:**
- Shared Test Config
- Introduction​
- Basic Usage​
- Available Configuration Options​
- Examples​
  - Example: Setting Retry Configuration​
  - Example: Setting Timeouts and Invocations​
  - Example: Using Tags and Assertion Mode​
- Overriding Default Configuration​

This page describes how to use DefaultTestConfig to share test configuration across multiple test cases in your specs.

This feature is available in Kotest 6.0 and later.

When writing tests, you often need to apply the same configuration to multiple test files. Instead of repeating the same configuration for each test, you can use DefaultTestConfig to define a shared configuration that applies to all tests in a spec.

DefaultTestConfig is a data class that allows for the configuration of test cases to be easily shared.

Each of the configuration values can be overridden on a per-test basis, but if you wish to have a common set of defaults that are shared across several tests, then you can create an instance of this class and declare it in each of the specs that you wish to share the configuration.

To set a default configuration for all tests in a spec, assign a DefaultTestConfig instance to the defaultTestConfig property in your spec:

DefaultTestConfig supports the following configuration options:

Individual tests can override any part of the default configuration using the .config() method:

The order of lookups is as follows:

**Examples:**

Example 1 (kotlin):
```kotlin
class MySpec : FunSpec() {  init {    defaultTestConfig = DefaultTestConfig(      timeout = 2.seconds,      invocations = 3,      threads = 2    )    // All tests in this spec will use the above configuration by default    test("test with default config") {      // This test will run 3 times with a timeout of 2 seconds    }    // You can still override the default config for specific tests    test("test with custom config").config(timeout = 5.seconds) {      // This test will run 3 times (from default config) with a timeout of 5 seconds    }  }}
```

Example 2 (kotlin):
```kotlin
class RetrySpec : DescribeSpec() {  init {    defaultTestConfig = DefaultTestConfig(retries = 5, retryDelay = 20.milliseconds)    describe("a flaky test") {      // This test will be retried up to 5 times with a 20ms delay between retries      it("should eventually pass") {        // Test logic here      }    }  }}
```

Example 3 (kotlin):
```kotlin
class PerformanceSpec : FreeSpec() {  init {    defaultTestConfig = DefaultTestConfig(      timeout = 1.minutes,      invocations = 10,      invocationTimeout = 5.seconds    )    "performance test" {      // This test will run 10 times, with each invocation having a 5 second timeout      // The entire test has a 1 minute timeout    }  }}
```

Example 4 (kotlin):
```kotlin
class IntegrationSpec : FunSpec() {  init {    defaultTestConfig = DefaultTestConfig(      tags = setOf(Tags.Integration, Tags.Slow),      assertionMode = AssertionMode.Error    )    test("database connection") {      // This test will be tagged as Integration and Slow      // Assertions will throw errors instead of exceptions    }  }}
```

---

## Test Timeouts | Kotest

**URL:** https://kotest.io/docs/5.5.x/framework/timeouts/test-timeouts.html

**Contents:**
- Test Timeouts
  - Test Timeout​
  - Invocation Timeout​
  - Project wide settings​
  - System Properties​

Kotest supports two types of test timeout. The first is the overall time for all invocations of a test. This is just called timeout. The second is per individual run of a test, and this is called invocation timeout.

To set a test timeout, we can use test config:

Alternatively, we can apply a test timeout for all tests in a spec file:

The time taken for a test includes the execution time taken for nested tests, so factor this into your timeouts.

Kotest can be configured to invoke a test multiple times. For example:

We can then apply a timeout per invocation using the invocationTimeout property.

In the previous example, each invocation must complete in 60 milliseconds or less. We can combine this with an overall test timeout:

Here we want all three tests to complete in 100 milliseconds or less, but allow any particular invocation to extend up to 60 milliseconds.

We can apply invocation timeouts at the spec level just like test timeouts:

We can apply a test and/or invocation timeout for all tests in a module using project config.

These values will take affect unless overriden at either the spec or the test level.

You can set a project wide timeout for tests and then override it per spec or per test

Both test timeout and invocation timeouts can be set using system properties, with values in milliseconds.

**Examples:**

Example 1 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   test("this test will timeout quickly!").config(timeout = 100.milliseconds) {      // test here   }})
```

Example 2 (kotlin):
```kotlin
class TimeoutTest : FunSpec({   timeout = 100.milliseconds   test("this test will timeout quickly!") {      // test here   }   test("so will this one!") {      // test here   }})
```

Example 3 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3) {            // this test will be invoked three times        }   }})
```

Example 4 (kotlin):
```kotlin
class TimeoutTest : DescribeSpec({   describe("my test context") {        it("run me three times").config(invocations = 3, invocationTimeout = 60.milliseconds) {            // this test will be invoked three times and each has a timeout of 60 milliseconds        }   }})
```

---

## Reproduce Race Conditions | Kotest

**URL:** https://kotest.io/docs/next/framework/race_conditions.html

**Contents:**
- Reproduce Race Conditions

A simple tool to reproduce some common race conditions such as deadlocks in automated tests. Whenever multiple coroutines or threads mutate shared state, there is a possibility of race conditions. In many common cases this tool allows to reproduce them easily.

If we are continuously running these two functions in parallel, eventually there should be deadlocks, but we don't know when exactly. With the help of runInParallel we can reliably reproduce the deadlock every time we run the following code:

Let's discuss a few more advanced scenarios where reproducing race conditions comes very handy.

Without concurrency, this code will always run correctly. Let us reproduce concurrency as follows:

For another example, suppose that we need to reproduce a deadlock between two threads that are trying to modify two Postgres tables in different order.

A brute force approach would be to run this scenario many times, hoping that eventually we shall reproduce the deadlock. Eventually this should work, but we shall have to spend some time setting up the test, and we might have to wait until it does reproduce.

This is a textbook scenario of a deadlock, and it is reliably reproduced every time we run this code. All the busywork of setting up threads and synchronizing them is handled by runInParallel.

**Examples:**

Example 1 (kotlin):
```kotlin
runInParallel({ runner: ParallelRunner ->    lockResourceA()    runner.await()    lockResourceB()  },    { runner: ParallelRunner ->      lockResourceB()      runner.await()      lockResourceA()    }  )
```

Example 2 (kotlin):
```kotlin
if(canRunTask()) {    runTask()}
```

Example 3 (kotlin):
```kotlin
private data class Box(val maxCapacity: Int) {      private val items = mutableListOf<String>()      fun addItem(item: String) = items.add(item)      fun hasCapacity() = items.size < maxCapacity      fun items() = items.toList()   }(snip)"two tasks share one mutable state, both make the same decision at the same time" {  val box = Box(maxCapacity = 2)  box.addItem("apple")  runInParallel({ runner: ParallelRunner ->    val hasCapacity = box.hasCapacity()    runner.await()    if(hasCapacity) {      box.addItem("banana")    }  },    { runner: ParallelRunner ->      val hasCapacity = box.hasCapacity()      runner.await()      if(hasCapacity) {        box.addItem("orange")      }    }  )  // capacity is exceeded as a result of race condition  box.items() shouldContainExactlyInAnyOrder listOf("apple", "banana", "orange")}
```

Example 4 (kotlin):
```kotlin
// Prerequisites:executeSql(  "DROP TABLE IF EXISTS test0",  "DROP TABLE IF EXISTS test1",  "SELECT 1 AS id, 'green' AS color INTO test0",  "SELECT 1 AS id, 'yellow' AS color INTO test1",)// reproduce a deadlockvar successCount = 0var thrownExceptions = mutableListOf<Throwable>()runInParallel(  { runner ->    try {      executeSql(jdbi, "UPDATE test0 SET color = 'blue' WHERE id = 1")      jdbi.useTransaction<Exception> { handle ->        handle.execute("UPDATE test0 SET color = 'blue' WHERE id = 1")        runner.await() // wait for the other thread to do its thing        handle.execute("UPDATE test1 SET color = 'purple' WHERE id = 1")        successCount++      }    } catch (ex: Throwable) {      thrownExceptions.add(ex)    }  },  { runner ->    try {      jdbi.useTransaction<Exception> { handle ->        handle.execute("UPDATE test1 SET color = 'blue' WHERE id = 1")        runner.await() // wait for the other thread to do its thing        handle.execute("UPDATE test0 SET color = 'purple' WHERE id = 1")        successCount++      }    } catch (ex: Throwable) {      thrownExceptions.add(ex)    }  })successCount shouldBe 1thrownExceptions shouldHaveSize 1isDeadlock(thrownExceptions[0]) shouldBe true
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/datatesting/data-driven-testing.html

**Contents:**
- Introduction
- Getting Started​
  - Callbacks​

Before data-driven-testing can be used, you need to add the module kotest-framework-datatest to your build.

This section covers the new and improved data driven testing support that was released with Kotest 4.6.0. To view the documentation for the previous data test support, click here

When writing tests that are logic based, one or two specific code paths that work through particular scenarios make sense. Other times we have tests that are more example based, and it would be helpful to test many combinations of parameters.

In these situations, data driven testing (also called table driven testing) is an easy technique to avoid tedious boilerplate.

Kotest has first class support for data driven testing built into the framework. This means Kotest will automatically generate test case entries, based on input values provided by you.

Let's consider writing tests for a pythagorean triple function that returns true if the input values are valid triples (a squared + b squared = c squared).

Since we need more than one element per row (we need 3), we start by defining a data class that will hold a single row of values (in our case, the two inputs, and the expected result).

We will create tests by using instances of this data class, passing them into the withData function, which also accepts a lambda that performs the test logic for that given row.

Notice that because we are using data classes, the input row can be destructured into the member properties. When this is executed, we will have 4 test cases in our input, one for each input row.

Kotest will automatically generate a test case for each input row, as if you had manually written a separate test case for each.

The test names are generated from the data classes themselves but can be customized.

If there is an error for any particular input row, then the test will fail and Kotest will output the values that failed. For example, if we change the previous example to include the row PythagTriple(5, 4, 3) then that test will be marked as a failure.

The error message will contain the error and the input row details:

Test failed for (a, 5), (b, 4), (c, 3) expected:<9> but was:<41>

In that previous example, we wrapped the withData call in a parent test, so we have more context when the test results appear. The syntax varies depending on the spec style used - here we used fun spec which uses context blocks for containers. In fact, data tests can be nested inside any number of containers.

But this is optional, you can define data tests at the root level as well.

Data tests can only be defined at the root or in container scopes. They cannot be defined inside leaf scopes.

If you wish to have before / after callbacks in data-driven tests, then you can use the standard beforeTest / afterTest support. Every test created using data-driven testing acts the same way as a regular test, so all standard callbacks work as if you had written all the test by hand.

**Examples:**

Example 1 (kotlin):
```kotlin
fun isPythagTriple(a: Int, b: Int, c: Int): Boolean = a * a + b * b == c * c
```

Example 2 (kotlin):
```kotlin
data class PythagTriple(val a: Int, val b: Int, val c: Int)
```

Example 3 (kotlin):
```kotlin
class MyTests : FunSpec({  context("Pythag triples tests") {    withData(      PythagTriple(3, 4, 5),      PythagTriple(6, 8, 10),      PythagTriple(8, 15, 17),      PythagTriple(7, 24, 25)    ) { (a, b, c) ->      isPythagTriple(a, b, c) shouldBe true    }  }})
```

Example 4 (kotlin):
```kotlin
class MyTests : FunSpec({  withData(    PythagTriple(3, 4, 5),    PythagTriple(6, 8, 10),    PythagTriple(8, 15, 17),    PythagTriple(7, 24, 25)  ) { (a, b, c) ->    isPythagTriple(a, b, c) shouldBe true  }})
```

---

## Fail Fast | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/fail-fast.html

**Contents:**
- Fail Fast

Kotest can eagerly fail a list of tests if one of those tests fails. This is called fail fast.

Fail fast can take affect at the spec level, or at a parent test level.

In the following example, we enable failfast for a parent test, and the first failure inside that context, will cause the rest to be skipped.

This can be enabled for all scopes in a Spec by setting failfast at the spec level.

**Examples:**

Example 1 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      context("context with fail fast enabled").config(failfast = true) {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

Example 2 (kotlin):
```kotlin
class FailFastTests() : FunSpec() {   init {      failfast = true      context("context with fail fast enabled at the spec level") {         test("a") {} // pass         test("b") { error("boom") } // fail         test("c") {} // skipped         context("d") {  // skipped            test("e") {} // skipped         }      }   }}
```

---

## Introduction | Kotest

**URL:** https://kotest.io/docs/framework/framework.html

**Contents:**
- Introduction
- Test with Style​
- Check all the Tricky Cases With Data Driven Testing​
- Fine Tune Test Execution​

Write simple and beautiful tests using one of the available styles:

Kotest allows tests to be created in several styles, so you can choose the style that suits you best.

Handle even an enormous amount of input parameter combinations easily with data driven tests:

You can specify the number of invocations, parallelism, test timeouts, and a host of other options. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({   test("length should return size of string") {      "hello".length shouldBe 5   }   test("startsWith should test for a prefix") {      "hello world" should startWith("hello")   }})
```

Example 2 (kotlin):
```kotlin
class DataTestExample : FreeSpec({   "maximum of two numbers" {      withData(         Triple(1, 5, 5),         Triple(1, 0, 1),         Triple(0, 0, 0)      ) { (a, b, max) ->         Math.max(a, b) shouldBe max      }   }})
```

Example 3 (kotlin):
```kotlin
class MySpec : DescribeSpec({   describe("should use config").config(timeout = 2.seconds, invocations = 10, tags = setOf(Database, Linux)) {      // test here   }})
```

---

## Setup | Kotest

**URL:** https://kotest.io/docs/5.8.x/framework/project-setup.html

**Contents:**
- Setup

The Kotest test framework is supported on JVM, Javascript and Native. To enable Kotest for multiple platforms, combine the steps for the individual platforms as detailed in the following tabs.

Kotest on the JVM uses the JUnit Platform gradle plugin. For Gradle 4.6 and higher this is as simple as adding useJUnitPlatform() inside the tasks with type Test and then adding the Kotest junit5 runner dependency.

If you are using Gradle + Groovy then:

Or if you are using Gradle + Kotlin then:

And then the dependency:

A working multiplatform project with JVM, native and Javascript all configured, with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples-multiplatform

Add the Kotest multiplatform gradle plugin to your build.

Add the engine dependency to your commonTest dependencies block:

Only the new IR compiler backend for Kotlin/JS is supported. If you are compiling JS with the legacy compiler backend then you will not be able to use Kotest for testing.

Write your tests using FunSpec, ShouldSpec or StringSpec. Tests can be placed in either commonTest or jsTest source sets. Run your tests using the gradle check command.

The Javascript test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to javascript code.

Tests for Javascript cannot nest tests. This is due to the underlying Javascript test runners (such as Mocha or Karma) not supporting promises in parent tests, which is incompatible with coroutines and in Kotest every test scope is a coroutine. This is why the supported specs are limited to FunSpec, ShouldSpec and StringSpec.

The IntelliJ Kotest plugin does not support running common, native or JS tests directly from the IDE using the green run icons. Only execution via gradle is supported.

A working multiplatform project with JVM, native and Javascript all configured, with unit and data driven test examples, can be found here: https://github.com/kotest/kotest-examples-multiplatform

Add the Kotest multiplatform gradle plugin to your build.

Add the engine dependency to your commonTest dependencies block:

Tests can be placed in either commonTest or a specific native sourceset. Run your tests using the gradle check command.

The native test engine is feature limited when compared to the JVM test engine. The major restriction is that annotation based configuration will not work as Kotlin does not expose annotations at runtime to native code.

The IntelliJ Kotest plugin does not support running common, native or JS tests from the IDE. You will need to use the gradle check task.

For maven you must configure the surefire plugin for junit tests.

And then add the Kotest JUnit5 runner to your dependencies section.

Currently, only JVM tests are officially supported in Kotest. We are open to suggestions on how to support UI tests.

The following steps enable Kotest to be used for unit and integration tests, where the Android framework is not needed or is mocked that usually reside in the src/test folder of your module.

Kotest on Android uses the JUnit Platform gradle plugin. This requires configuring the android test options block in your build file and then adding the Kotest junit5 runner dependency.

To configure the test framework for both JS and JVM, you just combine copy the steps for JVM and JS.

**Examples:**

Example 1 (unknown):
```unknown
test {   useJUnitPlatform()}
```

Example 2 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {   useJUnitPlatform()}
```

Example 3 (bash):
```bash
testImplementation 'io.kotest:kotest-runner-junit5:$version'
```

Example 4 (kotlin):
```kotlin
plugins {  id("io.kotest.multiplatform") version "5.0.2"}
```

---
