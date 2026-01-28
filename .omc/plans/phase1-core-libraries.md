# Phase 1: Core Libraries Implementation Plan

| Field | Value |
|-------|-------|
| **Plan Name** | phase1-core-libraries |
| **Created** | 2026-01-26 |
| **Phase** | 1 of 15 |
| **Reference** | ADR-009 (Core Library Stack) |

---

## Context

### Original Request
Implement Phase 1: Core Libraries of the Altair project, establishing dependency injection (Koin), navigation (Decompose), and error handling (Arrow) foundations across all KMP targets.

### Current State
- Phase 0 (Foundation) is complete
- Basic KMP project structure exists with:
  - `androidApp` - Android application shell
  - `composeApp` - KMP Compose client (Android library, iOS, Desktop/JVM)
  - `server` - Ktor backend (JVM only)
  - `shared` - KMP shared domain models
- Current dependencies: Compose Multiplatform 1.10.0, Kotlin 2.3.0, Ktor 3.4.0
- No DI, navigation, or error handling libraries currently configured

### Research Findings
From ADR-009:
- **Koin 4.x**: Runtime DI, ~0.25ms startup overhead, no codegen, 14M monthly downloads
- **Decompose 3.x**: 0 open issues, UI-agnostic, best-in-class back handling
- **Arrow 2.x**: Typed errors via Either, arrow-optics for nested state updates, KSP required
- **Mokkery 3.x**: Compiler plugin mocking, works on all KMP targets including iOS
- **Turbine 1.2+**: Flow testing assertions

---

## Work Objectives

### Core Objective
Establish foundational DI, navigation, and error handling patterns that all subsequent phases will build upon.

### Deliverables
1. Updated `gradle/libs.versions.toml` with all core library dependencies
2. Koin DI wired into composeApp with platform-specific initialization for Android, iOS, and Desktop
3. Decompose RootComponent with Config sealed class for navigation structure
4. Arrow added to shared module with KSP for optics generation
5. Test infrastructure with Mokkery and Turbine
6. All targets building successfully (Android, iOS, Desktop, JVM)

### Definition of Done
- [ ] `./gradlew build` succeeds for all modules
- [ ] `./gradlew :composeApp:assembleDebug` succeeds
- [ ] `./gradlew :composeApp:run` launches desktop app with Koin-injected components
- [ ] iOS framework builds via Xcode
- [ ] Koin module verification test passes
- [ ] At least one Arrow Either usage example in shared module
- [ ] Decompose navigation works with at least 2 screens (Home, Settings placeholder)

---

## Guardrails

### Must Have
- Koin 4.x with `koin-compose` and `koin-core-viewmodel`
- Decompose 3.x with `decompose` and `decompose-extensions-compose`
- Arrow 2.x with `arrow-core` and `arrow-optics` (KSP configured)
- Essenty lifecycle for Decompose LifecycleRegistry
- Mokkery for commonTest
- Turbine for Flow testing
- Platform-specific Koin initialization (Android Context, iOS/Desktop no-arg)

### Must NOT Have
- Runtime reflection-based mocking (no MockK in iOS targets)
- Navigation state persisted to disk (not needed yet)
- Complex screen implementations (placeholders only)
- Server module changes (Phase 2 handles API layer)

---

## Task Flow and Dependencies

```
Task 1: Update Version Catalog
    |
    v
Task 2: Configure KSP for Arrow Optics
    |
    +---> Task 3: Add Arrow to shared module (depends on KSP)
    |
    v
Task 4: Add Koin to composeApp (can parallel with Task 3)
    |
    v
Task 5: Add Decompose to composeApp (depends on Koin for DI)
    |
    v
Task 6: Wire navigation into App.kt (depends on Decompose)
    |
    +---> Task 7: Add test dependencies (can parallel with Task 6)
    |
    v
Task 8: Verify all targets build
    |
    v
Task 9: Create verification tests
```

---

## Detailed TODOs

### Task 1: Update Version Catalog

**File:** `gradle/libs.versions.toml`

**Changes:**

Add to `[versions]`:
```toml
koin = "4.1.0"
decompose = "3.3.0"
essenty = "2.4.0"
arrow = "2.1.0"
mokkery = "3.0.0"
turbine = "1.2.0"
ksp = "2.3.0-1.0.30"
```

