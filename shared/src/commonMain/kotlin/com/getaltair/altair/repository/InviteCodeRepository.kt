package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.model.system.InviteCode
import com.getaltair.altair.domain.types.Ulid

/**
 * Repository for managing invite codes.
 *
 * Invite codes are used for controlled registration on self-hosted servers.
 */
interface InviteCodeRepository {
    /**
     * Create a new invite code.
     *
     * @param inviteCode The invite code to create
     * @return Either an error or the created invite code
     */
    suspend fun create(inviteCode: InviteCode): Either<AuthError, InviteCode>

    /**
     * Find an invite code by its code string.
     *
     * @param code The invite code string
     * @return Either [AuthError.InvalidInviteCode] if not found or invalid, or the invite code
     */
    suspend fun findByCode(code: String): Either<AuthError, InviteCode>

    /**
     * Mark an invite code as used.
     *
     * @param id The invite code's ULID
     * @param usedBy The ULID of the user who used the code
     * @return Either an error or Unit on success
     */
    suspend fun markUsed(
        id: Ulid,
        usedBy: Ulid,
    ): Either<AuthError, Unit>

    /**
     * Find all invite codes created by a user.
     *
     * @param createdBy The ULID of the admin who created the codes
     * @return Either an error or the list of invite codes
     */
    suspend fun findByCreator(createdBy: Ulid): Either<AuthError, List<InviteCode>>

    /**
     * Delete expired and used invite codes (cleanup job).
     *
     * @return Either an error or the count of deleted codes
     */
    suspend fun deleteExpiredAndUsed(): Either<AuthError, Int>
}
