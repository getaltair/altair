val kotlin_version: String by project
val logback_version: String by project

plugins {
    alias(libs.plugins.kotlinJvm)
    alias(libs.plugins.ktor)
    alias(libs.plugins.kotlinxSerialization)
}

group = "com.getaltair.altair"
version = "1.0.0"

application {
    mainClass = "io.ktor.server.netty.EngineMain"
}

kotlin {
    jvmToolchain(21)
}

dependencies {
    implementation("io.ktor:ktor-server-cors")
    implementation("io.ktor:ktor-server-core")
    implementation("io.ktor:ktor-server-host-common")
    implementation("io.ktor:ktor-server-status-pages")
    implementation("io.ktor:ktor-server-content-negotiation")
    implementation("io.ktor:ktor-serialization-kotlinx-json")
    implementation("io.ktor:ktor-server-auth")
    implementation("io.ktor:ktor-server-auth-jwt")
    implementation("io.ktor:ktor-server-call-logging")
    implementation("io.ktor:ktor-server-netty")
    implementation("ch.qos.logback:logback-classic:$logback_version")

    implementation(projects.shared)

    testImplementation("io.ktor:ktor-server-test-host")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit:$kotlin_version")
}
