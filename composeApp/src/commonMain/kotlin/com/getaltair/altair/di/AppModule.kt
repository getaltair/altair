package com.getaltair.altair.di

import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * Core application Koin module.
 * Contains all shared dependencies used across platforms.
 */
val appModule: Module = module {
    // Placeholder for repositories (Phase 2+)
    // singleOf(::QuestRepository)
    // singleOf(::NoteRepository)

    // Placeholder for ViewModels (Phase 3+)
    // viewModelOf(::GuidanceViewModel)
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
