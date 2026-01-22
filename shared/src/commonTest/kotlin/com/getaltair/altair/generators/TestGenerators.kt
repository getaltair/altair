package com.getaltair.altair.generators

import com.getaltair.altair.domain.model.guidance.Quest
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.QuestStatus
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import io.kotest.property.Arb
import io.kotest.property.arbitrary.Codepoint
import io.kotest.property.arbitrary.arbitrary
import io.kotest.property.arbitrary.az
import io.kotest.property.arbitrary.enum
import io.kotest.property.arbitrary.int
import io.kotest.property.arbitrary.long
import io.kotest.property.arbitrary.string
import kotlin.time.Instant

/**
 * Kotest Arb generators for Altair domain types.
 *
 * Usage in tests:
 * ```kotlin
 * checkAll(Arb.ulid(), Arb.validEmail()) { id, email ->
 *     // Test with generated values
 * }
 * ```
 */
object TestGenerators {
    /**
     * Generates valid ULIDs.
     */
    fun Arb.Companion.ulid(): Arb<Ulid> =
        arbitrary {
            Ulid.generate()
        }

    /**
     * Generates valid email addresses.
     */
    fun Arb.Companion.validEmail(): Arb<String> =
        arbitrary {
            val username = string(3..15, Codepoint.az()).bind()
            val domain = string(3..15, Codepoint.az()).bind()
            val tld = string(2..4, Codepoint.az()).bind()
            "$username@$domain.$tld"
        }

    /**
     * Generates display names (1-100 characters).
     */
    fun Arb.Companion.displayName(): Arb<String> = string(1..100)

    /**
     * Generates timestamps (past, present, or near future).
     */
    fun Arb.Companion.timestamp(): Arb<Instant> =
        arbitrary {
            // Use a fixed base time (2024-01-01) + random offset for reproducibility
            val baseMillis = 1_704_067_200_000L // 2024-01-01T00:00:00Z
            val offsetMillis = long(-365L * 24 * 60 * 60 * 1000..365L * 24 * 60 * 60 * 1000).bind()
            Instant.fromEpochMilliseconds(baseMillis + offsetMillis)
        }

    /**
     * Generates valid User instances.
     */
    fun Arb.Companion.user(
        role: UserRole? = null,
        status: UserStatus? = null,
    ): Arb<User> =
        arbitrary {
            val createdAt = timestamp().bind()
            val updatedAt =
                Instant.fromEpochMilliseconds(
                    createdAt.toEpochMilliseconds() + long(0L..1_000_000L).bind(),
                )
            User(
                id = ulid().bind(),
                email = validEmail().bind(),
                displayName = displayName().bind(),
                role = role ?: enum<UserRole>().bind(),
                status = status ?: enum<UserStatus>().bind(),
                storageUsedBytes = long(0L..1_000_000_000L).bind(),
                storageQuotaBytes = long(1_000_000_000L..10_000_000_000L).bind(),
                createdAt = createdAt,
                updatedAt = updatedAt,
                deletedAt = null,
            )
        }

    /**
     * Generates quest titles (1-200 characters).
     */
    fun Arb.Companion.questTitle(): Arb<String> = string(1..200)

    /**
     * Generates valid Quest instances.
     */
    fun Arb.Companion.quest(
        userId: Ulid? = null,
        status: QuestStatus? = null,
    ): Arb<Quest> =
        arbitrary {
            val createdAt = timestamp().bind()
            val updatedAt =
                Instant.fromEpochMilliseconds(
                    createdAt.toEpochMilliseconds() + long(0L..1_000_000L).bind(),
                )
            Quest(
                id = ulid().bind(),
                userId = userId ?: ulid().bind(),
                title = questTitle().bind(),
                description = string(0..1000).bind(),
                energyCost = int(1..5).bind(),
                status = status ?: QuestStatus.BACKLOG,
                epicId = null,
                routineId = null,
                initiativeId = null,
                dueDate = null,
                scheduledDate = null,
                createdAt = createdAt,
                updatedAt = updatedAt,
                startedAt = null,
                completedAt = null,
                deletedAt = null,
            )
        }
}
