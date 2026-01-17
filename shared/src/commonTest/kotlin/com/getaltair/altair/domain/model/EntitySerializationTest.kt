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
import com.getaltair.altair.domain.model.system.SourceAnnotation
import com.getaltair.altair.domain.model.system.SourceDocument
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.model.system.WatchedFolder
import com.getaltair.altair.domain.model.tracking.Container
import com.getaltair.altair.domain.model.tracking.CustomField
import com.getaltair.altair.domain.model.tracking.FieldDefinition
import com.getaltair.altair.domain.model.tracking.Item
import com.getaltair.altair.domain.model.tracking.ItemTemplate
import com.getaltair.altair.domain.model.tracking.Location
import com.getaltair.altair.domain.types.Schedule
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.AnchorType
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

    @Test
    fun `Checkpoint round-trips through JSON`() {
        val checkpoint = Checkpoint(
            id = Ulid.generate(),
            userId = userId,
            questId = Ulid.generate(),
            title = "Step 1",
            sortOrder = 0,
            isCompleted = true,
            completedAt = now,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(checkpoint)
        val deserialized = json.decodeFromString<Checkpoint>(serialized)
        assertEquals(checkpoint, deserialized)
    }

    @Test
    fun `Folder round-trips through JSON`() {
        val folder = Folder(
            id = Ulid.generate(),
            userId = userId,
            name = "My Notes",
            parentId = null,
            sortOrder = 0,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(folder)
        val deserialized = json.decodeFromString<Folder>(serialized)
        assertEquals(folder, deserialized)
    }

    @Test
    fun `NoteLink round-trips through JSON`() {
        val noteLink = NoteLink(
            id = Ulid.generate(),
            userId = userId,
            sourceNoteId = Ulid.generate(),
            targetNoteId = Ulid.generate(),
            context = "Related concept",
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(noteLink)
        val deserialized = json.decodeFromString<NoteLink>(serialized)
        assertEquals(noteLink, deserialized)
    }

    @Test
    fun `Container round-trips through JSON`() {
        val container = Container(
            id = Ulid.generate(),
            userId = userId,
            name = "Storage Box",
            description = "A large cardboard box",
            locationId = Ulid.generate(),
            parentContainerId = null,
            label = "Box-001",
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(container)
        val deserialized = json.decodeFromString<Container>(serialized)
        assertEquals(container, deserialized)
    }

    @Test
    fun `Location round-trips through JSON`() {
        val location = Location(
            id = Ulid.generate(),
            userId = userId,
            name = "Home Office",
            description = "Work from home space",
            parentId = null,
            address = "123 Main St",
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(location)
        val deserialized = json.decodeFromString<Location>(serialized)
        assertEquals(location, deserialized)
    }

    @Test
    fun `ItemTemplate round-trips through JSON`() {
        val template = ItemTemplate(
            id = Ulid.generate(),
            userId = userId,
            name = "Book",
            description = "Template for tracking books",
            icon = "book",
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(template)
        val deserialized = json.decodeFromString<ItemTemplate>(serialized)
        assertEquals(template, deserialized)
    }

    @Test
    fun `CustomField round-trips through JSON`() {
        val customField = CustomField(
            id = Ulid.generate(),
            userId = userId,
            itemId = Ulid.generate(),
            fieldDefinitionId = Ulid.generate(),
            value = "Some value",
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(customField)
        val deserialized = json.decodeFromString<CustomField>(serialized)
        assertEquals(customField, deserialized)
    }

    @Test
    fun `WatchedFolder round-trips through JSON`() {
        val watchedFolder = WatchedFolder(
            id = Ulid.generate(),
            userId = userId,
            path = "/home/user/documents",
            name = "Documents",
            isActive = true,
            includeSubfolders = true,
            filePatterns = listOf("*.pdf", "*.epub"),
            lastScannedAt = now,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(watchedFolder)
        val deserialized = json.decodeFromString<WatchedFolder>(serialized)
        assertEquals(watchedFolder, deserialized)
    }

    @Test
    fun `ExtractionJob round-trips through JSON`() {
        val job = ExtractionJob(
            id = Ulid.generate(),
            userId = userId,
            sourceDocumentId = Ulid.generate(),
            status = JobStatus.COMPLETED,
            progress = 100,
            errorMessage = null,
            startedAt = now,
            completedAt = now,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(job)
        val deserialized = json.decodeFromString<ExtractionJob>(serialized)
        assertEquals(job, deserialized)
    }

    @Test
    fun `Attachment round-trips through JSON`() {
        val attachment = Attachment(
            id = Ulid.generate(),
            userId = userId,
            noteId = Ulid.generate(),
            inboxItemId = null,
            filename = "document.pdf",
            mimeType = "application/pdf",
            sizeBytes = 1_024_000,
            storagePath = "/attachments/abc123.pdf",
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(attachment)
        val deserialized = json.decodeFromString<Attachment>(serialized)
        assertEquals(attachment, deserialized)
    }

    @Test
    fun `EnergyBudget round-trips through JSON`() {
        val budget = EnergyBudget(
            id = Ulid.generate(),
            userId = userId,
            date = LocalDate(2024, 6, 15),
            totalBudget = 10,
            spentEnergy = 4,
            createdAt = now,
            updatedAt = now,
        )
        val serialized = json.encodeToString(budget)
        val deserialized = json.decodeFromString<EnergyBudget>(serialized)
        assertEquals(budget, deserialized)
    }
}