Add to `[libraries]`:
```toml
# Koin
koin-core = { module = "io.insert-koin:koin-core", version.ref = "koin" }
koin-compose = { module = "io.insert-koin:koin-compose", version.ref = "koin" }
koin-compose-viewmodel = { module = "io.insert-koin:koin-compose-viewmodel", version.ref = "koin" }
koin-android = { module = "io.insert-koin:koin-android", version.ref = "koin" }
koin-test = { module = "io.insert-koin:koin-test", version.ref = "koin" }

# Decompose
decompose = { module = "com.arkivanov.decompose:decompose", version.ref = "decompose" }
decompose-compose = { module = "com.arkivanov.decompose:extensions-compose", version.ref = "decompose" }

# Essenty (required for Decompose LifecycleRegistry)
essenty-lifecycle = { module = "com.arkivanov.essenty:lifecycle", version.ref = "essenty" }

# Arrow
arrow-core = { module = "io.arrow-kt:arrow-core", version.ref = "arrow" }
arrow-optics = { module = "io.arrow-kt:arrow-optics", version.ref = "arrow" }
arrow-optics-ksp = { module = "io.arrow-kt:arrow-optics-ksp-plugin", version.ref = "arrow" }

# Testing
mokkery-gradle = { module = "dev.mokkery:mokkery-gradle", version.ref = "mokkery" }
turbine = { module = "app.cash.turbine:turbine", version.ref = "turbine" }
```

Add to `[plugins]`:
```toml
ksp = { id = "com.google.devtools.ksp", version.ref = "ksp" }
mokkery = { id = "dev.mokkery", version.ref = "mokkery" }
```

**Acceptance Criteria:**
- [ ] All version references resolve correctly
- [ ] `./gradlew dependencies` shows new libraries

---

### Task 2: Configure KSP for Arrow Optics

**File:** `build.gradle.kts` (root)

**Changes:**

Add KSP plugin to root:
```kotlin
alias(libs.plugins.ksp) apply false
```

**File:** `shared/build.gradle.kts`

**Changes:**

Add plugins:
```kotlin
plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.kotlinxSerialization)
    alias(libs.plugins.ksp)
}
```

Add KSP configuration for all targets defined in the shared module:
```kotlin
dependencies {
    add("kspCommonMainMetadata", libs.arrow.optics.ksp)
    add("kspAndroid", libs.arrow.optics.ksp)
    add("kspIosX64", libs.arrow.optics.ksp)
    add("kspIosArm64", libs.arrow.optics.ksp)
    add("kspIosSimulatorArm64", libs.arrow.optics.ksp)
    add("kspJvm", libs.arrow.optics.ksp)
}

// KSP generated sources
kotlin.sourceSets.commonMain {
    kotlin.srcDir("build/generated/ksp/metadata/commonMain/kotlin")
}
```

**NOTE:** The shared module defines all three iOS targets (iosX64, iosArm64, iosSimulatorArm64), so KSP must be configured for all of them. The composeApp module only defines iosArm64 and iosSimulatorArm64, but that does not affect the shared module's KSP configuration.

**Acceptance Criteria:**
- [ ] KSP plugin applies without errors
- [ ] Generated optics code appears in build/generated/ksp

---

### Task 3: Add Arrow to Shared Module

**File:** `shared/build.gradle.kts`

**Changes to sourceSets:**
```kotlin
sourceSets {
    commonMain.dependencies {
        implementation(libs.kotlinx.serialization.json)
        implementation(libs.kotlinx.coroutines.core)
        implementation(libs.kotlinx.datetime)
        // Arrow
        implementation(libs.arrow.core)
        implementation(libs.arrow.optics)
    }

    commonTest.dependencies {
        implementation(libs.kotlin.test)
        implementation(libs.turbine)
    }
    // ... rest unchanged
}
```

**New File:** `shared/src/commonMain/kotlin/com/getaltair/altair/shared/core/error/DomainError.kt`

