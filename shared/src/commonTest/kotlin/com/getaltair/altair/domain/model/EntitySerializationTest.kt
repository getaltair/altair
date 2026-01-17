package com.getaltair.altair.domain.model

import com.getaltair.altair.domain.model.guidance.Epic
import com.getaltair.altair.domain.model.guidance.Quest
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.model.knowledge.Tag
import com.getaltair.altair.domain.model.system.InboxItem
import com.getaltair.altair.domain.model.system.Initiative
import com.getaltair.altair.domain.model.system.Routine
import com.getaltair.altair.domain.model.system.SourceAnnotation
import com.getaltair.altair.domain.model.system.SourceDocument
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.model.tracking.FieldDefinition
import com.getaltair.altair.domain.model.tracking.Item
import com.getaltair.altair.domain.types.Schedule
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.AnchorType
import com.getaltair.altair.domain.types.enums.CaptureSource
import com.getaltair.altair.domain.types.enums.EpicStatus
import com.getaltair.altair.domain.types.enums.ExtractionStatus
import com.getaltair.altair.domain.types.enums.FieldType
import com.getaltair.altair.domain.types.enums.InitiativeStatus
import com.getaltair.altair.domain.types.enums.QuestStatus
import com.getaltair.altair.domain.types.enums.SourceType
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

class EntitySerializationTest {
    private val json = Json { prettyPrint = false }
    private val now: Instant = Instant.fromEpochMilliseconds(kotlin.time.Clock.System.now().toEpochMilliseconds())
    private val userId = Ulid.generate()

