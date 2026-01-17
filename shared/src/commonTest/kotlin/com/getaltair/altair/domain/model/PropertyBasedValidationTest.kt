package com.getaltair.altair.domain.model

import com.getaltair.altair.domain.common.ColorValidation
import com.getaltair.altair.domain.common.DomainConstants.MAX_ENERGY_COST
import com.getaltair.altair.domain.common.DomainConstants.MAX_NAME_LENGTH
import com.getaltair.altair.domain.common.DomainConstants.MAX_TAG_NAME_LENGTH
import com.getaltair.altair.domain.common.DomainConstants.MAX_TITLE_LENGTH
import com.getaltair.altair.domain.model.guidance.Quest
import com.getaltair.altair.domain.model.knowledge.Folder
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.model.knowledge.NoteLink
import com.getaltair.altair.domain.model.knowledge.Tag
import com.getaltair.altair.domain.model.system.Initiative
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.model.tracking.Container
import com.getaltair.altair.domain.model.tracking.Location
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.InitiativeStatus
import com.getaltair.altair.domain.types.enums.QuestStatus
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import io.kotest.property.Arb
import io.kotest.property.arbitrary.Codepoint
import io.kotest.property.arbitrary.arbitrary
import io.kotest.property.arbitrary.az
import io.kotest.property.arbitrary.bind
import io.kotest.property.arbitrary.element
import io.kotest.property.arbitrary.filter
import io.kotest.property.arbitrary.filterNot
import io.kotest.property.arbitrary.hex
import io.kotest.property.arbitrary.int
import io.kotest.property.arbitrary.long
import io.kotest.property.arbitrary.map
import io.kotest.property.arbitrary.string
import io.kotest.property.checkAll
import kotlin.test.Test
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue
import kotlinx.coroutines.test.runTest
import kotlinx.datetime.Instant

/**
 * Property-based tests for domain model validation using Kotest.
 *
 * These tests complement the example-based tests in EntityValidationTest
 * by generating random inputs to find edge cases and ensure validation
 * rules are comprehensive.
 */
class PropertyBasedValidationTest {
    private val now: Instant = Instant.fromEpochMilliseconds(0)
    private val userId = Ulid.generate()

    // ============================================================================
    // Custom Arbitraries (Generators)
    // ============================================================================

    /** Generates valid email addresses */
    private val validEmailArb: Arb<String> = arbitrary {
        val localPart = Arb.string(1..20, Codepoint.az()).bind()
        val domain = Arb.string(1..10, Codepoint.az()).bind()
        val tld = Arb.element("com", "org", "net", "io", "dev").bind()
        "$localPart@$domain.$tld"
    }

    /** Generates invalid email addresses (missing @, domain, or tld) */
    private val invalidEmailArb: Arb<String> = Arb.element(
        "plaintext",
        "missing@domain",
        "@nodomain.com",
        "no@tld",
        "spaces in@email.com",
        "@",
        "@@",
        "",
        "  ",
    )

    /** Generates non-blank strings within length limit */
    private fun validStringArb(maxLength: Int): Arb<String> =
        Arb.string(1..maxLength, Codepoint.az())
            .filter { it.isNotBlank() }

    /** Generates strings that exceed the length limit */
    private fun tooLongStringArb(maxLength: Int): Arb<String> =
        Arb.int((maxLength + 1)..(maxLength + 50)).map { len -> "a".repeat(len) }

    /** Generates valid hex colors (#RGB, #RRGGBB, #RRGGBBAA) */
    private val validHexColorArb: Arb<String> = arbitrary {
        val length = Arb.element(3, 6, 8).bind()
        val hex = Arb.string(length, Codepoint.hex()).bind()
        "#$hex"
    }

    /** Generates invalid hex color strings */
    private val invalidHexColorArb: Arb<String> = Arb.element(
        "red",
        "blue",
        "#GGG",
        "#ZZZZZZ",
        "#12345",
        "#1234567",
        "#123456789",
        "123456",
        "##AABBCC",
        "",
    )

    /** Generates valid energy costs (1-5) */
    private val validEnergyCostArb: Arb<Int> = Arb.int(1..MAX_ENERGY_COST)

    /** Generates invalid energy costs (outside 1-5 range) */
    private val invalidEnergyCostArb: Arb<Int> = Arb.element(
        -10, -1, 0, 6, 7, 10, 100, Int.MIN_VALUE, Int.MAX_VALUE,
    )

    /** Generates non-negative longs for storage values */
    private val nonNegativeLongArb: Arb<Long> = Arb.long(0..Long.MAX_VALUE / 2)

