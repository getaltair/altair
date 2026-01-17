package com.getaltair.altair.dto

import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.ChangePasswordRequest
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.TokenRefreshRequest
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import com.getaltair.altair.dto.guidance.CheckpointResponse
import com.getaltair.altair.dto.guidance.CreateCheckpointRequest
import com.getaltair.altair.dto.guidance.CreateEpicRequest
import com.getaltair.altair.dto.guidance.CreateQuestRequest
import com.getaltair.altair.dto.guidance.EpicProgressResponse
import com.getaltair.altair.dto.guidance.EpicResponse
import com.getaltair.altair.dto.guidance.QuestResponse
import com.getaltair.altair.dto.guidance.UpdateQuestRequest
import com.getaltair.altair.dto.knowledge.CreateFolderRequest
import com.getaltair.altair.dto.knowledge.CreateNoteRequest
import com.getaltair.altair.dto.knowledge.CreateTagRequest
import com.getaltair.altair.dto.knowledge.FolderResponse
import com.getaltair.altair.dto.knowledge.LinkNotesRequest
import com.getaltair.altair.dto.knowledge.NoteResponse
import com.getaltair.altair.dto.knowledge.TagResponse
import com.getaltair.altair.dto.sync.ChangeOperation
import com.getaltair.altair.dto.sync.ChangeSet
import com.getaltair.altair.dto.sync.ConflictInfo
import com.getaltair.altair.dto.sync.ConflictResolution
import com.getaltair.altair.dto.sync.EntityChange
import com.getaltair.altair.dto.sync.SyncRequest
import com.getaltair.altair.dto.sync.SyncResponse
import com.getaltair.altair.dto.system.CaptureRequest
import com.getaltair.altair.dto.system.CreateInitiativeRequest
import com.getaltair.altair.dto.system.CreateRoutineRequest
import com.getaltair.altair.dto.system.InboxItemResponse
import com.getaltair.altair.dto.system.InitiativeResponse
import com.getaltair.altair.dto.system.RoutineResponse
import com.getaltair.altair.dto.system.ScheduleRequest
import com.getaltair.altair.dto.system.ScheduleResponse
import com.getaltair.altair.dto.tracking.ContainerResponse
import com.getaltair.altair.dto.tracking.CreateContainerRequest
import com.getaltair.altair.dto.tracking.CreateItemRequest
import com.getaltair.altair.dto.tracking.CreateItemTemplateRequest
import com.getaltair.altair.dto.tracking.CreateLocationRequest
import com.getaltair.altair.dto.tracking.FieldDefinitionRequest
import com.getaltair.altair.dto.tracking.ItemResponse
import com.getaltair.altair.dto.tracking.ItemTemplateResponse
import com.getaltair.altair.dto.tracking.LocationResponse
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonPrimitive
import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * Tests for DTO serialization round-trips.
 */
class DtoSerializationTest {
    private val json = Json { prettyPrint = false }

    // Auth DTOs

    @Test
    fun `AuthRequest round-trips through JSON`() {
        val request = AuthRequest(
            email = "test@example.com",
            password = "secret123",
        )
        val serialized = json.encodeToString(request)
        val deserialized = json.decodeFromString<AuthRequest>(serialized)
        assertEquals(request, deserialized)
    }

    @Test
    fun `AuthResponse round-trips through JSON`() {
        val response = AuthResponse(
            accessToken = "access.token.here",
            refreshToken = "refresh.token.here",
            expiresIn = 3600,
            userId = "01HYQVK...",
            displayName = "Test User",
            role = "member",
        )
        val serialized = json.encodeToString(response)
        val deserialized = json.decodeFromString<AuthResponse>(serialized)
        assertEquals(response, deserialized)
    }

    @Test
    fun `TokenRefreshRequest round-trips through JSON`() {
        val request = TokenRefreshRequest(refreshToken = "refresh.token.here")
        val serialized = json.encodeToString(request)
        val deserialized = json.decodeFromString<TokenRefreshRequest>(serialized)
        assertEquals(request, deserialized)
    }

    @Test
    fun `RegisterRequest round-trips with optional inviteCode`() {
        val withCode = RegisterRequest(
            email = "test@example.com",
            password = "secret123",
            displayName = "Test User",
            inviteCode = "ABC123",
        )
        val withoutCode = RegisterRequest(
            email = "test@example.com",
            password = "secret123",
            displayName = "Test User",
        )

        assertEquals(withCode, json.decodeFromString(json.encodeToString(withCode)))
        assertEquals(withoutCode, json.decodeFromString(json.encodeToString(withoutCode)))
    }

    // Sync DTOs

    @Test
    fun `SyncRequest round-trips through JSON`() {
        val request = SyncRequest(
            clientId = "client-123",
            lastSyncVersion = 42,
            changes = listOf(
                EntityChange(
                    entityType = "Quest",
                    entityId = "01HYQVK...",
                    operation = ChangeOperation.UPDATE,
                    version = 5,
                    data = JsonPrimitive("data here"),
                    timestamp = 1700000000000,
                ),
            ),
        )
        val serialized = json.encodeToString(request)
        val deserialized = json.decodeFromString<SyncRequest>(serialized)
        assertEquals(request, deserialized)
    }

