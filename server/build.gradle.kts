val kotlinVersion: String by project
val logbackVersion: String by project

plugins {
    kotlin("jvm")
    id("io.ktor.plugin") version "3.3.3"
    id("org.jetbrains.kotlin.plugin.serialization") version "2.3.0"
    alias(libs.plugins.kotlinx.rpc)
}

group = "com.getaltair"
version = "0.0.1"

application {
    mainClass = "io.ktor.server.netty.EngineMain"
}

// Force Netty version to fix CVE vulnerabilities (CRLF Injection in netty-codec-http)
val nettyVersion = "4.2.9.Final"

configurations.all {
    resolutionStrategy.eachDependency {
        if (requested.group == "io.netty") {
            useVersion(nettyVersion)
        }
    }
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
    implementation(libs.ktor.server.websockets)
    implementation("ch.qos.logback:logback-classic:$logbackVersion")
    implementation(projects.shared)
    // kotlinx-rpc server
    implementation(libs.kotlinx.rpc.krpc.server)
    implementation(libs.kotlinx.rpc.krpc.serialization.json)
    implementation(libs.kotlinx.rpc.krpc.ktor.server)
    // SurrealDB - Database (ADR-002)
    implementation(libs.surrealdb)
    // Koin - Dependency Injection
    implementation(libs.koin.core)
    implementation(libs.koin.ktor)
    implementation(libs.koin.logger.slf4j)
    // Arrow - Functional Error Handling
    implementation(libs.arrow.core)
    // kotlinx-datetime
    implementation(libs.kotlinx.datetime)
    // Argon2 - Password hashing (ADR-012: Authentication)
    implementation("de.mkammerer:argon2-jvm:2.11")

    testImplementation(libs.ktor.server.test.host)
    // Kotest - BDD testing framework
    testImplementation(libs.kotest.runner.junit5)
    testImplementation(libs.kotest.assertions.core)
    testImplementation(libs.kotest.assertions.arrow)
    testImplementation(libs.kotest.property)
    testImplementation(libs.kotest.extensions.testcontainers)
    // kotlinx-rpc client for testing
    testImplementation(libs.kotlinx.rpc.krpc.client)
    testImplementation(libs.kotlinx.rpc.krpc.ktor.client)
    testImplementation(libs.ktor.client.cio)
    testImplementation(libs.ktor.client.websockets)
    // Testcontainers for integration tests
    testImplementation(libs.testcontainers)
    testImplementation(libs.kotlinx.coroutines.test)
}

tasks.withType<Test> {
    useJUnitPlatform()
}
