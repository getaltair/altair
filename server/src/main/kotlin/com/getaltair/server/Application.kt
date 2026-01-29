package com.getaltair.server

import com.getaltair.server.di.persistenceModule
import io.ktor.server.application.*
import io.ktor.server.netty.EngineMain
import org.koin.ktor.plugin.Koin

fun main(args: Array<String>) {
    EngineMain.main(args)
}

fun Application.module() {
    // Install Koin for dependency injection
    install(Koin) {
        modules(persistenceModule)
    }

    configureHTTP()
    configureSerialization()
    configureSecurity()
    configureMonitoring()
    configureRouting()
}