```kotlin
package com.getaltair.altair.shared.core.error

import arrow.core.Either
import arrow.core.left
import arrow.core.right

/**
 * Base sealed interface for all domain errors in Altair.
 * Provides typed error handling via Arrow Either.
 */
sealed interface DomainError {
    val message: String
}

/**
 * Errors related to Quest operations.
 */
sealed interface QuestError : DomainError {
    data object WipLimitExceeded : QuestError {
        override val message = "Cannot start quest: WIP limit of 1 already reached"
    }

    data class NotFound(val id: String) : QuestError {
        override val message = "Quest not found: $id"
    }

    data class ValidationFailed(override val message: String) : QuestError
}

/**
 * Errors related to Note operations.
 */
sealed interface NoteError : DomainError {
    data class NotFound(val id: String) : NoteError {
        override val message = "Note not found: $id"
    }
}

/**
 * Errors related to Item (inventory) operations.
 */
sealed interface ItemError : DomainError {
    data class NotFound(val id: String) : ItemError {
        override val message = "Item not found: $id"
    }
}

/**
 * Type alias for domain operations that can fail.
 */
typealias DomainResult<E, T> = Either<E, T> where E : DomainError

/**
 * Extension to convert nullable to Either with error.
 */
inline fun <E : DomainError, T> T?.toEither(error: () -> E): Either<E, T> =
    this?.right() ?: error().left()
```

**New File:** `shared/src/commonMain/kotlin/com/getaltair/altair/shared/core/state/AppState.kt`

```kotlin
package com.getaltair.altair.shared.core.state

import arrow.optics.optics

/**
 * Root application state with Arrow Optics for nested updates.
 * This demonstrates optics usage - actual state will expand in later phases.
 */
@optics
data class AppState(
    val isLoading: Boolean = false,
    val currentUser: UserState? = null,
    val guidance: GuidanceState = GuidanceState()
) {
    companion object
}

@optics
data class UserState(
    val id: String,
    val displayName: String
) {
    companion object
}

@optics
data class GuidanceState(
    val activeQuestId: String? = null,
    val wipCount: Int = 0
) {
    companion object
}
```

**Acceptance Criteria:**
- [ ] Arrow core imports resolve
- [ ] Arrow optics `@optics` annotation generates companion extensions
- [ ] `DomainError` sealed interface compiles
- [ ] `./gradlew :shared:build` succeeds

---

### Task 4: Add Koin to composeApp

**File:** `composeApp/build.gradle.kts`

**Changes:**

Add Mokkery plugin:
```kotlin
plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.composeMultiplatform)
    alias(libs.plugins.composeCompiler)
    alias(libs.plugins.composeHotReload)
    alias(libs.plugins.mokkery)
}
```

Update sourceSets:
```kotlin
sourceSets {
    androidMain.dependencies {
        implementation(libs.compose.uiToolingPreview)
        implementation(libs.androidx.activity.compose)
        implementation(libs.koin.android)
    }
    commonMain.dependencies {
        implementation(libs.compose.runtime)
        implementation(libs.compose.foundation)
        implementation(libs.compose.material3)
        implementation(libs.compose.ui)
        implementation(libs.compose.components.resources)
        implementation(libs.compose.uiToolingPreview)
        implementation(libs.androidx.lifecycle.viewmodelCompose)
        implementation(libs.androidx.lifecycle.runtimeCompose)
        implementation(projects.shared)
        // Koin
        implementation(libs.koin.core)
        implementation(libs.koin.compose)
        implementation(libs.koin.compose.viewmodel)
        // Decompose
        implementation(libs.decompose)
        implementation(libs.decompose.compose)
        // Essenty (for LifecycleRegistry)
        implementation(libs.essenty.lifecycle)
    }
    commonTest.dependencies {
        implementation(libs.kotlin.test)
        implementation(libs.koin.test)
        implementation(libs.turbine)
    }
    jvmMain.dependencies {
        implementation(compose.desktop.currentOs)
        implementation(libs.kotlinx.coroutinesSwing)
    }
}
```

**New File:** `composeApp/src/commonMain/kotlin/com/getaltair/altair/di/AppModule.kt`

```kotlin
package com.getaltair.altair.di

import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * Core application Koin module.
 * Contains all shared dependencies used across platforms.
 */
val appModule: Module = module {
    // Placeholder for repositories (Phase 2+)
    // singleOf(::QuestRepository)
    // singleOf(::NoteRepository)

    // Placeholder for ViewModels (Phase 3+)
    // viewModelOf(::GuidanceViewModel)
}

/**
 * All modules to be loaded by Koin.
 */
val allModules: List<Module> = listOf(
    appModule
)
```

**New File:** `composeApp/src/commonMain/kotlin/com/getaltair/altair/di/KoinInit.kt`

