package com.getaltair.altair.db

import org.koin.core.module.Module

/**
 * Creates a platform-specific Koin module for SQLDelight database dependencies.
 *
 * This function must be implemented in platform-specific source sets
 * to provide the appropriate database driver and configuration.
 *
 * @return A Koin Module containing database dependencies
 */
expect fun sqlDelightDatabaseModule(): Module