    /** Generates negative longs for invalid storage values */
    private val negativeLongArb: Arb<Long> = Arb.long(Long.MIN_VALUE..-1)

    /** Generates tag names without spaces */
    private val validTagNameArb: Arb<String> =
        Arb.string(1..MAX_TAG_NAME_LENGTH, Codepoint.az())
            .filter { it.isNotBlank() && !it.contains(' ') }

    /** Generates tag names with spaces (invalid) */
    private val tagNameWithSpacesArb: Arb<String> = arbitrary {
        val before = Arb.string(1..10, Codepoint.az()).bind()
        val after = Arb.string(1..10, Codepoint.az()).bind()
        "$before $after"
    }

    // ============================================================================
    // User Validation Tests
    // ============================================================================

    @Test
    fun `User accepts any valid email format`() = runTest {
        checkAll(100, validEmailArb) { email ->
            val user = createUser(email = email)
            assertTrue(user.email == email)
        }
    }

    @Test
    fun `User rejects all invalid email formats`() = runTest {
        checkAll(invalidEmailArb) { email ->
            assertFailsWith<IllegalArgumentException> {
                createUser(email = email)
            }
        }
    }

    @Test
    fun `User accepts display names within length limit`() = runTest {
        checkAll(100, validStringArb(MAX_NAME_LENGTH)) { name ->
            val user = createUser(displayName = name)
            assertTrue(user.displayName == name)
        }
    }

    @Test
    fun `User rejects display names exceeding length limit`() = runTest {
        checkAll(20, tooLongStringArb(MAX_NAME_LENGTH)) { name ->
            assertFailsWith<IllegalArgumentException> {
                createUser(displayName = name)
            }
        }
    }

    @Test
    fun `User accepts non-negative storage values`() = runTest {
        checkAll(100, nonNegativeLongArb, nonNegativeLongArb) { used, quota ->
            val user = createUser(storageUsedBytes = used, storageQuotaBytes = quota)
            assertTrue(user.storageUsedBytes == used)
            assertTrue(user.storageQuotaBytes == quota)
        }
    }

    @Test
    fun `User rejects negative storage values`() = runTest {
        checkAll(20, negativeLongArb) { negativeValue ->
            assertFailsWith<IllegalArgumentException> {
                createUser(storageUsedBytes = negativeValue)
            }
            assertFailsWith<IllegalArgumentException> {
                createUser(storageQuotaBytes = negativeValue)
            }
        }
    }

    @Test
    fun `User correctly calculates storage remaining and over quota`() = runTest {
        checkAll(100, nonNegativeLongArb, nonNegativeLongArb) { used, quota ->
            val user = createUser(storageUsedBytes = used, storageQuotaBytes = quota)
            val expectedRemaining = (quota - used).coerceAtLeast(0)
            assertTrue(user.storageRemainingBytes == expectedRemaining)
            assertTrue(user.isOverQuota == (used > quota))
        }
    }

    // ============================================================================
    // Quest Validation Tests
    // ============================================================================

    @Test
    fun `Quest accepts titles within length limit`() = runTest {
        checkAll(100, validStringArb(MAX_TITLE_LENGTH)) { title ->
            val quest = createQuest(title = title)
            assertTrue(quest.title == title)
        }
    }

    @Test
    fun `Quest rejects titles exceeding length limit`() = runTest {
        checkAll(20, tooLongStringArb(MAX_TITLE_LENGTH)) { title ->
            assertFailsWith<IllegalArgumentException> {
                createQuest(title = title)
            }
        }
    }

    @Test
    fun `Quest accepts valid energy costs 1-5`() = runTest {
        checkAll(validEnergyCostArb) { cost ->
            val quest = createQuest(energyCost = cost)
            assertTrue(quest.energyCost == cost)
        }
    }

    @Test
    fun `Quest rejects invalid energy costs`() = runTest {
        checkAll(invalidEnergyCostArb) { cost ->
            assertFailsWith<IllegalArgumentException> {
                createQuest(energyCost = cost)
            }
        }
    }

    // ============================================================================
    // Note Validation Tests
    // ============================================================================

    @Test
    fun `Note accepts titles within length limit`() = runTest {
        checkAll(100, validStringArb(MAX_TITLE_LENGTH)) { title ->
            val note = createNote(title = title)
            assertTrue(note.title == title)
        }
    }

    @Test
    fun `Note rejects titles exceeding length limit`() = runTest {
        checkAll(20, tooLongStringArb(MAX_TITLE_LENGTH)) { title ->
            assertFailsWith<IllegalArgumentException> {
                createNote(title = title)
            }
        }
    }

