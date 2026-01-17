package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Platform-specific factory for creating root ComponentContext.
 * Each platform provides its own implementation with appropriate lifecycle management.
 */
expect fun createRootComponentContext(): ComponentContext
