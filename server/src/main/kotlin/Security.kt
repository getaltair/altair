package com.getaltair

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.auth.AuthContext
import com.getaltair.auth.JwtConfig
import com.getaltair.auth.RequestAuthContext
import io.ktor.server.application.Application
import io.ktor.server.auth.authentication
import io.ktor.server.auth.jwt.JWTPrincipal
import io.ktor.server.auth.jwt.jwt
import io.ktor.util.AttributeKey
import org.koin.ktor.ext.inject
import org.slf4j.LoggerFactory

private val logger = LoggerFactory.getLogger("Security")

/**
 * Attribute key for storing AuthContext in the request.
 */
val AuthContextKey = AttributeKey<AuthContext>("AuthContext")

fun Application.configureSecurity() {
    val jwtConfig: JwtConfig by inject()

    authentication {
        jwt("auth-jwt") {
            realm = "altair"
            verifier(
                JWT
                    .require(Algorithm.HMAC256(jwtConfig.secret))
                    .withAudience(jwtConfig.audience)
                    .withIssuer(jwtConfig.issuer)
                    .build(),
            )
            validate { credential ->
                val payload = credential.payload

                // Validate required claims
                val userId = payload.subject
                val email = payload.getClaim("email").asString()
                val role = payload.getClaim("role").asString()

                if (userId == null || email == null || role == null) {
                    logger.warn("JWT validation failed: missing required claims")
                    return@validate null
                }

                if (!payload.audience.contains(jwtConfig.audience)) {
                    logger.warn("JWT validation failed: invalid audience")
                    return@validate null
                }

                // Note: AuthContext created for validation purposes and future use when
                // kotlinx-rpc supports per-request context injection. Currently validation
                // occurs but context is not accessible in RPC services.
                RequestAuthContext(
                    userId = Ulid(userId),
                    email = email,
                    role = UserRole.valueOf(role.uppercase()),
                )

                // Return principal on success
                JWTPrincipal(payload)
            }
            challenge { _, _ ->
                // The default challenge is returned by Ktor
                logger.debug("JWT authentication challenge issued")
            }
        }
    }
}