```kotlin
package com.getaltair.altair.di

import org.koin.core.context.startKoin
import org.koin.core.module.Module
import org.koin.dsl.KoinAppDeclaration

/**
 * Common Koin initialization.
 * Platform-specific code calls this with additional configuration.
 */
fun initKoin(
    additionalModules: List<Module> = emptyList(),
    appDeclaration: KoinAppDeclaration = {}
) = startKoin {
    appDeclaration()
    modules(allModules + additionalModules)
}
```

**New File:** `composeApp/src/androidMain/kotlin/com/getaltair/altair/di/KoinInit.android.kt`

```kotlin
package com.getaltair.altair.di

import android.content.Context
import org.koin.android.ext.koin.androidContext
import org.koin.android.ext.koin.androidLogger
import org.koin.core.logger.Level

/**
 * Android-specific Koin initialization with Context.
 */
fun initKoinAndroid(context: Context) = initKoin {
    androidLogger(Level.DEBUG)
    androidContext(context)
}
```

**New File:** `composeApp/src/jvmMain/kotlin/com/getaltair/altair/di/KoinInit.jvm.kt`

```kotlin
package com.getaltair.altair.di

/**
 * Desktop/JVM-specific Koin initialization.
 * No special context needed for desktop.
 */
fun initKoinDesktop() = initKoin()
```

**New File:** `composeApp/src/iosMain/kotlin/com/getaltair/altair/di/KoinInit.ios.kt`

```kotlin
package com.getaltair.altair.di

/**
 * iOS-specific Koin initialization.
 * Called from Swift via MainViewController.
 */
fun initKoinIos() = initKoin()
```

**Acceptance Criteria:**
- [ ] Koin modules compile without errors
- [ ] Platform-specific initialization files exist
- [ ] `./gradlew :composeApp:compileKotlinAndroid` succeeds
- [ ] `./gradlew :composeApp:compileKotlinIosArm64` succeeds
- [ ] `./gradlew :composeApp:compileKotlinJvm` succeeds

---

### Task 5: Add Decompose Navigation

**New File:** `composeApp/src/commonMain/kotlin/com/getaltair/altair/navigation/RootComponent.kt`

```kotlin
package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.arkivanov.decompose.router.stack.ChildStack
import com.arkivanov.decompose.router.stack.StackNavigation
import com.arkivanov.decompose.router.stack.childStack
import com.arkivanov.decompose.router.stack.pop
import com.arkivanov.decompose.router.stack.push
import com.arkivanov.decompose.value.Value
import kotlinx.serialization.Serializable

/**
 * Root navigation component for Altair.
 * Manages the main navigation stack across all platforms.
 */
class RootComponent(
    componentContext: ComponentContext
) : ComponentContext by componentContext {

    private val navigation = StackNavigation<Config>()

    val stack: Value<ChildStack<Config, Child>> = childStack(
        source = navigation,
        serializer = Config.serializer(),
        initialConfiguration = Config.Home,
        handleBackButton = true,
        childFactory = ::createChild
    )

    private fun createChild(config: Config, componentContext: ComponentContext): Child =
        when (config) {
            is Config.Home -> Child.Home(HomeComponent(componentContext, ::onHomeOutput))
            is Config.Settings -> Child.Settings(SettingsComponent(componentContext, ::onSettingsOutput))
        }

    private fun onHomeOutput(output: HomeComponent.Output) {
        when (output) {
            HomeComponent.Output.NavigateToSettings -> navigation.push(Config.Settings)
        }
    }

    private fun onSettingsOutput(output: SettingsComponent.Output) {
        when (output) {
            SettingsComponent.Output.NavigateBack -> navigation.pop()
        }
    }

    /**
     * Navigation configuration - defines all possible destinations.
     * Serializable for state restoration.
     */
    @Serializable
    sealed interface Config {
        @Serializable
        data object Home : Config

        @Serializable
        data object Settings : Config
    }

    /**
     * Child components - the actual screen implementations.
     */
    sealed interface Child {
        data class Home(val component: HomeComponent) : Child
        data class Settings(val component: SettingsComponent) : Child
    }
}
```

**New File:** `composeApp/src/commonMain/kotlin/com/getaltair/altair/navigation/HomeComponent.kt`

