package com.getaltair.altair.di

import com.getaltair.altair.BuildConfig
import com.getaltair.altair.data.auth.AuthAuthenticator
import com.getaltair.altair.data.auth.AuthInterceptor
import com.getaltair.altair.data.network.AuthApi
import com.getaltair.altair.data.network.SyncApi
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import org.koin.dsl.module
import retrofit2.Retrofit
import retrofit2.converter.kotlinx.serialization.asConverterFactory

val networkModule =
    module {
        single<AuthInterceptor> { AuthInterceptor(get()) }
        single<AuthAuthenticator> { AuthAuthenticator(get(), get()) }

        single<OkHttpClient> {
            OkHttpClient
                .Builder()
                .addInterceptor(get<AuthInterceptor>())
                .authenticator(get<AuthAuthenticator>())
                .addInterceptor(
                    HttpLoggingInterceptor().apply {
                        level = HttpLoggingInterceptor.Level.BODY
                    },
                ).build()
        }

        single<Retrofit> {
            val json = Json { ignoreUnknownKeys = true }
            Retrofit
                .Builder()
                .baseUrl(BuildConfig.API_BASE_URL)
                .client(get())
                .addConverterFactory(json.asConverterFactory("application/json; charset=UTF8".toMediaType()))
                .build()
        }

        single<AuthApi> { get<Retrofit>().create(AuthApi::class.java) }
        single<SyncApi> { get<Retrofit>().create(SyncApi::class.java) }
    }
