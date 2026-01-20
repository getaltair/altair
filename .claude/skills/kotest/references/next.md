# Kotest - Next

**Pages:** 11

---

## Release 4.2 | Kotest

**URL:** https://kotest.io/docs/next/blog/release_4.2

**Contents:**
- Release 4.2
  - Module changes​
  - Multiplatform improvements​
  - Kotlinx Date/Time Matchers​
  - Multiple Project Configs​
  - Extended Callbacks​
  - Spec Ordering​
  - Tag Expressions​
  - Spec level Timeout Overrides​
  - Exhaustive Specific forAll / checkAll​

The Kotest team is pleased to announce the release of Kotest 4.2.0. This minor feature release continues on the excellent work that was included in the 4.1.0 release (which itself was almost as large as the 4.0.0 release!).

In this blog post we'll cover some of the more notable features and changes but for the full list see the changelog.

Firstly, the kotest-runner-console dependency is no longer required by the intellij plugin, and therefore no longer exists. So that can be removed completely from your build if you were using it.

Secondly, the kotest-core dependency has become kotest-framework-engine.

Finally this release of Kotest is fully compatible with Kotlin 1.4.

The core assertions library is now published for ios, watchos and tvos. This brings the list of support platforms to:

A new assertions module has been created kotest-assertions-kotlinx-time which contains matchers for the new Kotlinx Datetime library. Since the datetime library has an incubating status, this assertions module may require breaking changes in the future if the date/time API mandates it.

This assertions module is multiplatform and is released for the JVM, JS, Linux, Mac and Windows targets.

An example assertion is checking that a date time has a given hour.

For the full list of matchers, see the source code.

Kotest supports customizing test plans by extending the AbstractProjectConfig class and placing it in your classpath somewhere. From 4.2.0, you can now create more than one and all will be detected and configs merged. This is really nice if you want to have some shared config for all your tests in a root module, and then customize with more finer details per module.

In the case of clashes, one value will be arbitrarily picked, so it is not recommended to add competing settings to different configs.

Kotest has always had beforeTest / afterTest callbacks which run before / after any 'test scope'. However sometimes you need a way to run setup/teardown code only before leaf test scopes (called tests in Kotest) or branch test scopes (called containers in Kotest).

So in 4.2.0 we've introduced beforeEach, afterEach, beforeContainer, and afterContainer. The xxEach functions are invoked only for leaf level test scopes. The xxContainer functions are invoked only for branch level test scopes.

This distinction is only relevant to test styles that support nested scopes.

The output you would receive is:

Kotest previously allowed the execution order of Specs to be decided randomly, discovery order (the default), or lexicographically. Now, there is support for an annotation based approach. By selecting this, and annotating your Specs with @Order(int) you can specify any order you wish, with the specs with the lowest int values executing first.

Any spec without an @Order annotation is considered "last". Any specs that tie will be executed arbitrarily.

Tests and Specs can be tagged with Tag objects and then at runtime, tests can be enabled or disabled by specifying which tags to use. Previously, you could do this by specifying which tags to include and which tags to exclude but nothing more advanced.

Now, you are able to specfify full boolean expressions using the kotest.tags system property, for example:

gradle test -Dkotest.tags="Linux & !Database".

Expressions can be nested using parenthesis and can be arbitrarily complex. For full details see Tags.

Note: Existing system properties kotest.tags.include and kotest.tags.exclude are still supported, but the new functionality supersedes this.

It has always been possible to add a timeout to a test at the global level or via test case config for each specific test:

But it has not previously been possible to override this as the spec level for all tests in that spec. Now you can.

Note: You can apply a spec level timeout and then override this per test case, as you can see in the example above. The same functionality exists for invocation timeouts.

When property testing, if you are using only exhaustive generators, then the forAll / checkAll methods will now ensure that the number of iterations is equal to the number of combinations in the exhaustives, and that all combinations are executed.

As a contrived example, consider this:

Here, the number of iterations is 6 6 6 = 216 and each tuple combination of (0-5, 0-5, 0-5) will be executed. The first will be (0, 0, 0) and the last wil be (5, 5, 5) with every combination in between.

When using shouldBeInstanceOf<T> or shouldBeTypeOf<T>, the assertions can now use generic contracts to smart case down to generic instances.

For example, consider the following example where we are given an Any. After invoking shouldBeTypeOf with a generic type, the type is smart casted if the assertion passes.

The Kotest Intellij Plugin is released on a separate cadence from Kotest itself, but here are some notable changes since Kotest 4.1.0.

The Junit XML report (what JUnit refers to as the legacy XML report because it existed prior to JUnit5) has no concept of nested tests. Therefore, if you are using a spec style that supports nested tests, the gradle report generator will only use the leaf test name. This can be confusing if you are expecting the full test path for context.

In 4.2.0 Kotest has it's own implementation of this XML report that contains options to a) include the full test path and / or b) ignore parent tests completely.

Example usage from within project config:

If you are using the spring support and are using a final class, you will receive a warning from Kotest:

Using SpringListener on a final class. If any Spring annotation fails to work, try making this class open

You can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

Huge thanks to all who contributed to this release.

Alberto Ballano, Ali Albaali, amollberg, Ashish Kumar Joy, Christian Stoenescu, Cleidiano Oliveira ,Daniel Asztalos, fauscik, Juanjo Aguililla, Justin, Leonardo Colman, Matthew Mikolay, Neenad Ingole, Shane Lathrop, sksamuel, Timothy Lusk

**Examples:**

Example 1 (kotlin):
```kotlin
val date = LocalDateTime(2019, 2, 15, 12, 10, 0, 0)date.shouldHaveHour(12)
```