    // ============================================================================
    // Initiative Color Validation Tests
    // ============================================================================

    @Test
    fun `Initiative accepts valid hex colors`() = runTest {
        checkAll(100, validHexColorArb) { color ->
            val initiative = createInitiative(color = color)
            assertTrue(initiative.color == color)
        }
    }

    @Test
    fun `Initiative rejects invalid hex colors`() = runTest {
        checkAll(invalidHexColorArb.filterNot { it.isEmpty() }) { color ->
            assertFailsWith<IllegalArgumentException> {
                createInitiative(color = color)
            }
        }
    }

    @Test
    fun `Initiative accepts null color`() = runTest {
        val initiative = createInitiative(color = null)
        assertTrue(initiative.color == null)
    }

    @Test
    fun `ColorValidation regex matches expected patterns`() = runTest {
        checkAll(100, validHexColorArb) { color ->
            assertTrue(color.matches(ColorValidation.HEX_COLOR_REGEX))
        }
    }

    @Test
    fun `ColorValidation regex rejects invalid patterns`() = runTest {
        checkAll(invalidHexColorArb.filterNot { it.isEmpty() }) { color ->
            assertTrue(!color.matches(ColorValidation.HEX_COLOR_REGEX))
        }
    }

    // ============================================================================
    // Tag Validation Tests
    // ============================================================================

    @Test
    fun `Tag accepts names without spaces`() = runTest {
        checkAll(100, validTagNameArb) { name ->
            val tag = Tag(
                id = Ulid.generate(),
                userId = userId,
                name = name,
                color = null,
                createdAt = now,
                updatedAt = now,
            )
            assertTrue(tag.name == name)
        }
    }

    @Test
    fun `Tag rejects names with spaces`() = runTest {
        checkAll(50, tagNameWithSpacesArb) { name ->
            assertFailsWith<IllegalArgumentException> {
                Tag(
                    id = Ulid.generate(),
                    userId = userId,
                    name = name,
                    color = null,
                    createdAt = now,
                    updatedAt = now,
                )
            }
        }
    }

    // ============================================================================
    // Self-Reference Prevention Tests
    // ============================================================================

    @Test
    fun `NoteLink always prevents self-referencing for any ULID`() = runTest {
        repeat(50) {
            val noteId = Ulid.generate()
            assertFailsWith<IllegalArgumentException> {
                NoteLink(
                    id = Ulid.generate(),
                    userId = userId,
                    sourceNoteId = noteId,
                    targetNoteId = noteId,
                    context = null,
                    createdAt = now,
                    updatedAt = now,
                )
            }
        }
    }

    @Test
    fun `NoteLink accepts different source and target`() = runTest {
        repeat(50) {
            val sourceId = Ulid.generate()
            val targetId = Ulid.generate()
            val link = NoteLink(
                id = Ulid.generate(),
                userId = userId,
                sourceNoteId = sourceId,
                targetNoteId = targetId,
                context = null,
                createdAt = now,
                updatedAt = now,
            )
            assertTrue(link.sourceNoteId != link.targetNoteId)
        }
    }

    @Test
    fun `Folder always prevents self as parent for any ULID`() = runTest {
        repeat(50) {
            val folderId = Ulid.generate()
            assertFailsWith<IllegalArgumentException> {
                Folder(
                    id = folderId,
                    userId = userId,
                    name = "Test",
                    parentId = folderId,
                    sortOrder = 0,
                    createdAt = now,
                    updatedAt = now,
                )
            }
        }
    }

    @Test
    fun `Location always prevents self as parent for any ULID`() = runTest {
        repeat(50) {
            val locationId = Ulid.generate()
            assertFailsWith<IllegalArgumentException> {
                Location(
                    id = locationId,
                    userId = userId,
                    name = "Test",
                    description = null,
                    parentId = locationId,
                    address = null,
                    createdAt = now,
                    updatedAt = now,
                )
            }
        }
    }

    @Test
    fun `Container always prevents self as parent for any ULID`() = runTest {
        repeat(50) {
            val containerId = Ulid.generate()
            assertFailsWith<IllegalArgumentException> {
                Container(
                    id = containerId,
                    userId = userId,
                    name = "Test",
                    description = null,
                    locationId = null,
                    parentContainerId = containerId,
                    label = null,
                    createdAt = now,
                    updatedAt = now,
                )
            }
        }
    }

    // ============================================================================
    // Boundary Value Tests
    // ============================================================================

