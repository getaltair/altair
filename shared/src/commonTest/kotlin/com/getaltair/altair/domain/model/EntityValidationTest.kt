package com.getaltair.altair.domain.model

import com.getaltair.altair.domain.model.guidance.Checkpoint
import com.getaltair.altair.domain.model.guidance.EnergyBudget
import com.getaltair.altair.domain.model.guidance.Epic
import com.getaltair.altair.domain.model.guidance.Quest
import com.getaltair.altair.domain.model.knowledge.Attachment
import com.getaltair.altair.domain.model.knowledge.Folder
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.model.knowledge.NoteLink
import com.getaltair.altair.domain.model.knowledge.Tag
import com.getaltair.altair.domain.model.system.ExtractionJob
import com.getaltair.altair.domain.model.system.InboxItem
import com.getaltair.altair.domain.model.system.Initiative
import com.getaltair.altair.domain.model.system.Routine
import com.getaltair.altair.domain.model.system.SourceDocument
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.model.system.WatchedFolder
import com.getaltair.altair.domain.model.tracking.Container
import com.getaltair.altair.domain.model.tracking.FieldDefinition
import com.getaltair.altair.domain.model.tracking.Item
import com.getaltair.altair.domain.model.tracking.ItemTemplate
import com.getaltair.altair.domain.model.tracking.Location
import com.getaltair.altair.domain.types.Schedule
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.CaptureSource
import com.getaltair.altair.domain.types.enums.EpicStatus
import com.getaltair.altair.domain.types.enums.ExtractionStatus
import com.getaltair.altair.domain.types.enums.FieldType
import com.getaltair.altair.domain.types.enums.InitiativeStatus
import com.getaltair.altair.domain.types.enums.JobStatus
import com.getaltair.altair.domain.types.enums.QuestStatus
import com.getaltair.altair.domain.types.enums.SourceType
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate
import kotlin.test.Test
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue

@Suppress("LargeClass") // Test class covering validation for all domain entities
class EntityValidationTest {
    private val now: Instant =
        Instant.fromEpochMilliseconds(
            kotlin.time.Clock.System
                .now()
                .toEpochMilliseconds(),
        )
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
        val budget =
            EnergyBudget(
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
        val budget =
            EnergyBudget(
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

    private fun createNote(title: String = "Test Note") =
        Note(
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

    // Epic validation tests
    @Test
    fun `Epic rejects blank title`() {
        assertFailsWith<IllegalArgumentException> {
            createEpic(title = "")
        }
    }

    @Test
    fun `Epic rejects title over 200 characters`() {
        assertFailsWith<IllegalArgumentException> {
            createEpic(title = "a".repeat(201))
        }
    }

    @Test
    fun `Epic requires completedAt when status is COMPLETED`() {
        assertFailsWith<IllegalArgumentException> {
            Epic(
                id = Ulid.generate(),
                userId = userId,
                title = "Test Epic",
                description = null,
                status = EpicStatus.COMPLETED,
                initiativeId = null,
                targetDate = null,
                createdAt = now,
                updatedAt = now,
                completedAt = null,
            )
        }
    }

    @Test
    fun `Epic accepts completed status with completedAt`() {
        val epic =
            Epic(
                id = Ulid.generate(),
                userId = userId,
                title = "Test Epic",
                description = null,
                status = EpicStatus.COMPLETED,
                initiativeId = null,
                targetDate = null,
                createdAt = now,
                updatedAt = now,
                completedAt = now,
            )
        assertTrue(epic.status == EpicStatus.COMPLETED)
    }

    // Routine validation tests
    @Test
    fun `Routine rejects blank title`() {
        assertFailsWith<IllegalArgumentException> {
            createRoutine(title = "")
        }
    }

    @Test
    fun `Routine rejects title over 200 characters`() {
        assertFailsWith<IllegalArgumentException> {
            createRoutine(title = "a".repeat(201))
        }
    }

    @Test
    fun `Routine rejects energy cost below 1`() {
        assertFailsWith<IllegalArgumentException> {
            createRoutine(energyCost = 0)
        }
    }

    @Test
    fun `Routine rejects energy cost above 5`() {
        assertFailsWith<IllegalArgumentException> {
            createRoutine(energyCost = 6)
        }
    }

    // Checkpoint validation tests
    @Test
    fun `Checkpoint rejects blank title`() {
        assertFailsWith<IllegalArgumentException> {
            createCheckpoint(title = "")
        }
    }

    @Test
    fun `Checkpoint rejects title over 200 characters`() {
        assertFailsWith<IllegalArgumentException> {
            createCheckpoint(title = "a".repeat(201))
        }
    }

    @Test
    fun `Checkpoint rejects negative sortOrder`() {
        assertFailsWith<IllegalArgumentException> {
            createCheckpoint(sortOrder = -1)
        }
    }

    @Test
    fun `Checkpoint requires completedAt when isCompleted is true`() {
        assertFailsWith<IllegalArgumentException> {
            Checkpoint(
                id = Ulid.generate(),
                userId = userId,
                questId = Ulid.generate(),
                title = "Test",
                sortOrder = 0,
                isCompleted = true,
                completedAt = null,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    @Test
    fun `Checkpoint rejects completedAt when isCompleted is false`() {
        assertFailsWith<IllegalArgumentException> {
            Checkpoint(
                id = Ulid.generate(),
                userId = userId,
                questId = Ulid.generate(),
                title = "Test",
                sortOrder = 0,
                isCompleted = false,
                completedAt = now,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    // InboxItem validation tests
    @Test
    fun `InboxItem rejects blank content`() {
        assertFailsWith<IllegalArgumentException> {
            createInboxItem(content = "")
        }
    }

    @Test
    fun `InboxItem rejects content over 5000 characters`() {
        assertFailsWith<IllegalArgumentException> {
            createInboxItem(content = "a".repeat(5001))
        }
    }

    // SourceDocument validation tests
    @Test
    fun `SourceDocument rejects blank title`() {
        assertFailsWith<IllegalArgumentException> {
            createSourceDocument(title = "")
        }
    }

    @Test
    fun `SourceDocument rejects title over 500 characters`() {
        assertFailsWith<IllegalArgumentException> {
            createSourceDocument(title = "a".repeat(501))
        }
    }

    @Test
    fun `SourceDocument rejects blank sourcePath`() {
        assertFailsWith<IllegalArgumentException> {
            createSourceDocument(sourcePath = "")
        }
    }

    @Test
    fun `SourceDocument rejects negative fileSize`() {
        assertFailsWith<IllegalArgumentException> {
            createSourceDocument(fileSizeBytes = -1)
        }
    }

    @Test
    fun `SourceDocument rejects negative pageCount`() {
        assertFailsWith<IllegalArgumentException> {
            createSourceDocument(pageCount = -1)
        }
    }

    // Item validation tests
    @Test
    fun `Item rejects blank name`() {
        assertFailsWith<IllegalArgumentException> {
            createItem(name = "")
        }
    }

    @Test
    fun `Item rejects name over 200 characters`() {
        assertFailsWith<IllegalArgumentException> {
            createItem(name = "a".repeat(201))
        }
    }

    @Test
    fun `Item rejects negative quantity`() {
        assertFailsWith<IllegalArgumentException> {
            createItem(quantity = -1)
        }
    }

    // ExtractionJob validation tests
    @Test
    fun `ExtractionJob rejects progress below 0`() {
        assertFailsWith<IllegalArgumentException> {
            createExtractionJob(progress = -1, status = JobStatus.PROCESSING)
        }
    }

    @Test
    fun `ExtractionJob rejects progress above 100`() {
        assertFailsWith<IllegalArgumentException> {
            createExtractionJob(progress = 101, status = JobStatus.PROCESSING)
        }
    }

    @Test
    fun `ExtractionJob QUEUED status rejects startedAt`() {
        assertFailsWith<IllegalArgumentException> {
            ExtractionJob(
                id = Ulid.generate(),
                userId = userId,
                sourceDocumentId = Ulid.generate(),
                status = JobStatus.QUEUED,
                progress = 0,
                errorMessage = null,
                startedAt = now,
                completedAt = null,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    @Test
    fun `ExtractionJob PROCESSING status requires startedAt`() {
        assertFailsWith<IllegalArgumentException> {
            ExtractionJob(
                id = Ulid.generate(),
                userId = userId,
                sourceDocumentId = Ulid.generate(),
                status = JobStatus.PROCESSING,
                progress = 50,
                errorMessage = null,
                startedAt = null,
                completedAt = null,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    @Test
    fun `ExtractionJob COMPLETED status requires progress 100`() {
        assertFailsWith<IllegalArgumentException> {
            ExtractionJob(
                id = Ulid.generate(),
                userId = userId,
                sourceDocumentId = Ulid.generate(),
                status = JobStatus.COMPLETED,
                progress = 50,
                errorMessage = null,
                startedAt = now,
                completedAt = now,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    @Test
    fun `ExtractionJob FAILED status requires errorMessage`() {
        assertFailsWith<IllegalArgumentException> {
            ExtractionJob(
                id = Ulid.generate(),
                userId = userId,
                sourceDocumentId = Ulid.generate(),
                status = JobStatus.FAILED,
                progress = 50,
                errorMessage = null,
                startedAt = now,
                completedAt = now,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    // EnergyBudget validation tests
    @Test
    fun `EnergyBudget rejects negative totalBudget`() {
        assertFailsWith<IllegalArgumentException> {
            EnergyBudget(
                id = Ulid.generate(),
                userId = userId,
                date = LocalDate(2024, 1, 15),
                totalBudget = -1,
                spentEnergy = 0,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    @Test
    fun `EnergyBudget rejects negative spentEnergy`() {
        assertFailsWith<IllegalArgumentException> {
            EnergyBudget(
                id = Ulid.generate(),
                userId = userId,
                date = LocalDate(2024, 1, 15),
                totalBudget = 10,
                spentEnergy = -1,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    // NoteLink validation tests
    @Test
    fun `NoteLink rejects context over 500 characters`() {
        val sourceId = Ulid.generate()
        val targetId = Ulid.generate()
        assertFailsWith<IllegalArgumentException> {
            NoteLink(
                id = Ulid.generate(),
                userId = userId,
                sourceNoteId = sourceId,
                targetNoteId = targetId,
                context = "a".repeat(501),
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    // FieldDefinition validation tests
    @Test
    fun `FieldDefinition non-ENUM type rejects enumOptions`() {
        assertFailsWith<IllegalArgumentException> {
            FieldDefinition(
                id = Ulid.generate(),
                userId = userId,
                templateId = Ulid.generate(),
                name = "Notes",
                fieldType = FieldType.TEXT,
                isRequired = false,
                defaultValue = null,
                enumOptions = listOf("option1", "option2"),
                sortOrder = 0,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    // Quest state consistency tests
    @Test
    fun `Quest ACTIVE status requires startedAt`() {
        assertFailsWith<IllegalArgumentException> {
            Quest(
                id = Ulid.generate(),
                userId = userId,
                title = "Test Quest",
                description = null,
                energyCost = 2,
                status = QuestStatus.ACTIVE,
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
        }
    }

    @Test
    fun `Quest COMPLETED status requires completedAt`() {
        assertFailsWith<IllegalArgumentException> {
            Quest(
                id = Ulid.generate(),
                userId = userId,
                title = "Test Quest",
                description = null,
                energyCost = 2,
                status = QuestStatus.COMPLETED,
                epicId = null,
                routineId = null,
                initiativeId = null,
                dueDate = null,
                scheduledDate = null,
                createdAt = now,
                updatedAt = now,
                startedAt = now,
                completedAt = null,
            )
        }
    }

    @Test
    fun `Quest rejects startedAt after completedAt`() {
        val earlier = Instant.fromEpochMilliseconds(1000)
        val later = Instant.fromEpochMilliseconds(2000)
        assertFailsWith<IllegalArgumentException> {
            Quest(
                id = Ulid.generate(),
                userId = userId,
                title = "Test Quest",
                description = null,
                energyCost = 2,
                status = QuestStatus.COMPLETED,
                epicId = null,
                routineId = null,
                initiativeId = null,
                dueDate = null,
                scheduledDate = null,
                createdAt = earlier,
                updatedAt = later,
                startedAt = later,
                completedAt = earlier,
            )
        }
    }

    // WatchedFolder validation tests
    @Test
    fun `WatchedFolder rejects blank path`() {
        assertFailsWith<IllegalArgumentException> {
            WatchedFolder(
                id = Ulid.generate(),
                userId = userId,
                path = "",
                name = "Test",
                isActive = true,
                includeSubfolders = false,
                filePatterns = listOf("*.pdf"),
                lastScannedAt = null,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    @Test
    fun `WatchedFolder rejects blank name`() {
        assertFailsWith<IllegalArgumentException> {
            WatchedFolder(
                id = Ulid.generate(),
                userId = userId,
                path = "/path/to/folder",
                name = "",
                isActive = true,
                includeSubfolders = false,
                filePatterns = listOf("*.pdf"),
                lastScannedAt = null,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    @Test
    fun `WatchedFolder rejects name over 100 characters`() {
        assertFailsWith<IllegalArgumentException> {
            WatchedFolder(
                id = Ulid.generate(),
                userId = userId,
                path = "/path/to/folder",
                name = "a".repeat(101),
                isActive = true,
                includeSubfolders = false,
                filePatterns = listOf("*.pdf"),
                lastScannedAt = null,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    // ItemTemplate validation tests
    @Test
    fun `ItemTemplate rejects blank name`() {
        assertFailsWith<IllegalArgumentException> {
            ItemTemplate(
                id = Ulid.generate(),
                userId = userId,
                name = "",
                description = null,
                icon = null,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    @Test
    fun `ItemTemplate rejects name over 100 characters`() {
        assertFailsWith<IllegalArgumentException> {
            ItemTemplate(
                id = Ulid.generate(),
                userId = userId,
                name = "a".repeat(101),
                description = null,
                icon = null,
                createdAt = now,
                updatedAt = now,
            )
        }
    }

    // Additional helper functions
    private fun createEpic(title: String = "Test Epic") =
        Epic(
            id = Ulid.generate(),
            userId = userId,
            title = title,
            description = null,
            status = EpicStatus.ACTIVE,
            initiativeId = null,
            targetDate = null,
            createdAt = now,
            updatedAt = now,
            completedAt = null,
        )

    private fun createRoutine(
        title: String = "Test Routine",
        energyCost: Int = 2,
    ) = Routine(
        id = Ulid.generate(),
        userId = userId,
        title = title,
        description = null,
        energyCost = energyCost,
        schedule = Schedule.Daily,
        scheduledTime = null,
        initiativeId = null,
        isActive = true,
        lastSpawnedAt = null,
        createdAt = now,
        updatedAt = now,
    )

    private fun createCheckpoint(
        title: String = "Test Checkpoint",
        sortOrder: Int = 0,
    ) = Checkpoint(
        id = Ulid.generate(),
        userId = userId,
        questId = Ulid.generate(),
        title = title,
        sortOrder = sortOrder,
        isCompleted = false,
        completedAt = null,
        createdAt = now,
        updatedAt = now,
    )

    private fun createInboxItem(content: String = "Test content") =
        InboxItem(
            id = Ulid.generate(),
            userId = userId,
            content = content,
            source = CaptureSource.KEYBOARD,
            attachmentIds = emptyList(),
            createdAt = now,
            updatedAt = now,
        )

    private fun createSourceDocument(
        title: String = "Test Document",
        sourcePath: String = "/path/to/doc.pdf",
        fileSizeBytes: Long? = 1024,
        pageCount: Int? = 10,
    ) = SourceDocument(
        id = Ulid.generate(),
        userId = userId,
        title = title,
        sourceType = SourceType.FILE,
        sourcePath = sourcePath,
        mimeType = "application/pdf",
        fileSizeBytes = fileSizeBytes,
        pageCount = pageCount,
        extractionStatus = ExtractionStatus.PENDING,
        extractedText = null,
        watchedFolderId = null,
        initiativeId = null,
        createdAt = now,
        updatedAt = now,
    )

    private fun createItem(
        name: String = "Test Item",
        quantity: Int = 1,
    ) = Item(
        id = Ulid.generate(),
        userId = userId,
        name = name,
        description = null,
        templateId = null,
        locationId = null,
        containerId = null,
        quantity = quantity,
        photoAttachmentId = null,
        initiativeId = null,
        createdAt = now,
        updatedAt = now,
    )

    private fun createExtractionJob(
        progress: Int = 0,
        status: JobStatus = JobStatus.QUEUED,
    ) = ExtractionJob(
        id = Ulid.generate(),
        userId = userId,
        sourceDocumentId = Ulid.generate(),
        status = status,
        progress = progress,
        errorMessage = if (status == JobStatus.FAILED) "Error" else null,
        startedAt = if (status != JobStatus.QUEUED) now else null,
        completedAt = if (status == JobStatus.COMPLETED || status == JobStatus.FAILED) now else null,
        createdAt = now,
        updatedAt = now,
    )
}