```kotlin
package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Home screen component - main entry point of the app.
 * Will contain Guidance module entry in future phases.
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param onOutput Callback for navigation outputs (default no-op for previews)
 */
class HomeComponent(
    componentContext: ComponentContext,
    private val onOutput: (Output) -> Unit = {}
) : ComponentContext by componentContext {

    fun onSettingsClicked() {
        onOutput(Output.NavigateToSettings)
    }

    sealed interface Output {
        data object NavigateToSettings : Output
    }
}
```

**New File:** `composeApp/src/commonMain/kotlin/com/getaltair/altair/navigation/SettingsComponent.kt`

```kotlin
package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Settings screen component.
 * Placeholder for Phase 1 - will expand with actual settings in later phases.
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param onOutput Callback for navigation outputs (default no-op for previews)
 */
class SettingsComponent(
    componentContext: ComponentContext,
    private val onOutput: (Output) -> Unit = {}
) : ComponentContext by componentContext {

    fun onBackClicked() {
        onOutput(Output.NavigateBack)
    }

    sealed interface Output {
        data object NavigateBack : Output
    }
}
```

**Acceptance Criteria:**
- [ ] RootComponent compiles with Config sealed class
- [ ] Child components implement ComponentContext delegation
- [ ] Stack navigation builds without errors
- [ ] Serialization annotations on Config classes work with kotlinx.serialization

---

### Task 6: Wire Navigation into App.kt

**File:** `composeApp/src/commonMain/kotlin/com/getaltair/altair/App.kt`

**Replace entire file:**
```kotlin
package com.getaltair.altair

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.arkivanov.decompose.extensions.compose.stack.Children
import com.arkivanov.decompose.extensions.compose.stack.animation.slide
import com.arkivanov.decompose.extensions.compose.stack.animation.stackAnimation
import com.getaltair.altair.navigation.RootComponent
import com.getaltair.altair.ui.HomeScreen
import com.getaltair.altair.ui.SettingsScreen

/**
 * Main application composable.
 * Receives RootComponent for navigation and renders current screen.
 */
@Composable
fun App(rootComponent: RootComponent) {
    MaterialTheme {
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colorScheme.background
        ) {
            Children(
                stack = rootComponent.stack,
                animation = stackAnimation(slide())
            ) { child ->
                when (val instance = child.instance) {
                    is RootComponent.Child.Home -> HomeScreen(instance.component)
                    is RootComponent.Child.Settings -> SettingsScreen(instance.component)
                }
            }
        }
    }
}
```

**New File:** `composeApp/src/commonMain/kotlin/com/getaltair/altair/ui/HomeScreen.kt`

```kotlin
package com.getaltair.altair.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.getaltair.altair.navigation.HomeComponent

/**
 * Home screen UI.
 * Entry point showing Altair branding and navigation to other areas.
 */
@Composable
fun HomeScreen(component: HomeComponent) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Text(
            text = "Altair",
            style = MaterialTheme.typography.headlineLarge
        )
        Text(
            text = "Life Management Ecosystem",
            style = MaterialTheme.typography.bodyLarge,
            modifier = Modifier.padding(bottom = 32.dp)
        )
        Button(onClick = { component.onSettingsClicked() }) {
            Text("Settings")
        }
    }
}
```

**New File:** `composeApp/src/commonMain/kotlin/com/getaltair/altair/ui/SettingsScreen.kt`

```kotlin
package com.getaltair.altair.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.getaltair.altair.navigation.SettingsComponent

/**
 * Settings screen UI.
 * Placeholder for Phase 1 - will contain actual settings in later phases.
 */
@Composable
fun SettingsScreen(component: SettingsComponent) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Text(
            text = "Settings",
            style = MaterialTheme.typography.headlineLarge,
            modifier = Modifier.padding(bottom = 32.dp)
        )
        Text(
            text = "Settings will be available in a future release.",
            style = MaterialTheme.typography.bodyMedium,
            modifier = Modifier.padding(bottom = 16.dp)
        )
        Button(onClick = { component.onBackClicked() }) {
            Text("Back")
        }
    }
}
```

**File:** `composeApp/src/jvmMain/kotlin/com/getaltair/altair/main.kt`