Example 2 (kotlin):
```kotlin
class CallbacksTest : DescribeSpec({   beforeEach {      println("Test: " + it.displayName)   }   beforeContainer {      println("Container: " + it.displayName)   }   beforeTest {      println("All: " + it.displayName)   }   describe("I am a container scope") {      it("And I am a test scope") { }   }})
```

Example 3 (yaml):
```yaml
Container: I am a container scopeAll: I am a container scopeTest: And I am a test scopeAll: And I am a test scope
```

Example 4 (kotlin):
```kotlin
test("my test").config(timeout = 20.seconds) { }
```

---

## Features and Changes in Kotest 6.0 | Kotest

**URL:** https://kotest.io/docs/next/release6

**Contents:**
- Features and Changes in Kotest 6.0
- New Features​
  - Enhanced Concurrency Support​
  - Package-Level Configuration​
  - Shared Test Configuration​
  - New Isolation Mode: InstancePerRoot​
  - TestClock Implementation​
  - Enhanced Coroutine Debugging​
  - Decoroutinator Extension​
  - Power Assert Support​

This page lists the features and changes in Kotest 6.0.

Kotest 6.0 introduces a comprehensive set of concurrency features to improve test execution:

Spec Concurrency Mode: Controls how specs (test classes) are executed in relation to each other

Test Concurrency Mode: Controls how root tests within a spec are executed in relation to each other

Coroutine Dispatcher Factory: Customize the coroutine dispatcher used for executing specs and tests

Blocking Test Mode: Addresses issues with timeouts when working with blocking code

Package-level configuration allows you to define shared configuration that applies to all specs in a specific package and its sub-packages:

The new DefaultTestConfig feature allows you to define shared test configuration that applies to all tests in a spec:

A new isolation mode InstancePerRoot has been introduced:

A new TestClock implementation has been added for controlling time in tests:

Improved support for debugging coroutines in tests:

A new extension for improving coroutine stack traces:

Kotest 6.0 integrates with Kotlin 2.2's Power Assert feature to provide enhanced assertion failure messages:

Kotest 6.0 requires a minimum of JDK 11 and Kotlin 2.2.

The KMP support in Kotest 6.0 has changed from previous versions:

All extensions are now published under the io.kotest group:

The location of the project config instance is now required to be at a specific path:

Classpath scanning for extensions has been removed in Kotest 6.0:

To register extensions, use one of these approaches:

If you are using the Kotest 5.0+ withData support, you no longer need to add the kotest-framework-data dependency to your project as this has been merged into the core framework.

If you are using the Kotest 4.x era table driven testing, you will need to add the kotest-assertions-table dependency to your project as this has been moved out of the core framework.

Inside the project config, extensions are now a val not a function. So if you had before:

The System.exit and System.env override extensions have been removed due to the deprecation of the SecurityManager in Java.

The following isolation modes are now deprecated due to undefined behavior in edge cases:

It is recommended to use InstancePerRoot instead.

Enhanced support for coroutine debugging:

**Examples:**

Example 1 (kotlin):
```kotlin
object ProjectConfig : AbstractProjectConfig() {  override val extensions = listOf(    MyExtension(),    AnotherExtension()  )}
```

Example 2 (kotlin):
```kotlin
@ApplyExtension(MyExtension::class)class MySpec : FunSpec() {  // tests here}
```

Example 3 (kotlin):
```kotlin
override fun listeners() = ...
```

Example 4 (kotlin):
```kotlin
override fun extensions() = ...
```

---

## Properties | Kotest

**URL:** https://kotest.io/docs/next/intellij/intellij-properties.html

**Contents:**
- Properties
  - Common use case​
  - Specifying the properties filename​

When running tests via the intellij runner, properties set using gradle.properties or in a gradle build file won't be picked up because the runner is not set to use Gradle.

To support runtime system properties, the Kotest framework will always look for key value pairs inside a kotest.properties file located on the classpath (eg, in src/main/resources).

Any key value pairs located in this file will be set as a system property before any tests execute.

Any properties specified in the kotest.properties file work for both command line via Gradle, and tests executed via the Intellij plugin.

For example, after adding this file to your classpath as kotest.properties:

The following test would pass:

It is common to disable the classpath scanning capabilities of Kotest to save some startup time, if those features are not used. To do this place, the following lines into the kotest.properties file:

If you don't wish to name the file kotest.properties, or perhaps you want to support different files based on an environment, then you can use the system property kotest.properties.filename to set the properties filename.

For example, you could launch tests with kotest.properties.filename=cluster.prd.properties then the key value file named cluster.prd.properties would be loaded before any tests are executed.

**Examples:**

Example 1 (kotlin):
```kotlin
class FooTest : DescribeSpec() {  init {    describe("after adding kotest.properties") {      it("foo should be set") {         System.getProperty("foo") shouldBe "bar"      }    }  }}
```

Example 2 (unknown):
```unknown
kotest.framework.classpath.scanning.config.disable=truekotest.framework.classpath.scanning.autoscan.disable=true
```

---

## home | Kotest

**URL:** https://kotest.io/docs/next/

**Contents:**
- home
- Community​
- Test with Style​
- Multitude of Matchers​
- Let the Computer Generate Your Test Data​
- Check all the Tricky Cases With Data Driven Testing​
- Test Exceptions​
- Fine Tune Test Execution​

Kotest is a flexible and comprehensive testing project for Kotlin with multiplatform support.

For latest updates see Changelog. See our quick start guide to get up and running.

Write simple and beautiful tests using one of the available styles:

Kotest comes with several testing styles so you can choose one that fits your needs.

Use over 300 provided matchers to test assertions on many different types:

The withClue and asClue helpers can add extra context to assertions so failures are self explanatory:

Nesting is allowed in both cases and will show all available clues.

Matchers are extension methods and so your IDE will auto complete. See the full list of matchers or write your own.

Use property based testing to test your code with automatically generated test data:

Handle even an enormous amount of input parameter combinations easily with data driven tests:

