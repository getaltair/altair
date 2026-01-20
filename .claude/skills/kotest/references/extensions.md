# Kotest - Extensions

**Pages:** 170

---

## BlockHound | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/blockhound.html

**Contents:**
- BlockHound
  - Getting Started​
  - Detection​
  - Customization​

The Kotest BlockHound extension activates BlockHound support for coroutines. It helps to detect blocking code on non-blocking coroutine threads, e.g. when accidentally calling a blocking I/O library function on a UI thread.

To use this extension add the io.kotest.extensions:kotest-extensions-blockhound module to your test compile path.

Register the BlockHound extension in your test class:

The BlockHound extension can also be registered per test case or at the project level.

If BlockHound is enabled project-wide or spec-wide, you can disable it for an individual test:

You can also change BlockHoundMode for a section of code:

Blocking calls will be detected in coroutine threads which are expected not to block. Such threads are created by the default dispatcher as this example demonstrates:

The BlockHound extension will by default produce an exception like this whenever it detects a blocking call:

By invoking it as BlockHound(BlockHoundMode.PRINT), it will print detected calls and continue the test without interruption.

Whenever a blocking call is detected, you can

To customize BlockHound, familiarize yourself with the BlockHound documentation.

Exceptions for blocking calls considered harmless can be added via a separate BlockHoundIntegration class like this:

In order to allow BlockHound to auto-detect and load the integration, add its fully qualified class name to a service provider configuration file resources/META-INF/services/reactor.blockhound.integration.BlockHoundIntegration.

**Examples:**

Example 1 (kotlin):
```kotlin
class BlockHoundSpecTest : FunSpec({   extension(BlockHound())   test("detects for spec") {      blockInNonBlockingContext()   }})
```

Example 2 (kotlin):
```kotlin
test("allow blocking").config(extensions = listOf(BlockHound(BlockHoundMode.DISABLED))) {      blockInNonBlockingContext()   }
```

Example 3 (kotlin):
```kotlin
test("allow blocking section") {      // ...      withBlockHoundMode(BlockHoundMode.DISABLED) {        blockInNonBlockingContext()      }      // ...   }
```

Example 4 (kotlin):
```kotlin
private suspend fun blockInNonBlockingContext() {   withContext(Dispatchers.Default) {      @Suppress("BlockingMethodInNonBlockingContext")      Thread.sleep(2)   }}
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/next/extensions/pitest.html

**Contents:**
- Pitest
- Gradle configuration​
- Maven configuration​

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest:kotest-extensions-pitest module to your dependencies as well:

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

First of all, you need to configure the Maven Pitest plugin:

Then add the dependency on Pitest Kotest extension:

This should be enough to be able to run Pitest and get the reports as described in the Maven Pitest plugin.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

Example 3 (xml):
```xml
<plugin>    <groupId>org.pitest</groupId>    <artifactId>pitest-maven</artifactId>    <version>${pitest-maven.version}</version>    <configuration>        <targetClasses>...</targetClasses>        <coverageThreshold>...</coverageThreshold>        ... other configurations as needed    </configuration></plugin>
```

Example 4 (xml):
```xml
<dependencies>  ... the other Kotest dependencies like kotest-runner-junit5  <dependency>    <groupId>io.kotest</groupId>    <artifactId>kotest-extensions-pitest</artifactId>    <version>${kotest-extensions-pitest.version}</version>    <scope>test</scope>  </dependency></dependencies>
```

---

## BlockHound | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/blockhound.html

**Contents:**
- BlockHound
  - Getting Started​
  - Detection​
  - Customization​

The Kotest BlockHound extension activates BlockHound support for coroutines. It helps to detect blocking code on non-blocking coroutine threads, e.g. when accidentally calling a blocking I/O library function on a UI thread.

To use this extension add the io.kotest:kotest-extensions-blockhound module to your test compile path.

Register the BlockHound extension in your test class:

The BlockHound extension can also be registered per test case or at the project level.

If BlockHound is enabled project-wide or spec-wide, you can disable it for an individual test:

You can also change BlockHoundMode for a section of code:

Blocking calls will be detected in coroutine threads which are expected not to block. Such threads are created by the default dispatcher as this example demonstrates:

The BlockHound extension will by default produce an exception like this whenever it detects a blocking call:

By invoking it as BlockHound(BlockHoundMode.PRINT), it will print detected calls and continue the test without interruption.

Whenever a blocking call is detected, you can

To customize BlockHound, familiarize yourself with the BlockHound documentation.

Exceptions for blocking calls considered harmless can be added via a separate BlockHoundIntegration class like this:

In order to allow BlockHound to auto-detect and load the integration, add its fully qualified class name to a service provider configuration file resources/META-INF/services/reactor.blockhound.integration.BlockHoundIntegration.

**Examples:**

Example 1 (kotlin):
```kotlin
class BlockHoundSpecTest : FunSpec({   extension(BlockHound())   test("detects for spec") {      blockInNonBlockingContext()   }})
```

Example 2 (kotlin):
```kotlin
test("allow blocking").config(extensions = listOf(BlockHound(BlockHoundMode.DISABLED))) {      blockInNonBlockingContext()   }
```

Example 3 (kotlin):
```kotlin
test("allow blocking section") {      // ...      withBlockHoundMode(BlockHoundMode.DISABLED) {        blockInNonBlockingContext()      }      // ...   }
```

Example 4 (kotlin):
```kotlin
private suspend fun blockInNonBlockingContext() {   withContext(Dispatchers.Default) {      @Suppress("BlockingMethodInNonBlockingContext")      Thread.sleep(2)   }}
```

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
  - Dependencies​
  - Databases​
    - Initializing the Database Container​
  - General Containers​
  - Kafka Containers​
  - Lifecycle​
  - Startables​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 5.0+. For earlier versions see docs here

The Testcontainers project provides lightweight, ephemeral instances of common databases, elasticsearch, kafka, Selenium web browsers, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides several extensions - specialized extensions for databases and kafka and general containers support for any supported docker image.

To begin, add the following dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

For Maven, you will need these dependencies:

For JDBC compatible databases, Kotest provides the JdbcTestContainerExtension. This provides a pooled javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension wrapper, providing an optional configuration lambda.

If you don't wish to configure the pool, then you can omit the trailing lambda.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

This extension also supports the ContainerLifecycleMode flag to control when the container is started and stopped. See Lifecycle

There are two ways to initialize the database container: via a single init script added to the TestContainer config, or via a list of scripts added to the JdbcTestContainerExtension config lambda.

If adding a single script, via the TestContainer config, simply add the script to the TestContainer's withInitScript config option, like so:

If you have multiple init scripts or sets of changesets, you can add them as a list to the dbInitScripts extension config lambda, like so:

The list can contain absolute or relative paths, for files and folders on the filesystem or on the classpath.

The extension will process the list provided in order. If the list item is a folder, it will process all .sql scripts in the folder, sorted lexicographically. These scripts run every time the container is started, so it supports the ContainerLifecycleMode flag.

Similar to the JdbcDatabaseContainerExtension, this module also provides a ContainerExtension extension which can wrap any container, not just databases.

We can create the extension using either a docker image name, or a strongly typed container.

For example, using a docker image directly:

And then using a strongly typed container:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives us access to specific settings - such as the password option in the elasticsearch example above.

However, when a strongly typed container is not available, the former method allows us to spool up any docker image as a general container.

This extension also supports the ContainerLifecycleMode flag to control when the container is started and stopped. See Lifecycle

For Kafka, this module provides convenient extension methods to create a consumer, producer or admin client from the container.

Inside the configuration lambda, we can specify options for the Kafka container, such as embedded/external zookeeper, or kafka broker properties through env vars. For example, to enable dynamic topic creation:

Kafka only publishes a linux/amd64 version of the container. If you're on an Apple Silicon/ARM architecture computer, you'll need to explicitly specify the platform with the following added to the configuration lambda outlined above:

Once we have the container installed, we can create a client using the following methods:

Each of these accepts an optional configuration lambda to enable setting values on the properties object that is used to create the clients.

For example, in this test, we produce and consume a message from the same topic, and we use the configuration lambda to set max poll to 1.

When creating a consumer, the consumer group is set to a random uuid. To change this, provide a configuration lambda and specify your own group consumer group id.

By default, the lifecycle of a container is per spec - so it will be started at the install command, and shutdown as the spec is completed. This can be changed to start/stop per test, per leaf test, or per root test.

To do this, pass in a ContainerLifecycleMode parameter to the ContainerExtension or JdbcDatabaseContainerExtension.

This module also provides extension methodsscope which let you convert any Startable such as a DockerContainer into a kotest TestListener, which you can register with Kotest and then Kotest will manage the lifecycle of that container for you.

In above example, the perTest() extension method converts the container into a TestListener, which starts the redis container before each test and stops it after test. Similarly if you want to reuse the container for all tests in a single spec class you can use perSpec() extension method, which converts the container into a TestListener which starts the container before running any test in the spec, and stops it after all tests, thus a single container is used by all tests in spec class.

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-testcontainers</artifactId>    <version>${kotest.version}</version>    <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val mysql = MySQLContainer<Nothing>("mysql:8.0.26").apply {  startupAttempts = 1  withUrlParam("connectionTimeZone", "Z")  withUrlParam("zeroDateTimeBehavior", "convertToNull")}
```

Example 4 (kotlin):
```kotlin
val ds = install(JdbcDatabaseContainerExtension(mysql)) {  poolName = "myconnectionpool"  maximumPoolSize = 8  idleTimeout = 10000}
```

---

## Koin | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/koin.html

**Contents:**
- Koin
- Koin​

The Koin DI Framework can be used with Kotest through the KoinExtension extension.

To use the extension in your project, add the dependency to your project:

With the dependency added, we can easily use Koin in our tests!

By default, the extension will start/stop the Koin context between leaf tests. If you are using a nested spec style (like DescribeSpec) and instead want the Koin context to persist over all leafs of a root tests (for example to share mocked declarations between tests), you can specify the lifecycle mode as KoinLifecycleMode.Root in the KoinExtension constructor.

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest.extensions:kotest-extensions-koin:${version}
```

Example 2 (kotlin):
```kotlin
class KotestAndKoin : FunSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(myKoinModule))    val userService by inject<UserService>()    init {        test("use userService") {            userService.getUser().username shouldBe "LeoColman"        }    }}
```

Example 3 (kotlin):
```kotlin
class KotestAndKoin : DescribeSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(module = myKoinModule, mode = KoinLifecycleMode.Root))    val userService by inject<UserService>()    init {        describe("use userService") {            it("inside a leaf test") {                userService.getUser().username shouldBe "LeoColman"            }            it("this shares the same context") {                userService.getUser().username shouldBe "LeoColman"            }        }    }}
```

---

## Spring | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/spring.html

**Contents:**
- Spring
  - Constructor Injection​
  - TestContexts​
  - Test Method Callbacks​
  - Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest.extensions:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Note: The maven group id differs from the core test framework (io.kotest.extensions).

The Spring extension requires you to activate it for all test classes, or per test class. To activate it globally, register the SpringExtension in project config:

To activate it per test class:

In order to let Spring know which configuration class to use, you must annotate your Spec classes with @ContextConfiguration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

For constructor injection, Kotest automatically registers a SpringAutowireConstructorExtension when the spring module is added to the build, assuming auto scan is enabled (see Project Config). If Auto scan is disabled, you will need to manually load the extension in your Project config.

This extension will intercept each call to create a Spec instance and will autowire the beans declared in the primary constructor.

The following example is a test class which requires a service called UserService in its primary constructor. This service class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various test callbacks such as beforeTestMethod that are based around the idea that tests are methods. This assumption is fine for legacy test frameworks like JUnit but not applicable to modern test frameworks like Kotest where tests are functions.

Therefore, when using a spec style that is nested, you can customize when the test method callbacks are fired. By default, this is on the leaf node. You can set these to fire on the root nodes by passing a SpringTestLifecycleMode argument to the extension:

When using a final class, you may receive a warning from Kotest:

Using SpringListener on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(SpringExtension)}
```

Example 2 (kotlin):
```kotlin
class MyTestSpec : FunSpec() {   override fun extensions() = listOf(SpringExtension)}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {         println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Environment​
  - System Property Extension​
  - System Security Manager​
  - System Exit Extensions​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

If you need to test code that uses java.lang.System, Kotest provides extensions that can alter the system and restore it after each test. This extension is only available on the JVM.

To use this extension, add the dependency to your project:

This extension does not support concurrent test execution. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is undefined.

With System Environment Extension you can simulate how the System Environment is behaving. That is, what you're obtaining from System.getenv().

Kotest provides some extension functions that provides a System Environment in a specific scope:

To use withEnvironment with JDK17 you need to add --add-opens=java.base/java.util=ALL-UNNAMED to the arguments for the JVM that runs the tests.

If you run tests with gradle, you can add the following to your build.gradle.kts:

You can also use multiple values in this extension, through a map or list of pairs.

These functions will add the keys and values if they're not currently present in the environment, and will override them if they are. Any keys untouched by the function will remain in the environment, and won't be messed with.

Instead of extensions functions, you can also use the provided Listeners to apply these functionalities in a bigger scope. There's an alternative for the Spec/Per test level, and an alternative for the Project Level.

In the same fashion as the Environment Extensions, you can override the System Properties (System.getProperties()):

And with similar Listeners:

Similarly, with System Security Manager you can override the System Security Manager (System.getSecurityManager())

Sometimes you want to test that your code calls System.exit. For that you can use the System Exit Listeners. The Listener will throw an exception when the System.exit is called, allowing you to catch it and verify:

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

And with the listeners

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest:kotest-extensions-jvm:${version}
```

Example 2 (kotlin):
```kotlin
withEnvironment("FooKey", "BarValue") {    System.getenv("FooKey") shouldBe "BarValue" // System environment overridden!}
```