**Replace entire file:**
```kotlin
package com.getaltair.altair

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.getaltair.altair.di.initKoinDesktop
import com.getaltair.altair.navigation.RootComponent

fun main() {
    // Initialize Koin for desktop
    initKoinDesktop()

    // Create lifecycle for Decompose
    val lifecycle = LifecycleRegistry()

    // Create root component on main thread before application starts
    val rootComponent = RootComponent(
        componentContext = DefaultComponentContext(lifecycle = lifecycle)
    )

    application {
        Window(
            onCloseRequest = ::exitApplication,
            title = "Altair",
        ) {
            App(rootComponent)
        }
    }
}
```

**File:** `composeApp/src/iosMain/kotlin/com/getaltair/altair/MainViewController.kt`

**Replace entire file:**
```kotlin
package com.getaltair.altair

import androidx.compose.ui.window.ComposeUIViewController
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.getaltair.altair.di.initKoinIos
import com.getaltair.altair.navigation.RootComponent
import platform.UIKit.UIViewController

fun MainViewController(): UIViewController {
    // Initialize Koin for iOS
    initKoinIos()

    // Create lifecycle and root component
    val lifecycle = LifecycleRegistry()
    val rootComponent = RootComponent(
        componentContext = DefaultComponentContext(lifecycle = lifecycle)
    )

    return ComposeUIViewController { App(rootComponent) }
}
```

**File:** `androidApp/src/main/kotlin/com/getaltair/altair/MainActivity.kt`

**Replace entire file (NOTE: preserves AppAndroidPreview for Android Studio preview support):**
```kotlin
package com.getaltair.altair

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.decompose.defaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.getaltair.altair.di.initKoinAndroid
import com.getaltair.altair.navigation.HomeComponent
import com.getaltair.altair.navigation.RootComponent
import com.getaltair.altair.ui.HomeScreen

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        // Initialize Koin with Android context (only once)
        if (savedInstanceState == null) {
            initKoinAndroid(applicationContext)
        }

        enableEdgeToEdge()
        super.onCreate(savedInstanceState)

        // Create root component with activity's component context
        val rootComponent = RootComponent(
            componentContext = defaultComponentContext()
        )

        setContent {
            App(rootComponent)
        }
    }
}

/**
 * Preview composable for Android Studio.
 * Shows HomeScreen for preview purposes since App() requires RootComponent.
 * Uses HomeComponent with default no-op onOutput callback.
 */
@Preview
@Composable
fun AppAndroidPreview() {
    // Create a preview-safe ComponentContext using LifecycleRegistry
    val previewContext = DefaultComponentContext(lifecycle = LifecycleRegistry())

    // HomeComponent accepts default no-op onOutput, so we can instantiate directly
    HomeScreen(
        component = HomeComponent(
            componentContext = previewContext,
            onOutput = {} // explicit no-op for clarity
        )
    )
}
```

**NOTE:** The `AppAndroidPreview()` composable is preserved but updated to work with the new navigation architecture. Since `App()` now requires a `RootComponent`, the preview shows `HomeScreen` directly with a `HomeComponent` instance. The `HomeComponent` class now accepts a default no-op lambda for `onOutput`, allowing direct instantiation in previews without requiring an interface.

**File:** `androidApp/build.gradle.kts`

**Update dependencies:**
```kotlin
dependencies {
    implementation(projects.composeApp)
    implementation(libs.androidx.activity.compose)
    implementation(libs.decompose)
    implementation(libs.essenty.lifecycle) // Required for preview ComponentContext
    debugImplementation(libs.compose.uiTooling)
}
```

**Acceptance Criteria:**
- [ ] Desktop app launches with Home screen visible
- [ ] Settings button navigates to Settings screen
- [ ] Back button on Settings returns to Home
- [ ] iOS MainViewController creates RootComponent correctly
- [ ] Android MainActivity initializes Koin with context
- [ ] AppAndroidPreview compiles and shows in Android Studio previews

---

### Task 7: Add Test Dependencies and Verification Tests

**New File:** `composeApp/src/commonTest/kotlin/com/getaltair/altair/di/KoinModuleTest.kt`

```kotlin
package com.getaltair.altair.di

import org.koin.core.context.startKoin
import org.koin.core.context.stopKoin
import org.koin.test.KoinTest
import org.koin.test.check.checkModules
import kotlin.test.AfterTest
import kotlin.test.Test

/**
 * Verifies Koin module configuration is valid.
 * Catches DI wiring issues at test time rather than runtime.
 */
class KoinModuleTest : KoinTest {

    @AfterTest
    fun tearDown() {
        stopKoin()
    }

    @Test
    fun verifyKoinConfiguration() {
        startKoin {
            modules(allModules)
        }
        // checkModules verifies all dependencies can be resolved
        checkModules()
    }
}
```

