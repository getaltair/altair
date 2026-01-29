package com.getaltair.altair.di

import com.getaltair.altair.api.*
import io.ktor.client.*
import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * Koin module for API layer dependencies.
 * Registers HttpClient and all API class instances.
 *
 * Platform modules must provide TokenProvider implementation.
 */
val apiModule = module {
    // HttpClient (requires TokenProvider to be provided by platform module)
    single<HttpClient> { createHttpClient(get()) }

    // API classes
    single<AuthApi> { AuthApi(get()) }
    single<GuidanceApi> { GuidanceApi(get()) }
    single<KnowledgeApi> { KnowledgeApi(get()) }
    single<TrackingApi> { TrackingApi(get()) }
    single<SyncApi> { SyncApi(get()) }
    single<AiApi> { AiApi(get()) }
    single<InboxApi> { InboxApi(get()) }
    single<InitiativeApi> { InitiativeApi(get()) }
    single<RoutineApi> { RoutineApi(get()) }
}
