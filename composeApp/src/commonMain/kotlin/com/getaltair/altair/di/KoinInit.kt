package com.getaltair.altair.di

import org.koin.core.context.startKoin
import org.koin.core.module.Module
import org.koin.dsl.KoinAppDeclaration

/**
 * Common Koin initialization.
 * Platform-specific code calls this with additional configuration.
 */
fun initKoin(
    additionalModules: List<Module> = emptyList(),
    appDeclaration: KoinAppDeclaration = {}
) = startKoin {
    appDeclaration()
    modules(allModules + additionalModules)
}