    @Test
    fun `SyncResponse round-trips through JSON`() {
        val response = SyncResponse(
            serverVersion = 50,
            changes = emptyList(),
            conflicts = listOf(
                ConflictInfo(
                    entityType = "Note",
                    entityId = "01HYQVK...",
                    clientVersion = 3,
                    serverVersion = 5,
                    clientData = JsonPrimitive("client"),
                    serverData = JsonPrimitive("server"),
                ),
            ),
            acknowledged = listOf("01HYQVK1", "01HYQVK2"),
        )
        val serialized = json.encodeToString(response)
        val deserialized = json.decodeFromString<SyncResponse>(serialized)
        assertEquals(response, deserialized)
    }

    @Test
    fun `ChangeOperation enum serializes to lowercase`() {
        assertEquals("\"create\"", json.encodeToString(ChangeOperation.CREATE))
        assertEquals("\"update\"", json.encodeToString(ChangeOperation.UPDATE))
        assertEquals("\"delete\"", json.encodeToString(ChangeOperation.DELETE))
    }

    @Test
    fun `ConflictResolution enum serializes to snake_case`() {
        assertEquals("\"keep_client\"", json.encodeToString(ConflictResolution.KEEP_CLIENT))
        assertEquals("\"keep_server\"", json.encodeToString(ConflictResolution.KEEP_SERVER))
        assertEquals("\"merge\"", json.encodeToString(ConflictResolution.MERGE))
    }

    // Guidance DTOs

    @Test
    fun `CreateQuestRequest round-trips with defaults`() {
        val minimal = CreateQuestRequest(
            title = "My Quest",
            energyCost = 3,
        )
        val full = CreateQuestRequest(
            title = "My Quest",
            description = "Description here",
            energyCost = 3,
            epicId = "01HYQVK...",
            initiativeId = "01HYQVK...",
            dueDate = "2024-06-15",
            scheduledDate = "2024-06-10",
        )

        assertEquals(minimal, json.decodeFromString(json.encodeToString(minimal)))
        assertEquals(full, json.decodeFromString(json.encodeToString(full)))
    }

    @Test
    fun `QuestResponse round-trips through JSON`() {
        val response = QuestResponse(
            id = "01HYQVK...",
            title = "My Quest",
            description = null,
            energyCost = 3,
            status = "active",
            epicId = null,
            routineId = null,
            initiativeId = null,
            dueDate = "2024-06-15",
            scheduledDate = null,
            createdAt = "2024-06-01T10:00:00Z",
            updatedAt = "2024-06-01T10:00:00Z",
            startedAt = "2024-06-01T11:00:00Z",
            completedAt = null,
        )
        val serialized = json.encodeToString(response)
        val deserialized = json.decodeFromString<QuestResponse>(serialized)
        assertEquals(response, deserialized)
    }

    @Test
    fun `EpicResponse with progress round-trips through JSON`() {
        val response = EpicResponse(
            id = "01HYQVK...",
            title = "My Epic",
            description = "Epic description",
            status = "active",
            initiativeId = null,
            targetDate = "2024-12-31",
            createdAt = "2024-01-01T00:00:00Z",
            updatedAt = "2024-06-01T00:00:00Z",
            completedAt = null,
            progress = EpicProgressResponse(
                totalQuests = 10,
                completedQuests = 3,
                totalEnergy = 35,
                spentEnergy = 12,
                completionPercent = 30,
            ),
        )
        val serialized = json.encodeToString(response)
        val deserialized = json.decodeFromString<EpicResponse>(serialized)
        assertEquals(response, deserialized)
    }

    // Knowledge DTOs

    @Test
    fun `CreateNoteRequest round-trips with defaults`() {
        val minimal = CreateNoteRequest(title = "My Note")
        val full = CreateNoteRequest(
            title = "My Note",
            content = "# Content\n\nBody here",
            folderId = "01HYQVK...",
            initiativeId = "01HYQVK...",
            isPinned = true,
            tagIds = listOf("tag1", "tag2"),
        )

        assertEquals(minimal, json.decodeFromString(json.encodeToString(minimal)))
        assertEquals(full, json.decodeFromString(json.encodeToString(full)))
    }

    @Test
    fun `NoteResponse round-trips through JSON`() {
        val response = NoteResponse(
            id = "01HYQVK...",
            title = "My Note",
            content = "Content here",
            folderId = null,
            initiativeId = null,
            isPinned = false,
            createdAt = "2024-06-01T00:00:00Z",
            updatedAt = "2024-06-01T00:00:00Z",
            tagIds = listOf("tag1"),
            forwardLinkCount = 3,
            backlinkCount = 2,
        )
        val serialized = json.encodeToString(response)
        val deserialized = json.decodeFromString<NoteResponse>(serialized)
        assertEquals(response, deserialized)
    }

