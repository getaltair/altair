package com.getaltair.altair.di

import android.content.Context
import org.koin.android.ext.koin.androidContext
import org.koin.android.ext.koin.androidLogger
import org.koin.core.logger.Level

/**
 * Android-specific Koin initialization with Context.
 */
fun initKoinAndroid(context: Context) = initKoin {
    androidLogger(Level.DEBUG)
    androidContext(context)
}
