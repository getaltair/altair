package com.getaltair.altair.di

import com.getaltair.altair.api.GuidanceApi
import com.getaltair.altair.api.InboxApi
import com.getaltair.altair.viewmodel.GuidanceViewModel
import com.getaltair.altair.viewmodel.InboxViewModel
import com.getaltair.altair.viewmodel.TodayViewModel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * Core application Koin module.
 * Contains all shared dependencies used across platforms.
 */
val appModule: Module = module {
    // Application-scoped coroutine scope for ViewModels
    single<CoroutineScope> { CoroutineScope(SupervisorJob() + Dispatchers.Main) }

    // ViewModels for Phase 7: Inbox + Guidance MVP
    single<InboxViewModel> { InboxViewModel(get<InboxApi>(), get()) }
    single<GuidanceViewModel> { GuidanceViewModel(get<GuidanceApi>(), get()) }
    single<TodayViewModel> { TodayViewModel(get<GuidanceApi>(), get()) }
}

/**
 * All modules to be loaded by Koin.
 * Platform module provides TokenProvider, which apiModule depends on.
 */
val allModules: List<Module> = listOf(
    platformModule,  // Must come first - provides TokenProvider
    appModule,
    apiModule
)
