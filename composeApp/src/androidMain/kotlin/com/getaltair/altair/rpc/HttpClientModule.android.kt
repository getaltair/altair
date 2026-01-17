package com.getaltair.altair.rpc

import io.ktor.client.HttpClient
import io.ktor.client.engine.okhttp.OkHttp
import org.koin.dsl.module

/**
 * Android-specific Koin module providing HttpClient with OkHttp engine.
 */
val httpClientModule =
    module {
        single {
            HttpClient(OkHttp)
        }
    }
