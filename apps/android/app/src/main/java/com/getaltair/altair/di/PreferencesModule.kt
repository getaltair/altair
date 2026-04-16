package com.getaltair.altair.di

import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import com.getaltair.altair.data.auth.TokenPreferences
import org.koin.android.ext.koin.androidContext
import org.koin.dsl.module

val preferencesModule =
    module {
        single<TokenPreferences> {
            val masterKey =
                MasterKey
                    .Builder(androidContext())
                    .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
                    .build()
            val prefs =
                EncryptedSharedPreferences.create(
                    androidContext(),
                    "altair_secure_prefs",
                    masterKey,
                    EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
                    EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM,
                )
            TokenPreferences(prefs)
        }
    }
