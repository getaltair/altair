plugins {
    // this is necessary to avoid the plugins to be loaded multiple times
    // in each subproject's classloader
    alias(libs.plugins.androidApplication) apply false
    alias(libs.plugins.androidLibrary) apply false
    alias(libs.plugins.composeHotReload) apply false
    alias(libs.plugins.composeMultiplatform) apply false
    alias(libs.plugins.composeCompiler) apply false
    alias(libs.plugins.kotlinMultiplatform) apply false
    alias(libs.plugins.detekt)
    alias(libs.plugins.spotless)
}

val ktlintVersion = "1.5.0"
val composeRulesVersion = "0.4.22"

spotless {
    kotlin {
        target("**/*.kt")
        targetExclude("**/build/**")
        ktlint(ktlintVersion)
            .customRuleSets(listOf("io.nlopez.compose.rules:ktlint:$composeRulesVersion"))
            .editorConfigOverride(
                mapOf(
                    "ktlint_standard_max-line-length" to "disabled",
                    "compose_allowed_composition_locals" to "LocalAltairColors",
                ),
            )
    }
    kotlinGradle {
        target("**/*.gradle.kts")
        targetExclude("**/build/**")
        ktlint(ktlintVersion)
    }
}

detekt {
    buildUponDefaultConfig = true
    allRules = false
    config.setFrom("$rootDir/config/detekt/detekt.yml")
    baseline = file("$rootDir/config/detekt/baseline.xml")
    parallel = true
    source.setFrom(
        fileTree(rootDir) {
            include("**/*.kt")
            exclude("**/build/**")
        },
    )
}

dependencies {
    detektPlugins("io.nlopez.compose.rules:detekt:$composeRulesVersion")
}

tasks.withType<io.gitlab.arturbosch.detekt.Detekt>().configureEach {
    reports {
        html.required.set(true)
        xml.required.set(true)
        sarif.required.set(true)
    }
}
