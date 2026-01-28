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
 */
val allModules: List<Module> = listOf(
    appModule
)