    @Test
    fun `User round-trips through JSON`() {
        val user = User(
            id = Ulid.generate(),
            email = "test@example.com",
            displayName = "Test User",
            role = UserRole.MEMBER,
            status = UserStatus.ACTIVE,
            storageUsedBytes = 1024,
            storageQuotaBytes = 1_000_000_000,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(user)
        val deserialized = json.decodeFromString<User>(serialized)
        assertEquals(user, deserialized)
    }

    @Test
    fun `Initiative round-trips through JSON`() {
        val initiative = Initiative(
            id = Ulid.generate(),
            userId = userId,
            name = "Project Alpha",
            description = "A test project",
            color = "#FF5733",
            icon = "rocket",
            status = InitiativeStatus.ACTIVE,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(initiative)
        val deserialized = json.decodeFromString<Initiative>(serialized)
        assertEquals(initiative, deserialized)
    }

    @Test
    fun `InboxItem round-trips through JSON`() {
        val item = InboxItem(
            id = Ulid.generate(),
            userId = userId,
            content = "Remember to buy milk",
            source = CaptureSource.VOICE,
            attachmentIds = listOf(Ulid.generate()),
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(item)
        val deserialized = json.decodeFromString<InboxItem>(serialized)
        assertEquals(item, deserialized)
    }

    @Test
    fun `Routine with Schedule round-trips through JSON`() {
        val routine = Routine(
            id = Ulid.generate(),
            userId = userId,
            title = "Morning exercise",
            description = "30 min workout",
            energyCost = 3,
            schedule = Schedule.Daily,
            scheduledTime = null,
            initiativeId = null,
            isActive = true,
            lastSpawnedAt = null,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(routine)
        val deserialized = json.decodeFromString<Routine>(serialized)
        assertEquals(routine, deserialized)
    }

    @Test
    fun `Quest round-trips through JSON`() {
        val quest = Quest(
            id = Ulid.generate(),
            userId = userId,
            title = "Complete task",
            description = "Description here",
            energyCost = 2,
            status = QuestStatus.ACTIVE,
            epicId = null,
            routineId = null,
            initiativeId = null,
            dueDate = LocalDate(2024, 6, 15),
            scheduledDate = null,
            createdAt = now,
            updatedAt = now,
            startedAt = now,
            completedAt = null,
        )
        val serialized = json.encodeToString(quest)
        val deserialized = json.decodeFromString<Quest>(serialized)
        assertEquals(quest, deserialized)
    }

    @Test
    fun `Epic round-trips through JSON`() {
        val epic = Epic(
            id = Ulid.generate(),
            userId = userId,
            title = "Launch MVP",
            description = "Ship the first version",
            status = EpicStatus.ACTIVE,
            initiativeId = null,
            targetDate = LocalDate(2024, 12, 31),
            createdAt = now,
            updatedAt = now,
            completedAt = null,
        )
        val serialized = json.encodeToString(epic)
        val deserialized = json.decodeFromString<Epic>(serialized)
        assertEquals(epic, deserialized)
    }

    @Test
    fun `Note round-trips through JSON`() {
        val note = Note(
            id = Ulid.generate(),
            userId = userId,
            title = "Meeting notes",
            content = "# Notes\n\n- Item 1\n- Item 2",
            folderId = null,
            initiativeId = null,
            isPinned = true,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(note)
        val deserialized = json.decodeFromString<Note>(serialized)
        assertEquals(note, deserialized)
    }

    @Test
    fun `Tag round-trips through JSON`() {
        val tag = Tag(
            id = Ulid.generate(),
            userId = userId,
            name = "important",
            color = "#FF0000",
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(tag)
        val deserialized = json.decodeFromString<Tag>(serialized)
        assertEquals(tag, deserialized)
    }

    @Test
    fun `Item round-trips through JSON`() {
        val item = Item(
            id = Ulid.generate(),
            userId = userId,
            name = "Laptop",
            description = "MacBook Pro 14",
            templateId = null,
            locationId = null,
            containerId = null,
            quantity = 1,
            photoAttachmentId = null,
            initiativeId = null,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(item)
        val deserialized = json.decodeFromString<Item>(serialized)
        assertEquals(item, deserialized)
    }

    @Test
    fun `SourceDocument round-trips through JSON`() {
        val doc = SourceDocument(
            id = Ulid.generate(),
            userId = userId,
            title = "Research Paper",
            sourceType = SourceType.FILE,
            sourcePath = "/documents/paper.pdf",
            mimeType = "application/pdf",
            fileSizeBytes = 1_024_000,
            pageCount = 15,
            extractionStatus = ExtractionStatus.COMPLETED,
            extractedText = "Extracted content...",
            watchedFolderId = null,
            initiativeId = null,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(doc)
        val deserialized = json.decodeFromString<SourceDocument>(serialized)
        assertEquals(doc, deserialized)
    }

    @Test
    fun `SourceAnnotation round-trips through JSON`() {
        val annotation = SourceAnnotation(
            id = Ulid.generate(),
            userId = userId,
            sourceDocumentId = Ulid.generate(),
            anchorType = AnchorType.SELECTION,
            anchorData = """{"start":100,"end":200}""",
            content = "Important point here",
            highlightColor = "#FFFF00",
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(annotation)
        val deserialized = json.decodeFromString<SourceAnnotation>(serialized)
        assertEquals(annotation, deserialized)
    }

    @Test
    fun `FieldDefinition with enum options round-trips through JSON`() {
        val fieldDef = FieldDefinition(
            id = Ulid.generate(),
            userId = userId,
            templateId = Ulid.generate(),
            name = "Condition",
            fieldType = FieldType.ENUM,
            isRequired = true,
            defaultValue = "good",
            enumOptions = listOf("new", "good", "fair", "poor"),
            sortOrder = 0,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(fieldDef)
        val deserialized = json.decodeFromString<FieldDefinition>(serialized)
        assertEquals(fieldDef, deserialized)
    }

    @Test
    fun `all enum SerialNames are lowercase`() {
        // Verify enums serialize to lowercase
        assertEquals(""""admin"""", json.encodeToString(UserRole.ADMIN))
        assertEquals(""""member"""", json.encodeToString(UserRole.MEMBER))
        assertEquals(""""active"""", json.encodeToString(UserStatus.ACTIVE))
        assertEquals(""""backlog"""", json.encodeToString(QuestStatus.BACKLOG))
        assertEquals(""""keyboard"""", json.encodeToString(CaptureSource.KEYBOARD))
    }
}
