package com.getaltair.server.di

import com.getaltair.server.auth.*
import com.getaltair.altair.shared.repository.RefreshTokenRepository
import com.getaltair.altair.shared.repository.UserRepository
import com.getaltair.server.persistence.repository.SurrealRefreshTokenRepository
import com.getaltair.server.persistence.repository.SurrealUserRepository
import org.koin.dsl.module

/**
 * Koin module for authentication layer dependencies.
 *
 * Registers all auth-related services and their dependencies.
 * Includes JWT service, password hashing, and auth service.
 */
val authModule = module {
    // Configuration
    single { AuthConfig() }

    // Services
    single<JwtService> { DefaultJwtService(get()) }
    single<PasswordHasher> { Argon2PasswordHasher() }

    // Auth-specific repositories (bound to shared interfaces)
    single<UserRepository> { SurrealUserRepository(get()) }
    single<RefreshTokenRepository> { SurrealRefreshTokenRepository(get()) }

    // Auth service (depends on repositories and other services)
    single {
        AuthService(
            userRepository = get(),
            refreshTokenRepository = get(),
            jwtService = get(),
            passwordHasher = get(),
            config = get()
        )
    }
}
