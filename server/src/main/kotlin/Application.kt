package com.getaltair

import io.ktor.server.application.*

fun main(args: Array<String>) {
    io.ktor.server.netty.EngineMain
        .main(args)
}

fun Application.module() {
    configureDatabase()
    configureSerialization()
    configureSecurity()
    configureMonitoring()
    configureHTTP()
    configureRpc()
    configureRouting()
}
