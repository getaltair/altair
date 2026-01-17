package com.getaltair.altair.rpc

import io.ktor.client.HttpClient
import io.ktor.client.engine.cio.CIO
import org.koin.dsl.module

/**
 * JVM-specific Koin module providing HttpClient with CIO engine.
 */
val httpClientModule =
    module {
        single {
            HttpClient(CIO)
        }
    }
