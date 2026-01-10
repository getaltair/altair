plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidKotlinMultiplatformLibrary)
    alias(libs.plugins.sqldelight)
    alias(libs.plugins.kotlinxSerialization)
}

kotlin {
    androidLibrary {
        namespace = "com.getaltair.altair.shared"
        compileSdk =
            libs.versions.android.compileSdk
                .get()
                .toInt()
        minSdk =
            libs.versions.android.minSdk
                .get()
                .toInt()
    }

    listOf(
        iosArm64(),
        iosSimulatorArm64(),
    ).forEach { iosTarget ->
        iosTarget.binaries.framework {
            baseName = "shared"
            isStatic = true
            // Export Decompose for iOS
            export(libs.decompose)
            export(libs.essenty.lifecycle)
            export(libs.essenty.statekeeper)
            export(libs.essenty.backhandler)
        }
    }

    jvm()

    sourceSets {
        commonMain.dependencies {
            implementation(libs.sqldelight.runtime)
            implementation(libs.sqldelight.coroutines)
            implementation(libs.kotlinx.coroutinesCore)
            implementation(libs.kotlinx.datetime)

            // Koin DI
            implementation(project.dependencies.platform(libs.koin.bom))
            implementation(libs.koin.core)

            // Decompose navigation (api for consumers to access types)
            api(libs.decompose)
            api(libs.essenty.lifecycle)
            implementation(libs.essenty.lifecycle.coroutines)
            api(libs.essenty.statekeeper)
            api(libs.essenty.instancekeeper)
            api(libs.essenty.backhandler)

            // Arrow error handling
            implementation(project.dependencies.platform(libs.arrow.bom))
            implementation(libs.arrow.core)
            implementation(libs.arrow.fx.coroutines)

            // kotlinx-serialization
            implementation(libs.kotlinx.serialization.json)
        }
        commonTest.dependencies {
            implementation(libs.kotlin.test)
            implementation(libs.kotlinx.coroutinesTest)
        }
        androidMain.dependencies {
            implementation(libs.sqldelight.android.driver)
        }
        iosMain.dependencies {
            implementation(libs.sqldelight.native.driver)
            // Export Decompose for iOS
            api(libs.decompose)
            api(libs.essenty.lifecycle)
            api(libs.essenty.statekeeper)
            api(libs.essenty.backhandler)
        }
        jvmMain.dependencies {
            implementation(libs.surrealdb)
            implementation(libs.ulid.creator)
            implementation(libs.kotlinx.coroutinesCore)
            implementation(libs.sqldelight.sqlite.driver)
        }
        jvmTest.dependencies {
            implementation(libs.kotlin.test)
            implementation(libs.kotlin.testJunit)
            implementation(libs.kotlinx.coroutinesTest)
            implementation(libs.sqldelight.sqlite.driver)
        }
    }
}

sqldelight {
    databases {
        create("AltairDatabase") {
            packageName.set("com.getaltair.altair.database")
        }
    }
}
