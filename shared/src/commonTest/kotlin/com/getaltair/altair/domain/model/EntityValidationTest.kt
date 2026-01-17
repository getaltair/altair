package com.getaltair.altair.domain.model

import com.getaltair.altair.domain.model.guidance.EnergyBudget
import com.getaltair.altair.domain.model.guidance.Quest
import com.getaltair.altair.domain.model.knowledge.Attachment
import com.getaltair.altair.domain.model.knowledge.Folder
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.model.knowledge.NoteLink
import com.getaltair.altair.domain.model.knowledge.Tag
import com.getaltair.altair.domain.model.system.Initiative
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.model.tracking.Container
import com.getaltair.altair.domain.model.tracking.FieldDefinition
import com.getaltair.altair.domain.model.tracking.Location
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.FieldType
import com.getaltair.altair.domain.types.enums.InitiativeStatus
import com.getaltair.altair.domain.types.enums.QuestStatus
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import kotlin.test.Test
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate

class EntityValidationTest {
    private val now: Instant = Instant.fromEpochMilliseconds(kotlin.time.Clock.System.now().toEpochMilliseconds())
    private val userId = Ulid.generate()

    // User validation tests
    @Test
    fun `User rejects blank email`() {
        assertFailsWith<IllegalArgumentException> {
            createUser(email = "")
        }
    }

    @Test
    fun `User rejects email without at sign`() {
        assertFailsWith<IllegalArgumentException> {
            createUser(email = "invalidemail")
        }
    }

    @Test
    fun `User rejects email without domain`() {
        assertFailsWith<IllegalArgumentException> {
            createUser(email = "user@")
        }
        assertFailsWith<IllegalArgumentException> {
            createUser(email = "user@domain")
        }
    }

    @Test
    fun `User rejects email with only at sign`() {
        assertFailsWith<IllegalArgumentException> {
            createUser(email = "@")
        }
        assertFailsWith<IllegalArgumentException> {
            createUser(email = "@@")
        }
    }

    @Test
    fun `User accepts valid email formats`() {
        val user = createUser(email = "user@example.com")
        assertTrue(user.email == "user@example.com")

        val user2 = createUser(email = "user.name+tag@sub.domain.org")
        assertTrue(user2.email == "user.name+tag@sub.domain.org")
    }

    @Test
    fun `User rejects blank display name`() {
        assertFailsWith<IllegalArgumentException> {
            createUser(displayName = "")
        }
    }

    @Test
    fun `User rejects display name over 100 characters`() {
        assertFailsWith<IllegalArgumentException> {
            createUser(displayName = "a".repeat(101))
        }
    }

    @Test
    fun `User rejects negative storage values`() {
        assertFailsWith<IllegalArgumentException> {
            createUser(storageUsedBytes = -1)
        }
        assertFailsWith<IllegalArgumentException> {
            createUser(storageQuotaBytes = -1)
        }
    }

    // Quest validation tests
    @Test
    fun `Quest rejects blank title`() {
        assertFailsWith<IllegalArgumentException> {
            createQuest(title = "")
        }
    }

    @Test
    fun `Quest rejects title over 200 characters`() {
        assertFailsWith<IllegalArgumentException> {
            createQuest(title = "a".repeat(201))
        }
    }

    @Test
    fun `Quest rejects energy cost below 1`() {
        assertFailsWith<IllegalArgumentException> {
            createQuest(energyCost = 0)
        }
    }

    @Test
    fun `Quest rejects energy cost above 5`() {
        assertFailsWith<IllegalArgumentException> {
            createQuest(energyCost = 6)
        }
    }

    @Test
    fun `Quest accepts valid energy costs 1-5`() {
        (1..5).forEach { cost ->
            val quest = createQuest(energyCost = cost)
            assertTrue(quest.energyCost == cost)
        }
    }

    // Initiative validation tests
    @Test
    fun `Initiative rejects invalid hex color`() {
        assertFailsWith<IllegalArgumentException> {
            createInitiative(color = "red")
        }
        assertFailsWith<IllegalArgumentException> {
            createInitiative(color = "#GGG")
        }
        assertFailsWith<IllegalArgumentException> {
            createInitiative(color = "#12345") // 5 chars - invalid
        }
    }

    @Test
    fun `Initiative accepts valid hex color formats`() {
        // 6-character format
        val initiative1 = createInitiative(color = "#FF5733")
        assertTrue(initiative1.color == "#FF5733")

        // 3-character shorthand format
        val initiative2 = createInitiative(color = "#F53")
        assertTrue(initiative2.color == "#F53")

        // 8-character format with alpha
        val initiative3 = createInitiative(color = "#FF5733CC")
        assertTrue(initiative3.color == "#FF5733CC")
    }

    // Note validation tests
    @Test
    fun `Note rejects blank title`() {
        assertFailsWith<IllegalArgumentException> {
            createNote(title = "")
        }
    }

    // NoteLink validation tests
    @Test
    fun `NoteLink rejects self-referencing link`() {
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

    // Folder validation tests
    @Test
    fun `Folder rejects self as parent`() {
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

    // Tag validation tests
    @Test
    fun `Tag rejects spaces in name`() {
        assertFailsWith<IllegalArgumentException> {
            Tag(
                id = Ulid.generate(),
                userId = userId,
                name = "invalid tag",
                color = null,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    // Attachment validation tests
    @Test
    fun `Attachment requires either noteId or inboxItemId`() {
        assertFailsWith<IllegalArgumentException> {
            Attachment(
                id = Ulid.generate(),
                userId = userId,
                noteId = null,
                inboxItemId = null,
                filename = "test.pdf",
                mimeType = "application/pdf",
                sizeBytes = 1000,
                storagePath = "/path/to/file",
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    // Location validation tests
    @Test
    fun `Location rejects self as parent`() {
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

    // Container validation tests
    @Test
    fun `Container rejects self as parent`() {
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

    // FieldDefinition validation tests
    @Test
    fun `FieldDefinition enum type requires options`() {
        assertFailsWith<IllegalArgumentException> {
            FieldDefinition(
                id = Ulid.generate(),
                userId = userId,
                templateId = Ulid.generate(),
                name = "Category",
                fieldType = FieldType.ENUM,
                isRequired = false,
                defaultValue = null,
                enumOptions = null,
                sortOrder = 0,
                createdAt = now,
                updatedAt = now,
            )
        }
        assertFailsWith<IllegalArgumentException> {
            FieldDefinition(
                id = Ulid.generate(),
                userId = userId,
                templateId = Ulid.generate(),
                name = "Category",
                fieldType = FieldType.ENUM,
                isRequired = false,
                defaultValue = null,
                enumOptions = emptyList(),
                sortOrder = 0,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    // EnergyBudget tests
    @Test
    fun `EnergyBudget calculates remaining energy correctly`() {
        val budget = EnergyBudget(
            id = Ulid.generate(),
            userId = userId,
            date = LocalDate(2024, 1, 15),
            totalBudget = 10,
            spentEnergy = 3,
            createdAt = now,
            updatedAt = now,
        )
        assertTrue(budget.remainingEnergy == 7)
        assertTrue(!budget.isOverBudget)
    }

    @Test
    fun `EnergyBudget detects over budget`() {
        val budget = EnergyBudget(
            id = Ulid.generate(),
            userId = userId,
            date = LocalDate(2024, 1, 15),
            totalBudget = 5,
            spentEnergy = 8,
            createdAt = now,
            updatedAt = now,
        )
        assertTrue(budget.remainingEnergy == 0)
        assertTrue(budget.isOverBudget)
    }

    // Helper functions
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
