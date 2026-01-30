package com.getaltair.server.routes

import com.getaltair.server.auth.AuthService
import com.getaltair.altair.shared.dto.auth.*
import io.ktor.http.*
import io.ktor.server.auth.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import org.koin.ktor.ext.inject

/**
 * Authentication routes for user management and JWT token operations.
 *
 * Provides endpoints for:
 * - User registration and login
 * - JWT token refresh
 * - Logout (refresh token revocation)
 * - Password change
 * - User profile retrieval
 *
 * ## Public Routes (no authentication)
 * - POST /api/auth/login
 * - POST /api/auth/register
 * - POST /api/auth/refresh
 *
 * ## Protected Routes (JWT required)
 * - POST /api/auth/logout
 * - POST /api/auth/change-password
 * - GET /api/auth/me
 */
fun Route.authRoutes() {
    val authService by inject<AuthService>()

    route("/api/auth") {
        // Public routes (no authentication required)

        /**
         * POST /api/auth/login
         * Authenticate user with username and password.
         * Returns JWT access and refresh tokens.
         */
        post("/login") {
            val request = call.receive<LoginRequest>()
            authService.login(request).fold(
                ifLeft = { error ->
                    val (status, response) = error.toHttpResponse()
                    call.respond(status, response)
                },
                ifRight = { token ->
                    call.respond(HttpStatusCode.OK, token)
                }
            )
        }

        /**
         * POST /api/auth/register
         * Create new user account with invite code.
         * Returns JWT tokens on success.
         */
        post("/register") {
            val request = call.receive<RegisterRequest>()
            authService.register(request).fold(
                ifLeft = { error ->
                    val (status, response) = error.toHttpResponse()
                    call.respond(status, response)
                },
                ifRight = { token ->
                    call.respond(HttpStatusCode.Created, token)
                }
            )
        }

        /**
         * POST /api/auth/refresh
         * Refresh access token using valid refresh token.
         * Returns new access token.
         */
        post("/refresh") {
            val request = call.receive<RefreshTokenRequest>()
            authService.refresh(request).fold(
                ifLeft = { error ->
                    val (status, response) = error.toHttpResponse()
                    call.respond(status, response)
                },
                ifRight = { token ->
                    call.respond(HttpStatusCode.OK, token)
                }
            )
        }

        // Protected routes (require JWT authentication)
        authenticate("jwt") {
            /**
             * POST /api/auth/logout
             * Invalidate current session and refresh token.
             * Requires X-Refresh-Token header.
             */
            post("/logout") {
                val userId = call.userId
                val refreshToken = call.request.header("X-Refresh-Token")
                if (refreshToken == null) {
                    call.respond(
                        HttpStatusCode.BadRequest,
                        ErrorResponse(
                            code = "MISSING_TOKEN",
                            message = "Refresh token required in X-Refresh-Token header"
                        )
                    )
                    return@post
                }

                authService.logout(userId.toString(), refreshToken).fold(
                    ifLeft = { error ->
                        val (status, response) = error.toHttpResponse()
                        call.respond(status, response)
                    },
                    ifRight = {
                        call.respond(HttpStatusCode.NoContent)
                    }
                )
            }

            /**
             * POST /api/auth/change-password
             * Change authenticated user's password.
             * Requires current password for verification.
             * Revokes all refresh tokens on success.
             */
            post("/change-password") {
                val userId = call.userId
                val request = call.receive<ChangePasswordRequest>()
                authService.changePassword(userId.toString(), request).fold(
                    ifLeft = { error ->
                        val (status, response) = error.toHttpResponse()
                        call.respond(status, response)
                    },
                    ifRight = {
                        call.respond(HttpStatusCode.NoContent)
                    }
                )
            }

            /**
             * GET /api/auth/me
             * Get current authenticated user's information.
             * Extracts user ID from JWT token.
             */
            get("/me") {
                val userId = call.userId
                authService.getUser(userId.toString()).fold(
                    ifLeft = { error ->
                        val (status, response) = error.toHttpResponse()
                        call.respond(status, response)
                    },
                    ifRight = { user ->
                        call.respond(HttpStatusCode.OK, user)
                    }
                )
            }
        }
    }
}
