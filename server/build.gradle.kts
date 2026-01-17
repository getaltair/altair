val kotlinVersion: String by project
val logbackVersion: String by project

plugins {
    kotlin("jvm")
    id("io.ktor.plugin") version "3.3.3"
    id("org.jetbrains.kotlin.plugin.serialization") version "2.3.0"
}

group = "com.getaltair"
version = "0.0.1"

application {
    mainClass = "io.ktor.server.netty.EngineMain"
}

dependencies {
    implementation(libs.ktor.server.core)
    implementation(libs.ktor.server.content.negotiation)
    implementation(libs.ktor.serialization.kotlinx.json)
    implementation(libs.ktor.server.auth)
    implementation(libs.ktor.server.auth.jwt)
    implementation(libs.ktor.server.call.logging)
    implementation(libs.ktor.server.cors)
    implementation(libs.ktor.server.host.common)
    implementation(libs.ktor.server.status.pages)
    implementation(libs.ktor.server.netty)
    implementation("ch.qos.logback:logback-classic:$logbackVersion")
    implementation(projects.shared)

    testImplementation(libs.ktor.server.test.host)
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit:$kotlinVersion")
}