Testing for exceptions is easy with Kotest:

You can specify the number of invocations, parallelism, and a timeout for each test or for all tests. And you can group tests by tags or disable them conditionally. All you need is config:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTests : FunSpec({  test("length should return size of string") {    "hello".length shouldBe 5  }  test("startsWith should test for a prefix") {    "hello world" should startWith("hello")  }})
```

Example 2 (kotlin):
```kotlin
"substring".shouldContain("str")user.email.shouldBeLowerCase()myImageFile.shouldHaveExtension(".jpg")cityMap.shouldContainKey("London")
```

Example 3 (kotlin):
```kotlin
withClue("Name should be present") { user.name shouldNotBe null }data class HttpResponse(val status: Int, body: String)val response = HttpResponse(200, "the content")response.asClue {    it.status shouldBe 200    it.body shouldBe "the content"}
```

Example 4 (kotlin):
```kotlin
class PropertyExample: FreeSpec({  "String size" {    checkAll<String, String> { a, b ->      (a + b) shouldHaveLength a.length + b.length    }  }})
```

---

## Test Explorer | Kotest

**URL:** https://kotest.io/docs/next/intellij/intellij-test-explorer.html

**Contents:**
- Test Explorer

The plugin provides a tool window view which displays the structure of your tests. The window describes the currently selected test file, which includes any specs defined in that file and tests contained inside those specs. The tree layout will mirror the structure of your tests for easy navigation.

The tool window will include lifecycle callback methods (such as before / after test) if defined, as well as included test factories.

Clicking on a spec, test, include or callback will navigate directly to that element in the source editor.

Any tests that have been disabled using the bang prefix will have a different icon.

You can execute (run/debug/run with coverage) a test or spec directly from this window. In addition, the window shows all test modules and allows you to run all tests in that module.

Modules, callbacks, and includes can be filtered out if you don't wish to see them. They are included by default.

---

## Release 4.1 | Kotest

**URL:** https://kotest.io/docs/next/blog/release_4.1

**Contents:**
- Release 4.1
  - Kotest Plugin​
  - Kotlintest aliases removed​
  - Highlight diff when comparing data classes​
  - Integration with Testcontainers​
  - 'x' variants for Specs​
  - Removing test prefixes from test output​
  - Invocation level timeouts​
  - Parallel test execution​
  - All scopes are now coroutine scopes​

The Kotest team is pleased to announce the release of Kotest 4.1.0. This minor feature release is packed with goodies including the first public release of the Intellij plugin. In this blog post we'll cover some of the more notable features and changes but for the full list see the changelog.

Let's start with the most exciting news. As part of the 4.1.0 release cycle, we've released the first public version of the Kotest plugin for Intellij. The plugin is available in the Jetbrains plugin repository, so hop on over to settings -> plugins and search for "kotest".

As this is the first release that will be used by the majority of users, bugs will likely be found. If you do encounter an issue, please open a ticket here.

The plugin provides gutter run icons for specs, top level tests, and nested tests.

The plugin additionally provides a tool window view which displays the structure of your tests. The window describes the currently selected test file, which includes any specs defined in that file and tests contained inside those specs. The tree layout will mirror the structure of your tests for easy navigation.

The tool window will include lifecycle callback methods (such as before / after test) if defined, as well as included test factories.

Clicking on a spec, test, include or callback will navigate directly to that element in the source editor.

For full details on the features provided by the plugin, check out the readme.

Note: In order to support this plugin, the behind the scenes code that fooled Intellij into thinking Kotest specs were Junit tests has been removed. This means that unless you have the plugin installed, you won't see the green play icon anymore on the class name.

With release 4.0 of Kotest, the project was renamed from Kotlintest. To aid migration, we created aliases from the kotlintest packages to the kotest packages for common imports.

With the release of 4.1 these aliases have been removed.

When comparing two data classes for equality, previously you had to look through the fields to see which one(s) didn't match up. Instead now, the failure output will highlight the differences for you.

For example, given the following data class:

And then executing this:

Will give the following output:

Testcontainers is a popular Java library that supports lightweight, throwaway instances of databases, message queues, elasticsearch and so on. And now Kotest has a module that allows easy integration into the test lifecycle.

Add the kotest-extensions-testcontainers module to your build and then you can register a test container like this:

Notice the .perTest() function which creates a listener that will stop and start the container between tests. If you want a container that only starts and stops once per spec, then use the following:

The popular javascript frameworks and RSpec in Ruby have popularized the describe / it layout style for tests. Kotest has supported this since version 1.0 in the form of the DescribeSpec. These other frameworks also provide an easy way to disable a test, by replacing describe with xdescribe and it with xit. Kotest also supports this.

Starting with 4.1 Kotest now rolled out the same functionality to the other styles. For example, you can disable a given block in BehaviorSpec by using xgiven, you can describe a context block in FunSpec with xcontext and so on.

A full example in the FunSpec style.

See full details on the styles page.

Following on from the previous section, when you use certain specs, the test names are prefixed with Describe:, or Feature: and so on in the output.

This adds extra noise to the output and in retrospect should not have been added. Starting with 4.1 you can now disable these test prefixes by setting includeTestScopePrefixes to false in your project config.

Note: In 4.2.0 this setting will be true by default.

Kotest has the option to apply a timeout to your tests through config on the test case.

This timeout applies to all invocations of that test case. So if you have invocations set greater than 1, then the timeout is shared between all invocations. Starting with 4.1 you can now apply a timeout at the invocation level.

Kotest has for a long time, had the ability to run specs in parallel. Starting with 4.1 you can run individual test cases in parallel. Override the threads val inside your spec class to greater than 1. Note: This feature is experimental and only applies to the single instance isolation mode.

Leaf test cases have always been coroutine scopes since release 3.2 of Ko(tlin)Test. This means you can launch a coroutine directly in the test block without needing to provide a scope like GlobalScope or your own instance of CoroutineScope.

Previously, parent scopes in test styles that allow nesting, were not themselves coroutine scopes. This has been changed in 4.1.

Now you can write a test like this:

Another feature that was more an oversight than anything else - the beforeProject and afterProject callbacks inside ProjectListener are now suspendable functions.

You might already be using assertSoftly to allow a test to finish before throwing all the failures at once. Now you can do the same but with a receiver.

For example, rather than write

If you're using the property test framework you'll notice the improved shrinking output. This now includes both the reason for the original failure (with the original args) and the reason for the shrunk failure (with the shrunks args).

For example, given a silly test that checks that any string reversed is the same as the input string:

This will be true for the empty string and all single char strings, and then false for most other strings.

The forAll and checkAll property test functions accept a PropTestConfig object to configure a property test. This object now contains a listeners field, to which you can attach PropTestListener instances. This allows you to run setup / teardown code before and after a property test, like you can for regular tests.

Huge thanks to all who contributed to this release.

AJ Alt, Albert Attard, Amy, Ashish Kumar Joy, ataronet, Attila Domokos, bbaldino, bright_spark, Caroline Ribeiro, Christian Nedregård, crazyk2, George Wilkins, Harry JinHyeok Kang, James Pittendreigh, Leonardo Colman Lopes, Lyall Jonathan Di Trapani, Martin Nonnenmacher, Maxime Suret, mwfpope, Nikita Klimenko, Nimamoh, Octogonapus, Paul, Robert Macaulay, Robert Stoll, Ron Gebauer, Sebastian Schuberth, Sergei Bulgakov, sharmabhawna, sksamuel, Steffen Rehberg

**Examples:**

Example 1 (kotlin):
```kotlin
data class Foo(val a: String, val b: Boolean, val c: Double)
```

Example 2 (kotlin):
```kotlin
val a = Foo("hello", true, 1.0)val b = Foo("world", true, 1.3)a shouldBe b
```

Example 3 (jsx):
```jsx
data class diff for FooExpected :Foo(a=world, b=true, c=1.3)Actual   :Foo(a=hello, b=true, c=1.0)<Click to see difference>org.opentest4j.AssertionFailedError: data class diff for Foo├ a: expected:<"world"> but was:<"hello">└ c: expected:<1.3> but was:<1.0>
```

Example 4 (kotlin):
```kotlin
val testStartable = SomeTestContainer()listeners(testStartable.perTest())
```

---

## Quick Start | Kotest

**URL:** https://kotest.io/docs/next/quickstart

**Contents:**
- Quick Start
- Test Framework​
- Assertions Library​
- Property Testing​
- Snapshots​

Kotest is a flexible and comprehensive testing project for Kotlin with multiplatform support.

For latest updates see Changelog.

Kotest is divided into three, stand-alone projects, each of which can be used independently:

You can decide to go all in on Kotest and use all three together, or you can choose to one or more modules in conjunction with other projects. For example, you could use the assertion library with JUnit, or you could use the test framework with another assertion library like AssertJ.

This page gives setup instructions for various combinations of projects and targets.

Kotest is a multiplatform project and supports all targets - JVM, JS, Native, iOS and so on.

The Kotest test framework is supported on all targets. On the JVM it builds on top of the JUnit Platform project, and on Kotlin Multiplatform, it leverages the existing Gradle Test infrastructure. To set up kotest as your testing framework, follow detailed instructions in the framework documentation page.

The core assertions library framework is supported on all targets. Extensions are supported on the platforms that are applicable to that extension. For example, the JDBC matchers are only provided for the JVM since JDBC is a Java library.

Add the following dependency to your dependencies block:

Add the following dependency to your build.

Add the following dependency to your commonTest or target specific dependencies block:

View the assertions library documentation for more information.

The property test framework is supported on all targets.

Add the following dependency to your build:

Add the following dependency to your build.

Add the following dependency to your commonTest or target specific dependencies block:

View the property testing documentation for more information.

Snapshots are automatically published on each commit to master. If you want to use the latest snapshot build, set up the dependencies in the same way described above, changing the version to the current snapshot version and add the following repository to your repositories block:

The latest snapshot version can be found on the GitHub README page in the badges section.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest:kotest-assertions-core:$version")
```