    @Test
    fun `String fields accept exactly max length`() = runTest {
        // User display name at exactly MAX_NAME_LENGTH
        val maxDisplayName = "a".repeat(MAX_NAME_LENGTH)
        val user = createUser(displayName = maxDisplayName)
        assertTrue(user.displayName.length == MAX_NAME_LENGTH)

        // Quest title at exactly MAX_TITLE_LENGTH
        val maxTitle = "a".repeat(MAX_TITLE_LENGTH)
        val quest = createQuest(title = maxTitle)
        assertTrue(quest.title.length == MAX_TITLE_LENGTH)

        // Tag name at exactly MAX_TAG_NAME_LENGTH
        val maxTagName = "a".repeat(MAX_TAG_NAME_LENGTH)
        val tag = Tag(
            id = Ulid.generate(),
            userId = userId,
            name = maxTagName,
            color = null,
            createdAt = now,
            updatedAt = now,
        )
        assertTrue(tag.name.length == MAX_TAG_NAME_LENGTH)
    }

    @Test
    fun `String fields reject exactly max length plus one`() = runTest {
        // User display name at MAX_NAME_LENGTH + 1
        assertFailsWith<IllegalArgumentException> {
            createUser(displayName = "a".repeat(MAX_NAME_LENGTH + 1))
        }

        // Quest title at MAX_TITLE_LENGTH + 1
        assertFailsWith<IllegalArgumentException> {
            createQuest(title = "a".repeat(MAX_TITLE_LENGTH + 1))
        }

        // Tag name at MAX_TAG_NAME_LENGTH + 1
        assertFailsWith<IllegalArgumentException> {
            Tag(
                id = Ulid.generate(),
                userId = userId,
                name = "a".repeat(MAX_TAG_NAME_LENGTH + 1),
                color = null,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    @Test
    fun `Energy cost boundary values`() = runTest {
        // Min valid (1)
        val questMin = createQuest(energyCost = 1)
        assertTrue(questMin.energyCost == 1)

        // Max valid (5)
        val questMax = createQuest(energyCost = 5)
        assertTrue(questMax.energyCost == 5)

        // Just below min (0)
        assertFailsWith<IllegalArgumentException> {
            createQuest(energyCost = 0)
        }

        // Just above max (6)
        assertFailsWith<IllegalArgumentException> {
            createQuest(energyCost = 6)
        }
    }

    @Test
    fun `Storage boundary at zero`() = runTest {
        // Zero is valid
        val user = createUser(storageUsedBytes = 0, storageQuotaBytes = 0)
        assertTrue(user.storageUsedBytes == 0L)
        assertTrue(user.storageQuotaBytes == 0L)
        assertTrue(user.storageRemainingBytes == 0L)
        assertTrue(!user.isOverQuota) // 0 is not over 0
    }

    // ============================================================================
    // Helper Functions
    // ============================================================================

    private fun createUser(
        email: String = "test@example.com",
        displayName: String = "Test User",
        storageUsedBytes: Long = 0,
        storageQuotaBytes: Long = 1_000_000_000,
    ) = User(
        id = Ulid.generate(),
        email = email,
        displayName = displayName,
        role = UserRole.MEMBER,
        status = UserStatus.ACTIVE,
        storageUsedBytes = storageUsedBytes,
        storageQuotaBytes = storageQuotaBytes,
        createdAt = now,
        updatedAt = now,
    )

    private fun createQuest(
        title: String = "Test Quest",
        energyCost: Int = 2,
    ) = Quest(
        id = Ulid.generate(),
        userId = userId,
        title = title,
        description = null,
        energyCost = energyCost,
        status = QuestStatus.BACKLOG,
        epicId = null,
        routineId = null,
        initiativeId = null,
        dueDate = null,
        scheduledDate = null,
        createdAt = now,
        updatedAt = now,
        startedAt = null,
        completedAt = null,
    )

    private fun createInitiative(
        name: String = "Test Initiative",
        color: String? = null,
    ) = Initiative(
        id = Ulid.generate(),
        userId = userId,
        name = name,
        description = null,
        color = color,
        icon = null,
        status = InitiativeStatus.ACTIVE,
        createdAt = now,
        updatedAt = now,
    )

    private fun createNote(
        title: String = "Test Note",
    ) = Note(
        id = Ulid.generate(),
        userId = userId,
        title = title,
        content = "Test content",
        folderId = null,
        initiativeId = null,
        isPinned = false,
        createdAt = now,
        updatedAt = now,
    )
}
