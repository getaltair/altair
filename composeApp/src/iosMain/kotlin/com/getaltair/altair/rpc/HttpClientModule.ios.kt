package com.getaltair.altair.rpc

import io.ktor.client.HttpClient
import io.ktor.client.engine.darwin.Darwin
import org.koin.dsl.module

/**
 * iOS-specific Koin module providing HttpClient with Darwin engine.
 */
val httpClientModule =
    module {
        single {
            HttpClient(Darwin)
        }
    }
