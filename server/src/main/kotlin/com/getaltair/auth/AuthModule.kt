package com.getaltair.auth

import com.getaltair.altair.db.repository.SurrealInviteCodeRepository
import com.getaltair.altair.db.repository.SurrealRefreshTokenRepository
import com.getaltair.altair.repository.InviteCodeRepository
import com.getaltair.altair.repository.RefreshTokenRepository
import com.getaltair.altair.rpc.AuthService
import com.getaltair.altair.rpc.PublicAuthService
import com.getaltair.altair.service.auth.JwtTokenService
import com.getaltair.altair.service.auth.PasswordService
import com.getaltair.rpc.AuthServiceImpl
import com.getaltair.rpc.PublicAuthServiceImpl
import org.koin.dsl.module

/**
 * Koin module for authentication dependencies.
 *
 * Provides:
 * - JwtConfig (from environment)
 * - PasswordService (Argon2 implementation)
 * - JwtTokenService (JWT generation/validation)
 * - RefreshTokenRepository
 * - InviteCodeRepository
 * - PublicAuthService (for login, register, refresh)
 * - AuthService (for logout, password change, invite codes)
 *
 * Note: AuthService methods that require user context will have limited
 * functionality until kotlinx-rpc provides per-request context support.
 */
val authModule =
    module {
        // JWT configuration from environment
        single { JwtConfig.fromEnvironment() }

        // Password hashing service
        single<PasswordService> { Argon2PasswordService() }

        // JWT token service
        single<JwtTokenService> { JwtTokenServiceImpl(get()) }

        // Auth-related repositories
        single<RefreshTokenRepository> { SurrealRefreshTokenRepository(get()) }
        single<InviteCodeRepository> { SurrealInviteCodeRepository(get()) }

        // PublicAuthService implementation (no auth required)
        single<PublicAuthService> {
            PublicAuthServiceImpl(
                userRepository = get(),
                refreshTokenRepository = get(),
                inviteCodeRepository = get(),
                passwordService = get(),
                jwtTokenService = get(),
                jwtConfig = get(),
            )
        }

        // AuthService implementation (authenticated endpoint)
        single<AuthService> {
            AuthServiceImpl(
                userRepository = get(),
                refreshTokenRepository = get(),
                inviteCodeRepository = get(),
                passwordService = get(),
                jwtTokenService = get(),
                jwtConfig = get(),
            )
        }
    }
