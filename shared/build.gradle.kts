plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.kotlinxSerialization)
    alias(libs.plugins.ksp)
    alias(libs.plugins.sqldelight)
}

kotlin {
    // Android target
    androidTarget {
        compilations.all {
            kotlinOptions {
                jvmTarget = "11"
            }
        }
    }

    // iOS targets
    iosX64()
    iosArm64()
    iosSimulatorArm64()

    // JVM target (for server and desktop)
    jvm()

    sourceSets {
        commonMain {
            dependencies {
                implementation(libs.kotlinx.serialization.json)
                implementation(libs.kotlinx.coroutines.core)
                implementation(libs.kotlinx.datetime)
                api(libs.arrow.core)
                api(libs.arrow.optics)
                implementation(libs.sqldelight.runtime)
                implementation(libs.sqldelight.coroutines.extensions)
            }
            kotlin.srcDir("build/generated/ksp/metadata/commonMain/kotlin")
        }

        commonTest.dependencies {
            implementation(libs.kotlin.test)
            implementation(libs.turbine)
            implementation(libs.kotlinx.datetime)
        }

        jvmMain.dependencies {
            // JVM-specific dependencies (server + desktop)
        }

        androidMain.dependencies {
            // Android-specific dependencies
            implementation(libs.sqldelight.android.driver)
        }

        iosMain.dependencies {
            // iOS-specific dependencies
            implementation(libs.sqldelight.native.driver)
        }
    }
}

android {
    namespace = "com.getaltair.altair.shared"
    compileSdk = libs.versions.android.compileSdk.get().toInt()

    defaultConfig {
        minSdk = libs.versions.android.minSdk.get().toInt()
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
}

dependencies {
    // Only add KSP for commonMain to avoid duplicate generation
    add("kspCommonMainMetadata", libs.arrow.optics.ksp)
}

// Ensure all Kotlin compilation tasks depend on KSP metadata generation
kotlin.targets.configureEach {
    compilations.configureEach {
        compileTaskProvider.configure {
            dependsOn("kspCommonMainKotlinMetadata")
        }
    }
}

sqldelight {
    databases {
        create("AltairDatabase") {
            packageName.set("com.getaltair.altair.shared.database")
            generateAsync.set(true)
        }
    }
}