plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.kotlinxSerialization)
    alias(libs.plugins.ksp)
    alias(libs.plugins.mokkery)
    alias(libs.plugins.kotlinx.rpc)
    alias(libs.plugins.sqldelight)
}

kotlin {
    // Android target
    androidTarget {
        compilerOptions {
            jvmTarget.set(org.jetbrains.kotlin.gradle.dsl.JvmTarget.JVM_11)
        }
    }

    // iOS targets
    iosX64()
    iosArm64()
    iosSimulatorArm64()

    // JVM target (for server and desktop)
    jvm()

    sourceSets {
        commonMain.dependencies {
            implementation(libs.kotlinx.serialization.json)
            implementation(libs.kotlinx.coroutines.core)
            implementation(libs.kotlinx.datetime)
            // Arrow - Functional Error Handling
            implementation(libs.arrow.core)
            implementation(libs.arrow.optics)
            // Koin - Dependency Injection
            implementation(libs.koin.core)
            // kotlinx-rpc - Type-safe RPC interfaces
            implementation(libs.kotlinx.rpc.core)
            implementation(libs.kotlinx.rpc.krpc.serialization.json)
            // SQLDelight - Type-safe SQL
            implementation(libs.sqldelight.runtime)
            implementation(libs.sqldelight.coroutines)
        }

        commonTest.dependencies {
            implementation(libs.kotlin.test)
            implementation(libs.turbine)
            implementation(libs.kotlinx.coroutines.test)
            // Kotest - Property-based testing
            implementation(libs.kotest.framework.engine)
            implementation(libs.kotest.assertions.core)
            implementation(libs.kotest.property)
        }

        jvmMain.dependencies {
            // JVM-specific dependencies (server + desktop)
            // SurrealDB - Database (ADR-002)
            implementation(libs.surrealdb)
            // SQLDelight JVM driver for testing
            implementation(libs.sqldelight.jvm.driver)
        }

        androidMain.dependencies {
            // Android-specific dependencies
            // Koin Android
            implementation(libs.koin.android)
            // SQLDelight Android driver
            implementation(libs.sqldelight.android.driver)
            // EncryptedSharedPreferences for secure token storage
            implementation(libs.androidx.security.crypto)
        }

        iosMain.dependencies {
            // iOS-specific dependencies
            // SQLDelight Native driver
            implementation(libs.sqldelight.native.driver)
        }
    }
}

// Arrow Optics KSP configuration
dependencies {
    add("kspCommonMainMetadata", libs.arrow.optics.ksp)
}

android {
    namespace = "com.getaltair.altair.shared"
    compileSdk =
        libs.versions.android.compileSdk
            .get()
            .toInt()

    defaultConfig {
        minSdk =
            libs.versions.android.minSdk
                .get()
                .toInt()
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
}

// SQLDelight configuration (ADR-002)
sqldelight {
    databases {
        create("AltairDatabase") {
            packageName.set("com.getaltair.altair.db")
            generateAsync.set(true)
        }
    }
}