Example 2 (xml):
```xml
<dependency>   <groupId>io.kotest</groupId>   <artifactId>kotest-assertions-core-jvm</artifactId>   <version>${version}</version>   <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
implementation("io.kotest:kotest-assertions-core:$version")
```

Example 4 (kotlin):
```kotlin
testImplementation("io.kotest:kotest-property:$version")
```

---

## Changelog | Kotest

**URL:** https://kotest.io/docs/next/changelog.html

**Contents:**
- Changelog
- 6.0.2 September 2025​
  - What's Changed​
  - New Contributors​
- 5.7.2 September 2023​
    - Fixes​
- 5.7.1 September 2023​
    - Fixes​
- 5.7.0 September 2023​
    - Improvements​

Thank you to all contributors since the 5.6.0 release

With Kotest 5.6.0, Codepoint.ascii() was changed to include a wider range of ascii chararacters, and Codepoint.printableAscii() was introduced with the historic range used by Codepoint.ascii().

Arb.string() has been using Codepoint.ascii() as it's default for generating chars for the string. This caused issues for some users, and we decided to revert Arb.string() to the historic behavior by changing the default to the new Codepoint.printableAscii().

Hopefully this doesn't cause any issues for you. If it does, you can revert to the 5.6.0 ~ 5.6.1 behavior by using Codepoint.ascii() explicitly.

If you added explicit usage of Codepoint.printableAscii() to circumvent the issue, you can safely remove the explicit parameter starting with Kotest 5.6.2.

