package com.getaltair.altair.di

import org.koin.dsl.module

/**
 * Main application module for dependency injection.
 * Feature-specific modules (guidanceModule, knowledgeModule, trackingModule)
 * will be added as features are implemented.
 */
val appModule =
    module {
        // Empty for now - features will add their dependencies here
    }