**New File:** `shared/src/commonTest/kotlin/com/getaltair/altair/shared/core/error/DomainErrorTest.kt`

```kotlin
package com.getaltair.altair.shared.core.error

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Tests for Arrow Either usage with domain errors.
 */
class DomainErrorTest {

    @Test
    fun eitherRightContainsValue() {
        val result: Either<QuestError, String> = "success".right()

        assertTrue(result.isRight())
        assertEquals("success", result.getOrNull())
    }

    @Test
    fun eitherLeftContainsError() {
        val result: Either<QuestError, String> = QuestError.WipLimitExceeded.left()

        assertTrue(result.isLeft())
        result.onLeft { error ->
            assertEquals("Cannot start quest: WIP limit of 1 already reached", error.message)
        }
    }

    @Test
    fun toEitherConvertsNullableCorrectly() {
        val found: String? = "found"
        val notFound: String? = null

        val foundResult = found.toEither { QuestError.NotFound("123") }
        val notFoundResult = notFound.toEither { QuestError.NotFound("456") }

        assertTrue(foundResult.isRight())
        assertTrue(notFoundResult.isLeft())
    }

    @Test
    fun questErrorNotFoundHasCorrectMessage() {
        val error = QuestError.NotFound("quest-123")
        assertEquals("Quest not found: quest-123", error.message)
    }
}
```

**New File:** `shared/src/commonTest/kotlin/com/getaltair/altair/shared/core/state/AppStateTest.kt`

```kotlin
package com.getaltair.altair.shared.core.state

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull

/**
 * Tests for AppState with Arrow Optics.
 * Verifies optics-generated extensions work correctly.
 */
class AppStateTest {

    @Test
    fun appStateDefaultsAreCorrect() {
        val state = AppState()

        assertEquals(false, state.isLoading)
        assertNull(state.currentUser)
        assertEquals(GuidanceState(), state.guidance)
    }

    @Test
    fun guidanceStateDefaultsAreCorrect() {
        val state = GuidanceState()

        assertNull(state.activeQuestId)
        assertEquals(0, state.wipCount)
    }

    @Test
    fun nestedStateCanBeUpdatedWithOptics() {
        val initial = AppState(
            guidance = GuidanceState(wipCount = 0)
        )

        // Using optics-generated lens to modify nested state
        // Path: AppState -> guidance (GuidanceState) -> wipCount (Int)
        val updated = AppState.guidance.wipCount.modify(initial) { it + 1 }

        assertEquals(1, updated.guidance.wipCount)
        // Original unchanged (immutable)
        assertEquals(0, initial.guidance.wipCount)
    }
}
```

**NOTE on optics lens path:** The correct path is `AppState.guidance.wipCount`, NOT `AppState.guidance.guidanceState.wipCount`. Arrow Optics generates:
- `AppState.guidance` - a lens from AppState to GuidanceState
- `GuidanceState.wipCount` - a lens from GuidanceState to Int

These can be composed as `AppState.guidance.wipCount` to get a lens from AppState directly to the nested wipCount field.

**Acceptance Criteria:**
- [ ] `./gradlew :composeApp:testDebugUnitTest` passes Koin verification
- [ ] `./gradlew :shared:test` passes Arrow Either tests
- [ ] `./gradlew :shared:test` passes AppState optics tests

---

### Task 8: Verify All Targets Build

**Commands to run:**
```bash
# Full build
./gradlew build

# Specific target verification
./gradlew :shared:build
./gradlew :composeApp:build
./gradlew :composeApp:assembleDebug
./gradlew :composeApp:linkDebugFrameworkIosArm64
./gradlew :composeApp:run  # Desktop launch test
```

**Acceptance Criteria:**
- [ ] `./gradlew build` completes without errors
- [ ] All KMP targets compile (JVM, Android, iOS arm64, iOS simulator)
- [ ] iOS framework links successfully
- [ ] Desktop app launches and shows Home screen

---

### Task 9: Delete Legacy Files

