package com.getaltair.altair.com.getaltair.altair.server

import io.ktor.server.application.*
import io.ktor.server.netty.EngineMain

fun main(args: Array<String>) {
    EngineMain.main(args)
}

fun Application.module() {
    configureHTTP()
    configureSerialization()
    configureSecurity()
    configureMonitoring()
    configureRouting()
}