Full Changelog: https://github.com/kotest/kotest/compare/v5.6.1...v5.6.2

This release is mainly to add some missing klib dependencies for ios

Note this release bumps the minimum required version of Kotlin to 1.8.0. If you are using < 1.8.0 you can continue to use Kotest 5.5.x

Thank you to all the contributors since the 5.5.0 release:

Kotest now requires the runtime version of JUnit to be at least 5.8.x

Note: If your build somehow manages to put both JUnit 5.7 and 5.8 onto the classpath and they load in the wrong order, you might see problems related to ClassOrderer not being found. Please make sure that only JUnit 5.8+ is loaded

Thanks to all the contributors since the 5.4.0 release:

Thanks to all contributors since the 5.3.0 release:

Thanks to all the contributors since the 5.2.0 release:

Thanks to all the contributors:

Thanks to all the contributors:

See detailed post about 5.0 features and changes here

Thanks to all authors who contributed to this huge release. In alphabetical order (all commits since 4.6.0)

AJ Alt, Ali Khaleqi Yekta, Alphonse Bendt, Andrew Tasso, Ashish Kumar, Ashish Kumar Joy, Bart van Helvert, Charles Korn, Christoph Pickl, Cory Thomas, dave08, Derek Chen-Becker, dimsuz, Emil Kantis, Federico Aloi, Hugo Martins, IgorTs2004, Imran Settuba, Ing. Jan Kaláb, IvanPavlov1995, Javier Segovia Córdoba, Jean-Michel Fayard, Jerry Preissler, Jim Schneidereit, Leonardo Colman, Marcono1234, Marvin Sielenkemper, Mervyn McCreight, Michael Werner, Mikhail Pogorelov, Mitchell Yuwono, Nico Richard, niqdev, OliverO2, Rustam Musin, Scott Busche, Sebastian Schuberth, Simon Vergauwen, sksamuel, Srki Rakic, SuhyeonPark, Tobie Wee

This is a small release which adds support for Kotlin 1.5 while remaining compatible with Kotlin 1.4.x

Ashish Kumar Joy, Jim Schneidereit, Łukasz Wasylkowski, sksamuel

As part of this release, third party extensions were promoted to top level repositories instead of modules inside the main kotest repo. This allows the extensions to iterate quickly, without needing to wait for a full Kotest release.

From 4.5 onwards, the namespace for all extensions has changed to io.kotest.extensions and the versioning reset to 1.0.0.

So, for example, if you used the Spring extension, you would previously have added io.kotest:kotest-extensions-spring:4.x.y to your build. Now you would use io.kotest.extensions:kotest-extensions-spring:1.x.y

See the full list of extension modules.

Note: Release 4.4.2 is compiled against Kotlin 1.4.31 and coroutines 1.4.3

Note: Release 4.4.1 bumps the minimum required version of Kotlin to 1.4.30

Note: Release 4.4.0 bumps the minimum required version of Kotlin to 1.4.21

The 4.0.0 release is a large release. With the project rename, the packages have changed and module names have changed.

In KotlinTest 3.1.x it is sufficient to enable JUnit in the test block of your gradle build instead of using the gradle junit plugin. This step is the same as for any test framework that uses the JUnit Platform.

Assuming you have gradle 4.6 or above, then setup your test block like this:

You can additionally enable extra test logging:

In the 3.0.x train, the ability to allow an instance per test was removed from some spec styles due to implementation difficulties. This has been addressed in 3.1.x and so all spec styles now allow instance per test as in the 2.0.x releases. Note: The default value is false, so tests will use a single shared instance of the spec for all tests unless the isInstancePerTest() function is overridden to return true.

The syntax for config has now changed. Instead of a function call after the test has been defined, it is now specified after the name of the test.

All matchers can now be used as extension functions. So instead of:

Note: The infix style is not deprecated and will be supported in future releases, but the extension function is intended to be the preferred style moving forward as it allows discovery in the IDE.

Tests that an Int is even or odd:

Asserts that an int or long is in the given range:

Checks that a collection contains the given element at a specified index:

Help out the type inferrer when using nulls:

readable, writeable, executable and hidden

Tests if a file is readable, writeable, or hidden:

absolute and relative

Tests if a file's path is relative or absolute.

Tests if a file's path begins with the specified prefix:

haveSameHashCodeAs(other)

Asserts that two objects have the same hash code.

haveSameLengthAs(other)

Asserts that two strings have the same length.

haveScheme, havePort, haveHost, haveParameter, havePath, haveFragment

Date matchers - before / after / haveSameYear / haveSameDay / haveSameMonth / within

Collections - containNull, containDuplicates

Futures - completed, cancelled

String - haveLineCount, contain(regex)

Types - haveAnnotation(class)

A new module has been added which includes matchers for Arrow - the popular and awesome functional programming library for Kotlin. To include this module add kotlintest-assertions-arrow to your build.

The included matchers are:

Option - Test that an Option has the given value or is a None. For example:

Either- Test that an Either is either a Right or Left. For example:

NonEmptyList- A collection (no pun intended) of matchers for Arrow's NonEmptyList. These mostly mirror the equivalent Collection matchers but for NELs. For example:

Try - Test that a Try is either Success or Failure.

Validation - Asserts that a Validation is either Valid or an Invalid

A powerful way of generating random class instances from primitive generators is to use the new bind function. A simple example is to take a data class of two fields, and then use two base generators and bind them to create random values of that class.

When using property testing, it can be useful to see the distribution of values generated, to ensure you're getting a good spread of values and not just trival ones. For example, you might want to run a test on a String and you want to ensure you're getting good amounts of strings with whitespace.

To generate stats on the distribution, use classify with a predicate, a label if the predicate passes, and a label if the predicate fails. For example:

And this will output something like:

So we can see we're getting a good spread of both types of value.

