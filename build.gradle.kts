import io.gitlab.arturbosch.detekt.Detekt
import io.gitlab.arturbosch.detekt.DetektCreateBaselineTask

plugins {
    // this is necessary to avoid the plugins to be loaded multiple times
    // in each subproject's classloader
    alias(libs.plugins.androidApplication) apply false
    alias(libs.plugins.androidLibrary) apply false
    alias(libs.plugins.androidKotlinMultiplatformLibrary) apply false
    alias(libs.plugins.composeHotReload) apply false
    alias(libs.plugins.composeMultiplatform) apply false
    alias(libs.plugins.composeCompiler) apply false
    alias(libs.plugins.kotlinAndroid) apply false
    alias(libs.plugins.kotlinJvm) apply false
    alias(libs.plugins.kotlinMultiplatform) apply false
    alias(libs.plugins.ktor) apply false
    alias(libs.plugins.detekt)
    alias(libs.plugins.spotless)
}

// Detekt configuration
detekt {
    buildUponDefaultConfig = true
    allRules = false
    config.setFrom("$projectDir/config/detekt/detekt.yml")
    baseline = file("$projectDir/config/detekt/baseline.xml")
    parallel = true
    autoCorrect = true
}

tasks.withType<Detekt>().configureEach {
    jvmTarget = "17"
    reports {
        html.required.set(true)
        xml.required.set(true)
        sarif.required.set(true)
        md.required.set(true)
    }
}

tasks.withType<DetektCreateBaselineTask>().configureEach {
    jvmTarget = "17"
}

// Spotless configuration for ktlint and Prettier
spotless {
    kotlin {
        target("**/*.kt")
        targetExclude("**/build/**")
        ktlint(libs.versions.ktlint.get())
            .customRuleSets(
                listOf(
                    "io.nlopez.compose.rules:ktlint:0.4.28",
                ),
            ).editorConfigOverride(
                mapOf(
                    "ktlint_code_style" to "intellij_idea",
                    "max_line_length" to "120",
                    // Allow PascalCase for @Composable functions
                    "ktlint_function_naming_ignore_when_annotated_with" to "Composable",
                    // Compose rules configuration
                    "compose_allowed_composition_locals" to
                        "LocalAltairColors,LocalAltairTypography,LocalAltairSpacing,LocalAltairShapes",
                    // Enforce preview naming convention (suffix pattern like "FooPreview")
                    "compose_preview_naming_enabled" to "true",
                    "compose_preview_naming_strategy" to "suffix",
                ),
            )
    }

    kotlinGradle {
        target("**/*.gradle.kts")
        targetExclude("**/build/**")
        ktlint(libs.versions.ktlint.get())
    }

    format("markdown") {
        target("**/*.md")
        targetExclude(
            ".claude/**",
            ".moai/**",
            "**/build/**",
        )
        prettier()
            .config(
                mapOf(
                    "printWidth" to 100,
                    "proseWrap" to "always",
                    "tabWidth" to 2,
                ),
            )
    }
}