    @Test
    fun `LinkNotesRequest round-trips through JSON`() {
        val request = LinkNotesRequest(
            sourceNoteId = "01HYQVK1",
            targetNoteId = "01HYQVK2",
            context = "Related concept",
        )
        val serialized = json.encodeToString(request)
        val deserialized = json.decodeFromString<LinkNotesRequest>(serialized)
        assertEquals(request, deserialized)
    }

    // Tracking DTOs

    @Test
    fun `CreateItemRequest round-trips with custom fields`() {
        val request = CreateItemRequest(
            name = "Laptop",
            description = "Work laptop",
            templateId = "01HYQVK...",
            locationId = "01HYQVK...",
            quantity = 1,
            customFields = mapOf(
                "brand" to "Apple",
                "model" to "MacBook Pro 14",
            ),
        )
        val serialized = json.encodeToString(request)
        val deserialized = json.decodeFromString<CreateItemRequest>(serialized)
        assertEquals(request, deserialized)
    }

    @Test
    fun `ItemResponse round-trips through JSON`() {
        val response = ItemResponse(
            id = "01HYQVK...",
            name = "Laptop",
            description = "Work laptop",
            templateId = "01HYQVK...",
            locationId = "01HYQVK...",
            containerId = null,
            quantity = 1,
            photoAttachmentId = null,
            initiativeId = null,
            createdAt = "2024-06-01T00:00:00Z",
            updatedAt = "2024-06-01T00:00:00Z",
            customFields = mapOf("brand" to "Apple"),
            locationPath = "Home > Office",
            containerPath = null,
        )
        val serialized = json.encodeToString(response)
        val deserialized = json.decodeFromString<ItemResponse>(serialized)
        assertEquals(response, deserialized)
    }

    @Test
    fun `CreateItemTemplateRequest with fields round-trips through JSON`() {
        val request = CreateItemTemplateRequest(
            name = "Book",
            description = "Template for books",
            icon = "book",
            fields = listOf(
                FieldDefinitionRequest(
                    name = "Author",
                    fieldType = "text",
                    isRequired = true,
                ),
                FieldDefinitionRequest(
                    name = "Genre",
                    fieldType = "enum",
                    isRequired = false,
                    enumOptions = listOf("Fiction", "Non-Fiction", "Reference"),
                ),
            ),
        )
        val serialized = json.encodeToString(request)
        val deserialized = json.decodeFromString<CreateItemTemplateRequest>(serialized)
        assertEquals(request, deserialized)
    }

    // System DTOs

    @Test
    fun `CreateInitiativeRequest round-trips through JSON`() {
        val request = CreateInitiativeRequest(
            name = "Project Alpha",
            description = "A test project",
            color = "#FF5733",
            icon = "rocket",
        )
        val serialized = json.encodeToString(request)
        val deserialized = json.decodeFromString<CreateInitiativeRequest>(serialized)
        assertEquals(request, deserialized)
    }

    @Test
    fun `CaptureRequest round-trips through JSON`() {
        val request = CaptureRequest(
            content = "Remember to buy milk",
            source = "voice",
            attachmentIds = listOf("att1", "att2"),
        )
        val serialized = json.encodeToString(request)
        val deserialized = json.decodeFromString<CaptureRequest>(serialized)
        assertEquals(request, deserialized)
    }

    @Test
    fun `CreateRoutineRequest with schedule round-trips through JSON`() {
        val request = CreateRoutineRequest(
            title = "Morning exercise",
            description = "30 min workout",
            energyCost = 3,
            schedule = ScheduleRequest(
                type = "weekly",
                daysOfWeek = listOf("monday", "wednesday", "friday"),
            ),
            scheduledTime = "07:00",
        )
        val serialized = json.encodeToString(request)
        val deserialized = json.decodeFromString<CreateRoutineRequest>(serialized)
        assertEquals(request, deserialized)
    }

    @Test
    fun `RoutineResponse round-trips through JSON`() {
        val response = RoutineResponse(
            id = "01HYQVK...",
            title = "Morning exercise",
            description = "30 min workout",
            energyCost = 3,
            schedule = ScheduleResponse(
                type = "weekly",
                displayText = "Every Mon, Wed, Fri",
                daysOfWeek = listOf("monday", "wednesday", "friday"),
                dayOfMonth = null,
                weekOfMonth = null,
                intervalDays = null,
            ),
            scheduledTime = "07:00",
            initiativeId = null,
            isActive = true,
            lastSpawnedAt = null,
            createdAt = "2024-01-01T00:00:00Z",
            updatedAt = "2024-01-01T00:00:00Z",
        )
        val serialized = json.encodeToString(response)
        val deserialized = json.decodeFromString<RoutineResponse>(serialized)
        assertEquals(response, deserialized)
    }
}
