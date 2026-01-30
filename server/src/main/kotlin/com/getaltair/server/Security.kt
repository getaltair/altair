package com.getaltair.server

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import com.getaltair.server.auth.AuthConfig
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.auth.jwt.*
import org.koin.ktor.ext.inject

/**
 * Configures JWT authentication for the Ktor application.
 *
 * Uses [AuthConfig] for JWT configuration parameters.
 * The JWT verifier validates:
 * - Token signature (HMAC256)
 * - Audience claim
 * - Issuer claim
 *
 * On successful validation, the JWT payload is made available
 * via [JWTPrincipal] for route handlers to extract user info.
 */
fun Application.configureSecurity() {
    val authConfig by inject<AuthConfig>()

    authentication {
        jwt("jwt") {
            realm = authConfig.jwtRealm
            verifier(
                JWT
                    .require(Algorithm.HMAC256(authConfig.jwtSecret))
                    .withAudience(authConfig.jwtAudience)
                    .withIssuer(authConfig.jwtIssuer)
                    .build()
            )
            validate { credential ->
                // Validate that the token has a subject (userId)
                if (credential.payload.subject != null &&
                    credential.payload.audience.contains(authConfig.jwtAudience)) {
                    JWTPrincipal(credential.payload)
                } else {
                    null
                }
            }
        }
    }
}