**Files to remove:**
- `composeApp/src/commonMain/kotlin/com/getaltair/altair/Greeting.kt`
- `composeApp/src/commonMain/kotlin/com/getaltair/altair/Platform.kt`
- `composeApp/src/androidMain/kotlin/com/getaltair/altair/Platform.android.kt`
- `composeApp/src/iosMain/kotlin/com/getaltair/altair/Platform.ios.kt`
- `composeApp/src/jvmMain/kotlin/com/getaltair/altair/Platform.jvm.kt`
- `shared/src/commonMain/kotlin/com/getaltair/altair/shared/Main.kt`
- `shared/src/main.backup/` (entire directory)

**Files to RETAIN:**
- `composeApp/src/commonTest/kotlin/com/getaltair/altair/ComposeAppCommonTest.kt` - This file does NOT reference Greeting (verified: it only contains a simple example test `assertEquals(3, 1 + 2)`) and should be kept.

**Acceptance Criteria:**
- [ ] No references to `Greeting` or `Platform` classes remain
- [ ] `ComposeAppCommonTest.kt` is retained
- [ ] Build still succeeds after cleanup

---

## Commit Strategy

### Commit 1: Add core library dependencies to version catalog
```
feat(deps): add Koin, Decompose, Arrow, Mokkery, Turbine to version catalog

- Koin 4.1.0 for dependency injection
- Decompose 3.3.0 for navigation
- Essenty 2.4.0 for Decompose lifecycle support
- Arrow 2.1.0 for typed error handling
- Mokkery 3.0.0 for KMP mocking
- Turbine 1.2.0 for Flow testing
- KSP 2.3.0-1.0.30 for Arrow optics generation

Ref: ADR-009
```

### Commit 2: Configure Arrow with KSP in shared module
```
feat(shared): add Arrow core and optics with KSP

- Configure KSP plugin for all KMP targets
- Add DomainError sealed interface with typed errors
- Add AppState with @optics annotation
- Add unit tests for Either and optics usage

Ref: ADR-009
```

### Commit 3: Add Koin DI to composeApp
```
feat(compose): add Koin dependency injection

- Add appModule with placeholder dependencies
- Platform-specific initialization:
  - Android: androidContext integration
  - iOS: no-arg initialization
  - Desktop: no-arg initialization
- Add Koin module verification test

Ref: ADR-009
```

### Commit 4: Add Decompose navigation
```
feat(compose): add Decompose navigation with RootComponent

- Create RootComponent with Config sealed class
- Add HomeComponent and SettingsComponent
- Implement stack navigation with slide animation
- Wire navigation into all platform entry points
- Preserve AppAndroidPreview for Android Studio support

Ref: ADR-009
```

### Commit 5: Clean up Phase 0 scaffolding
```
chore: remove Phase 0 placeholder files

- Remove Greeting and Platform classes
- Remove legacy shared/Main.kt
- Clean up backup directory
- Retain ComposeAppCommonTest.kt (no Greeting reference)
```

---

## Success Criteria

| Criterion | Verification Method |
|-----------|---------------------|
| Dependencies resolve | `./gradlew dependencies --configuration commonMainApi` |
| All targets build | `./gradlew build` exits 0 |
| Android assembles | `./gradlew :composeApp:assembleDebug` exits 0 |
| iOS framework links | `./gradlew :composeApp:linkDebugFrameworkIosArm64` exits 0 |
| Desktop runs | `./gradlew :composeApp:run` shows Home screen |
| Tests pass | `./gradlew test` exits 0 |
| Navigation works | Manual: Home -> Settings -> Back works |
| Koin initializes | No runtime DI errors on any platform |
| Arrow optics generate | `AppState.guidance` lens exists in generated code |

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| KSP version incompatibility with Kotlin 2.3.0 | HIGH | Pin to known-good KSP version 2.3.0-1.0.30 |
| Decompose API changes in 3.x | MEDIUM | Lock version, follow migration guide if needed |
| Koin runtime errors | MEDIUM | Add checkModules() test, catch early |
| iOS build fails | MEDIUM | Test iOS target early in implementation |

---

## Notes

- This plan establishes patterns that all subsequent phases will follow
- The navigation structure (RootComponent + Children) scales to support Guidance, Knowledge, and Tracking modules
- Arrow Either pattern should be used for ALL repository operations going forward
- Koin modules will grow per-feature (one module per major feature area)
- Essenty lifecycle library is required for Decompose's LifecycleRegistry usage in JVM/iOS entry points
