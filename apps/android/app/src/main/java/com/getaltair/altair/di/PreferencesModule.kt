package com.getaltair.altair.di

import com.getaltair.altair.data.preferences.ThemePreferences
import org.koin.android.ext.koin.androidContext
import org.koin.dsl.module

val preferencesModule = module {
    single { ThemePreferences(androidContext()) }
}
