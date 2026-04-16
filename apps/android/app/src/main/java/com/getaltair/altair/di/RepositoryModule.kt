package com.getaltair.altair.di

import com.getaltair.altair.data.repository.AuthRepositoryImpl
import com.getaltair.altair.domain.repository.AuthRepository
import org.koin.dsl.module

val repositoryModule =
    module {
        single<AuthRepository> { AuthRepositoryImpl(get(), get(), get()) }
    }