Example 3 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {    jvmArgs("--add-opens=java.base/java.util=ALL-UNNAMED")}
```

Example 4 (kotlin):
```kotlin
withEnvironment(mapOf("FooKey" to "BarValue", "BarKey" to "FooValue")) {  // Use FooKey and BarKey}
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has three parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override fun extensions(): List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false,      useTestPathAsName = true    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Since Kotest 5.6.0, Current instant listeners are located in the artifact io.kotest:kotest-extensions-now:${kotest-version}.

Add it as a dependency to use any of the functionality mentioned below.

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has three parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override fun extensions(): List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false, // don't write out status for all tests      useTestPathAsName = true, // use the full test path (ie, includes parent test names)      outputDir = "../target/junit-xml" // include to set output dir for maven    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Spring | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/spring.html

**Contents:**
- Spring
  - Constructor Injection​
  - TestContexts​
  - Test Method Callbacks​
  - Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest.extensions:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Note: The maven group id differs from the core test framework (io.kotest.extensions).

The Spring extension requires you to activate it for all test classes, or per test class. To activate it globally, register the SpringExtension in project config:

To activate it per test class:

In order to let Spring know which configuration class to use, you must annotate your Spec classes with @ContextConfiguration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

For constructor injection, Kotest automatically registers a SpringAutowireConstructorExtension when the spring module is added to the build.

This extension will intercept each call to create a Spec instance and will autowire the beans declared in the primary constructor.

The following example is a test class which requires a service called UserService in its primary constructor. This service class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various test callbacks such as beforeTestMethod that are based around the idea that tests are methods. This assumption is fine for legacy test frameworks like JUnit but not applicable to modern test frameworks like Kotest where tests are functions.

Therefore, when using a spec style that is nested, you can customize when the test method callbacks are fired. By default, this is on the leaf node. You can set these to fire on the root nodes by passing a SpringTestLifecycleMode argument to the extension:

When using a final class, you may receive a warning from Kotest:

Using SpringListener on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(SpringExtension)}
```

Example 2 (kotlin):
```kotlin
class MyTestSpec : FunSpec() {   override fun extensions() = listOf(SpringExtension)}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {         println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
  - Dependencies​
  - Databases​
    - Initializing the Database Container​
  - General Containers​
  - Kafka Containers​
  - Lifecycle​
  - Startables​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 5.0+. For earlier versions see docs here

The Testcontainers project provides lightweight, ephemeral instances of common databases, elasticsearch, kafka, Selenium web browsers, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides several extensions - specialized extensions for databases and kafka and general containers support for any supported docker image.

To begin, add the following dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

For Maven, you will need these dependencies:

For JDBC compatible databases, Kotest provides the JdbcTestContainerExtension. This provides a pooled javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension wrapper, providing an optional configuration lambda.

If you don't wish to configure the pool, then you can omit the trailing lambda.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

This extension also supports the LifecycleMode flag to control when the container is started and stopped. See Lifecycle

There are two ways to initialize the database container: via a single init script added to the TestContainer config, or via a list of scripts added to the JdbcTestContainerExtension config lambda.

If adding a single script, via the TestContainer config, simply add the script to the TestContainer's withInitScript config option, like so:

If you have multiple init scripts or sets of changesets, you can add them as a list to the dbInitScripts extension config lambda, like so:

The list can contain absolute or relative paths, for files and folders on the filesystem or on the classpath.

The extension will process the list provided in order. If the list item is a folder, it will process all .sql scripts in the folder, sorted lexicographically. These scripts run every time the container is started, so it supports the LifecycleMode flag.

Similar to the JdbcTestContainerExtension, this module also provides a TestContainerExtension extension which can wrap any container, not just databases.

We can create the extension using either a docker image name, or a strongly typed container.

For example, using a docker image directly:

And then using a strongly typed container:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives us access to specific settings - such as the password option in the elasticsearch example above.

However, when a strongly typed container is not available, the former method allows us to spool up any docker image as a general container.

This extension also supports the LifecycleMode flag to control when the container is started and stopped. See Lifecycle

For Kafka, this module provides convenient extension methods to create a consumer, producer or admin client from the container.

Inside the configuration lambda, we can specify options for the Kafka container, such as embedded/external zookeeper, or kafka broker properties through env vars. For example, to enable dynamic topic creation:

Kafka only publishes a linux/amd64 version of the container. If you're on an Apple Silicon/ARM architecture computer, you'll need to explicitly specify the platform with the following added to the configuration lambda outlined above:

Once we have the container installed, we can create a client using the following methods:

Each of these accepts an optional configuration lambda to enable setting values on the properties object that is used to create the clients.

For example, in this test, we produce and consume a message from the same topic, and we use the configuration lambda to set max poll to 1.

When creating a consumer, the consumer group is set to a random uuid. To change this, provide a configuration lambda and specify your own group consumer group id.

By default, the lifecycle of a container is per spec - so it will be started at the install command, and shutdown as the spec is completed. This can be changed to start/stop per test, per leaf test, or per root test.

To do this, pass in a LifecycleMode parameter to the TestContainerExtension or JdbcTestContainerExtension.

If you change the lifecycle mode from Spec then the container will not be started in the constructor, and so any operations that act on the container must be placed inside the test scopes.

This module also provides extension methodsscope which let you convert any Startable such as a DockerContainer into a kotest TestListener, which you can register with Kotest and then Kotest will manage the lifecycle of that container for you.

In above example, the perTest() extension method converts the container into a TestListener, which starts the redis container before each test and stops it after test. Similarly if you want to reuse the container for all tests in a single spec class you can use perSpec() extension method, which converts the container into a TestListener which starts the container before running any test in the spec, and stops it after all tests, thus a single container is used by all tests in spec class.

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-testcontainers</artifactId>    <version>${kotest.version}</version>    <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val mysql = MySQLContainer<Nothing>("mysql:8.0.26").apply {  startupAttempts = 1  withUrlParam("connectionTimeZone", "Z")  withUrlParam("zeroDateTimeBehavior", "convertToNull")}
```

Example 4 (kotlin):
```kotlin
val ds = install(JdbcTestContainerExtension(mysql)) {  poolName = "myconnectionpool"  maximumPoolSize = 8  idleTimeout = 10000}
```

---

## Koin | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/koin.html

**Contents:**
- Koin
- Koin​

The Koin DI Framework can be used with Kotest through the KoinExtension extension.

To use the extension in your project, add the dependency to your project:

With the dependency added, we can easily use Koin in our tests!

By default, the extension will start/stop the Koin context between leaf tests. If you are using a nested spec style (like DescribeSpec) and instead want the Koin context to persist over all leafs of a root tests (for example to share mocked declarations between tests), you can specify the lifecycle mode as KoinLifecycleMode.Root in the KoinExtension constructor.

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest.extensions:kotest-extensions-koin:${version}
```

Example 2 (kotlin):
```kotlin
class KotestAndKoin : FunSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(myKoinModule))    val userService by inject<UserService>()    init {        test("use userService") {            userService.getUser().username shouldBe "LeoColman"        }    }}
```

Example 3 (kotlin):
```kotlin
class KotestAndKoin : DescribeSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(module = myKoinModule, mode = KoinLifecycleMode.Root))    val userService by inject<UserService>()    init {        describe("use userService") {            it("inside a leaf test") {                userService.getUser().username shouldBe "LeoColman"            }            it("this shares the same context") {                userService.getUser().username shouldBe "LeoColman"            }        }    }}
```

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Additionally, prevent Gradle from generating its own html reports by adding html.required.set(false) to the test task.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override fun extensions(): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

Example 2 (css):
```css
tasks.test {  useJUnitPlatform()  reports {    html.required.set(false)    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/pitest.html

**Contents:**
- Pitest
- Gradle configuration​
- Maven configuration​

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest.extensions:kotest-extensions-pitest module to your dependencies as well:

Note: Since pitest is an extension, we use a different maven group name (io.kotest.extensions) from the core modules.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

First of all, you need to configure the Maven Pitest plugin:

Then add the dependency on Pitest Kotest extension:

This should be enough to be able to run Pitest and get the reports as described in the Maven Pitest plugin.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

Example 3 (xml):
```xml
<plugin>    <groupId>org.pitest</groupId>    <artifactId>pitest-maven</artifactId>    <version>${pitest-maven.version}</version>    <configuration>        <targetClasses>...</targetClasses>        <coverageThreshold>...</coverageThreshold>        ... other configurations as needed            </configuration></plugin>
```

Example 4 (xml):
```xml
<dependencies>  ... the other Kotest dependencies like kotest-runner-junit5-jvm   <dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-pitest</artifactId>    <version>${kotest-extensions-pitest.version}</version>    <scope>test</scope>  </dependency></dependencies>
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Since Kotest 5.6.0, Current instant listeners are located in the artifact io.kotest:kotest-extensions-now:${kotest-version}.

Add it as a dependency to use any of the functionality mentioned below.

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Setting Build Dir​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional junit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using allure with kotest. To start, add the below dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your buld, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will write out test data in the allure json format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the allure binary, or we can use the allure gradle plugin. To use the gradle plugin, first add the plugin to your build's plugins block.

Next, add an allure configuration section to set the version and disable autoconfigure (because allure can only auto configure junit and kotest takes care of this for you anyway).

Finally, execute the gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the gradle plugin then you will need to inform Allure where the build dir is by setting the allure.results.directory system property on your tests configuration. If you are using the gradle plugin, then this can be skipped as the gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.8.1"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Environment​
  - System Property Extension​
  - System Security Manager​
  - System Exit Extensions​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

If you need to test code that uses java.lang.System, Kotest provides extensions that can alter the system and restore it after each test. This extension is only available on the JVM.

To use this extension, add the dependency to your project:

This extension does not support concurrent test execution. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is undefined.

With System Environment Extension you can simulate how the System Environment is behaving. That is, what you're obtaining from System.getenv().

Kotest provides some extension functions that provides a System Environment in a specific scope:

To use withEnvironment with JDK17 you need to add --add-opens=java.base/java.util=ALL-UNNAMED to the arguments for the JVM that runs the tests.

If you run tests with gradle, you can add the following to your build.gradle.kts:

You can also use multiple values in this extension, through a map or list of pairs.

These functions will add the keys and values if they're not currently present in the environment, and will override them if they are. Any keys untouched by the function will remain in the environment, and won't be messed with.

Instead of extensions functions, you can also use the provided Listeners to apply these functionalities in a bigger scope. There's an alternative for the Spec/Per test level, and an alternative for the Project Level.

In the same fashion as the Environment Extensions, you can override the System Properties (System.getProperties()):

And with similar Listeners:

Similarly, with System Security Manager you can override the System Security Manager (System.getSecurityManager())

Sometimes you want to test that your code calls System.exit. For that you can use the System Exit Listeners. The Listener will throw an exception when the System.exit is called, allowing you to catch it and verify:

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

And with the listeners

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest:kotest-extensions-jvm:${version}
```

Example 2 (kotlin):
```kotlin
withEnvironment("FooKey", "BarValue") {    System.getenv("FooKey") shouldBe "BarValue" // System environment overridden!}
```

Example 3 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {    jvmArgs("--add-opens=java.base/java.util=ALL-UNNAMED")}
```

Example 4 (kotlin):
```kotlin
withEnvironment(mapOf("FooKey" to "BarValue", "BarKey" to "FooValue")) {  // Use FooKey and BarKey}
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/next/extensions/extensions.html

**Contents:**
- Extensions
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties. For extensions provided directly by the Kotest team, see the links on the left.

---

## Spring | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/spring.html

**Contents:**
- Spring
  - Constructor Injection​
  - TestContexts​
  - Test Method Callbacks​
  - Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest.extensions:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Note: The maven group id differs from the core test framework (io.kotest.extensions).

The Spring extension requires you to activate it for all test classes, or per test class. To activate it globally, register the SpringExtension in project config:

To activate it per test class:

In order to let Spring know which configuration class to use, you must annotate your Spec classes with @ContextConfiguration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

For constructor injection, Kotest automatically registers a SpringAutowireConstructorExtension when the spring module is added to the build.

This extension will intercept each call to create a Spec instance and will autowire the beans declared in the primary constructor.

The following example is a test class which requires a service called UserService in its primary constructor. This service class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various test callbacks such as beforeTestMethod that are based around the idea that tests are methods. This assumption is fine for legacy test frameworks like JUnit but not applicable to modern test frameworks like Kotest where tests are functions.

Therefore, when using a spec style that is nested, you can customize when the test method callbacks are fired. By default, this is on the leaf node. You can set these to fire on the root nodes by passing a SpringTestLifecycleMode argument to the extension:

When using a final class, you may receive a warning from Kotest:

Using SpringListener on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(SpringExtension)}
```

Example 2 (kotlin):
```kotlin
class MyTestSpec : FunSpec() {   override fun extensions() = listOf(SpringExtension)}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {         println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/mockserver.html

**Contents:**
- MockServer

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest.extensions:kotest-extensions-mockserver module to be added to your build.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we attach an instance of MockServerListener to the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      listener(MockServerListener(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override fun extensions(): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/mockserver.html

**Contents:**
- MockServer

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest.extensions:kotest-extensions-mockserver module to be added to your build.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we attach an instance of MockServerListener to the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      listener(MockServerListener(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Property Extension​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

If you need to test code that uses java.lang.System, Kotest provides extensions that can alter the system and restore it after each test. This extension is only available on the JVM.

To use this extension, add the dependency to your project:

This extension does not support concurrent test execution. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is undefined.

You can override the System Properties (System.getProperties()) by either using a listener at the spec level, or by using the withSystemProperty function to wrap any arbitrary code.

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest:kotest-extensions:${version}
```

Example 2 (kotlin):
```kotlin
withSystemProperty("foo", "bar") {  System.getProperty("foo") shouldBe "bar"}
```

Example 3 (kotlin):
```kotlin
class MyTest : FreeSpec() {  override val extensions = listOf(SystemPropertyTestListener("foo", "bar"))  init {    "MyTest" {      System.getProperty("foo") shouldBe "bar"    }  }}
```

Example 4 (kotlin):
```kotlin
// In Project or in Specoverride val extensions = listOf(NoSystemOutListener, NoSystemErrListener)
```

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/mockserver.html

**Contents:**
- MockServer

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest.extensions:kotest-extensions-mockserver module to be added to your build.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we attach an instance of MockServerListener to the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      listener(MockServerListener(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/pitest.html

**Contents:**
- Pitest
- Gradle configuration​
- Maven configuration​

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest.extensions:kotest-extensions-pitest module to your dependencies as well:

Note: Since pitest is an extension, we use a different maven group name (io.kotest.extensions) from the core modules.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

First of all, you need to configure the Maven Pitest plugin:

Then add the dependency on Pitest Kotest extension:

This should be enough to be able to run Pitest and get the reports as described in the Maven Pitest plugin.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

Example 3 (xml):
```xml
<plugin>    <groupId>org.pitest</groupId>    <artifactId>pitest-maven</artifactId>    <version>${pitest-maven.version}</version>    <configuration>        <targetClasses>...</targetClasses>        <coverageThreshold>...</coverageThreshold>        ... other configurations as needed            </configuration></plugin>
```

Example 4 (xml):
```xml
<dependencies>  ... the other Kotest dependencies like kotest-runner-junit5-jvm   <dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-pitest</artifactId>    <version>${kotest-extensions-pitest.version}</version>    <scope>test</scope>  </dependency></dependencies>
```

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-extensions-wiremock:${kotestVersion}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  extension(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  extension(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/extensions.html

**Contents:**
- Extensions
  - Kotest Team Extensions​
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties.

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Environment​
  - System Property Extension​
  - System Security Manager​
  - System Exit Extensions​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

Sometimes your code might use some functionalities straight from the JVM, which are very hard to simulate. With Kotest System Extensions, these difficulties are made easy to mock and simulate, and your code can be tested correctly. After changing the system and using the extensions, the previous state will be restored.

This code is sensitive to concurrency. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is unknown.

With System Environment Extension you can simulate how the System Environment is behaving. That is, what you're obtaining from System.getenv().

Kotest provides some extension functions that provides a System Environment in a specific scope:

To use withEnvironment with JDK17 you need to add --add-opens=java.base/java.util=ALL-UNNAMED to the arguments for the JVM that runs the tests.

If you run tests with gradle, you can add the following to your build.gradle.kts:

You can also use multiple values in this extension, through a map or list of pairs.

These functions will add the keys and values if they're not currently present in the environment, and will override them if they are. Any keys untouched by the function will remain in the environment, and won't be messed with.

Instead of extensions functions, you can also use the provided Listeners to apply these functionalities in a bigger scope. There's an alternative for the Spec/Per test level, and an alternative for the Project Level.

In the same fashion as the Environment Extensions, you can override the System Properties (System.getProperties()):

And with similar Listeners:

Similarly, with System Security Manager you can override the System Security Manager (System.getSecurityManager())

Sometimes you want to test that your code calls System.exit. For that you can use the System Exit Listeners. The Listener will throw an exception when the System.exit is called, allowing you to catch it and verify:

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

And with the listeners

**Examples:**

Example 1 (kotlin):
```kotlin
withEnvironment("FooKey", "BarValue") {    System.getenv("FooKey") shouldBe "BarValue" // System environment overridden!}
```

Example 2 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {  jvmArgs("--add-opens=java.base/java.util=ALL-UNNAMED")}
```

Example 3 (kotlin):
```kotlin
withEnvironment(mapOf("FooKey" to "BarValue", "BarKey" to "FooValue")) {  // Use FooKey and BarKey}
```

Example 4 (kotlin):
```kotlin
class MyTest : FreeSpec() {      override fun listeners() = listOf(SystemEnvironmentTestListener("foo", "bar"))    init {      "MyTest" {        System.getenv("foo") shouldBe "bar"      }    }}
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/mockserver.html

**Contents:**
- MockServer

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest.extensions:kotest-extensions-mockserver module to be added to your build.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we attach an instance of MockServerListener to the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      listener(MockServerListener(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has three parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override fun extensions(): List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false, // don't write out status for all tests      useTestPathAsName = true, // use the full test path (ie, includes parent test names)      outputDir = "../target/junit-xml" // include to set output dir for maven    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/pitest.html

**Contents:**
- Pitest
- Gradle configuration​
- Maven configuration​

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest.extensions:kotest-extensions-pitest module to your dependencies as well:

Note: Since pitest is an extension, we use a different maven group name (io.kotest.extensions) from the core modules.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

First of all, you need to configure the Maven Pitest plugin:

Then add the dependency on Pitest Kotest extension:

This should be enough to be able to run Pitest and get the reports as described in the Maven Pitest plugin.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

Example 3 (xml):
```xml
<plugin>    <groupId>org.pitest</groupId>    <artifactId>pitest-maven</artifactId>    <version>${pitest-maven.version}</version>    <configuration>        <targetClasses>...</targetClasses>        <coverageThreshold>...</coverageThreshold>        ... other configurations as needed            </configuration></plugin>
```

Example 4 (xml):
```xml
<dependencies>  ... the other Kotest dependencies like kotest-runner-junit5-jvm   <dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-pitest</artifactId>    <version>${kotest-extensions-pitest.version}</version>    <scope>test</scope>  </dependency></dependencies>
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---

## Embedded Kafka Extension | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/embedded-kafka.html

**Contents:**
- Embedded Kafka Extension
  - Getting started:​
  - Consumer / Producer​
  - Custom Ports​

Kotest offers an extension that spins up an embedded Kafka instance. This can help in situations where using the kafka docker images are an issue.

To use this extension add the io.kotest.extensions:kotest-extensions-embedded-kafka module to your test compile path.

Register the embeddedKafkaListener listener in your test class:

And the broker will be started once the spec is created and stopped once the spec completes.

Note: The underlying embedded kafka library uses a global object for state. Do not start multiple kafka instances at the same time.

To create a consumer and producer we can use convenience methods on the listener:

The stringStringProducer and stringStringConsumer methods return a producer / consumer that accept strings for the keys and values. Similar methods exist for byte pairs.

Alternatively, you can access the host/port the Kafka instance was deployed on and create the clients yourself:

You can create a new instance of the listener specifying a port and then use that instance rather than the default instance.

You can also do specify the zookeeper port using an alternative overload.

**Examples:**

Example 1 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({  listener(embeddedKafkaListener)})
```

Example 2 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec() {  init {    listener(embeddedKafkaListener)  }}
```

Example 3 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)   test("send / receive") {     val producer = embeddedKafkaListener.stringStringProducer()     producer.send(ProducerRecord("foo", "a"))     producer.close()     val consumer = embeddedKafkaListener.stringStringConsumer("foo")     eventually(10.seconds) {       consumer.poll(1000).first().value() shouldBe "a"     }     consumer.close()   }})
```

Example 4 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)      val props = Properties().apply {      put(CommonClientConfigs.BOOTSTRAP_SERVERS_CONFIG, "${embeddedKafkaListener.host}:${embeddedKafkaListener.port}")   }      val producer = KafkaProducer<String, String>(props)   }
```

---

## Test Clock | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/test_clock.html

**Contents:**
- Test Clock

The JVM provides the java.time.Clock interface which is used to generate Instants. When we have code that relies on time, we can use a Clock to generate the values, rather than using things like Instant.now() or System.currentTimeMillis().

Then in tests we can provide a fixed or controllable clock which avoids issues where the time changes on each test run. In your real code, you provide an instance of Clock.systemUTC() or whatever.

The following module is needed: io.kotest.extensions:kotest-extensions-clock in your build. Search maven central for latest version here.

In order to use it, we create an instance of the TestClock passing in an instant and a zone offset.

We can control the clock via plus and minus which accept durations, eg

Note that the clock is mutable, and the internal state is changed when you use plus or minus.

**Examples:**

Example 1 (unknown):
```unknown
val timestamp = Instant.ofEpochMilli(1234)val clock = TestClock(timestamp, ZoneOffset.UTC)
```

Example 2 (unknown):
```unknown
clock.plus(6.minutes)
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has three parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override val extensions: List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false, // don't write out status for all tests      useTestPathAsName = true, // use the full test path (ie, includes parent test names)      outputDir = "../target/junit-xml" // include to set output dir for maven    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Decoroutinator | Kotest

**URL:** https://kotest.io/docs/extensions/decoroutinator.html

**Contents:**
- Decoroutinator
  - Getting Started​
  - How It Works​
  - Example​

The Kotest Decoroutinator extension integrates Stacktrace Decoroutinator with Kotest. Decoroutinator improves stack traces in Kotlin coroutines by removing the internal coroutine implementation details, making stack traces cleaner and easier to understand.

To use this extension add the io.kotest:kotest-extensions-decoroutinator module to your test compile path.

Register the DecoroutinatorExtension in your test class:

The DecoroutinatorExtension can also be registered at the project level for all tests:

When a test fails due to an exception in a coroutine, the stack trace typically contains many internal coroutine implementation details that make it difficult to understand the actual cause of the failure. The Decoroutinator extension automatically installs the Decoroutinator JVM API, which:

Without Decoroutinator, a stack trace might look like:

With Decoroutinator, the same stack trace becomes:

This makes it much easier to identify and fix issues in your coroutine-based tests.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({   extension(DecoroutinatorExtension())   test("with clean stack traces") {      // Your test code with coroutines   }})
```

Example 2 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(DecoroutinatorExtension())}
```

Example 3 (swift):
```swift
java.lang.IllegalStateException: Test exception    at com.example.MyTest$1$1.invokeSuspend(MyTest.kt:15)    at kotlin.coroutines.jvm.internal.BaseContinuationImpl.resumeWith(ContinuationImpl.kt:33)    at kotlinx.coroutines.DispatchedTask.run(DispatchedTask.kt:106)    at kotlinx.coroutines.scheduling.CoroutineScheduler.runSafely(CoroutineScheduler.kt:570)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.executeTask(CoroutineScheduler.kt:750)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.runWorker(CoroutineScheduler.kt:677)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.run(CoroutineScheduler.kt:664)    ... many more internal details
```

Example 4 (swift):
```swift
java.lang.IllegalStateException: Test exception    at com.example.MyTest$1$1.invokeSuspend(MyTest.kt:15)    ... coroutine implementation details removed for clarity
```

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (json):
```json
io.kotest.extensions:kotest-extensions-wiremock:{version}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## Test Clock | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/test_clock.html

**Contents:**
- Test Clock

The JVM provides the java.time.Clock interface which is used to generate Instants. When we have code that relies on time, we can use a Clock to generate the values, rather than using things like Instant.now() or System.currentTimeMillis().

Then in tests we can provide a fixed or controllable clock which avoids issues where the time changes on each test run. In your real code, you provide an instance of Clock.systemUTC() or whatever.

The following module is needed: io.kotest:kotest-extensions in your build. Search maven central for latest version here.

Since Kotest 6.0, all extensions are published under the io.kotest group, with version cadence tied to main Kotest releases.

In order to use it, we create an instance of the TestClock passing in an instant and a zone offset.

We can control the clock via plus and minus which accept durations, eg

Note that the clock is mutable, and the internal state is changed when you use plus or minus.

**Examples:**

Example 1 (unknown):
```unknown
val timestamp = Instant.ofEpochMilli(1234)val clock = TestClock(timestamp, ZoneOffset.UTC)
```

Example 2 (unknown):
```unknown
clock.plus(6.minutes)
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/next/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has three parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override val extensions: List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false, // don't write out status for all tests      useTestPathAsName = true, // use the full test path (ie, includes parent test names)      outputDir = "../target/junit-xml" // include to set output dir for maven    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/extensions.html

**Contents:**
- Extensions
  - Kotest Team Extensions​
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties.

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override fun extensions(): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/pitest.html

**Contents:**
- Pitest
- Gradle configuration​
- Maven configuration​

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest.extensions:kotest-extensions-pitest module to your dependencies as well:

Note: Since pitest is an extension, we use a different maven group name (io.kotest.extensions) from the core modules.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

First of all, you need to configure the Maven Pitest plugin:

Then add the dependency on Pitest Kotest extension:

This should be enough to be able to run Pitest and get the reports as described in the Maven Pitest plugin.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

Example 3 (xml):
```xml
<plugin>    <groupId>org.pitest</groupId>    <artifactId>pitest-maven</artifactId>    <version>${pitest-maven.version}</version>    <configuration>        <targetClasses>...</targetClasses>        <coverageThreshold>...</coverageThreshold>        ... other configurations as needed    </configuration></plugin>
```

Example 4 (xml):
```xml
<dependencies>  ... the other Kotest dependencies like kotest-runner-junit5  <dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-pitest</artifactId>    <version>${kotest-extensions-pitest.version}</version>    <scope>test</scope>  </dependency></dependencies>
```

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-extensions-wiremock:${kotestVersion}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  extension(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  extension(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (json):
```json
io.kotest.extensions:kotest-extensions-wiremock:{version}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Environment​
  - System Property Extension​
  - System Security Manager​
  - System Exit Extensions​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

Sometimes your code might use some functionalities straight from the JVM, which are very hard to simulate. With Kotest System Extensions, these difficulties are made easy to mock and simulate, and your code can be tested correctly. After changing the system and using the extensions, the previous state will be restored.

This code is sensitive to concurrency. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is unknown.

With System Environment Extension you can simulate how the System Environment is behaving. That is, what you're obtaining from System.getenv().

Kotest provides some extension functions that provides a System Environment in a specific scope:

To use withEnvironment with JDK17 you need to add --add-opens=java.base/java.util=ALL-UNNAMED to the arguments for the JVM that runs the tests.

If you run tests with gradle, you can add the following to your build.gradle.kts:

You can also use multiple values in this extension, through a map or list of pairs.

These functions will add the keys and values if they're not currently present in the environment, and will override them if they are. Any keys untouched by the function will remain in the environment, and won't be messed with.

Instead of extensions functions, you can also use the provided Listeners to apply these functionalities in a bigger scope. There's an alternative for the Spec/Per test level, and an alternative for the Project Level.

In the same fashion as the Environment Extensions, you can override the System Properties (System.getProperties()):

And with similar Listeners:

Similarly, with System Security Manager you can override the System Security Manager (System.getSecurityManager())

Sometimes you want to test that your code calls System.exit. For that you can use the System Exit Listeners. The Listener will throw an exception when the System.exit is called, allowing you to catch it and verify:

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

And with the listeners

**Examples:**

Example 1 (kotlin):
```kotlin
withEnvironment("FooKey", "BarValue") {    System.getenv("FooKey") shouldBe "BarValue" // System environment overridden!}
```

Example 2 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {  jvmArgs("--add-opens=java.base/java.util=ALL-UNNAMED")}
```

Example 3 (kotlin):
```kotlin
withEnvironment(mapOf("FooKey" to "BarValue", "BarKey" to "FooValue")) {  // Use FooKey and BarKey}
```

Example 4 (kotlin):
```kotlin
class MyTest : FreeSpec() {      override fun listeners() = listOf(SystemEnvironmentTestListener("foo", "bar"))    init {      "MyTest" {        System.getenv("foo") shouldBe "bar"      }    }}
```

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
  - Dependencies​
  - Databases​
    - Initializing the Database Container​
  - General Containers​
  - Kafka Containers​
  - Lifecycle​
  - Startables​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 5.0+. For earlier versions see docs here

The Testcontainers project provides lightweight, ephemeral instances of common databases, elasticsearch, kafka, Selenium web browsers, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides several extensions - specialized extensions for databases and kafka and general containers support for any supported docker image.

To begin, add the following dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

For Maven, you will need these dependencies:

For JDBC compatible databases, Kotest provides the JdbcTestContainerExtension. This provides a pooled javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension wrapper, providing an optional configuration lambda.

If you don't wish to configure the pool, then you can omit the trailing lambda.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

This extension also supports the LifecycleMode flag to control when the container is started and stopped. See Lifecycle

There are two ways to initialize the database container: via a single init script added to the TestContainer config, or via a list of scripts added to the JdbcTestContainerExtension config lambda.

If adding a single script, via the TestContainer config, simply add the script to the TestContainer's withInitScript config option, like so:

If you have multiple init scripts or sets of changesets, you can add them as a list to the dbInitScripts extension config lambda, like so:

The list can contain absolute or relative paths, for files and folders on the filesystem or on the classpath.

The extension will process the list provided in order. If the list item is a folder, it will process all .sql scripts in the folder, sorted lexicographically. These scripts run every time the container is started, so it supports the LifecycleMode flag.

Similar to the JdbcTestContainerExtension, this module also provides a TestContainerExtension extension which can wrap any container, not just databases.

We can create the extension using either a docker image name, or a strongly typed container.

For example, using a docker image directly:

And then using a strongly typed container:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives us access to specific settings - such as the password option in the elasticsearch example above.

However, when a strongly typed container is not available, the former method allows us to spool up any docker image as a general container.

This extension also supports the LifecycleMode flag to control when the container is started and stopped. See Lifecycle

For Kafka, this module provides convenient extension methods to create a consumer, producer or admin client from the container.

Inside the configuration lambda, we can specify options for the Kafka container, such as embedded/external zookeeper, or kafka broker properties through env vars. For example, to enable dynamic topic creation:

Kafka only publishes a linux/amd64 version of the container. If you're on an Apple Silicon/ARM architecture computer, you'll need to explicitly specify the platform with the following added to the configuration lambda outlined above:

Once we have the container installed, we can create a client using the following methods:

Each of these accepts an optional configuration lambda to enable setting values on the properties object that is used to create the clients.

For example, in this test, we produce and consume a message from the same topic, and we use the configuration lambda to set max poll to 1.

When creating a consumer, the consumer group is set to a random uuid. To change this, provide a configuration lambda and specify your own group consumer group id.

By default, the lifecycle of a container is per spec - so it will be started at the install command, and shutdown as the spec is completed. This can be changed to start/stop per test, per leaf test, or per root test.

To do this, pass in a LifecycleMode parameter to the TestContainerExtension or JdbcTestContainerExtension.

If you change the lifecycle mode from Spec then the container will not be started in the constructor, and so any operations that act on the container must be placed inside the test scopes.

This module also provides extension methodsscope which let you convert any Startable such as a DockerContainer into a kotest TestListener, which you can register with Kotest and then Kotest will manage the lifecycle of that container for you.

In above example, the perTest() extension method converts the container into a TestListener, which starts the redis container before each test and stops it after test. Similarly if you want to reuse the container for all tests in a single spec class you can use perSpec() extension method, which converts the container into a TestListener which starts the container before running any test in the spec, and stops it after all tests, thus a single container is used by all tests in spec class.

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-testcontainers</artifactId>    <version>${kotest.version}</version>    <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val mysql = MySQLContainer<Nothing>("mysql:8.0.26").apply {  startupAttempts = 1  withUrlParam("connectionTimeZone", "Z")  withUrlParam("zeroDateTimeBehavior", "convertToNull")}
```

Example 4 (kotlin):
```kotlin
val ds = install(JdbcTestContainerExtension(mysql)) {  poolName = "myconnectionpool"  maximumPoolSize = 8  idleTimeout = 10000}
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Setting Build Dir​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional junit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using allure with kotest. To start, add the below dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your buld, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will write out test data in the allure json format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the allure binary, or we can use the allure gradle plugin. To use the gradle plugin, first add the plugin to your build's plugins block.

Next, add an allure configuration section to set the version and disable autoconfigure (because allure can only auto configure junit and kotest takes care of this for you anyway).

Finally, execute the gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the gradle plugin then you will need to inform Allure where the build dir is by setting the allure.results.directory system property on your tests configuration. If you are using the gradle plugin, then this can be skipped as the gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.8.1"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Additionally, prevent Gradle from generating its own html reports by adding html.required.set(false) to the test task.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override fun extensions(): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

Example 2 (css):
```css
tasks.test {  useJUnitPlatform()  reports {    html.required.set(false)    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/extensions.html

**Contents:**
- Extensions
  - Kotest Team Extensions​
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties.

---

## Robolectric | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/robolectric.html

**Contents:**
- Robolectric
- Robolectric​

Robolectric can be used with Kotest through the RobolectricExtension which can be found in a separate repository,kotest-extensions-robolectric

To add this module to project you need specify following in your build.gradle:

This dependency brings in RobolectricExtension, which is autoregistered to your projects.

Now all you need to do is annotate Robolectric specs with @RobolectricTest and you're set!

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-robolectric:${version}")
```

Example 2 (kotlin):
```kotlin
@RobolectricTestclass MyTest : ShouldSpec({    should("Access Robolectric normally!") {    }})
```

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/next/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
- Dependencies​
- Generic Containers​
- Databases​
- Compose Containers​
- Container Logs​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 6.0+.

The Testcontainers project provides lightweight, ephemeral instances of common databases, Elasticsearch, Kafka, Redis, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides support for:

To begin, add the following dependency to your Gradle build file.

For Maven, you will need these dependencies:

Since Kotest 6.0, all extensions are published under the io.kotest group with version cadence tied to the main Kotest releases.

Kotest provides two GenericContainer extensions - TestContainerSpecExtension and TestContainerProjectExtension which can be used to wrap any container and which tie the lifecycle of the container to the lifecycle of the spec or project depending on which extension is used.

We can create the extension using either a strongly typed container, or docker image name.

Using a strongly typed container:

Or using a docker image name:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives access to specific settings - such as the password option in the elasticsearch example above. However, when a strongly typed container is not available, the former method allows us to fall back to any docker image as a general container.

Once the container is installed, you can use the containers's host and port to configure a client to access that container. For example, to connect a Jedis client to a Redis container:

Use the TestContainerProjectExtension if you want to share the same container across multiple specs in a project for faster startup times.

For JDBC compatible databases, Kotest provides the JdbcDatabaseContainerSpecExtension and JdbcDatabaseContainerProjectExtension. These return not the container directly, but a javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension, providing an optional configuration lambda for Hikari.

If you don't wish to configure the pool, then you can omit the trailing lambda. If you don't want to use Hikari, then you can use the generic extensions instead.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

Use the JdbcDatabaseContainerProjectExtension if you want to share the same container across multiple specs in a project for faster startup times.

Kotest provides two extensions for ComposeContainers - ComposeContainerSpecExtension and ComposeContainerProjectExtension. This extension can be used to wrap ComposeContainer and startup multiple containers at once defined in a docker compose file.

We can create the extension using a File pointing to the docker compose file:

Alternative, if our docker compose file is in the resources folder of our project, we can use the following shortcut:

Kotest provides the option to capture the logs from the containers that are started by the extensions and output those in the test console. This can be enabled by passing an instance of ContainerExtensionConfig to the extension. In the config instance, set the logConsumer option to be StandardLogConsumer, specifying the level of logs to capture. For example:

The log types can be set to capture ALL, STDOUT, STDERR or NONE.

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>  <groupId>io.kotest</groupId>  <artifactId>kotest-extensions-testcontainers</artifactId>  <version>${kotest.version}</version>  <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val elasticsearch = install(ContainerExtension(ElasticsearchContainer(ELASTICSEARCH_IMAGE))) {  withPassword(ELASTICSEARCH_PASSWORD)}
```

Example 4 (kotlin):
```kotlin
val container = install(TestContainerSpecExtension(GenericContainer("redis:5.0.3-alpine"))) {  startupAttempts = 2  withExposedPorts(6379)}val jedis = JedisPool(container.host, container.firstMappedPort)
```

---

## Decoroutinator | Kotest

**URL:** https://kotest.io/docs/next/extensions/decoroutinator.html

**Contents:**
- Decoroutinator
  - Getting Started​
  - How It Works​
  - Example​

The Kotest Decoroutinator extension integrates Stacktrace Decoroutinator with Kotest. Decoroutinator improves stack traces in Kotlin coroutines by removing the internal coroutine implementation details, making stack traces cleaner and easier to understand.

To use this extension add the io.kotest:kotest-extensions-decoroutinator module to your test compile path.

Register the DecoroutinatorExtension in your test class:

The DecoroutinatorExtension can also be registered at the project level for all tests:

When a test fails due to an exception in a coroutine, the stack trace typically contains many internal coroutine implementation details that make it difficult to understand the actual cause of the failure. The Decoroutinator extension automatically installs the Decoroutinator JVM API, which:

Without Decoroutinator, a stack trace might look like:

With Decoroutinator, the same stack trace becomes:

This makes it much easier to identify and fix issues in your coroutine-based tests.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({   extension(DecoroutinatorExtension())   test("with clean stack traces") {      // Your test code with coroutines   }})
```

Example 2 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(DecoroutinatorExtension())}
```

Example 3 (swift):
```swift
java.lang.IllegalStateException: Test exception    at com.example.MyTest$1$1.invokeSuspend(MyTest.kt:15)    at kotlin.coroutines.jvm.internal.BaseContinuationImpl.resumeWith(ContinuationImpl.kt:33)    at kotlinx.coroutines.DispatchedTask.run(DispatchedTask.kt:106)    at kotlinx.coroutines.scheduling.CoroutineScheduler.runSafely(CoroutineScheduler.kt:570)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.executeTask(CoroutineScheduler.kt:750)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.runWorker(CoroutineScheduler.kt:677)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.run(CoroutineScheduler.kt:664)    ... many more internal details
```

Example 4 (swift):
```swift
java.lang.IllegalStateException: Test exception    at com.example.MyTest$1$1.invokeSuspend(MyTest.kt:15)    ... coroutine implementation details removed for clarity
```

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override fun extensions(): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

---

## Decoroutinator | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/decoroutinator.html

**Contents:**
- Decoroutinator
  - Getting Started​
  - How It Works​
  - Example​

The Kotest Decoroutinator extension integrates Stacktrace Decoroutinator with Kotest. Decoroutinator improves stack traces in Kotlin coroutines by removing the internal coroutine implementation details, making stack traces cleaner and easier to understand.

To use this extension add the io.kotest:kotest-extensions-decoroutinator module to your test compile path.

Register the DecoroutinatorExtension in your test class:

The DecoroutinatorExtension can also be registered at the project level for all tests:

When a test fails due to an exception in a coroutine, the stack trace typically contains many internal coroutine implementation details that make it difficult to understand the actual cause of the failure. The Decoroutinator extension automatically installs the Decoroutinator JVM API, which:

Without Decoroutinator, a stack trace might look like:

With Decoroutinator, the same stack trace becomes:

This makes it much easier to identify and fix issues in your coroutine-based tests.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyTest : FunSpec({   extension(DecoroutinatorExtension())   test("with clean stack traces") {      // Your test code with coroutines   }})
```

Example 2 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(DecoroutinatorExtension())}
```

Example 3 (swift):
```swift
java.lang.IllegalStateException: Test exception    at com.example.MyTest$1$1.invokeSuspend(MyTest.kt:15)    at kotlin.coroutines.jvm.internal.BaseContinuationImpl.resumeWith(ContinuationImpl.kt:33)    at kotlinx.coroutines.DispatchedTask.run(DispatchedTask.kt:106)    at kotlinx.coroutines.scheduling.CoroutineScheduler.runSafely(CoroutineScheduler.kt:570)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.executeTask(CoroutineScheduler.kt:750)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.runWorker(CoroutineScheduler.kt:677)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.run(CoroutineScheduler.kt:664)    ... many more internal details
```

Example 4 (swift):
```swift
java.lang.IllegalStateException: Test exception    at com.example.MyTest$1$1.invokeSuspend(MyTest.kt:15)    ... coroutine implementation details removed for clarity
```

---

## Spring | Kotest

**URL:** https://kotest.io/docs/extensions/spring.html

**Contents:**
- Spring
- Getting Started​
- Constructor Injection​
- Test Contexts​
- Test Execution Events​
  - Order of Events​
- Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

The Spring extension requires you to activate it for all spec classes, or per spec class. To activate it globally, you can register the SpringExtension in project config:

Alternatively, you can register the extension per spec class via the @ApplyExtension annotation.

In order to let Spring know which configuration class to use, you must annotate your Spec classes with Spring Test annotation such as @Configuration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

When the Spring extension is active, Kotest will automatically take care of instantating your test classes using the primary constructor. This inclues autowiring constructor parameters.

The following example is a test class which requires a service called UserService in its primary constructor. This UserService class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various Test Execution Events such as BeforeTestMethod and BeforeTestExecution. These events make an assumption that tests are methods, so they do not map exactly one to one to tests defined in frameworks like Kotest where tests are functions and can be nested arbitrarily.

You can customize when these callbacks are fired by using the SpringTestLifecycleMode enum when creating the extension. By default, this is on leaf tests. You can set these to fire on root tests by passing a SpringTestLifecycleMode.Root argument to the extension:

If you want to use root mode with @ApplyExtension, you must use the SpringRootTestExtension subclass, eg @ApplyExtension(SpringRootTestExtension::class). ::

Spring defines a precedence order for when callbacks are fired. There are four events that can fire related to tests. These are

Spring requests that the beforeTestMethod and afterTestMethod events be fired before other user callbacks, and that the beforeTestExecution and afterTestExecution events be fired after other user callbacks. It is not possible in Kotest to define the order of callbacks, but we try to follow the request as closely as possible.

Kotest executes these callbacks in the following groups, where the order between groups is guaranteed, but the order inside any group is not guaranteed:

Group 1: Spring's BeforeTestMethodEvent and any other TestCaseExtensions. Group 2: Spring's BeforeTestExecutionEvent and any other BeforeTest, BeforeAny, and BeforeEach callbacks. Group 3: Spring's AfterTestExecutionEvent and any other AfterTest, AfterAny, and AfterEach callbacks. Group 4: Spring's AfterTestMethodEvent and any other TestCaseExtensions.

When using a final class, you may receive a warning from Kotest:

Using SpringExtension on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override val extensions = listOf(SpringExtension())}
```

Example 2 (kotlin):
```kotlin
@ApplyExtension(SpringExtension::class)class MyTestSpec : FunSpec() {}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {        println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
- Dependencies​
- Generic Containers​
- Databases​
- Compose Containers​
- Container Logs​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 6.0+.

The Testcontainers project provides lightweight, ephemeral instances of common databases, Elasticsearch, Kafka, Redis, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides support for:

To begin, add the following dependency to your Gradle build file.

For Maven, you will need these dependencies:

Since Kotest 6.0, all extensions are published under the io.kotest group with version cadence tied to the main Kotest releases.

Kotest provides two GenericContainer extensions - TestContainerSpecExtension and TestContainerProjectExtension which can be used to wrap any container and which tie the lifecycle of the container to the lifecycle of the spec or project depending on which extension is used.

We can create the extension using either a strongly typed container, or docker image name.

Using a strongly typed container:

Or using a docker image name:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives access to specific settings - such as the password option in the elasticsearch example above. However, when a strongly typed container is not available, the former method allows us to fall back to any docker image as a general container.

Once the container is installed, you can use the containers's host and port to configure a client to access that container. For example, to connect a Jedis client to a Redis container:

Use the TestContainerProjectExtension if you want to share the same container across multiple specs in a project for faster startup times.

For JDBC compatible databases, Kotest provides the JdbcDatabaseContainerSpecExtension and JdbcDatabaseContainerProjectExtension. These return not the container directly, but a javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension, providing an optional configuration lambda for Hikari.

If you don't wish to configure the pool, then you can omit the trailing lambda. If you don't want to use Hikari, then you can use the generic extensions instead.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

Use the JdbcDatabaseContainerProjectExtension if you want to share the same container across multiple specs in a project for faster startup times.

Kotest provides two extensions for ComposeContainers - ComposeContainerSpecExtension and ComposeContainerProjectExtension. This extension can be used to wrap ComposeContainer and startup multiple containers at once defined in a docker compose file.

We can create the extension using a File pointing to the docker compose file:

Alternative, if our docker compose file is in the resources folder of our project, we can use the following shortcut:

Kotest provides the option to capture the logs from the containers that are started by the extensions and output those in the test console. This can be enabled by passing an instance of ContainerExtensionConfig to the extension. In the config instance, set the logConsumer option to be StandardLogConsumer, specifying the level of logs to capture. For example:

The log types can be set to capture ALL, STDOUT, STDERR or NONE.

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>  <groupId>io.kotest</groupId>  <artifactId>kotest-extensions-testcontainers</artifactId>  <version>${kotest.version}</version>  <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val elasticsearch = install(ContainerExtension(ElasticsearchContainer(ELASTICSEARCH_IMAGE))) {  withPassword(ELASTICSEARCH_PASSWORD)}
```

Example 4 (kotlin):
```kotlin
val container = install(TestContainerSpecExtension(GenericContainer("redis:5.0.3-alpine"))) {  startupAttempts = 2  withExposedPorts(6379)}val jedis = JedisPool(container.host, container.firstMappedPort)
```

---

## Test Clock | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/test_clock.html

**Contents:**
- Test Clock

The JVM provides the java.time.Clock interface which is used to generate Instants. When we have code that relies on time, we can use a Clock to generate the values, rather than using things like Instant.now() or System.currentTimeMillis().

Then in tests we can provide a fixed or controllable clock which avoids issues where the time changes on each test run. In your real code, you provide an instance of Clock.systemUTC() or whatever.

The following module is needed: io.kotest.extensions:kotest-extensions-clock in your build. Search maven central for latest version here.

In order to use it, we create an instance of the TestClock passing in an instant and a zone offset.

We can control the clock via plus and minus which accept durations, eg

Note that the clock is mutable, and the internal state is changed when you use plus or minus.

**Examples:**

Example 1 (unknown):
```unknown
val timestamp = Instant.ofEpochMilli(1234)val clock = TestClock(timestamp, ZoneOffset.UTC)
```

Example 2 (unknown):
```unknown
clock.plus(6.minutes)
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has three parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override fun extensions(): List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false, // don't write out status for all tests      useTestPathAsName = true, // use the full test path (ie, includes parent test names)      outputDir = "../target/junit-xml" // include to set output dir for maven    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Spring | Kotest

**URL:** https://kotest.io/docs/next/extensions/spring.html

**Contents:**
- Spring
- Getting Started​
- Constructor Injection​
- Test Contexts​
- Test Execution Events​
  - Order of Events​
- Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

The Spring extension requires you to activate it for all spec classes, or per spec class. To activate it globally, you can register the SpringExtension in project config:

Alternatively, you can register the extension per spec class via the @ApplyExtension annotation.

In order to let Spring know which configuration class to use, you must annotate your Spec classes with Spring Test annotation such as @Configuration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

When the Spring extension is active, Kotest will automatically take care of instantating your test classes using the primary constructor. This inclues autowiring constructor parameters.

The following example is a test class which requires a service called UserService in its primary constructor. This UserService class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various Test Execution Events such as BeforeTestMethod and BeforeTestExecution. These events make an assumption that tests are methods, so they do not map exactly one to one to tests defined in frameworks like Kotest where tests are functions and can be nested arbitrarily.

You can customize when these callbacks are fired by using the SpringTestLifecycleMode enum when creating the extension. By default, this is on leaf tests. You can set these to fire on root tests by passing a SpringTestLifecycleMode.Root argument to the extension:

If you want to use root mode with @ApplyExtension, you must use the SpringRootTestExtension subclass, eg @ApplyExtension(SpringRootTestExtension::class). ::

Spring defines a precedence order for when callbacks are fired. There are four events that can fire related to tests. These are

Spring requests that the beforeTestMethod and afterTestMethod events be fired before other user callbacks, and that the beforeTestExecution and afterTestExecution events be fired after other user callbacks. It is not possible in Kotest to define the order of callbacks, but we try to follow the request as closely as possible.

Kotest executes these callbacks in the following groups, where the order between groups is guaranteed, but the order inside any group is not guaranteed:

Group 1: Spring's BeforeTestMethodEvent and any other TestCaseExtensions. Group 2: Spring's BeforeTestExecutionEvent and any other BeforeTest, BeforeAny, and BeforeEach callbacks. Group 3: Spring's AfterTestExecutionEvent and any other AfterTest, AfterAny, and AfterEach callbacks. Group 4: Spring's AfterTestMethodEvent and any other TestCaseExtensions.

When using a final class, you may receive a warning from Kotest:

Using SpringExtension on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {  override val extensions = listOf(SpringExtension())}
```

Example 2 (kotlin):
```kotlin
@ApplyExtension(SpringExtension::class)class MyTestSpec : FunSpec() {}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {        println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## Spring | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/spring.html

**Contents:**
- Spring
  - Constructor Injection​
  - TestContexts​
  - Test Method Callbacks​
  - Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest.extensions:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Note: The maven group id differs from the core test framework (io.kotest.extensions).

The Spring extension requires you to activate it for all test classes, or per test class. To activate it globally, register the SpringExtension in project config:

To activate it per test class:

In order to let Spring know which configuration class to use, you must annotate your Spec classes with @ContextConfiguration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

For constructor injection, Kotest automatically registers a SpringAutowireConstructorExtension when the spring module is added to the build, assuming auto scan is enabled (see Project Config). If Auto scan is disabled, you will need to manually load the extension in your Project config.

This extension will intercept each call to create a Spec instance and will autowire the beans declared in the primary constructor.

The following example is a test class which requires a service called UserService in its primary constructor. This service class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various test callbacks such as beforeTestMethod that are based around the idea that tests are methods. This assumption is fine for legacy test frameworks like JUnit but not applicable to modern test frameworks like Kotest where tests are functions.

Therefore, when using a spec style that is nested, you can customize when the test method callbacks are fired. By default, this is on the leaf node. You can set these to fire on the root nodes by passing a SpringTestLifecycleMode argument to the extension:

When using a final class, you may receive a warning from Kotest:

Using SpringListener on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(SpringExtension)}
```

Example 2 (kotlin):
```kotlin
class MyTestSpec : FunSpec() {   override fun extensions() = listOf(SpringExtension)}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {         println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## Koin | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/koin.html

**Contents:**
- Koin
- Koin​

The Koin DI Framework can be used with Kotest through the KoinExtension extension.

To use the extension in your project, add the dependency to your project:

With the dependency added, we can easily use Koin in our tests!

By default, the extension will start/stop the Koin context between leaf tests. If you are using a nested spec style (like DescribeSpec) and instead want the Koin context to persist over all leafs of a root tests (for example to share mocked declarations between tests), you can specify the lifecycle mode as KoinLifecycleMode.Root in the KoinExtension constructor.

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest.extensions:kotest-extensions-koin:${version}
```

Example 2 (kotlin):
```kotlin
class KotestAndKoin : FunSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(myKoinModule))    val userService by inject<UserService>()    init {        test("use userService") {            userService.getUser().username shouldBe "LeoColman"        }    }}
```

Example 3 (kotlin):
```kotlin
class KotestAndKoin : DescribeSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(module = myKoinModule, mode = KoinLifecycleMode.Root))    val userService by inject<UserService>()    init {        describe("use userService") {            it("inside a leaf test") {                userService.getUser().username shouldBe "LeoColman"            }            it("this shares the same context") {                userService.getUser().username shouldBe "LeoColman"            }        }    }}
```

---

## Koin | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/koin.html

**Contents:**
- Koin
- Koin​

The Koin DI Framework can be used with Kotest through the KoinExtension extension.

To use the extension in your project, add the dependency to your project:

With the dependency added, we can easily use Koin in our tests!

By default, the extension will start/stop the Koin context between leaf tests. If you are using a nested spec style (like DescribeSpec) and instead want the Koin context to persist over all leafs of a root tests (for example to share mocked declarations between tests), you can specify the lifecycle mode as KoinLifecycleMode.Root in the KoinExtension constructor.

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest.extensions:kotest-extensions-koin:${version}
```

Example 2 (kotlin):
```kotlin
class KotestAndKoin : FunSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(myKoinModule))    val userService by inject<UserService>()    init {        test("use userService") {            userService.getUser().username shouldBe "LeoColman"        }    }}
```

Example 3 (kotlin):
```kotlin
class KotestAndKoin : DescribeSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(module = myKoinModule, mode = KoinLifecycleMode.Root))    val userService by inject<UserService>()    init {        describe("use userService") {            it("inside a leaf test") {                userService.getUser().username shouldBe "LeoColman"            }            it("this shares the same context") {                userService.getUser().username shouldBe "LeoColman"            }        }    }}
```

---

## Embedded Kafka Extension | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/embedded-kafka.html

**Contents:**
- Embedded Kafka Extension
  - Getting started:​
  - Consumer / Producer​
  - Custom Ports​

Kotest offers an extension that spins up an embedded Kafka instance. This can help in situations where using the kafka docker images are an issue.

To use this extension add the io.kotest.extensions:kotest-extensions-embedded-kafka module to your test compile path.

Register the embeddedKafkaListener listener in your test class:

And the broker will be started once the spec is created and stopped once the spec completes.

Note: The underlying embedded kafka library uses a global object for state. Do not start multiple kafka instances at the same time.

To create a consumer and producer we can use convenience methods on the listener:

The stringStringProducer and stringStringConsumer methods return a producer / consumer that accept strings for the keys and values. Similar methods exist for byte pairs.

Alternatively, you can access the host/port the Kafka instance was deployed on and create the clients yourself:

You can create a new instance of the listener specifying a port and then use that instance rather than the default instance.

You can also do specify the zookeeper port using an alternative overload.

**Examples:**

Example 1 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({  listener(embeddedKafkaListener)})
```

Example 2 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec() {  init {    listener(embeddedKafkaListener)  }}
```

Example 3 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)   test("send / receive") {     val producer = embeddedKafkaListener.stringStringProducer()     producer.send(ProducerRecord("foo", "a"))     producer.close()     val consumer = embeddedKafkaListener.stringStringConsumer("foo")     eventually(10.seconds) {       consumer.poll(1000).first().value() shouldBe "a"     }     consumer.close()   }})
```

Example 4 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)      val props = Properties().apply {      put(CommonClientConfigs.BOOTSTRAP_SERVERS_CONFIG, "${embeddedKafkaListener.host}:${embeddedKafkaListener.port}")   }      val producer = KafkaProducer<String, String>(props)   }
```

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Property Extension​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

If you need to test code that uses java.lang.System, Kotest provides extensions that can alter the system and restore it after each test. This extension is only available on the JVM.

To use this extension, add the dependency to your project:

This extension does not support concurrent test execution. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is undefined.

You can override the System Properties (System.getProperties()) by either using a listener at the spec level, or by using the withSystemProperty function to wrap any arbitrary code.

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest:kotest-extensions:${version}
```

Example 2 (kotlin):
```kotlin
withSystemProperty("foo", "bar") {  System.getProperty("foo") shouldBe "bar"}
```

Example 3 (kotlin):
```kotlin
class MyTest : FreeSpec() {  override val extensions = listOf(SystemPropertyTestListener("foo", "bar"))  init {    "MyTest" {      System.getProperty("foo") shouldBe "bar"    }  }}
```

Example 4 (kotlin):
```kotlin
// In Project or in Specoverride val extensions = listOf(NoSystemOutListener, NoSystemErrListener)
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---

## Embedded Kafka Extension | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/embedded-kafka.html

**Contents:**
- Embedded Kafka Extension
  - Getting started:​
  - Consumer / Producer​
  - Custom Ports​

Kotest offers an extension that spins up an embedded Kafka instance. This can help in situations where using the kafka docker images are an issue.

To use this extension add the io.kotest.extensions:kotest-extensions-embedded-kafka module to your test compile path.

Register the embeddedKafkaListener listener in your test class:

And the broker will be started once the spec is created and stopped once the spec completes.

Note: The underlying embedded kafka library uses a global object for state. Do not start multiple kafka instances at the same time.

To create a consumer and producer we can use convenience methods on the listener:

The stringStringProducer and stringStringConsumer methods return a producer / consumer that accept strings for the keys and values. Similar methods exist for byte pairs.

Alternatively, you can access the host/port the Kafka instance was deployed on and create the clients yourself:

You can create a new instance of the listener specifying a port and then use that instance rather than the default instance.

You can also do specify the zookeeper port using an alternative overload.

**Examples:**

Example 1 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({  listener(embeddedKafkaListener)})
```

Example 2 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec() {  init {    listener(embeddedKafkaListener)  }}
```

Example 3 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)   test("send / receive") {     val producer = embeddedKafkaListener.stringStringProducer()     producer.send(ProducerRecord("foo", "a"))     producer.close()     val consumer = embeddedKafkaListener.stringStringConsumer("foo")     eventually(10.seconds) {       consumer.poll(1000).first().value() shouldBe "a"     }     consumer.close()   }})
```

Example 4 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)      val props = Properties().apply {      put(CommonClientConfigs.BOOTSTRAP_SERVERS_CONFIG, "${embeddedKafkaListener.host}:${embeddedKafkaListener.port}")   }      val producer = KafkaProducer<String, String>(props)   }
```

---

## Robolectric | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/robolectric.html

**Contents:**
- Robolectric
- Robolectric​

Robolectric can be used with Kotest through the RobolectricExtension which can be found in a separate repository,kotest-extensions-robolectric

To add this module to project you need specify following in your build.gradle:

This dependency brings in RobolectricExtension, which is autoregistered to your projects.

Now all you need to do is annotate Robolectric specs with @RobolectricTest and you're set!

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-robolectric:${version}")
```

Example 2 (kotlin):
```kotlin
@RobolectricTestclass MyTest : ShouldSpec({    should("Access Robolectric normally!") {    }})
```

---

## Test Clock | Kotest

**URL:** https://kotest.io/docs/next/extensions/test_clock.html

**Contents:**
- Test Clock

The JVM provides the java.time.Clock interface which is used to generate Instants. When we have code that relies on time, we can use a Clock to generate the values, rather than using things like Instant.now() or System.currentTimeMillis().

Then in tests we can provide a fixed or controllable clock which avoids issues where the time changes on each test run. In your real code, you provide an instance of Clock.systemUTC() or whatever.

The following module is needed: io.kotest:kotest-extensions in your build. Search maven central for latest version here.

Since Kotest 6.0, all extensions are published under the io.kotest group, with version cadence tied to main Kotest releases.

In order to use it, we create an instance of the TestClock passing in an instant and a zone offset.

We can control the clock via plus and minus which accept durations, eg

Note that the clock is mutable, and the internal state is changed when you use plus or minus.

**Examples:**

Example 1 (unknown):
```unknown
val timestamp = Instant.ofEpochMilli(1234)val clock = TestClock(timestamp, ZoneOffset.UTC)
```

Example 2 (unknown):
```unknown
clock.plus(6.minutes)
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Setting Build Dir​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional junit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using allure with kotest. To start, add the below dependency to your Gradle build file.

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your buld, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will write out test data in the allure json format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the allure binary, or we can use the allure gradle plugin. To use the gradle plugin, first add the plugin to your build's plugins block.

Next, add an allure configuration section to set the version and disable autoconfigure (because allure can only auto configure junit and kotest takes care of this for you anyway).

Finally, execute the gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the gradle plugin then you will need to inform Allure where the build dir is by setting the allure.results.directory system property on your tests configuration. If you are using the gradle plugin, then this can be skipped as the gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.8.1"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Environment​
  - System Property Extension​
  - System Security Manager​
  - System Exit Extensions​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

If you need to test code that uses java.lang.System, Kotest provides extensions that can alter the system and restore it after each test. This extension is only available on the JVM.

To use this extension, add the dependency to your project:

This extension does not support concurrent test execution. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is undefined.

With System Environment Extension you can simulate how the System Environment is behaving. That is, what you're obtaining from System.getenv().

Kotest provides some extension functions that provides a System Environment in a specific scope:

To use withEnvironment with JDK17 you need to add --add-opens=java.base/java.util=ALL-UNNAMED to the arguments for the JVM that runs the tests.

If you run tests with gradle, you can add the following to your build.gradle.kts:

You can also use multiple values in this extension, through a map or list of pairs.

These functions will add the keys and values if they're not currently present in the environment, and will override them if they are. Any keys untouched by the function will remain in the environment, and won't be messed with.

Instead of extension functions, you can also use the provided Listeners to apply these functionalities in a bigger scope. There's an alternative for the Spec/Per test level, and an alternative for the Project Level.

In the same fashion as the Environment Extensions, you can override the System Properties (System.getProperties()):

And with similar Listeners:

Similarly, with System Security Manager you can override the System Security Manager (System.getSecurityManager())

Sometimes you want to test that your code calls System.exit. For that you can use the System Exit Listeners. The Listener will throw an exception when the System.exit is called, allowing you to catch it and verify:

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

And with the listeners

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest:kotest-extensions-jvm:${version}
```

Example 2 (kotlin):
```kotlin
withEnvironment("FooKey", "BarValue") {    System.getenv("FooKey") shouldBe "BarValue" // System environment overridden!}
```

Example 3 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {    jvmArgs("--add-opens=java.base/java.util=ALL-UNNAMED")}
```

Example 4 (kotlin):
```kotlin
withEnvironment(mapOf("FooKey" to "BarValue", "BarKey" to "FooValue")) {    // Use FooKey and BarKey}
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/next/extensions/mockserver.html

**Contents:**
- MockServer
- Dynamic Ports​

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest:kotest-extensions-mockserver module to be added to your build.

Since Kotest 6.0, all extensions are published under the io.kotest group, with version cadence tied to main Kotest releases.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we install an instance of MockServerExtension in the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

When using the MockServerExtension, you can specify one or more ports if you wish to hardcore them. Otherwise, you can not specify them at all, and Kotest will automatically allocate a free port for the server to run on. Then, you can use the returned server instance from the install function to retrieve the allocated port.

Here is an example of using dynamic ports:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      install(MockServerExtension(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

Example 2 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {    val server = install(MockServerExtension())    beforeTest {      MockServerClient("localhost", server.port).`when`(        HttpRequest.request()          .withMethod("GET")          .withPath("/v")      ).respond(        HttpResponse.response()          .withStatusCode(200)      )    }    test("test /health returns 200") {      val client = HttpClient(CIO)      val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:${healthcheck.port}/health")      resp.shouldHaveStatus(HttpStatusCode.OK)    }  }}
```

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (json):
```json
io.kotest.extensions:kotest-extensions-wiremock:{version}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## Test Clock | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/test_clock.html

**Contents:**
- Test Clock

The JVM provides the java.time.Clock interface which is used to generate Instants. When we have code that relies on time, we can use a Clock to generate the values, rather than using things like Instant.now() or System.currentTimeMillis().

Then in tests we can provide a fixed or controllable clock which avoids issues where the time changes on each test run. In your real code, you provide an instance of Clock.systemUTC() or whatever.

The following module is needed: io.kotest.extensions:kotest-extensions-clock in your build. Search maven central for latest version here.

In order to use it, we create an instance of the TestClock passing in an instant and a zone offset.

We can control the clock via plus and minus which accept durations, eg

Note that the clock is mutable, and the internal state is changed when you use plus or minus.

**Examples:**

Example 1 (unknown):
```unknown
val timestamp = Instant.ofEpochMilli(1234)val clock = TestClock(timestamp, ZoneOffset.UTC)
```

Example 2 (unknown):
```unknown
clock.plus(6.minutes)
```

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (json):
```json
io.kotest.extensions:kotest-extensions-wiremock:{version}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
  - Dependencies​
  - Databases​
    - Initializing the Database Container​
  - General Containers​
  - Kafka Containers​
  - Lifecycle​
  - Startables​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 5.0+. For earlier versions see docs here

The Testcontainers project provides lightweight, ephemeral instances of common databases, elasticsearch, kafka, Selenium web browsers, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides several extensions - specialized extensions for databases and kafka and general containers support for any supported docker image.

To begin, add the following dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

For Maven, you will need these dependencies:

For JDBC compatible databases, Kotest provides the JdbcTestContainerExtension. This provides a pooled javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension wrapper, providing an optional configuration lambda.

If you don't wish to configure the pool, then you can omit the trailing lambda.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

This extension also supports the LifecycleMode flag to control when the container is started and stopped. See Lifecycle

There are two ways to initialize the database container: via a single init script added to the TestContainer config, or via a list of scripts added to the JdbcTestContainerExtension config lambda.

If adding a single script, via the TestContainer config, simply add the script to the TestContainer's withInitScript config option, like so:

If you have multiple init scripts or sets of changesets, you can add them as a list to the dbInitScripts extension config lambda, like so:

The list can contain absolute or relative paths, for files and folders on the filesystem or on the classpath.

The extension will process the list provided in order. If the list item is a folder, it will process all .sql scripts in the folder, sorted lexicographically. These scripts run every time the container is started, so it supports the LifecycleMode flag.

Similar to the JdbcTestContainerExtension, this module also provides a TestContainerExtension extension which can wrap any container, not just databases.

We can create the extension using either a docker image name, or a strongly typed container.

For example, using a docker image directly:

And then using a strongly typed container:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives us access to specific settings - such as the password option in the elasticsearch example above.

However, when a strongly typed container is not available, the former method allows us to spool up any docker image as a general container.

This extension also supports the LifecycleMode flag to control when the container is started and stopped. See Lifecycle

For Kafka, this module provides convenient extension methods to create a consumer, producer or admin client from the container.

Inside the configuration lambda, we can specify options for the Kafka container, such as embedded/external zookeeper, or kafka broker properties through env vars. For example, to enable dynamic topic creation:

Kafka only publishes a linux/amd64 version of the container. If you're on an Apple Silicon/ARM architecture computer, you'll need to explicitly specify the platform with the following added to the configuration lambda outlined above:

Once we have the container installed, we can create a client using the following methods:

Each of these accepts an optional configuration lambda to enable setting values on the properties object that is used to create the clients.

For example, in this test, we produce and consume a message from the same topic, and we use the configuration lambda to set max poll to 1.

When creating a consumer, the consumer group is set to a random uuid. To change this, provide a configuration lambda and specify your own group consumer group id.

By default, the lifecycle of a container is per spec - so it will be started at the install command, and shutdown as the spec is completed. This can be changed to start/stop per test, per leaf test, or per root test.

To do this, pass in a LifecycleMode parameter to the TestContainerExtension or JdbcTestContainerExtension.

If you change the lifecycle mode from Spec then the container will not be started in the constructor, and so any operations that act on the container must be placed inside the test scopes.

This module also provides extension methodsscope which let you convert any Startable such as a DockerContainer into a kotest TestListener, which you can register with Kotest and then Kotest will manage the lifecycle of that container for you.

In above example, the perTest() extension method converts the container into a TestListener, which starts the redis container before each test and stops it after test. Similarly if you want to reuse the container for all tests in a single spec class you can use perSpec() extension method, which converts the container into a TestListener which starts the container before running any test in the spec, and stops it after all tests, thus a single container is used by all tests in spec class.

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-testcontainers</artifactId>    <version>${kotest.version}</version>    <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val mysql = MySQLContainer<Nothing>("mysql:8.0.26").apply {  startupAttempts = 1  withUrlParam("connectionTimeZone", "Z")  withUrlParam("zeroDateTimeBehavior", "convertToNull")}
```

Example 4 (kotlin):
```kotlin
val ds = install(JdbcTestContainerExtension(mysql)) {  poolName = "myconnectionpool"  maximumPoolSize = 8  idleTimeout = 10000}
```

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
  - Dependencies​
  - Databases​
    - Initializing the Database Container​
  - General Containers​
  - Kafka Containers​
  - Lifecycle​
  - Startables​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 5.0+. For earlier versions see docs here

The Testcontainers project provides lightweight, ephemeral instances of common databases, elasticsearch, kafka, Selenium web browsers, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides several extensions - specialized extensions for databases and kafka and general containers support for any supported docker image.

To begin, add the following dependency to your Gradle build file.

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

For Maven, you will need these dependencies:

For JDBC compatible databases, Kotest provides the JdbcTestContainerExtension. This provides a pooled javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension wrapper, providing an optional configuration lambda.

If you don't wish to configure the pool, then you can omit the trailing lambda.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

This extension also supports the ContainerLifecycleMode flag to control when the container is started and stopped. See Lifecycle

There are two ways to initialize the database container: via a single init script added to the TestContainer config, or via a list of scripts added to the JdbcTestContainerExtension config lambda.

If adding a single script, via the TestContainer config, simply add the script to the TestContainer's withInitScript config option, like so:

If you have multiple init scripts or sets of changesets, you can add them as a list to the dbInitScripts extension config lambda, like so:

The list can contain absolute or relative paths, for files and folders on the filesystem or on the classpath.

The extension will process the list provided in order. If the list item is a folder, it will process all .sql scripts in the folder, sorted lexicographically. These scripts run every time the container is started, so it supports the ContainerLifecycleMode flag.

Similar to the JdbcDatabaseContainerExtension, this module also provides a ContainerExtension extension which can wrap any container, not just databases.

We can create the extension using either a docker image name, or a strongly typed container.

For example, using a docker image directly:

And then using a strongly typed container:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives us access to specific settings - such as the password option in the elasticsearch example above.

However, when a strongly typed container is not available, the former method allows us to spool up any docker image as a general container.

This extension also supports the ContainerLifecycleMode flag to control when the container is started and stopped. See Lifecycle

For Kafka, this module provides convenient extension methods to create a consumer, producer or admin client from the container.

Inside the configuration lambda, we can specify options for the Kafka container, such as embedded/external zookeeper, or kafka broker properties through env vars. For example, to enable dynamic topic creation:

Kafka only publishes a linux/amd64 version of the container. If you're on an Apple Silicon/ARM architecture computer, you'll need to explicitly specify the platform with the following added to the configuration lambda outlined above:

Once we have the container installed, we can create a client using the following methods:

Each of these accepts an optional configuration lambda to enable setting values on the properties object that is used to create the clients.

For example, in this test, we produce and consume a message from the same topic, and we use the configuration lambda to set max poll to 1.

When creating a consumer, the consumer group is set to a random uuid. To change this, provide a configuration lambda and specify your own group consumer group id.

By default, the lifecycle of a container is per spec - so it will be started at the install command, and shutdown as the spec is completed. This can be changed to start/stop per test, per leaf test, or per root test.

To do this, pass in a ContainerLifecycleMode parameter to the ContainerExtension or JdbcDatabaseContainerExtension.

This module also provides extension methodsscope which let you convert any Startable such as a DockerContainer into a kotest TestListener, which you can register with Kotest and then Kotest will manage the lifecycle of that container for you.

In above example, the perTest() extension method converts the container into a TestListener, which starts the redis container before each test and stops it after test. Similarly if you want to reuse the container for all tests in a single spec class you can use perSpec() extension method, which converts the container into a TestListener which starts the container before running any test in the spec, and stops it after all tests, thus a single container is used by all tests in spec class.

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>    <groupId>io.kotest</groupId>    <artifactId>kotest-extensions-testcontainers</artifactId>    <version>${kotest.version}</version>    <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val mysql = MySQLContainer<Nothing>("mysql:8.0.26").apply {  startupAttempts = 1  withUrlParam("connectionTimeZone", "Z")  withUrlParam("zeroDateTimeBehavior", "convertToNull")}
```

Example 4 (kotlin):
```kotlin
val ds = install(JdbcDatabaseContainerExtension(mysql)) {  poolName = "myconnectionpool"  maximumPoolSize = 8  idleTimeout = 10000}
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Manually setting result directory​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional JUnit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to Allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using Allure with Kotest. To start, add the below dependency to your Gradle build file.

Note: The group ID is different (io.kotest.extensions) from the main Kotest dependencies (io.kotest).

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your build, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will produce test reports in the Allure JSON format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the Allure binary, or we can use the Allure Gradle plugin. To use the Gradle plugin, first add the plugin to your build's plugins block.

Next, add an Allure configuration section to set the version and disable autoconfigure (because Allure can only autoconfigure JUnit, and Kotest takes care of this for you anyway).

Finally, execute the Gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the Gradle plugin then you will need to inform Allure where the results directory is by setting the allure.results.directory system property on your test task configuration. If you are using the Gradle plugin, then this can be skipped as the Gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.11.2"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/pitest.html

**Contents:**
- Pitest
- Gradle configuration​
- Maven configuration​

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest:kotest-extensions-pitest module to your dependencies as well:

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

First of all, you need to configure the Maven Pitest plugin:

Then add the dependency on Pitest Kotest extension:

This should be enough to be able to run Pitest and get the reports as described in the Maven Pitest plugin.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

Example 3 (xml):
```xml
<plugin>    <groupId>org.pitest</groupId>    <artifactId>pitest-maven</artifactId>    <version>${pitest-maven.version}</version>    <configuration>        <targetClasses>...</targetClasses>        <coverageThreshold>...</coverageThreshold>        ... other configurations as needed    </configuration></plugin>
```

Example 4 (xml):
```xml
<dependencies>  ... the other Kotest dependencies like kotest-runner-junit5  <dependency>    <groupId>io.kotest</groupId>    <artifactId>kotest-extensions-pitest</artifactId>    <version>${kotest-extensions-pitest.version}</version>    <scope>test</scope>  </dependency></dependencies>
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has three parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override fun extensions(): List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false,      useTestPathAsName = true    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Environment​
  - System Property Extension​
  - System Security Manager​
  - System Exit Extensions​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

Sometimes your code might use some functionalities straight from the JVM, which are very hard to simulate. With Kotest System Extensions, these difficulties are made easy to mock and simulate, and your code can be tested correctly. After changing the system and using the extensions, the previous state will be restored.

This code is sensitive to concurrency. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is unknown.

With System Environment Extension you can simulate how the System Environment is behaving. That is, what you're obtaining from System.getenv().

Kotest provides some extension functions that provides a System Environment in a specific scope:

To use withEnvironment with JDK17 you need to add --add-opens=java.base/java.util=ALL-UNNAMED to the arguments for the JVM that runs the tests.

If you run tests with gradle, you can add the following to your build.gradle.kts:

You can also use multiple values in this extension, through a map or list of pairs.

These functions will add the keys and values if they're not currently present in the environment, and will override them if they are. Any keys untouched by the function will remain in the environment, and won't be messed with.

Instead of extensions functions, you can also use the provided Listeners to apply these functionalities in a bigger scope. There's an alternative for the Spec/Per test level, and an alternative for the Project Level.

In the same fashion as the Environment Extensions, you can override the System Properties (System.getProperties()):

And with similar Listeners:

Similarly, with System Security Manager you can override the System Security Manager (System.getSecurityManager())

Sometimes you want to test that your code calls System.exit. For that you can use the System Exit Listeners. The Listener will throw an exception when the System.exit is called, allowing you to catch it and verify:

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

And with the listeners

**Examples:**

Example 1 (kotlin):
```kotlin
withEnvironment("FooKey", "BarValue") {    System.getenv("FooKey") shouldBe "BarValue" // System environment overridden!}
```

Example 2 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {  jvmArgs("--add-opens=java.base/java.util=ALL-UNNAMED")}
```

Example 3 (kotlin):
```kotlin
withEnvironment(mapOf("FooKey" to "BarValue", "BarKey" to "FooValue")) {  // Use FooKey and BarKey}
```

Example 4 (kotlin):
```kotlin
class MyTest : FreeSpec() {      override fun listeners() = listOf(SystemEnvironmentTestListener("foo", "bar"))    init {      "MyTest" {        System.getenv("foo") shouldBe "bar"      }    }}
```

---

## Embedded Kafka Extension | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/embedded-kafka.html

**Contents:**
- Embedded Kafka Extension
  - Getting started:​
  - Consumer / Producer​
  - Custom Ports​

Kotest offers an extension that spins up an embedded Kafka instance. This can help in situations where using the kafka docker images are an issue.

To use this extension add the io.kotest.extensions:kotest-extensions-embedded-kafka module to your test compile path.

Register the embeddedKafkaListener listener in your test class:

And the broker will be started once the spec is created and stopped once the spec completes.

Note: The underlying embedded kafka library uses a global object for state. Do not start multiple kafka instances at the same time.

To create a consumer and producer we can use convenience methods on the listener:

The stringStringProducer and stringStringConsumer methods return a producer / consumer that accept strings for the keys and values. Similar methods exist for byte pairs.

Alternatively, you can access the host/port the Kafka instance was deployed on and create the clients yourself:

You can create a new instance of the listener specifying a port and then use that instance rather than the default instance.

You can also do specify the zookeeper port using an alternative overload.

**Examples:**

Example 1 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({  listener(embeddedKafkaListener)})
```

Example 2 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec() {  init {    listener(embeddedKafkaListener)  }}
```

Example 3 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)   test("send / receive") {     val producer = embeddedKafkaListener.stringStringProducer()     producer.send(ProducerRecord("foo", "a"))     producer.close()     val consumer = embeddedKafkaListener.stringStringConsumer("foo")     eventually(10.seconds) {       consumer.poll(1000).first().value() shouldBe "a"     }     consumer.close()   }})
```

Example 4 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)      val props = Properties().apply {      put(CommonClientConfigs.BOOTSTRAP_SERVERS_CONFIG, "${embeddedKafkaListener.host}:${embeddedKafkaListener.port}")   }      val producer = KafkaProducer<String, String>(props)   }
```

---

## Koin | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/koin.html

**Contents:**
- Koin
- Koin​

The Koin DI Framework can be used with Kotest through the KoinExtension extension.

To use the extension in your project, add the dependency to your project:

With the dependency added, we can easily use Koin in our tests!

By default, the extension will start/stop the Koin context between leaf tests. If you are using a nested spec style (like DescribeSpec) and instead want the Koin context to persist over all leafs of a root tests (for example to share mocked declarations between tests), you can specify the lifecycle mode as KoinLifecycleMode.Root in the KoinExtension constructor.

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest.extensions:kotest-extensions-koin:${version}
```

Example 2 (kotlin):
```kotlin
class KotestAndKoin : FunSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(myKoinModule))    val userService by inject<UserService>()    init {        test("use userService") {            userService.getUser().username shouldBe "LeoColman"        }    }}
```

Example 3 (kotlin):
```kotlin
class KotestAndKoin : DescribeSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(module = myKoinModule, mode = KoinLifecycleMode.Root))    val userService by inject<UserService>()    init {        describe("use userService") {            it("inside a leaf test") {                userService.getUser().username shouldBe "LeoColman"            }            it("this shares the same context") {                userService.getUser().username shouldBe "LeoColman"            }        }    }}
```

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/next/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-extensions-wiremock:${kotestVersion}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  extension(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  extension(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has two parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override fun extensions(): List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false,      useTestPathAsName = true    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Since Kotest 5.6.0, Current instant listeners are located in the artifact io.kotest:kotest-extensions-now:${kotest-version}.

Add it as a dependency to use any of the functionality mentioned below.

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/next/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Property Extension​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

If you need to test code that uses java.lang.System, Kotest provides extensions that can alter the system and restore it after each test. This extension is only available on the JVM.

To use this extension, add the dependency to your project:

This extension does not support concurrent test execution. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is undefined.

You can override the System Properties (System.getProperties()) by either using a listener at the spec level, or by using the withSystemProperty function to wrap any arbitrary code.

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest:kotest-extensions:${version}
```

Example 2 (kotlin):
```kotlin
withSystemProperty("foo", "bar") {  System.getProperty("foo") shouldBe "bar"}
```

Example 3 (kotlin):
```kotlin
class MyTest : FreeSpec() {  override val extensions = listOf(SystemPropertyTestListener("foo", "bar"))  init {    "MyTest" {      System.getProperty("foo") shouldBe "bar"    }  }}
```

Example 4 (kotlin):
```kotlin
// In Project or in Specoverride val extensions = listOf(NoSystemOutListener, NoSystemErrListener)
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/extensions.html

**Contents:**
- Extensions
  - Kotest Team Extensions​
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties.

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (json):
```json
io.kotest.extensions:kotest-extensions-wiremock:{version}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## BlockHound | Kotest

**URL:** https://kotest.io/docs/extensions/blockhound.html

**Contents:**
- BlockHound
  - Getting Started​
  - Detection​
  - Customization​

The Kotest BlockHound extension activates BlockHound support for coroutines. It helps to detect blocking code on non-blocking coroutine threads, e.g. when accidentally calling a blocking I/O library function on a UI thread.

To use this extension add the io.kotest:kotest-extensions-blockhound module to your test compile path.

Register the BlockHound extension in your test class:

The BlockHound extension can also be registered per test case or at the project level.

If BlockHound is enabled project-wide or spec-wide, you can disable it for an individual test:

You can also change BlockHoundMode for a section of code:

Blocking calls will be detected in coroutine threads which are expected not to block. Such threads are created by the default dispatcher as this example demonstrates:

The BlockHound extension will by default produce an exception like this whenever it detects a blocking call:

By invoking it as BlockHound(BlockHoundMode.PRINT), it will print detected calls and continue the test without interruption.

Whenever a blocking call is detected, you can

To customize BlockHound, familiarize yourself with the BlockHound documentation.

Exceptions for blocking calls considered harmless can be added via a separate BlockHoundIntegration class like this:

In order to allow BlockHound to auto-detect and load the integration, add its fully qualified class name to a service provider configuration file resources/META-INF/services/reactor.blockhound.integration.BlockHoundIntegration.

**Examples:**

Example 1 (kotlin):
```kotlin
class BlockHoundSpecTest : FunSpec({   extension(BlockHound())   test("detects for spec") {      blockInNonBlockingContext()   }})
```

Example 2 (kotlin):
```kotlin
test("allow blocking").config(extensions = listOf(BlockHound(BlockHoundMode.DISABLED))) {      blockInNonBlockingContext()   }
```

Example 3 (kotlin):
```kotlin
test("allow blocking section") {      // ...      withBlockHoundMode(BlockHoundMode.DISABLED) {        blockInNonBlockingContext()      }      // ...   }
```

Example 4 (kotlin):
```kotlin
private suspend fun blockInNonBlockingContext() {   withContext(Dispatchers.Default) {      @Suppress("BlockingMethodInNonBlockingContext")      Thread.sleep(2)   }}
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Since Kotest 5.6.0, Current instant listeners are located in the artifact io.kotest:kotest-extensions-now:${kotest-version}.

Add it as a dependency to use any of the functionality mentioned below.

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/pitest.html

**Contents:**
- Pitest
- Gradle configuration​
- Maven configuration​

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest.extensions:kotest-extensions-pitest module to your dependencies as well:

Note: Since pitest is an extension, we use a different maven group name (io.kotest.extensions) from the core modules.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

First of all, you need to configure the Maven Pitest plugin:

Then add the dependency on Pitest Kotest extension:

This should be enough to be able to run Pitest and get the reports as described in the Maven Pitest plugin.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

Example 3 (xml):
```xml
<plugin>    <groupId>org.pitest</groupId>    <artifactId>pitest-maven</artifactId>    <version>${pitest-maven.version}</version>    <configuration>        <targetClasses>...</targetClasses>        <coverageThreshold>...</coverageThreshold>        ... other configurations as needed            </configuration></plugin>
```

Example 4 (xml):
```xml
<dependencies>  ... the other Kotest dependencies like kotest-runner-junit5-jvm   <dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-pitest</artifactId>    <version>${kotest-extensions-pitest.version}</version>    <scope>test</scope>  </dependency></dependencies>
```

---

## BlockHound | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/blockhound.html

**Contents:**
- BlockHound
  - Getting Started​
  - Detection​
  - Customization​

The Kotest BlockHound extension activates BlockHound support for coroutines. It helps to detect blocking code on non-blocking coroutine threads, e.g. when accidentally calling a blocking I/O library function on a UI thread.

To use this extension add the io.kotest.extensions:kotest-extensions-blockhound module to your test compile path.

Register the BlockHound extension in your test class:

The BlockHound extension can also be registered per test case or at the project level.

If BlockHound is enabled project-wide or spec-wide, you can disable it for an individual test:

You can also change BlockHoundMode for a section of code:

Blocking calls will be detected in coroutine threads which are expected not to block. Such threads are created by the default dispatcher as this example demonstrates:

The BlockHound extension will by default produce an exception like this whenever it detects a blocking call:

By invoking it as BlockHound(BlockHoundMode.PRINT), it will print detected calls and continue the test without interruption.

Whenever a blocking call is detected, you can

To customize BlockHound, familiarize yourself with the BlockHound documentation.

Exceptions for blocking calls considered harmless can be added via a separate BlockHoundIntegration class like this:

In order to allow BlockHound to auto-detect and load the integration, add its fully qualified class name to a service provider configuration file resources/META-INF/services/reactor.blockhound.integration.BlockHoundIntegration.

**Examples:**

Example 1 (kotlin):
```kotlin
class BlockHoundSpecTest : FunSpec({   extension(BlockHound())   test("detects for spec") {      blockInNonBlockingContext()   }})
```

Example 2 (kotlin):
```kotlin
test("allow blocking").config(extensions = listOf(BlockHound(BlockHoundMode.DISABLED))) {      blockInNonBlockingContext()   }
```

Example 3 (kotlin):
```kotlin
test("allow blocking section") {      // ...      withBlockHoundMode(BlockHoundMode.DISABLED) {        blockInNonBlockingContext()      }      // ...   }
```

Example 4 (kotlin):
```kotlin
private suspend fun blockInNonBlockingContext() {   withContext(Dispatchers.Default) {      @Suppress("BlockingMethodInNonBlockingContext")      Thread.sleep(2)   }}
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has three parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override fun extensions(): List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false, // don't write out status for all tests      useTestPathAsName = true, // use the full test path (ie, includes parent test names)      outputDir = "../target/junit-xml" // include to set output dir for maven    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/extensions.html

**Contents:**
- Extensions
  - Kotest Team Extensions​
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties.

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
  - Dependencies​
  - Databases​
    - Initializing the Database Container​
  - General Containers​
  - Kafka Containers​
  - Lifecycle​
  - Startables​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 5.0+. For earlier versions see docs here

The Testcontainers project provides lightweight, ephemeral instances of common databases, elasticsearch, kafka, Selenium web browsers, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides several extensions - specialized extensions for databases and kafka and general containers support for any supported docker image.

To begin, add the following dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

For Maven, you will need these dependencies:

For JDBC compatible databases, Kotest provides the JdbcTestContainerExtension. This provides a pooled javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension wrapper, providing an optional configuration lambda.

If you don't wish to configure the pool, then you can omit the trailing lambda.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

This extension also supports the ContainerLifecycleMode flag to control when the container is started and stopped. See Lifecycle

There are two ways to initialize the database container: via a single init script added to the TestContainer config, or via a list of scripts added to the JdbcTestContainerExtension config lambda.

If adding a single script, via the TestContainer config, simply add the script to the TestContainer's withInitScript config option, like so:

If you have multiple init scripts or sets of changesets, you can add them as a list to the dbInitScripts extension config lambda, like so:

The list can contain absolute or relative paths, for files and folders on the filesystem or on the classpath.

The extension will process the list provided in order. If the list item is a folder, it will process all .sql scripts in the folder, sorted lexicographically. These scripts run every time the container is started, so it supports the ContainerLifecycleMode flag.

Similar to the JdbcDatabaseContainerExtension, this module also provides a ContainerExtension extension which can wrap any container, not just databases.

We can create the extension using either a docker image name, or a strongly typed container.

For example, using a docker image directly:

And then using a strongly typed container:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives us access to specific settings - such as the password option in the elasticsearch example above.

However, when a strongly typed container is not available, the former method allows us to spool up any docker image as a general container.

This extension also supports the ContainerLifecycleMode flag to control when the container is started and stopped. See Lifecycle

For Kafka, this module provides convenient extension methods to create a consumer, producer or admin client from the container.

Inside the configuration lambda, we can specify options for the Kafka container, such as embedded/external zookeeper, or kafka broker properties through env vars. For example, to enable dynamic topic creation:

Kafka only publishes a linux/amd64 version of the container. If you're on an Apple Silicon/ARM architecture computer, you'll need to explicitly specify the platform with the following added to the configuration lambda outlined above:

Once we have the container installed, we can create a client using the following methods:

Each of these accepts an optional configuration lambda to enable setting values on the properties object that is used to create the clients.

For example, in this test, we produce and consume a message from the same topic, and we use the configuration lambda to set max poll to 1.

When creating a consumer, the consumer group is set to a random uuid. To change this, provide a configuration lambda and specify your own group consumer group id.

By default, the lifecycle of a container is per spec - so it will be started at the install command, and shutdown as the spec is completed. This can be changed to start/stop per test, per leaf test, or per root test.

To do this, pass in a ContainerLifecycleMode parameter to the ContainerExtension or JdbcDatabaseContainerExtension.

This module also provides extension methodsscope which let you convert any Startable such as a DockerContainer into a kotest TestListener, which you can register with Kotest and then Kotest will manage the lifecycle of that container for you.

In above example, the perTest() extension method converts the container into a TestListener, which starts the redis container before each test and stops it after test. Similarly if you want to reuse the container for all tests in a single spec class you can use perSpec() extension method, which converts the container into a TestListener which starts the container before running any test in the spec, and stops it after all tests, thus a single container is used by all tests in spec class.

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-testcontainers</artifactId>    <version>${kotest.version}</version>    <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val mysql = MySQLContainer<Nothing>("mysql:8.0.26").apply {  startupAttempts = 1  withUrlParam("connectionTimeZone", "Z")  withUrlParam("zeroDateTimeBehavior", "convertToNull")}
```

Example 4 (kotlin):
```kotlin
val ds = install(JdbcDatabaseContainerExtension(mysql)) {  poolName = "myconnectionpool"  maximumPoolSize = 8  idleTimeout = 10000}
```

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/next/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Additionally, prevent Gradle from generating its own html reports by adding html.required.set(false) to the test task.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override val extensions): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

Example 2 (css):
```css
tasks.test {  useJUnitPlatform()  reports {    html.required.set(false)    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/extensions.html

**Contents:**
- Extensions
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties. For extensions provided directly by the Kotest team, see the links on the left.

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/mockserver.html

**Contents:**
- MockServer

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest.extensions:kotest-extensions-mockserver module to be added to your build.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we attach an instance of MockServerListener to the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      listener(MockServerListener(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/extensions/pitest.html

**Contents:**
- Pitest
- Gradle configuration​
- Maven configuration​

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest:kotest-extensions-pitest module to your dependencies as well:

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

First of all, you need to configure the Maven Pitest plugin:

Then add the dependency on Pitest Kotest extension:

This should be enough to be able to run Pitest and get the reports as described in the Maven Pitest plugin.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

Example 3 (xml):
```xml
<plugin>    <groupId>org.pitest</groupId>    <artifactId>pitest-maven</artifactId>    <version>${pitest-maven.version}</version>    <configuration>        <targetClasses>...</targetClasses>        <coverageThreshold>...</coverageThreshold>        ... other configurations as needed    </configuration></plugin>
```

Example 4 (xml):
```xml
<dependencies>  ... the other Kotest dependencies like kotest-runner-junit5  <dependency>    <groupId>io.kotest</groupId>    <artifactId>kotest-extensions-pitest</artifactId>    <version>${kotest-extensions-pitest.version}</version>    <scope>test</scope>  </dependency></dependencies>
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/next/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Since Kotest 5.6.0, Current instant listeners are located in the artifact io.kotest:kotest-extensions-now:${kotest-version}.

Add it as a dependency to use any of the functionality mentioned below.

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## Test Clock | Kotest

**URL:** https://kotest.io/docs/extensions/test_clock.html

**Contents:**
- Test Clock

The JVM provides the java.time.Clock interface which is used to generate Instants. When we have code that relies on time, we can use a Clock to generate the values, rather than using things like Instant.now() or System.currentTimeMillis().

Then in tests we can provide a fixed or controllable clock which avoids issues where the time changes on each test run. In your real code, you provide an instance of Clock.systemUTC() or whatever.

The following module is needed: io.kotest:kotest-extensions in your build. Search maven central for latest version here.

Since Kotest 6.0, all extensions are published under the io.kotest group, with version cadence tied to main Kotest releases.

In order to use it, we create an instance of the TestClock passing in an instant and a zone offset.

We can control the clock via plus and minus which accept durations, eg

Note that the clock is mutable, and the internal state is changed when you use plus or minus.

**Examples:**

Example 1 (unknown):
```unknown
val timestamp = Instant.ofEpochMilli(1234)val clock = TestClock(timestamp, ZoneOffset.UTC)
```

Example 2 (unknown):
```unknown
clock.plus(6.minutes)
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Setting Build Dir​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional junit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using allure with kotest. To start, add the below dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your buld, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will write out test data in the allure json format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the allure binary, or we can use the allure gradle plugin. To use the gradle plugin, first add the plugin to your build's plugins block.

Next, add an allure configuration section to set the version and disable autoconfigure (because allure can only auto configure junit and kotest takes care of this for you anyway).

Finally, execute the gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the gradle plugin then you will need to inform Allure where the build dir is by setting the allure.results.directory system property on your tests configuration. If you are using the gradle plugin, then this can be skipped as the gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.8.1"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/next/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---

## Embedded Kafka Extension | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/embedded-kafka.html

**Contents:**
- Embedded Kafka Extension
  - Getting started:​
  - Consumer / Producer​
  - Custom Ports​

Kotest offers an extension that spins up an embedded Kafka instance. This can help in situations where using the kafka docker images are an issue.

To use this extension add the io.kotest.extensions:kotest-extensions-embedded-kafka module to your test compile path.

Register the embeddedKafkaListener listener in your test class:

And the broker will be started once the spec is created and stopped once the spec completes.

Note: The underlying embedded kafka library uses a global object for state. Do not start multiple kafka instances at the same time.

To create a consumer and producer we can use convenience methods on the listener:

The stringStringProducer and stringStringConsumer methods return a producer / consumer that accept strings for the keys and values. Similar methods exist for byte pairs.

Alternatively, you can access the host/port the Kafka instance was deployed on and create the clients yourself:

You can create a new instance of the listener specifying a port and then use that instance rather than the default instance.

You can also do specify the zookeeper port using an alternative overload.

**Examples:**

Example 1 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({  listener(embeddedKafkaListener)})
```

Example 2 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec() {  init {    listener(embeddedKafkaListener)  }}
```

Example 3 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)   test("send / receive") {     val producer = embeddedKafkaListener.stringStringProducer()     producer.send(ProducerRecord("foo", "a"))     producer.close()     val consumer = embeddedKafkaListener.stringStringConsumer("foo")     eventually(10.seconds) {       consumer.poll(1000).first().value() shouldBe "a"     }     consumer.close()   }})
```

Example 4 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)      val props = Properties().apply {      put(CommonClientConfigs.BOOTSTRAP_SERVERS_CONFIG, "${embeddedKafkaListener.host}:${embeddedKafkaListener.port}")   }      val producer = KafkaProducer<String, String>(props)   }
```

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Environment​
  - System Property Extension​
  - System Security Manager​
  - System Exit Extensions​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

Sometimes your code might use some functionalities straight from the JVM, which are very hard to simulate. With Kotest System Extensions, these difficulties are made easy to mock and simulate, and your code can be tested correctly. After changing the system and using the extensions, the previous state will be restored.

This code is sensitive to concurrency. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is unknown.

With System Environment Extension you can simulate how the System Environment is behaving. That is, what you're obtaining from System.getenv().

Kotest provides some extension functions that provides a System Environment in a specific scope:

To use withEnvironment with JDK17 you need to add --add-opens=java.base/java.util=ALL-UNNAMED to the arguments for the JVM that runs the tests.

If you run tests with gradle, you can add the following to your build.gradle.kts:

You can also use multiple values in this extension, through a map or list of pairs.

These functions will add the keys and values if they're not currently present in the environment, and will override them if they are. Any keys untouched by the function will remain in the environment, and won't be messed with.

Instead of extensions functions, you can also use the provided Listeners to apply these functionalities in a bigger scope. There's an alternative for the Spec/Per test level, and an alternative for the Project Level.

In the same fashion as the Environment Extensions, you can override the System Properties (System.getProperties()):

And with similar Listeners:

Similarly, with System Security Manager you can override the System Security Manager (System.getSecurityManager())

Sometimes you want to test that your code calls System.exit. For that you can use the System Exit Listeners. The Listener will throw an exception when the System.exit is called, allowing you to catch it and verify:

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

And with the listeners

**Examples:**

Example 1 (kotlin):
```kotlin
withEnvironment("FooKey", "BarValue") {    System.getenv("FooKey") shouldBe "BarValue" // System environment overridden!}
```

Example 2 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {  jvmArgs("--add-opens=java.base/java.util=ALL-UNNAMED")}
```

Example 3 (kotlin):
```kotlin
withEnvironment(mapOf("FooKey" to "BarValue", "BarKey" to "FooValue")) {  // Use FooKey and BarKey}
```

Example 4 (kotlin):
```kotlin
class MyTest : FreeSpec() {      override fun listeners() = listOf(SystemEnvironmentTestListener("foo", "bar"))    init {      "MyTest" {        System.getenv("foo") shouldBe "bar"      }    }}
```

---

## BlockHound | Kotest

**URL:** https://kotest.io/docs/next/extensions/blockhound.html

**Contents:**
- BlockHound
  - Getting Started​
  - Detection​
  - Customization​

The Kotest BlockHound extension activates BlockHound support for coroutines. It helps to detect blocking code on non-blocking coroutine threads, e.g. when accidentally calling a blocking I/O library function on a UI thread.

To use this extension add the io.kotest:kotest-extensions-blockhound module to your test compile path.

Register the BlockHound extension in your test class:

The BlockHound extension can also be registered per test case or at the project level.

If BlockHound is enabled project-wide or spec-wide, you can disable it for an individual test:

You can also change BlockHoundMode for a section of code:

Blocking calls will be detected in coroutine threads which are expected not to block. Such threads are created by the default dispatcher as this example demonstrates:

The BlockHound extension will by default produce an exception like this whenever it detects a blocking call:

By invoking it as BlockHound(BlockHoundMode.PRINT), it will print detected calls and continue the test without interruption.

Whenever a blocking call is detected, you can

To customize BlockHound, familiarize yourself with the BlockHound documentation.

Exceptions for blocking calls considered harmless can be added via a separate BlockHoundIntegration class like this:

In order to allow BlockHound to auto-detect and load the integration, add its fully qualified class name to a service provider configuration file resources/META-INF/services/reactor.blockhound.integration.BlockHoundIntegration.

**Examples:**

Example 1 (kotlin):
```kotlin
class BlockHoundSpecTest : FunSpec({   extension(BlockHound())   test("detects for spec") {      blockInNonBlockingContext()   }})
```

Example 2 (kotlin):
```kotlin
test("allow blocking").config(extensions = listOf(BlockHound(BlockHoundMode.DISABLED))) {      blockInNonBlockingContext()   }
```

Example 3 (kotlin):
```kotlin
test("allow blocking section") {      // ...      withBlockHoundMode(BlockHoundMode.DISABLED) {        blockInNonBlockingContext()      }      // ...   }
```

Example 4 (kotlin):
```kotlin
private suspend fun blockInNonBlockingContext() {   withContext(Dispatchers.Default) {      @Suppress("BlockingMethodInNonBlockingContext")      Thread.sleep(2)   }}
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Setting Build Dir​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional junit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using allure with kotest. To start, add the below dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your buld, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will write out test data in the allure json format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the allure binary, or we can use the allure gradle plugin. To use the gradle plugin, first add the plugin to your build's plugins block.

Next, add an allure configuration section to set the version and disable autoconfigure (because allure can only auto configure junit and kotest takes care of this for you anyway).

Finally, execute the gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the gradle plugin then you will need to inform Allure where the build dir is by setting the allure.results.directory system property on your tests configuration. If you are using the gradle plugin, then this can be skipped as the gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.8.1"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## Spring | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/spring.html

**Contents:**
- Spring
  - Constructor Injection​
  - TestContexts​
  - Test Method Callbacks​
  - Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest.extensions:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Note: The maven group id differs from the core test framework (io.kotest.extensions).

The Spring extension requires you to activate it for all test classes, or per test class. To activate it globally, register the SpringExtension in project config:

To activate it per test class:

In order to let Spring know which configuration class to use, you must annotate your Spec classes with @ContextConfiguration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

For constructor injection, Kotest automatically registers a SpringAutowireConstructorExtension when the spring module is added to the build.

This extension will intercept each call to create a Spec instance and will autowire the beans declared in the primary constructor.

The following example is a test class which requires a service called UserService in its primary constructor. This service class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various test callbacks such as beforeTestMethod that are based around the idea that tests are methods. This assumption is fine for legacy test frameworks like JUnit but not applicable to modern test frameworks like Kotest where tests are functions.

Therefore, when using a spec style that is nested, you can customize when the test method callbacks are fired. By default, this is on the leaf node. You can set these to fire on the root nodes by passing a SpringTestLifecycleMode argument to the extension:

When using a final class, you may receive a warning from Kotest:

Using SpringListener on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(SpringExtension)}
```

Example 2 (kotlin):
```kotlin
class MyTestSpec : FunSpec() {   override fun extensions() = listOf(SpringExtension)}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {         println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## Spring | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/spring.html

**Contents:**
- Spring
  - Constructor Injection​
  - TestContexts​
  - Test Method Callbacks​
  - Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest.extensions:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Note: The maven group id differs from the core test framework (io.kotest.extensions).

The Spring extension requires you to activate it for all test classes, or per test class. To activate it globally, register the SpringExtension in project config:

To activate it per test class:

In order to let Spring know which configuration class to use, you must annotate your Spec classes with @ContextConfiguration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

For constructor injection, Kotest automatically registers a SpringAutowireConstructorExtension when the spring module is added to the build, assuming auto scan is enabled (see Project Config). If Auto scan is disabled, you will need to manually load the extension in your Project config.

This extension will intercept each call to create a Spec instance and will autowire the beans declared in the primary constructor.

The following example is a test class which requires a service called UserService in its primary constructor. This service class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various test callbacks such as beforeTestMethod that are based around the idea that tests are methods. This assumption is fine for legacy test frameworks like JUnit but not applicable to modern test frameworks like Kotest where tests are functions.

Therefore, when using a spec style that is nested, you can customize when the test method callbacks are fired. By default, this is on the leaf node. You can set these to fire on the root nodes by passing a SpringTestLifecycleMode argument to the extension:

When using a final class, you may receive a warning from Kotest:

Using SpringListener on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(SpringExtension)}
```

Example 2 (kotlin):
```kotlin
class MyTestSpec : FunSpec() {   override fun extensions() = listOf(SpringExtension)}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {         println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/pitest.html

**Contents:**
- Pitest
- Gradle configuration​
- Maven configuration​

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest.extensions:kotest-extensions-pitest module to your dependencies as well:

Note: Since pitest is an extension, we use a different maven group name (io.kotest.extensions) from the core modules.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

First of all, you need to configure the Maven Pitest plugin:

Then add the dependency on Pitest Kotest extension:

This should be enough to be able to run Pitest and get the reports as described in the Maven Pitest plugin.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

Example 3 (xml):
```xml
<plugin>    <groupId>org.pitest</groupId>    <artifactId>pitest-maven</artifactId>    <version>${pitest-maven.version}</version>    <configuration>        <targetClasses>...</targetClasses>        <coverageThreshold>...</coverageThreshold>        ... other configurations as needed            </configuration></plugin>
```

Example 4 (xml):
```xml
<dependencies>  ... the other Kotest dependencies like kotest-runner-junit5-jvm   <dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-pitest</artifactId>    <version>${kotest-extensions-pitest.version}</version>    <scope>test</scope>  </dependency></dependencies>
```

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
  - Dependencies​
  - Databases​
    - Initializing the Database Container​
  - General Containers​
  - Kafka Containers​
  - Lifecycle​
  - Startables​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 5.0+. For earlier versions see docs here

The Testcontainers project provides lightweight, ephemeral instances of common databases, elasticsearch, kafka, Selenium web browsers, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides several extensions - specialized extensions for databases and kafka and general containers support for any supported docker image.

To begin, add the following dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

For Maven, you will need these dependencies:

For JDBC compatible databases, Kotest provides the JdbcTestContainerExtension. This provides a pooled javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension wrapper, providing an optional configuration lambda.

If you don't wish to configure the pool, then you can omit the trailing lambda.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

This extension also supports the LifecycleMode flag to control when the container is started and stopped. See Lifecycle

There are two ways to initialize the database container: via a single init script added to the TestContainer config, or via a list of scripts added to the JdbcTestContainerExtension config lambda.

If adding a single script, via the TestContainer config, simply add the script to the TestContainer's withInitScript config option, like so:

If you have multiple init scripts or sets of changesets, you can add them as a list to the dbInitScripts extension config lambda, like so:

The list can contain absolute or relative paths, for files and folders on the filesystem or on the classpath.

The extension will process the list provided in order. If the list item is a folder, it will process all .sql scripts in the folder, sorted lexicographically. These scripts run every time the container is started, so it supports the LifecycleMode flag.

Similar to the JdbcTestContainerExtension, this module also provides a TestContainerExtension extension which can wrap any container, not just databases.

We can create the extension using either a docker image name, or a strongly typed container.

For example, using a docker image directly:

And then using a strongly typed container:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives us access to specific settings - such as the password option in the elasticsearch example above.

However, when a strongly typed container is not available, the former method allows us to spool up any docker image as a general container.

This extension also supports the LifecycleMode flag to control when the container is started and stopped. See Lifecycle

For Kafka, this module provides convenient extension methods to create a consumer, producer or admin client from the container.

Inside the configuration lambda, we can specify options for the Kafka container, such as embedded/external zookeeper, or kafka broker properties through env vars. For example, to enable dynamic topic creation:

Kafka only publishes a linux/amd64 version of the container. If you're on an Apple Silicon/ARM architecture computer, you'll need to explicitly specify the platform with the following added to the configuration lambda outlined above:

Once we have the container installed, we can create a client using the following methods:

Each of these accepts an optional configuration lambda to enable setting values on the properties object that is used to create the clients.

For example, in this test, we produce and consume a message from the same topic, and we use the configuration lambda to set max poll to 1.

When creating a consumer, the consumer group is set to a random uuid. To change this, provide a configuration lambda and specify your own group consumer group id.

By default, the lifecycle of a container is per spec - so it will be started at the install command, and shutdown as the spec is completed. This can be changed to start/stop per test, per leaf test, or per root test.

To do this, pass in a LifecycleMode parameter to the TestContainerExtension or JdbcTestContainerExtension.

If you change the lifecycle mode from Spec then the container will not be started in the constructor, and so any operations that act on the container must be placed inside the test scopes.

This module also provides extension methodsscope which let you convert any Startable such as a DockerContainer into a kotest TestListener, which you can register with Kotest and then Kotest will manage the lifecycle of that container for you.

In above example, the perTest() extension method converts the container into a TestListener, which starts the redis container before each test and stops it after test. Similarly if you want to reuse the container for all tests in a single spec class you can use perSpec() extension method, which converts the container into a TestListener which starts the container before running any test in the spec, and stops it after all tests, thus a single container is used by all tests in spec class.

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-testcontainers</artifactId>    <version>${kotest.version}</version>    <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val mysql = MySQLContainer<Nothing>("mysql:8.0.26").apply {  startupAttempts = 1  withUrlParam("connectionTimeZone", "Z")  withUrlParam("zeroDateTimeBehavior", "convertToNull")}
```

Example 4 (kotlin):
```kotlin
val ds = install(JdbcTestContainerExtension(mysql)) {  poolName = "myconnectionpool"  maximumPoolSize = 8  idleTimeout = 10000}
```

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (json):
```json
io.kotest.extensions:kotest-extensions-wiremock:{version}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## Spring | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/spring.html

**Contents:**
- Spring
  - Constructor Injection​
  - TestContexts​
  - Test Method Callbacks​
  - Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest.extensions:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Note: The maven group id differs from the core test framework (io.kotest.extensions).

The Spring extension requires you to activate it for all test classes, or per test class. To activate it globally, register the SpringExtension in project config:

To activate it per test class:

In order to let Spring know which configuration class to use, you must annotate your Spec classes with @ContextConfiguration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

For constructor injection, Kotest automatically registers a SpringAutowireConstructorExtension when the spring module is added to the build, assuming auto scan is enabled (see Project Config). If Auto scan is disabled, you will need to manually load the extension in your Project config.

This extension will intercept each call to create a Spec instance and will autowire the beans declared in the primary constructor.

The following example is a test class which requires a service called UserService in its primary constructor. This service class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various test callbacks such as beforeTestMethod that are based around the idea that tests are methods. This assumption is fine for legacy test frameworks like JUnit but not applicable to modern test frameworks like Kotest where tests are functions.

Therefore, when using a spec style that is nested, you can customize when the test method callbacks are fired. By default, this is on the leaf node. You can set these to fire on the root nodes by passing a SpringTestLifecycleMode argument to the extension:

When using a final class, you may receive a warning from Kotest:

Using SpringListener on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(SpringExtension)}
```

Example 2 (kotlin):
```kotlin
class MyTestSpec : FunSpec() {   override fun extensions() = listOf(SpringExtension)}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {         println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/pitest.html

**Contents:**
- Pitest

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest.extensions:kotest-extensions-pitest module to your dependencies as well:

Note: Since pitest is an extension, we use a different maven group name (io.kotest.extensions) from the core modules.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has three parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override val extensions: List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false, // don't write out status for all tests      useTestPathAsName = true, // use the full test path (ie, includes parent test names)      outputDir = "../target/junit-xml" // include to set output dir for maven    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/extensions/mockserver.html

**Contents:**
- MockServer
- Dynamic Ports​

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest:kotest-extensions-mockserver module to be added to your build.

Since Kotest 6.0, all extensions are published under the io.kotest group, with version cadence tied to main Kotest releases.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we install an instance of MockServerExtension in the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

When using the MockServerExtension, you can specify one or more ports if you wish to hardcore them. Otherwise, you can not specify them at all, and Kotest will automatically allocate a free port for the server to run on. Then, you can use the returned server instance from the install function to retrieve the allocated port.

Here is an example of using dynamic ports:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      install(MockServerExtension(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

Example 2 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {    val server = install(MockServerExtension())    beforeTest {      MockServerClient("localhost", server.port).`when`(        HttpRequest.request()          .withMethod("GET")          .withPath("/v")      ).respond(        HttpResponse.response()          .withStatusCode(200)      )    }    test("test /health returns 200") {      val client = HttpClient(CIO)      val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:${healthcheck.port}/health")      resp.shouldHaveStatus(HttpStatusCode.OK)    }  }}
```

---

## Test Clock | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/test_clock.html

**Contents:**
- Test Clock

The JVM provides the java.time.Clock interface which is used to generate Instants. When we have code that relies on time, we can use a Clock to generate the values, rather than using things like Instant.now() or System.currentTimeMillis().

Then in tests we can provide a fixed or controllable clock which avoids issues where the time changes on each test run. In your real code, you provide an instance of Clock.systemUTC() or whatever.

The following module is needed: io.kotest.extensions:kotest-extensions-clock in your build. Search maven central for latest version here.

In order to use it, we create an instance of the TestClock passing in an instant and a zone offset.

We can control the clock via plus and minus which accept durations, eg

Note that the clock is mutable, and the internal state is changed when you use plus or minus.

**Examples:**

Example 1 (unknown):
```unknown
val timestamp = Instant.ofEpochMilli(1234)val clock = TestClock(timestamp, ZoneOffset.UTC)
```

Example 2 (unknown):
```unknown
clock.plus(6.minutes)
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/next/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Setting Build Dir​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional junit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using allure with kotest. To start, add the below dependency to your Gradle build file.

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your buld, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will write out test data in the allure json format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the allure binary, or we can use the allure gradle plugin. To use the gradle plugin, first add the plugin to your build's plugins block.

Next, add an allure configuration section to set the version and disable autoconfigure (because allure can only auto configure junit and kotest takes care of this for you anyway).

Finally, execute the gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the gradle plugin then you will need to inform Allure where the build dir is by setting the allure.results.directory system property on your tests configuration. If you are using the gradle plugin, then this can be skipped as the gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.8.1"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## Pitest | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/pitest.html

**Contents:**
- Pitest
- Gradle configuration​
- Maven configuration​

The Mutation Testing tool Pitest is integrated with Kotest via an extension module.

After configuring Pitest, add the io.kotest.extensions:kotest-extensions-pitest module to your dependencies as well:

Note: Since pitest is an extension, we use a different maven group name (io.kotest.extensions) from the core modules.

After doing that, we need to inform Pitest that we're going to use Kotest as a testPlugin:

This should set everything up, and running ./gradlew pitest will generate reports in the way you configured.

First of all, you need to configure the Maven Pitest plugin:

Then add the dependency on Pitest Kotest extension:

This should be enough to be able to run Pitest and get the reports as described in the Maven Pitest plugin.

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-pitest:<version>")
```

Example 2 (kotlin):
```kotlin
// Assuming that you have already configured the Gradle/Maven extensionconfigure<PitestPluginExtension> {    // testPlugin.set("Kotest")    // needed only with old PIT <1.6.7, otherwise having kotest-extensions-pitest on classpath is enough    targetClasses.set(listOf("my.company.package.*"))}
```

Example 3 (xml):
```xml
<plugin>    <groupId>org.pitest</groupId>    <artifactId>pitest-maven</artifactId>    <version>${pitest-maven.version}</version>    <configuration>        <targetClasses>...</targetClasses>        <coverageThreshold>...</coverageThreshold>        ... other configurations as needed            </configuration></plugin>
```

Example 4 (xml):
```xml
<dependencies>  ... the other Kotest dependencies like kotest-runner-junit5-jvm   <dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-pitest</artifactId>    <version>${kotest-extensions-pitest.version}</version>    <scope>test</scope>  </dependency></dependencies>
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Setting Build Dir​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional junit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using allure with kotest. To start, add the below dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your buld, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will write out test data in the allure json format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the allure binary, or we can use the allure gradle plugin. To use the gradle plugin, first add the plugin to your build's plugins block.

Next, add an allure configuration section to set the version and disable autoconfigure (because allure can only auto configure junit and kotest takes care of this for you anyway).

Finally, execute the gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the gradle plugin then you will need to inform Allure where the build dir is by setting the allure.results.directory system property on your tests configuration. If you are using the gradle plugin, then this can be skipped as the gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.8.1"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
  - Dependencies​
  - Databases​
    - Initializing the Database Container​
  - General Containers​
  - Kafka Containers​
  - Lifecycle​
  - Startables​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 5.0+. For earlier versions see docs here

The Testcontainers project provides lightweight, ephemeral instances of common databases, elasticsearch, kafka, Selenium web browsers, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides several extensions - specialized extensions for databases and kafka and general containers support for any supported docker image.

To begin, add the following dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

For Maven, you will need these dependencies:

For JDBC compatible databases, Kotest provides the JdbcTestContainerExtension. This provides a pooled javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension wrapper, providing an optional configuration lambda.

If you don't wish to configure the pool, then you can omit the trailing lambda.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

This extension also supports the ContainerLifecycleMode flag to control when the container is started and stopped. See Lifecycle

There are two ways to initialize the database container: via a single init script added to the TestContainer config, or via a list of scripts added to the JdbcTestContainerExtension config lambda.

If adding a single script, via the TestContainer config, simply add the script to the TestContainer's withInitScript config option, like so:

If you have multiple init scripts or sets of changesets, you can add them as a list to the dbInitScripts extension config lambda, like so:

The list can contain absolute or relative paths, for files and folders on the filesystem or on the classpath.

The extension will process the list provided in order. If the list item is a folder, it will process all .sql scripts in the folder, sorted lexicographically. These scripts run every time the container is started, so it supports the ContainerLifecycleMode flag.

Similar to the JdbcDatabaseContainerExtension, this module also provides a ContainerExtension extension which can wrap any container, not just databases.

We can create the extension using either a docker image name, or a strongly typed container.

For example, using a docker image directly:

And then using a strongly typed container:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives us access to specific settings - such as the password option in the elasticsearch example above.

However, when a strongly typed container is not available, the former method allows us to spool up any docker image as a general container.

This extension also supports the ContainerLifecycleMode flag to control when the container is started and stopped. See Lifecycle

For Kafka, this module provides convenient extension methods to create a consumer, producer or admin client from the container.

Inside the configuration lambda, we can specify options for the Kafka container, such as embedded/external zookeeper, or kafka broker properties through env vars. For example, to enable dynamic topic creation:

Kafka only publishes a linux/amd64 version of the container. If you're on an Apple Silicon/ARM architecture computer, you'll need to explicitly specify the platform with the following added to the configuration lambda outlined above:

Once we have the container installed, we can create a client using the following methods:

Each of these accepts an optional configuration lambda to enable setting values on the properties object that is used to create the clients.

For example, in this test, we produce and consume a message from the same topic, and we use the configuration lambda to set max poll to 1.

When creating a consumer, the consumer group is set to a random uuid. To change this, provide a configuration lambda and specify your own group consumer group id.

By default, the lifecycle of a container is per spec - so it will be started at the install command, and shutdown as the spec is completed. This can be changed to start/stop per test, per leaf test, or per root test.

To do this, pass in a ContainerLifecycleMode parameter to the ContainerExtension or JdbcDatabaseContainerExtension.

This module also provides extension methodsscope which let you convert any Startable such as a DockerContainer into a kotest TestListener, which you can register with Kotest and then Kotest will manage the lifecycle of that container for you.

In above example, the perTest() extension method converts the container into a TestListener, which starts the redis container before each test and stops it after test. Similarly if you want to reuse the container for all tests in a single spec class you can use perSpec() extension method, which converts the container into a TestListener which starts the container before running any test in the spec, and stops it after all tests, thus a single container is used by all tests in spec class.

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-testcontainers</artifactId>    <version>${kotest.version}</version>    <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val mysql = MySQLContainer<Nothing>("mysql:8.0.26").apply {  startupAttempts = 1  withUrlParam("connectionTimeZone", "Z")  withUrlParam("zeroDateTimeBehavior", "convertToNull")}
```

Example 4 (kotlin):
```kotlin
val ds = install(JdbcDatabaseContainerExtension(mysql)) {  poolName = "myconnectionpool"  maximumPoolSize = 8  idleTimeout = 10000}
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Setting Build Dir​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional junit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using allure with kotest. To start, add the below dependency to your Gradle build file.

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your buld, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will write out test data in the allure json format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the allure binary, or we can use the allure gradle plugin. To use the gradle plugin, first add the plugin to your build's plugins block.

Next, add an allure configuration section to set the version and disable autoconfigure (because allure can only auto configure junit and kotest takes care of this for you anyway).

Finally, execute the gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the gradle plugin then you will need to inform Allure where the build dir is by setting the allure.results.directory system property on your tests configuration. If you are using the gradle plugin, then this can be skipped as the gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.8.1"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Additionally, prevent Gradle from generating its own html reports by adding html.required.set(false) to the test task.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override val extensions): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

Example 2 (css):
```css
tasks.test {  useJUnitPlatform()  reports {    html.required.set(false)    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Additionally, prevent Gradle from generating its own html reports by adding html.required.set(false) to the test task.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override fun extensions(): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

Example 2 (css):
```css
tasks.test {  useJUnitPlatform()  reports {    html.required.set(false)    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Setting Build Dir​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional junit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using allure with kotest. To start, add the below dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your buld, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will write out test data in the allure json format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the allure binary, or we can use the allure gradle plugin. To use the gradle plugin, first add the plugin to your build's plugins block.

Next, add an allure configuration section to set the version and disable autoconfigure (because allure can only auto configure junit and kotest takes care of this for you anyway).

Finally, execute the gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the gradle plugin then you will need to inform Allure where the build dir is by setting the allure.results.directory system property on your tests configuration. If you are using the gradle plugin, then this can be skipped as the gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.8.1"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## Koin | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/koin.html

**Contents:**
- Koin
- Koin​

The Koin DI Framework can be used with Kotest through the KoinExtension extension.

To use the extension in your project, add the dependency to your project:

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

With the dependency added, we can easily use Koin in our tests!

By default, the extension will start/stop the Koin context between leaf tests. If you are using a nested spec style (like DescribeSpec) and instead want the Koin context to persist over all leafs of a root tests (for example to share mocked declarations between tests), you can specify the lifecycle mode as KoinLifecycleMode.Root in the KoinExtension constructor.

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest:kotest-extensions-koin:${kotestVersion}
```

Example 2 (kotlin):
```kotlin
class KotestAndKoin : KoinTest, FunSpec() {  init {    extension(KoinExtension(koinModule) { mockk<UserService>() })    test("use userService") {      val userService by inject<UserService>()      userService.getUser().username shouldBe "LeoColman"    }  }}
```

Example 3 (kotlin):
```kotlin
class KotestAndKoin : KoinTest, DescribeSpec() {  init {    extension(KoinExtension(module = myKoinModule, mode = KoinLifecycleMode.Root))    describe("use userService") {      val userService by inject<UserService>()      it("inside a leaf test") {        userService.getUser().username shouldBe "LeoColman"      }      it("this shares the same context") {        userService.getUser().username shouldBe "LeoColman"      }    }  }}
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## BlockHound | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/blockhound.html

**Contents:**
- BlockHound
  - Getting Started​
  - Detection​
  - Customization​

The Kotest BlockHound extension activates BlockHound support for coroutines. It helps to detect blocking code on non-blocking coroutine threads, e.g. when accidentally calling a blocking I/O library function on a UI thread.

To use this extension add the io.kotest.extensions:kotest-extensions-blockhound module to your test compile path.

Register the BlockHound extension in your test class:

The BlockHound extension can also be registered per test case or at the project level.

If BlockHound is enabled project-wide or spec-wide, you can disable it for an individual test:

You can also change BlockHoundMode for a section of code:

Blocking calls will be detected in coroutine threads which are expected not to block. Such threads are created by the default dispatcher as this example demonstrates:

The BlockHound extension will by default produce an exception like this whenever it detects a blocking call:

By invoking it as BlockHound(BlockHoundMode.PRINT), it will print detected calls and continue the test without interruption.

Whenever a blocking call is detected, you can

To customize BlockHound, familiarize yourself with the BlockHound documentation.

Exceptions for blocking calls considered harmless can be added via a separate BlockHoundIntegration class like this:

In order to allow BlockHound to auto-detect and load the integration, add its fully qualified class name to a service provider configuration file resources/META-INF/services/reactor.blockhound.integration.BlockHoundIntegration.

**Examples:**

Example 1 (kotlin):
```kotlin
class BlockHoundSpecTest : FunSpec({   extension(BlockHound())   test("detects for spec") {      blockInNonBlockingContext()   }})
```

Example 2 (kotlin):
```kotlin
test("allow blocking").config(extensions = listOf(BlockHound(BlockHoundMode.DISABLED))) {      blockInNonBlockingContext()   }
```

Example 3 (kotlin):
```kotlin
test("allow blocking section") {      // ...      withBlockHoundMode(BlockHoundMode.DISABLED) {        blockInNonBlockingContext()      }      // ...   }
```

Example 4 (kotlin):
```kotlin
private suspend fun blockInNonBlockingContext() {   withContext(Dispatchers.Default) {      @Suppress("BlockingMethodInNonBlockingContext")      Thread.sleep(2)   }}
```

---

## JUnit XML Format Reporter | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/junit_xml.html

**Contents:**
- JUnit XML Format Reporter
  - Parameters​

JUnit includes an XML report generator that it calls the legacy xml report . Many tools integrate with this format so it is very useful. However, this report has no concept of nesting tests. Therefore when used with a nested test style in Kotest, it will include parent tests as orphans.

To solve this, Kotest has it's own implementation of the same format, that is configurable on whether to include parent tests and/or collapse the names.

The following module is needed: io.kotest:kotest-extensions-junitxml in your build. Search maven central for latest version here.

To configure in your project, you need to add the JunitXmlReporter using project config.

Additionally, the reporter needs to know where your build output folder is by setting a system property. Gradle also needs to know that it should not generate JUnit XML reports by itself. We configure that in the tests block in gradle.

The reporter has two parameters:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig() {  override fun extensions(): List<Extension> = listOf(    JunitXmlReporter(      includeContainers = false,      useTestPathAsName = true    )  )}
```

Example 2 (kotlin):
```kotlin
tasks.named<Test>("test") {  useJUnitPlatform()  reports {    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Test Clock | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/test_clock.html

**Contents:**
- Test Clock

The JVM provides the java.time.Clock interface which is used to generate Instants. When we have code that relies on time, we can use a Clock to generate the values, rather than using things like Instant.now() or System.currentTimeMillis().

Then in tests we can provide a fixed or controllable clock which avoids issues where the time changes on each test run. In your real code, you provide an instance of Clock.systemUTC() or whatever.

The following module is needed: io.kotest.extensions:kotest-extensions-clock in your build. Search maven central for latest version here.

In order to use it, we create an instance of the TestClock passing in an instant and a zone offset.

We can control the clock via plus and minus which accept durations, eg

Note that the clock is mutable, and the internal state is changed when you use plus or minus.

**Examples:**

Example 1 (unknown):
```unknown
val timestamp = Instant.ofEpochMilli(1234)val clock = TestClock(timestamp, ZoneOffset.UTC)
```

Example 2 (unknown):
```unknown
clock.plus(6.minutes)
```

---

## Testcontainers | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/test_containers.html

**Contents:**
- Testcontainers
- Testcontainers​
  - Dependencies​
  - Databases​
    - Initializing the Database Container​
  - General Containers​
  - Kafka Containers​
  - Lifecycle​
  - Startables​

This documentation is for the latest release of the Testcontainers module and is compatible with Kotest 5.0+. For earlier versions see docs here

The Testcontainers project provides lightweight, ephemeral instances of common databases, elasticsearch, kafka, Selenium web browsers, or anything else that can run in a Docker container - ideal for use inside tests.

Kotest provides integration with Testcontainers through an additional module which provides several extensions - specialized extensions for databases and kafka and general containers support for any supported docker image.

To begin, add the following dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

For Maven, you will need these dependencies:

For JDBC compatible databases, Kotest provides the JdbcTestContainerExtension. This provides a pooled javax.sql.DataSource, backed by an instance of HikariCP, which can be configured during setup.

Firstly, create the container.

Secondly, install the container inside an extension wrapper, providing an optional configuration lambda.

If you don't wish to configure the pool, then you can omit the trailing lambda.

Then the datasource can be used in a test. For example, here is a full example of inserting some objects and then retrieving them to test that the insert was successful.

This extension also supports the ContainerLifecycleMode flag to control when the container is started and stopped. See Lifecycle

There are two ways to initialize the database container: via a single init script added to the TestContainer config, or via a list of scripts added to the JdbcTestContainerExtension config lambda.

If adding a single script, via the TestContainer config, simply add the script to the TestContainer's withInitScript config option, like so:

If you have multiple init scripts or sets of changesets, you can add them as a list to the dbInitScripts extension config lambda, like so:

The list can contain absolute or relative paths, for files and folders on the filesystem or on the classpath.

The extension will process the list provided in order. If the list item is a folder, it will process all .sql scripts in the folder, sorted lexicographically. These scripts run every time the container is started, so it supports the ContainerLifecycleMode flag.

Similar to the JdbcDatabaseContainerExtension, this module also provides a ContainerExtension extension which can wrap any container, not just databases.

We can create the extension using either a docker image name, or a strongly typed container.

For example, using a docker image directly:

And then using a strongly typed container:

The strongly typed container is preferred when one is provided by the Testcontainers project, because it gives us access to specific settings - such as the password option in the elasticsearch example above.

However, when a strongly typed container is not available, the former method allows us to spool up any docker image as a general container.

This extension also supports the ContainerLifecycleMode flag to control when the container is started and stopped. See Lifecycle

For Kafka, this module provides convenient extension methods to create a consumer, producer or admin client from the container.

Inside the configuration lambda, we can specify options for the Kafka container, such as embedded/external zookeeper, or kafka broker properties through env vars. For example, to enable dynamic topic creation:

Kafka only publishes a linux/amd64 version of the container. If you're on an Apple Silicon/ARM architecture computer, you'll need to explicitly specify the platform with the following added to the configuration lambda outlined above:

Once we have the container installed, we can create a client using the following methods:

Each of these accepts an optional configuration lambda to enable setting values on the properties object that is used to create the clients.

For example, in this test, we produce and consume a message from the same topic, and we use the configuration lambda to set max poll to 1.

When creating a consumer, the consumer group is set to a random uuid. To change this, provide a configuration lambda and specify your own group consumer group id.

By default, the lifecycle of a container is per spec - so it will be started at the install command, and shutdown as the spec is completed. This can be changed to start/stop per test, per leaf test, or per root test.

To do this, pass in a ContainerLifecycleMode parameter to the ContainerExtension or JdbcDatabaseContainerExtension.

This module also provides extension methodsscope which let you convert any Startable such as a DockerContainer into a kotest TestListener, which you can register with Kotest and then Kotest will manage the lifecycle of that container for you.

In above example, the perTest() extension method converts the container into a TestListener, which starts the redis container before each test and stops it after test. Similarly if you want to reuse the container for all tests in a single spec class you can use perSpec() extension method, which converts the container into a TestListener which starts the container before running any test in the spec, and stops it after all tests, thus a single container is used by all tests in spec class.

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-testcontainers:${kotest.version}
```

Example 2 (xml):
```xml
<dependency>    <groupId>io.kotest.extensions</groupId>    <artifactId>kotest-extensions-testcontainers</artifactId>    <version>${kotest.version}</version>    <scope>test</scope></dependency>
```

Example 3 (kotlin):
```kotlin
val mysql = MySQLContainer<Nothing>("mysql:8.0.26").apply {  startupAttempts = 1  withUrlParam("connectionTimeZone", "Z")  withUrlParam("zeroDateTimeBehavior", "convertToNull")}
```

Example 4 (kotlin):
```kotlin
val ds = install(JdbcDatabaseContainerExtension(mysql)) {  poolName = "myconnectionpool"  maximumPoolSize = 8  idleTimeout = 10000}
```

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/mockserver.html

**Contents:**
- MockServer

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest.extensions:kotest-extensions-mockserver module to be added to your build.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we attach an instance of MockServerListener to the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      listener(MockServerListener(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

---

## Koin | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/koin.html

**Contents:**
- Koin
- Koin​

The Koin DI Framework can be used with Kotest through the KoinExtension extension.

To use the extension in your project, add the dependency to your project:

With the dependency added, we can easily use Koin in our tests!

By default, the extension will start/stop the Koin context between leaf tests. If you are using a nested spec style (like DescribeSpec) and instead want the Koin context to persist over all leafs of a root tests (for example to share mocked declarations between tests), you can specify the lifecycle mode as KoinLifecycleMode.Root in the KoinExtension constructor.

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest.extensions:kotest-extensions-koin:${version}
```

Example 2 (kotlin):
```kotlin
class KotestAndKoin : FunSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(myKoinModule))    val userService by inject<UserService>()    init {        test("use userService") {            userService.getUser().username shouldBe "LeoColman"        }    }}
```

Example 3 (kotlin):
```kotlin
class KotestAndKoin : DescribeSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(module = myKoinModule, mode = KoinLifecycleMode.Root))    val userService by inject<UserService>()    init {        describe("use userService") {            it("inside a leaf test") {                userService.getUser().username shouldBe "LeoColman"            }            it("this shares the same context") {                userService.getUser().username shouldBe "LeoColman"            }        }    }}
```

---

## Allure | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/allure.html

**Contents:**
- Allure
  - Collect Data​
  - Gradle Plugin​
  - Setting Build Dir​
  - Final Report​

Allure is an open-source framework designed for detailed and interactive test reports. It works by generating report files which are then used to create the final HTML report. You can think of it as like the traditional junit report but more advanced and detailed.

If you prefer to see an example rather than read docs, then there is a sample project here

There are two steps to allure. The first is to generate the raw data when executing tests, the second is to compile that data into the interactive HTML report.

This module provides integration for using allure with kotest. To start, add the below dependency to your Gradle build file.

Note: The group id is different (io.kotest.extensions) from the main kotest dependencies (io.kotest).

Allure has data collectors for most test frameworks and this module provides the integration for Kotest. Once the module has been added to your buld, wire in the AllureTestReporter class globally using project config.

Now, whenever tests are executed, Kotest will write out test data in the allure json format.

Now that the tests have completed, we can compile them into the final report.

This can be done manually using the allure binary, or we can use the allure gradle plugin. To use the gradle plugin, first add the plugin to your build's plugins block.

Next, add an allure configuration section to set the version and disable autoconfigure (because allure can only auto configure junit and kotest takes care of this for you anyway).

Finally, execute the gradle task allureReport and the report will be generated in ./build/reports/allure-report and inside you should find the index.html entry point for the report.

If you are not using the gradle plugin then you will need to inform Allure where the build dir is by setting the allure.results.directory system property on your tests configuration. If you are using the gradle plugin, then this can be skipped as the gradle plugin does this for you.

If all was successful, after test execution and report generation, you will see something like this:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-extensions-allure:${kotest.version}
```

Example 2 (kotlin):
```kotlin
class MyConfig : AbstractProjectConfig {    override fun listeners() = listOf(AllureTestReporter())}
```

Example 3 (kotlin):
```kotlin
plugins {  ...  id("io.qameta.allure") version "2.8.1"}
```

Example 4 (kotlin):
```kotlin
allure {  autoconfigure = false  version = "2.13.1"}
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/extensions.html

**Contents:**
- Extensions
  - Kotest Team Extensions​
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties.

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/mockserver.html

**Contents:**
- MockServer

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest.extensions:kotest-extensions-mockserver module to be added to your build.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we attach an instance of MockServerListener to the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      listener(MockServerListener(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (json):
```json
io.kotest.extensions:kotest-extensions-wiremock:{version}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## Koin | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/koin.html

**Contents:**
- Koin
- Koin​

The Koin DI Framework can be used with Kotest through the KoinExtension extension.

To use the extension in your project, add the dependency to your project:

With the dependency added, we can easily use Koin in our tests!

By default, the extension will start/stop the Koin context between leaf tests. If you are using a nested spec style (like DescribeSpec) and instead want the Koin context to persist over all leafs of a root tests (for example to share mocked declarations between tests), you can specify the lifecycle mode as KoinLifecycleMode.Root in the KoinExtension constructor.

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest.extensions:kotest-extensions-koin:${version}
```

Example 2 (kotlin):
```kotlin
class KotestAndKoin : FunSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(myKoinModule))    val userService by inject<UserService>()    init {        test("use userService") {            userService.getUser().username shouldBe "LeoColman"        }    }}
```

Example 3 (kotlin):
```kotlin
class KotestAndKoin : DescribeSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(module = myKoinModule, mode = KoinLifecycleMode.Root))    val userService by inject<UserService>()    init {        describe("use userService") {            it("inside a leaf test") {                userService.getUser().username shouldBe "LeoColman"            }            it("this shares the same context") {                userService.getUser().username shouldBe "LeoColman"            }        }    }}
```

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/mockserver.html

**Contents:**
- MockServer

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest.extensions:kotest-extensions-mockserver module to be added to your build.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we attach an instance of MockServerListener to the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      listener(MockServerListener(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Additionally, prevent Gradle from generating its own html reports by adding html.required.set(false) to the test task.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override fun extensions(): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

Example 2 (css):
```css
tasks.test {  useJUnitPlatform()  reports {    html.required.set(false)    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/5.3.x/extensions/extensions.html

**Contents:**
- Extensions
  - Kotest Team Extensions​
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties.

---

## BlockHound | Kotest

**URL:** https://kotest.io/docs/5.6.x/extensions/blockhound.html

**Contents:**
- BlockHound
  - Getting Started​
  - Detection​
  - Customization​

The Kotest BlockHound extension activates BlockHound support for coroutines. It helps to detect blocking code on non-blocking coroutine threads, e.g. when accidentally calling a blocking I/O library function on a UI thread.

To use this extension add the io.kotest.extensions:kotest-extensions-blockhound module to your test compile path.

Register the BlockHound extension in your test class:

The BlockHound extension can also be registered per test case or at the project level.

This code is sensitive to concurrency. There can only be one instance of this extension running at a time as it will take effect globally.

You cannot register the BlockHound extension multiple times at different levels.

Use @DoNotParallelize for BlockHound-enabled tests.

Blocking calls will be detected in coroutine threads which are expected not to block. Such threads are created by the default dispatcher as this example demonstrates:

The BlockHound extension will by default produce an exception like this whenever it detects a blocking call:

By invoking it as BlockHound(BlockHoundMode.PRINT), it will print detected calls and continue the test without interruption.

Whenever a blocking call is detected, you can

To customize BlockHound, familiarize yourself with the BlockHound documentation.

Exceptions for blocking calls considered harmless can be added via a separate BlockHoundIntegration class like this:

In order to allow BlockHound to auto-detect and load the integration, add its fully qualified class name to a service provider configuration file resources/META-INF/services/reactor.blockhound.integration.BlockHoundIntegration.

**Examples:**

Example 1 (kotlin):
```kotlin
@DoNotParallelizeclass BlockHoundSpecTest : FunSpec({   extension(BlockHound())   test("detects for spec") {      blockInNonBlockingContext()   }})
```

Example 2 (kotlin):
```kotlin
private suspend fun blockInNonBlockingContext() {   withContext(Dispatchers.Default) {      @Suppress("BlockingMethodInNonBlockingContext")      Thread.sleep(2)   }}
```

Example 3 (swift):
```swift
reactor.blockhound.BlockingOperationError: Blocking call! java.lang.Thread.sleep    at io.kotest.extensions.blockhound.KotestBlockHoundIntegration.applyTo$lambda-2$lambda-1(KotestBlockHoundIntegration.kt:27)    at reactor.blockhound.BlockHound$Builder.lambda$install$8(BlockHound.java:427)    at reactor.blockhound.BlockHoundRuntime.checkBlocking(BlockHoundRuntime.java:89)    at java.base/java.lang.Thread.sleep(Thread.java)    at io.kotest.extensions.blockhound.BlockHoundTestKt$blockInNonBlockingContext$2.invokeSuspend(BlockHoundTest.kt:17)    at kotlin.coroutines.jvm.internal.BaseContinuationImpl.resumeWith(ContinuationImpl.kt:33)    at kotlinx.coroutines.DispatchedTask.run(DispatchedTask.kt:106)    at kotlinx.coroutines.scheduling.CoroutineScheduler.runSafely(CoroutineScheduler.kt:570)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.executeTask(CoroutineScheduler.kt:750)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.runWorker(CoroutineScheduler.kt:677)    at kotlinx.coroutines.scheduling.CoroutineScheduler$Worker.run(CoroutineScheduler.kt:664)
```

Example 4 (kotlin):
```kotlin
import reactor.blockhound.BlockHoundimport reactor.blockhound.integration.BlockHoundIntegrationclass MyBlockHoundIntegration : BlockHoundIntegration {   override fun applyTo(builder: BlockHound.Builder): Unit = with(builder) {      allowBlockingCallsInside("org.slf4j.LoggerFactory", "performInitialization")   }}
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/extensions/extensions.html

**Contents:**
- Extensions
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties. For extensions provided directly by the Kotest team, see the links on the left.

---

## Embedded Kafka Extension | Kotest

**URL:** https://kotest.io/docs/5.9.x/extensions/embedded-kafka.html

**Contents:**
- Embedded Kafka Extension
  - Getting started:​
  - Consumer / Producer​
  - Custom Ports​

Kotest offers an extension that spins up an embedded Kafka instance. This can help in situations where using the kafka docker images are an issue.

To use this extension add the io.kotest.extensions:kotest-extensions-embedded-kafka module to your test compile path.

Register the embeddedKafkaListener listener in your test class:

And the broker will be started once the spec is created and stopped once the spec completes.

Note: The underlying embedded kafka library uses a global object for state. Do not start multiple kafka instances at the same time.

To create a consumer and producer we can use convenience methods on the listener:

The stringStringProducer and stringStringConsumer methods return a producer / consumer that accept strings for the keys and values. Similar methods exist for byte pairs.

Alternatively, you can access the host/port the Kafka instance was deployed on and create the clients yourself:

You can create a new instance of the listener specifying a port and then use that instance rather than the default instance.

You can also do specify the zookeeper port using an alternative overload.

**Examples:**

Example 1 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({  listener(embeddedKafkaListener)})
```

Example 2 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec() {  init {    listener(embeddedKafkaListener)  }}
```

Example 3 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)   test("send / receive") {     val producer = embeddedKafkaListener.stringStringProducer()     producer.send(ProducerRecord("foo", "a"))     producer.close()     val consumer = embeddedKafkaListener.stringStringConsumer("foo")     eventually(10.seconds) {       consumer.poll(1000).first().value() shouldBe "a"     }     consumer.close()   }})
```

Example 4 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)      val props = Properties().apply {      put(CommonClientConfigs.BOOTSTRAP_SERVERS_CONFIG, "${embeddedKafkaListener.host}:${embeddedKafkaListener.port}")   }      val producer = KafkaProducer<String, String>(props)   }
```

---

## Robolectric | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/robolectric.html

**Contents:**
- Robolectric
- Robolectric​

Robolectric can be used with Kotest through the RobolectricExtension which can be found in a separate repository,kotest-extensions-robolectric

To add this module to project you need specify following in your build.gradle:

This dependency brings in RobolectricExtension, which is autoregistered to your projects.

Now all you need to do is annotate Robolectric specs with @RobolectricTest and you're set!

**Examples:**

Example 1 (kotlin):
```kotlin
testImplementation("io.kotest.extensions:kotest-extensions-robolectric:${version}")
```

Example 2 (kotlin):
```kotlin
@RobolectricTestclass MyTest : ShouldSpec({    should("Access Robolectric normally!") {    }})
```

---

## System Extensions | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/system_extensions.html

**Contents:**
- System Extensions
- System Extensions​
  - System Environment​
  - System Property Extension​
  - System Security Manager​
  - System Exit Extensions​
  - No-stdout / no-stderr listeners​
  - Locale/Timezone listeners​

If you need to test code that uses java.lang.System, Kotest provides extensions that can alter the system and restore it after each test. This extension is only available on the JVM.

To use this extension, add the dependency to your project:

This extension does not support concurrent test execution. Due to the JVM specification there can only be one instance of these extensions running (For example: Only one Environment map must exist). If you try to run more than one instance at a time, the result is undefined.

With System Environment Extension you can simulate how the System Environment is behaving. That is, what you're obtaining from System.getenv().

Kotest provides some extension functions that provides a System Environment in a specific scope:

To use withEnvironment with JDK17 you need to add --add-opens=java.base/java.util=ALL-UNNAMED to the arguments for the JVM that runs the tests.

If you run tests with gradle, you can add the following to your build.gradle.kts:

You can also use multiple values in this extension, through a map or list of pairs.

These functions will add the keys and values if they're not currently present in the environment, and will override them if they are. Any keys untouched by the function will remain in the environment, and won't be messed with.

Instead of extensions functions, you can also use the provided Listeners to apply these functionalities in a bigger scope. There's an alternative for the Spec/Per test level, and an alternative for the Project Level.

In the same fashion as the Environment Extensions, you can override the System Properties (System.getProperties()):

And with similar Listeners:

Similarly, with System Security Manager you can override the System Security Manager (System.getSecurityManager())

Sometimes you want to test that your code calls System.exit. For that you can use the System Exit Listeners. The Listener will throw an exception when the System.exit is called, allowing you to catch it and verify:

Maybe you want to guarantee that you didn't leave any debug messages around, or that you're always using a Logger in your logging.

For that, Kotest provides you with NoSystemOutListener and NoSystemErrListener. These listeners won't allow any messages to be printed straight to System.out or System.err, respectively:

Some codes use and/or are sensitive to the default Locale and default Timezone. Instead of manipulating the system defaults no your own, let Kotest do it for you!

And with the listeners

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest:kotest-extensions-jvm:${version}
```

Example 2 (kotlin):
```kotlin
withEnvironment("FooKey", "BarValue") {    System.getenv("FooKey") shouldBe "BarValue" // System environment overridden!}
```

Example 3 (kotlin):
```kotlin
tasks.withType<Test>().configureEach {    jvmArgs("--add-opens=java.base/java.util=ALL-UNNAMED")}
```

Example 4 (kotlin):
```kotlin
withEnvironment(mapOf("FooKey" to "BarValue", "BarKey" to "FooValue")) {  // Use FooKey and BarKey}
```

---

## MockServer | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/mockserver.html

**Contents:**
- MockServer
- Dynamic Ports​

Kotest provides an extension for integration with the MockServer library.

Requires the io.kotest:kotest-extensions-mockserver module to be added to your build.

Since Kotest 6.0, all extensions are published under the io.kotest group, with version cadence tied to main Kotest releases.

Mockserver allows us to define an in process HTTP server which is hard coded for routes that we want to test against.

To use in Kotest, we install an instance of MockServerExtension in the spec under test, and Kotest will control the lifecycle automatically.

Then it is a matter of using MockServerClient to wire in our responses.

In the above example, we are of course just testing the mock itself, but it shows how a real test could be configured. For example, you may have an API client that you want to test, so you would configure the API routes using mock server, and then invoke methods on your API client, ensuring it handles the responses correctly.

When using the MockServerExtension, you can specify one or more ports if you wish to hardcore them. Otherwise, you can not specify them at all, and Kotest will automatically allocate a free port for the server to run on. Then, you can use the returned server instance from the install function to retrieve the allocated port.

Here is an example of using dynamic ports:

**Examples:**

Example 1 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {      // this attaches the server to the lifeycle of the spec      install(MockServerExtension(1080))      // we can use the client to create routes. Here we are setting them up      // before each test by using the beforeTest callback.      beforeTest {         MockServerClient("localhost", 1080).`when`(            HttpRequest.request()               .withMethod("POST")               .withPath("/login")               .withHeader("Content-Type", "application/json")               .withBody("""{"username": "foo", "password": "bar"}""")         ).respond(            HttpResponse.response()               .withStatusCode(202)               .withHeader("X-Test", "foo")         )      }      // this test will confirm the endpoint works      test("login endpoint should accept username and password json") {         // using the ktor client to send requests         val client = HttpClient(CIO)         val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:1080/login") {            contentType(ContentType.Application.Json)            body = """{"username": "foo", "password": "bar"}"""         }         // these handy matchers come from the kotest-assertions-ktor module         resp.shouldHaveStatus(HttpStatusCode.Accepted)         resp.shouldHaveHeader("X-Test", "foo")      }  }}
```

Example 2 (kotlin):
```kotlin
class MyMockServerTest : FunSpec() {  init {    val server = install(MockServerExtension())    beforeTest {      MockServerClient("localhost", server.port).`when`(        HttpRequest.request()          .withMethod("GET")          .withPath("/v")      ).respond(        HttpResponse.response()          .withStatusCode(200)      )    }    test("test /health returns 200") {      val client = HttpClient(CIO)      val resp = client.post<io.ktor.client.statement.HttpResponse>("http://localhost:${healthcheck.port}/health")      resp.shouldHaveStatus(HttpStatusCode.OK)    }  }}
```

---

## Embedded Kafka Extension | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/embedded-kafka.html

**Contents:**
- Embedded Kafka Extension
  - Getting started:​
  - Consumer / Producer​
  - Custom Ports​

Kotest offers an extension that spins up an embedded Kafka instance. This can help in situations where using the kafka docker images are an issue.

To use this extension add the io.kotest.extensions:kotest-extensions-embedded-kafka module to your test compile path.

Register the embeddedKafkaListener listener in your test class:

And the broker will be started once the spec is created and stopped once the spec completes.

Note: The underlying embedded kafka library uses a global object for state. Do not start multiple kafka instances at the same time.

To create a consumer and producer we can use convenience methods on the listener:

The stringStringProducer and stringStringConsumer methods return a producer / consumer that accept strings for the keys and values. Similar methods exist for byte pairs.

Alternatively, you can access the host/port the Kafka instance was deployed on and create the clients yourself:

You can create a new instance of the listener specifying a port and then use that instance rather than the default instance.

You can also do specify the zookeeper port using an alternative overload.

**Examples:**

Example 1 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({  listener(embeddedKafkaListener)})
```

Example 2 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec() {  init {    listener(embeddedKafkaListener)  }}
```

Example 3 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)   test("send / receive") {     val producer = embeddedKafkaListener.stringStringProducer()     producer.send(ProducerRecord("foo", "a"))     producer.close()     val consumer = embeddedKafkaListener.stringStringConsumer("foo")     eventually(10.seconds) {       consumer.poll(1000).first().value() shouldBe "a"     }     consumer.close()   }})
```

Example 4 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)      val props = Properties().apply {      put(CommonClientConfigs.BOOTSTRAP_SERVERS_CONFIG, "${embeddedKafkaListener.host}:${embeddedKafkaListener.port}")   }      val producer = KafkaProducer<String, String>(props)   }
```

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Additionally, prevent Gradle from generating its own html reports by adding html.required.set(false) to the test task.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override fun extensions(): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

Example 2 (css):
```css
tasks.test {  useJUnitPlatform()  reports {    html.required.set(false)    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Spring | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/spring.html

**Contents:**
- Spring
  - Constructor Injection​
  - TestContexts​
  - Test Method Callbacks​
  - Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Since Kotest 6.0, all extensions are published under the io.kotest group once again, with version cadence tied to main Kotest releases.

The Spring extension requires you to activate it for all test classes, or per test class. To activate it globally, register the SpringExtension in project config:

To activate it per test class:

In order to let Spring know which configuration class to use, you must annotate your Spec classes with @ContextConfiguration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

For constructor injection, Kotest automatically registers a SpringAutowireConstructorExtension when the spring module is added to the build, assuming auto scan is enabled (see Project Config). If Auto scan is disabled, you will need to manually load the extension in your Project config.

This extension will intercept each call to create a Spec instance and will autowire the beans declared in the primary constructor.

The following example is a test class which requires a service called UserService in its primary constructor. This service class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various test callbacks such as beforeTestMethod that are based around the idea that tests are methods. This assumption is fine for legacy test frameworks like JUnit but not applicable to modern test frameworks like Kotest where tests are functions.

Therefore, when using a spec style that is nested, you can customize when the test method callbacks are fired. By default, this is on the leaf node. You can set these to fire on the root nodes by passing a SpringTestLifecycleMode argument to the extension:

When using a final class, you may receive a warning from Kotest:

Using SpringListener on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
package io.kotest.providedimport io.kotest.core.config.AbstractProjectConfigimport io.kotest.extensions.spring.SpringExtensionclass ProjectConfig : AbstractProjectConfig() {   override val extensions = listOf(SpringExtension())}
```

Example 2 (kotlin):
```kotlin
import io.kotest.core.extensions.ApplyExtensionimport io.kotest.extensions.spring.SpringExtension@ApplyExtension(SpringExtension::class)class MyTestSpec : FunSpec() {}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {         println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/6.0/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Since Kotest 5.6.0, Current instant listeners are located in the artifact io.kotest:kotest-extensions-now:${kotest-version}.

Add it as a dependency to use any of the functionality mentioned below.

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## WireMock | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/wiremock.html

**Contents:**
- WireMock
- WireMock​

WireMock is a library which provides HTTP response stubbing, matchable on URL, header and body content patterns etc.

Kotest provides a module kotest-extensions-wiremock for integration with wiremock.

To begin, add the following dependency to your build:

Having this dependency in the classpath brings WireMockListener into scope. WireMockListener manages the lifecycle of a WireMockServer during your test.

In above example we created an instance of WireMockListener which starts a WireMockServer before running the tests in the spec and stops it after completing all the tests in the spec.

You can use WireMockServer.perSpec(customerServiceServer) to achieve same result.

In above example we created an instance of WireMockListener which starts a WireMockServer before running every test in the spec and stops it after completing every test in the spec. You can use WireMockServer.perTest(customerServiceServer) to achieve same result.

**Examples:**

Example 1 (json):
```json
io.kotest.extensions:kotest-extensions-wiremock:{version}
```

Example 2 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_SPEC))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }    //  ------------OTHER TEST BELOW ----------------})
```

Example 3 (kotlin):
```kotlin
class SomeTest : FunSpec({  val customerServiceServer = WireMockServer(9000)  listener(WireMockListener(customerServiceServer, ListenerMode.PER_TEST))  test("let me get customer information") {    customerServiceServer.stubFor(      WireMock.get(WireMock.urlEqualTo("/customers/123"))        .willReturn(WireMock.ok())    )    val connection = URL("http://localhost:9000/customers/123").openConnection() as HttpURLConnection    connection.responseCode shouldBe 200  }  //  ------------OTHER TEST BELOW ----------------})
```

---

## HTML Reporter | Kotest

**URL:** https://kotest.io/docs/extensions/html_reporter.html

**Contents:**
- HTML Reporter

When using JUnit XML, we can generate XML results from tests that are able to produce output with nested tests. Unfortunately, Gradle generates its HTML reports with the results it has in-memory, which doesn't support nested tests, and it doesn't seem to be able to fetch results from a different XML.

To solve this, Kotest has a listener that is able to generate HTML reports based on the XML reports that are generated by JUnit XML.

The following module is needed: io.kotest:kotest-extensions-htmlreporter in your build. Search maven central for latest version here.

In order to use it, we simply need to add it as a listener through project config.

Additionally, prevent Gradle from generating its own html reports by adding html.required.set(false) to the test task.

Notice that we also add JunitXmlReporter. This will generate the necessary XML reports, used to generate the HTML reports. There's no additional configuration needed, it should simply start generating HTML reports.

By default, it stores reports in path/to/buildDir/reports/tests/test but this can be modified by changing the parameter outputDir.

**Examples:**

Example 1 (swift):
```swift
class ProjectConfig : AbstractProjectConfig() {   override val specExecutionOrder = SpecExecutionOrder.Annotated    override val extensions): List<Extension> = listOf(        JunitXmlReporter(            includeContainers = false,            useTestPathAsName = true,        ),        HtmlReporter()    )}
```

Example 2 (css):
```css
tasks.test {  useJUnitPlatform()  reports {    html.required.set(false)    junitXml.required.set(false)  }  systemProperty("gradle.build.dir", project.buildDir)}
```

---

## Extensions | Kotest

**URL:** https://kotest.io/docs/5.8.x/extensions/extensions.html

**Contents:**
- Extensions
  - Kotest Team Extensions​
  - Third Party Extensions​

Kotest integrates with many other libraries and frameworks. Some are provided by the Kotest team, and others are maintained and hosted by third parties.

---

## Koin | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/koin.html

**Contents:**
- Koin
- Koin​

The Koin DI Framework can be used with Kotest through the KoinExtension extension.

To use the extension in your project, add the dependency to your project:

With the dependency added, we can easily use Koin in our tests!

By default, the extension will start/stop the Koin context between leaf tests. If you are using a nested spec style (like DescribeSpec) and instead want the Koin context to persist over all leafs of a root tests (for example to share mocked declarations between tests), you can specify the lifecycle mode as KoinLifecycleMode.Root in the KoinExtension constructor.

**Examples:**

Example 1 (kotlin):
```kotlin
io.kotest.extensions:kotest-extensions-koin:${version}
```

Example 2 (kotlin):
```kotlin
class KotestAndKoin : FunSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(myKoinModule))    val userService by inject<UserService>()    init {        test("use userService") {            userService.getUser().username shouldBe "LeoColman"        }    }}
```

Example 3 (kotlin):
```kotlin
class KotestAndKoin : DescribeSpec(), KoinTest {    override fun extensions() = listOf(KoinExtension(module = myKoinModule, mode = KoinLifecycleMode.Root))    val userService by inject<UserService>()    init {        describe("use userService") {            it("inside a leaf test") {                userService.getUser().username shouldBe "LeoColman"            }            it("this shares the same context") {                userService.getUser().username shouldBe "LeoColman"            }        }    }}
```

---

## Spring | Kotest

**URL:** https://kotest.io/docs/5.5.x/extensions/spring.html

**Contents:**
- Spring
  - Constructor Injection​
  - TestContexts​
  - Test Method Callbacks​
  - Final Classes​

Kotest offers a Spring extension that allows you to test code that uses the Spring framework for dependency injection.

If you prefer to see an example rather than read docs, then there is a sample project using spring webflux here

In order to use this extension, you need to add io.kotest.extensions:kotest-extensions-spring module to your test compile path. The latest version can always be found on maven central here.

Note: The maven group id differs from the core test framework (io.kotest.extensions).

The Spring extension requires you to activate it for all test classes, or per test class. To activate it globally, register the SpringExtension in project config:

To activate it per test class:

In order to let Spring know which configuration class to use, you must annotate your Spec classes with @ContextConfiguration. This should point to a class annotated with the Spring @Configuration annotation. Alternatively, you can use @ActiveProfiles to point to a specific application context file.

In Kotest 4.3 and earlier, the Spring extension was called SpringListener. This extension has now been deprecated in favour of SpringExtension. The usage is the same, but the SpringExtension has more functionality.

when the spring module is added to the build, assuming auto scan is enabled (see Project Config). If Auto scan is disabled, you will need to manually load the extension in your Project config.

This extension will intercept each call to create a Spec instance and will autowire the beans declared in the primary constructor.

The following example is a test class which requires a service called UserService in its primary constructor. This service class is just a regular spring bean which has been annotated with @Component.

The Spring extensions makes available the TestContextManager via the coroutine context that tests execute in. You can gain a handle to this instance through the testContextManager() extension method.

From this you can get the testContext that Spring is using.

Spring has various test callbacks such as beforeTestMethod that are based around the idea that tests are methods. This assumption is fine for legacy test frameworks like JUnit but not applicable to modern test frameworks like Kotest where tests are functions.

Therefore, when using a spec style that is nested, you can customize when the test method callbacks are fired. By default, this is on the leaf node. You can set these to fire on the root nodes by passing a SpringTestLifecycleMode argument to the extension:

When using a final class, you may receive a warning from Kotest:

Using SpringListener on a final class. If any Spring annotation fails to work, try making this class open

If you wish, you can disable this warning by setting the system property kotest.listener.spring.ignore.warning to true.

**Examples:**

Example 1 (kotlin):
```kotlin
class ProjectConfig : AbstractProjectConfig() {   override fun extensions() = listOf(SpringExtension)}
```

Example 2 (kotlin):
```kotlin
class MyTestSpec : FunSpec() {   override fun extensions() = listOf(SpringExtension)}
```

Example 3 (kotlin):
```kotlin
@ContextConfiguration(classes = [(Components::class)])class SpringAutowiredConstructorTest(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "have autowired the service" {        service.repository.findUser().name shouldBe "system_user"      }    }  }}
```

Example 4 (kotlin):
```kotlin
class MySpec(service: UserService) : WordSpec() {  init {    "SpringExtension" should {      "provide the test context manager" {         println("The context is " + testContextManager().testContext)      }    }  }}
```

---

## Embedded Kafka Extension | Kotest

**URL:** https://kotest.io/docs/5.4.x/extensions/embedded-kafka.html

**Contents:**
- Embedded Kafka Extension
  - Getting started:​
  - Consumer / Producer​
  - Custom Ports​

Kotest offers an extension that spins up an embedded Kafka instance. This can help in situations where using the kafka docker images are an issue.

To use this extension add the io.kotest.extensions:kotest-extensions-embedded-kafka module to your test compile path.

Register the embeddedKafkaListener listener in your test class:

And the broker will be started once the spec is created and stopped once the spec completes.

Note: The underlying embedded kafka library uses a global object for state. Do not start multiple kafka instances at the same time.

To create a consumer and producer we can use convenience methods on the listener:

The stringStringProducer and stringStringConsumer methods return a producer / consumer that accept strings for the keys and values. Similar methods exist for byte pairs.

Alternatively, you can access the host/port the Kafka instance was deployed on and create the clients yourself:

You can create a new instance of the listener specifying a port and then use that instance rather than the default instance.

You can also do specify the zookeeper port using an alternative overload.

**Examples:**

Example 1 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({  listener(embeddedKafkaListener)})
```

Example 2 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec() {  init {    listener(embeddedKafkaListener)  }}
```

Example 3 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)   test("send / receive") {     val producer = embeddedKafkaListener.stringStringProducer()     producer.send(ProducerRecord("foo", "a"))     producer.close()     val consumer = embeddedKafkaListener.stringStringConsumer("foo")     eventually(10.seconds) {       consumer.poll(1000).first().value() shouldBe "a"     }     consumer.close()   }})
```

Example 4 (kotlin):
```kotlin
class EmbeddedKafkaListenerTest : FunSpec({   listener(embeddedKafkaListener)      val props = Properties().apply {      put(CommonClientConfigs.BOOTSTRAP_SERVERS_CONFIG, "${embeddedKafkaListener.host}:${embeddedKafkaListener.port}")   }      val producer = KafkaProducer<String, String>(props)   }
```

---

## Current Instant Listeners | Kotest

**URL:** https://kotest.io/docs/5.7.x/extensions/instant.html

**Contents:**
- Current Instant Listeners
  - Current instant listeners​

Since Kotest 5.6.0, Current instant listeners are located in the artifact io.kotest:kotest-extensions-now:${kotest-version}.

Add it as a dependency to use any of the functionality mentioned below.

Sometimes you may want to use the now static functions located in java.time classes for multiple reasons, such as setting the creation date of an entity

data class MyEntity(creationDate: LocalDateTime = LocalDateTime.now()).

But what to do when you want to test that value? now will be different each time you call it!

For that, Kotest provides ConstantNowListener and withConstantNow functions.

While executing your code, your now will always be the value that you want to test against.

Or, with a listener for all the tests:

withContantNow and ConstantNowTestListener are very sensitive to race conditions. Using them, mocks the static method now which is global to the whole JVM instance, if you're using it while running test in parallel, the results may be inconsistent.

**Examples:**

Example 1 (kotlin):
```kotlin
val foreverNow = LocalDateTime.now()withConstantNow(foreverNow) {  LocalDateTime.now() shouldBe foreverNow  delay(10) // Code is taking a small amount of time to execute, but `now` changed!  LocalDateTime.now() shouldBe foreverNow}
```

Example 2 (kotlin):
```kotlin
override fun listeners() = listOf(    ConstantNowTestListener(foreverNow)  )
```

---

## Ktor | Kotest

**URL:** https://kotest.io/docs/5.2.x/extensions/ktor.html

**Contents:**
- Ktor

The kotest-assertions-ktor module provides response matchers for a Ktor application. There are matchers for both TestApplicationResponse if you are using the server side test support, and for HttpResponse if you are using the ktor HTTP client.

To add Ktor matchers, add the following dependency to your project

An example of using the matchers with the server side test support:

And an example of using the client support:

**Examples:**

Example 1 (bash):
```bash
io.kotest.extensions:kotest-assertions-ktor:${version}
```

Example 2 (kotlin):
```kotlin
withTestApplication({ module(testing = true) }) {   handleRequest(HttpMethod.Get, "/").apply {      response shouldHaveStatus HttpStatusCode.OK      response shouldNotHaveContent "failure"      response.shouldHaveHeader(name = "Authorization", value = "Bearer")      response.shouldNotHaveCookie(name = "Set-Cookie", cookieValue = "id=1234")   }}
```

Example 3 (kotlin):
```kotlin
val client = HttpClient(CIO)val response = client.post("http://mydomain.com/foo")response.shouldHaveStatus(HttpStatusCode.OK)response.shouldHaveHeader(name = "Authorization", value = "Bearer")
```

---