You don't have to include two labels if you just wish to tag the "true" case, and you can include more than one classification. For example:

This will output something like:

Property Testing: Shrinking

A new type of extension has been added called TagExtension. Implementations can override the tags() function defined in this interface to dynamically return the Tag instances that should be active at any moment. The existing system properties kotlintest.tags.include and kotlintest.tags.exclude are still valid and are not deprecated, but adding this new extension means extended scope for more complicated logic at runtime.

An example might be to disable any Hadoop tests when not running in an environment that doesn't have the hadoop home env variable set. After creating a TagExtension it must be registered with the project config.

Inside the DiscoveryExtension interface the function fun <T : Spec> instantiate(clazz: KClass<T\>): Spec? has been added which allows you to extend the way new instances of Spec are created. By default, a no-args constructor is assumed. However, if this function is overridden then it's possible to support Spec classes which have other constructors. For example, the Spring module now supports constructor injection using this extension. Other use cases might be when you want to always inject some config class, or if you want to ensure that all your tests extend some custom interface or superclass.

As a reminder, DiscoveryExtension instances are added to Project config.

An extension that allows you to test for a function that writes to System.out or System.err. To use this extension add the module kotlintest-extensions-system to your build.

By adding the NoSystemOutListener or NoSystemErrListener to your config or spec classes, anytime a function tries to write to either of these streams, a SystemOutWriteException or SystemErrWriteException will be raised with the string that the function tried to write. This allows you to test for the exception in your code.

Another extension that is part of the kotlintest-extensions-system module. This extension will allow you to test if System.exit(Int) is invoked in a function. It achieves this by intercepting any calls to System.exit and instead of terminating the JVM, it will throw a SystemExitException with the exit code.

