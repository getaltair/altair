plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.kotlinxSerialization)
    alias(libs.plugins.ksp)
    alias(libs.plugins.mokkery)
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
        }

        commonTest.dependencies {
            implementation(libs.kotlin.test)
            implementation(libs.turbine)
        }

        jvmMain.dependencies {
            // JVM-specific dependencies (server + desktop)
        }

        androidMain.dependencies {
            // Android-specific dependencies
        }

        iosMain.dependencies {
            // iOS-specific dependencies
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