The spring extension module kotlintest-extensions-spring has been updated to allow for constructor injection. This new extension is called SpringAutowireConstructorExtension and must be added to your `ProjectConfig. Then you can use injected dependencies directly in the primary constructor of your test class.

A JUnit 4 runner has been added which allows KotlinTest to run using the legacy JUnit 4 platform. To use this, add kotlintest-runner-junit4 to your build instead of kotlintest-runner-junit5.

Note: This is intended for use when junit5 cannot be used. It should not be the first choice as functionality is restricted.

KotlinTest has been split into multiple modules. These include core, assertions, the junit runner, and extensions such as spring, allure and junit-xml.

The idea is that in a future release, further runners could be added (TestNG) or for JS support (once multiplatform Kotlin is out of beta). When upgrading you will typically want to add the kotlintest-core, kotlintest-assertions and kotlintest-runner-junit5 to your build rather than the old kotlintest module which is now defunct. When upgrading, you might find that you need to update imports to some matchers.

Also you must include apply plugin: 'org.junit.platform.gradle.plugin' in your project and classpath "org.junit.platform:junit-platform-gradle-plugin:1.1.0" to the dependencies section of your buildscript or tests will not run (or worse, will hang). This allows gradle to execute jUnit-platform-5 based tests (which KotlinTest builds upon). Note: Gradle says that this is not required as of 4.6 but even with 4.6 it seems to be required.

You need to include the following in your plugins:

as a regular dependency.

Project wide config in KotlinTest is controlled by implementing a subclass of AbstractProjectConfig. In previous versions you could call this what you wanted, and place it where you wanted, and KotlinTest would attempt to find it and use it. This was the cause of many bug reports about project start up times and reflection errors. So in version 3.0.x onwards, KotlinTest will no longer attempt to scan the classpath.

Instead you must call this class ProjectConfig and place it in a package io.kotlintest.provided. It must still be a subclass of AbstractProjectConfig This means kotlintest can do a simple Class.forName to find it, and so there is no startup penalty nor reflection issues.

Project config now allows you to register multiple types of extensions and listeners, as well as setting parallelism.

The previous inteceptors were sometimes confusing. You had to invoke the continuation function or the spec/test would not execute. Not invoking the function didn't mean the spec/test was skipped, but that it would hang.

So interceptors are deprecated, and in some places removed. Those are not removed are now located in classes called SpecExtension and TestCaseExtension and those interfaces should be used rather than functions directly.

Here is an example of a migrated interceptor.

As a replacement, in 3.0.0 we've added the TestListener interface which is the more traditional before/after style callbacks. In addition, these methods include the result of the test (success, fail, error, skipped) which gives you more context in writing plugins. The TestListener interface offers everything the old interceptors could do, and more.

Here is an example of a simple listener.

If you want to use these methods in a Spec itself, then you can just override the functions directly because a Spec is already a TestListener.

Listeners can be added project wide by overriding listeners() in the ProjectConfig.

Note: In the next release, new Extension functions will be added which will be similar to the old interceptors, but with complete control over the lifecycle. For instance, a future intercept method will enforce that the user skip, run or abort a test in the around advice. They will be more complex, and so suited to more advanced use cases. The new TestListener interface will remain of course, and is the preferred option.

If you want to run more than one spec class in parallel, you can by overriding parallelism inside your projects ProjectConfig or by supplying the system property kotlintest.parallelism.

Note the system property always takes precedence over the config.

Test cases now support waiting on futures in a neat way. If you have a value in a CompletableFuture that you want to test against once it completes, then you can do this like this:

The shouldThrow<T\> method has been changed to also test for subclasses. For example, shouldThrow<IOException> will also match exceptions of type FileNotFoundException. This is different to the behavior in all previous KotlinTest versions. If you wish to have functionality as before - testing exactly for that type - then you can use the newly added shouldThrowExactly<T\>.

Support for writing out reports in junit-format XML has added via the kotlintest-extensions-junitxml module which you will need to add to your build. This module provides a JUnitXmlListener which you can register with your project to autowire your tests. You can register this by overriding listeners() in ProjectConfig.

Spring support has been added via the kotlintest-extensions-spring module which you will need to add to your build. This module provides a SpringListener which you can register with your project to autowire your tests. You can register this for just some classes by overriding the listeners() function inside your spec, for example:

Or you can register this for all classes by adding it to the ProjectConfig. See the section on ProjectConfig for how to do this.

The system property used to include/exclude tags has been renamed to kotlintest.tags.include and kotlintest.tags.exclude. Make sure you update your build jobs to set the right properties as the old ones no longer have any effect. If the old tags are detected then a warning message will be emitted on startup.

beInstanceOf<T\> has been added to easily test that a class is an instance of T. This is in addition to the more verbose beInstanceOf(SomeType::class).

The following matchers have been added for maps: containAll, haveKeys, haveValues. These will output helpful error messages showing you which keys/values or entries were missing.

New matchers added for Strings: haveSameLengthAs(other), beEmpty(), beBlank(), containOnlyDigits(), containADigit(), containIgnoringCase(substring), lowerCase(), upperCase().

New matchers for URIs: haveHost(hostname), havePort(port), haveScheme(scheme).

New matchers for collections: containNoNulls(), containOnlyNulls()

One instance per test is no longer supported for specs which offer nested scopes. For example, WordSpec. This is because of the tricky nature of having nested closures work across fresh instances of the spec. When using one instance per test, a fresh spec class is required for each test, but that means selectively executing some closures and not others in order to ensure the correct state. This has proved the largest source of bugs in previous versions.

KotlinTest 3.0.x takes a simplified approach. If you want the flexibilty to lay out your tests with nested scopes, then all tests will execute in the same instance (like Spek and ScalaTest). If you want each test to have it's own instance (like jUnit) then you can either split up your tests into multiple files, or use a "flat" spec like FunSpec or StringSpec.

This keeps the implementation an order of magnitude simplier (and therefore less likely to lead to bugs) while offering a pragmatic approach to keeping both sets of fans happy.

Multiple new specs have been added. These are: AnnotationSpec, DescribeSpec and ExpectSpec. Expect spec allows you to use the context and expect keywords in your tests, like so:

The AnnotationSpec offers functionality to mimic jUnit, in that tests are simply functions annotated with @io.kotlintest.specs.Test. For example:

And finally, the DescribeSpec is similar to SpekFramework, using describe, and, and it. This makes it very useful for those people who are looking to migrate to KotlinTest from SpekFramework.

The ability to use matchers in property testing has been added. Previously property testing worked only with functions that returned a Boolean, like:

But now you can use assertAll and assertNone and then use regular matchers inside the block. For example:

This gives you the ability to use multiple matchers inside the same block, and not have to worry about combining all possible errors into a single boolean result.

Staying with property testing - the Generator interface has been changed to now provide two types of data.

The first are values that should always be included - those edge cases values which are common sources of bugs. For example, a generator for Ints should always include values like zero, minus 1, positive 1, Integer.MAXVALUE and Integer.MIN_VALUE. Another example would be for a generator for enums. That should include _all the values of the enum to ensure each value is tested.

The second set of values are random values, which are used to give us a greater breadth of values tested. The Int generator should return random ints from across the entire integer range.

Previously generators used by property testing would only include random values, which meant you were very unlikely to see the edge cases that usually cause issues - like the aforementioned Integer MAX / MIN. Now you are guaranteed to get the edge cases first and the random values afterwards.

This interface added a couple of helpers for Mockito, and was used primarily before Kotlin specific mocking libraries appeared. Now there is little value in this mini-wrapper so it was removed. Simply add whatever mocking library you like to your build and use it as normal.

This class has been added for loading data for table testing. A simple example:

All matchers now have the ability to report a better error when used with shouldNot and shouldNotBe. Previously a generic error was generated - which was usually the normal error but with a prefix like "NOT:" but now each built in matcher will provide a full message, for example: Collection should not contain element 'foo'

Interceptors have been added. Interceptors allow code to be executed before and after a test. See the main readme for more info.

Simplified ability to add custom matchers. Simple implement Matcher<T\> interface. See readme for more information.

Added shouldNot to invert matchers. Eg, "hello" shouldNot include("hallo")

Deprecated matchers which do not implement Matcher<T>. Eg, should have substring(x) has been deprecated in favour of "hello" should include("l"). This is because instances of Matcher<T> can be combined with or and and and can be negated with shouldNot.

Added between matcher for int and long, eg

3 shouldBe between(2, 5)

x shouldBe singleElement(y)

listOf(1,2,3) shouldBe sorted<Int>()

Now supports comparsion of arrays #116

Added a shouldBe exactly(b) matcher for doubles

kotlintest only pulls in mockito-core now instead of mockito-all

That test will be executed 100 times with random values in each test. See more in the readme.

You can now write config(timeout = 2.seconds) instead of config(timeout = 2, timeoutUnit = TimeUnit.SECONDS).

**Examples:**

Example 1 (unknown):
```unknown
test {    useJUnitPlatform()}
```

Example 2 (unknown):
```unknown
test {    useJUnitPlatform()    testLogging {        events "FAILED", "SKIPPED", "STANDARD_OUT", "STANDARD_ERROR"    }}
```

Example 3 (kotlin):
```kotlin
"this is a test" {}.config(...)
```

Example 4 (kotlin):
```kotlin
"this is a test".config(...) {}
```

---

## Blogs and Articles on Kotest | Kotest

**URL:** https://kotest.io/docs/next/blogs

**Contents:**
- Blogs and Articles on Kotest

These blogs and articles can be useful in addition to the official docs to show how people are using Kotest in the wild.

Please open a PR to add your blog or article here, preferably at the top of the list.

---

## Release 5.0 | Kotest

**URL:** https://kotest.io/docs/next/blog/release_5.0

**Contents:**
- Release 5.0
- Breaking Changes​
    - Kotlin 1.6 is now the minimum supported version​
    - Legacy Javascript compiler support dropped​
    - Global configuration object dropped​
    - Experimental data testing classes moved​
    - Deprecated property test Arb.value removed​
    - Startup configuration dump off by default​
    - Deprecated method removals​
    - Inspector changes.​

This document is a work in progress and will be finalized within a few days of the 5.0 release.

From version 5 onwards, Kotest requires Kotlin 1.6. The decision to do this is twofold.

Firstly, the main feature in the 5.0 release train is support for multiplatform tests and there are incompatibilities in the compiler between Kotlin 1.5 and 1.6. To support both would add needless complexity to the build.

Secondly, kotlin.time.Duration's have finally gone stable as of 1.6 and Kotest builds on these internally. We wanted to be able to depend on the multiplatform functionality provided by Kotlin durations without any issues arising from changes in these classes from previous versions.

Javascript support for the legacy compiler is no longer supported. If you are running tests on JS legacy then you will need to continue using Kotest 4.6.x or set your Javascript test build to use only use the IR compiler.

Test support for the legacy Javascript compiler relied on functionality that has been removed in the IR compiler (Namely, that the framework adapter no longer works with third party modules). For the 5.0 release, Kotest provides a compiler plugin which integrates tests directly into the compiled output exactly how the kotlin.test support works.

In previous versions Kotest supported configuration updates via a global configuration object called configuration (and called project in even earlier versions). From this release onwards, this top level val has been removed.

The reason for the removal is that having global state complicated using multiple instances of the Test Engine in the same JVM and also because there was not precise semanatics around the orders of updates to a top level val.

The former was mainly an issue when testing Kotest itself, since users don't typically create instances of TestEngine directly but instead run tests via gradle or intellij.

This top level val was not included in documentation so users should have been largely unaware it existed anyway. The recommended approach to defining Kotest configuration remains either ProjectConfig or system properties.

The experimental data-test withData functions added in 4.5 under the package name io.kotest.datatest have moved to a new module kotest-framework-datatest.

Note: These are separate from the forAll and forNone data test functions which have been part of Kotest since version 2.0.

The Arb.values() method has been removed from the Arb interface. This method was deprecated in 4.3 in favour of using Arb.sample which was introduced to allow for Arb flat-mapping. This will only affect anyone who has written a custom arb that extends Arb directly and is still using the deprecated method. Any existing uses of the arbitrary builders is unaffected and those builders are always the preferred way to create custom arbitraries.

The Engine no longer logs config to the console during start by default. To enable output, set the system property or env var kotest.framework.dump.config to true.

When using inspectors, the deprecated kotlintest.assertions.output.max system property has been removed. This was replaced with kotest.assertions.output.max in release 4.0 when the project was renamed from KotlinTest to Kotest.

The TestStatus enum has been deprecated in favour of the TestResult ADT. Instead of matching onresult.status in AfterTestListener you should now match directly on the result. Eg

The intercept(KClass) method in SpecExtension has been deprecated and SpecRefExtension has been added. The deprecated method had ambigious behavior when used with an IsolationMode that created multiple instances of a spec. The new methods have precise guarantees of when they will execute.

The defaultTestCaseConfig containers in Spec's and project configuration have been deprecated. This is because it was not possible to specify at both the spec level and the project-configuration level and allow settings to fall through.

Instead, you should set per-setting defaults, and these will fall through from test -> spec -> configuration.

For example, instead of this:

Note that the second variation has always been possible, but the first variation is no longer recommended.

The val name inside Listener has been deprecated. This was used so that errors from multiple before/after spec callbacks could appear with customized unique names. The framework now takes ensures that names are unique so this val is no longer needed and is now ignored.

**Examples:**

Example 1 (kotlin):
```kotlin
when (result) {  is TestResult.Success -> ...  is TestResult.Error -> ...}
```

Example 2 (kotlin):
```kotlin
class MySpec: FunSpec() {  init {    override fun defaultTestCaseConfig() = TestCaseConfig(tags = setOf(Foo, Bar), timeout = 100.seconds)    test("foo") {      // will time out after 100 seconds and has tags Foo and Bar applied    }  }}
```

Example 3 (kotlin):
```kotlin
class MySpec: FunSpec() {  init {    tags(Foo, Bar)    timeout = 100.seconds    test("foo") {       // will time out after 100 seconds and has tags Foo and Bar applied    }  }}
```

---

## IntelliJ Plugin | Kotest

**URL:** https://kotest.io/docs/next/intellij/intellij-plugin.html

**Contents:**
- IntelliJ Plugin
- Gutter Icons​
- Running Tests​
- Duplicated Test Highlighting​
- Context Menu Run / Debug​
- Intentions​

Kotest offers an IntelliJ plugin available at the jetbrains plugin marketplace (search from within IntelliJ).

This plugin provides run icons for each test, a tool window for test navigation, duplicated test highlighting, assertion intentions, and more.

The Intellij plugin requires Kotest 4.2 or higher and will not run common tests of a multiplatform project

The plugin provides gutter run icons for specs, top level tests, and nested tests.

Any tests disabled via a bang or by xfunctions such as xdescribe, will have a disabled test icon in the gutter.

If you execute a spec from the gutter icon, then all tests in that spec will be executed. If you execute a test, then that test and all nested tests will be executed.

For Gradle based projects: to run tests with the KoTest runner ensure your project's Gradle Settings are set to run tests with IntelliJ, not Gradle:

You cannot have two tests with the same name. The plugin will highlight any duplicated test names as errors.

Right clicking on a package will allow you to run, debug or run with coverage all the tests inside that package.

This plugin has some basic intentions. For example, you can quickly mark a test as disabled.

Or you can highlight some text and mark it as should throw, or surround with a soft assertion block.

---
