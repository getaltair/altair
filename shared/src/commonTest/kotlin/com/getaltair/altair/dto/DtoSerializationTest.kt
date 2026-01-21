package com.getaltair.altair.dto

import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.TokenRefreshRequest
import com.getaltair.altair.dto.guidance.CreateQuestRequest
import com.getaltair.altair.dto.guidance.EpicProgressResponse
import com.getaltair.altair.dto.guidance.EpicResponse
import com.getaltair.altair.dto.guidance.QuestResponse
import com.getaltair.altair.dto.knowledge.CreateNoteRequest
import com.getaltair.altair.dto.knowledge.LinkNotesRequest
import com.getaltair.altair.dto.knowledge.NoteResponse
import com.getaltair.altair.dto.sync.ChangeOperation
import com.getaltair.altair.dto.sync.ConflictInfo
import com.getaltair.altair.dto.sync.ConflictResolution
import com.getaltair.altair.dto.sync.EntityChange
import com.getaltair.altair.dto.sync.SyncRequest
import com.getaltair.altair.dto.sync.SyncResponse
import com.getaltair.altair.dto.system.CaptureRequest
import com.getaltair.altair.dto.system.CreateInitiativeRequest
import com.getaltair.altair.dto.system.CreateRoutineRequest
import com.getaltair.altair.dto.system.RoutineResponse
import com.getaltair.altair.dto.system.ScheduleRequest
import com.getaltair.altair.dto.system.ScheduleResponse
import com.getaltair.altair.dto.tracking.CreateItemRequest
import com.getaltair.altair.dto.tracking.CreateItemTemplateRequest
import com.getaltair.altair.dto.tracking.FieldDefinitionRequest
import com.getaltair.altair.dto.tracking.ItemResponse
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe
import kotlinx.serialization.SerializationException
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonPrimitive

/**
 * Tests for DTO serialization round-trips.
 *
 * Validates that all DTOs used in the API can be serialized and deserialized correctly.
 */
class DtoSerializationTest :
    DescribeSpec({
        val json = Json { prettyPrint = false }

        describe("Auth DTOs") {
            describe("AuthRequest") {
                it("round-trips through JSON") {
                    val request =
                        AuthRequest(
                            email = "test@example.com",
                            password = "secret123",
                        )
                    val serialized = json.encodeToString(request)
                    val deserialized = json.decodeFromString<AuthRequest>(serialized)
                    deserialized shouldBe request
                }
            }

            describe("AuthResponse") {
                it("round-trips through JSON") {
                    val response =
                        AuthResponse(
                            accessToken = "access.token.here",
                            refreshToken = "refresh.token.here",
                            expiresIn = 3600,
                            userId = Ulid("01HW0ABCD00000000000000001"),
                            displayName = "Test User",
                            role = UserRole.MEMBER,
                        )
                    val serialized = json.encodeToString(response)
                    val deserialized = json.decodeFromString<AuthResponse>(serialized)
                    deserialized shouldBe response
                }
            }

            describe("TokenRefreshRequest") {
                it("round-trips through JSON") {
                    val request = TokenRefreshRequest(refreshToken = "refresh.token.here")
                    val serialized = json.encodeToString(request)
                    val deserialized = json.decodeFromString<TokenRefreshRequest>(serialized)
                    deserialized shouldBe request
                }
            }

            describe("RegisterRequest") {
                it("round-trips with optional inviteCode") {
                    val withCode =
                        RegisterRequest(
                            email = "test@example.com",
                            password = "secret123",
                            displayName = "Test User",
                            inviteCode = "ABC123",
                        )
                    val withoutCode =
                        RegisterRequest(
                            email = "test@example.com",
                            password = "secret123",
                            displayName = "Test User",
                        )

                    json.decodeFromString<RegisterRequest>(json.encodeToString(withCode)) shouldBe withCode
                    json.decodeFromString<RegisterRequest>(json.encodeToString(withoutCode)) shouldBe withoutCode
                }
            }
        }

        describe("Sync DTOs") {
            describe("SyncRequest") {
                it("round-trips through JSON") {
                    val request =
                        SyncRequest(
                            clientId = "client-123",
                            lastSyncVersion = 42,
                            changes =
                                listOf(
                                    EntityChange(
                                        entityType = "Quest",
                                        entityId = "01HYQVK...",
                                        operation = ChangeOperation.UPDATE,
                                        version = 5,
                                        data = JsonPrimitive("data here"),
                                        timestamp = 1_700_000_000_000,
                                    ),
                                ),
                        )
                    val serialized = json.encodeToString(request)
                    val deserialized = json.decodeFromString<SyncRequest>(serialized)
                    deserialized shouldBe request
                }
            }

            describe("SyncResponse") {
                it("round-trips through JSON") {
                    val response =
                        SyncResponse(
                            serverVersion = 50,
                            changes = emptyList(),
                            conflicts =
                                listOf(
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
                    deserialized shouldBe response
                }
            }

            describe("ChangeOperation enum") {
                it("serializes to lowercase") {
                    json.encodeToString(ChangeOperation.CREATE) shouldBe """"create""""
                    json.encodeToString(ChangeOperation.UPDATE) shouldBe """"update""""
                    json.encodeToString(ChangeOperation.DELETE) shouldBe """"delete""""
                }
            }

            describe("ConflictResolution enum") {
                it("serializes to snake_case") {
                    json.encodeToString(ConflictResolution.KEEP_CLIENT) shouldBe """"keep_client""""
                    json.encodeToString(ConflictResolution.KEEP_SERVER) shouldBe """"keep_server""""
                    json.encodeToString(ConflictResolution.MERGE) shouldBe """"merge""""
                }
            }
        }

        describe("Guidance DTOs") {
            describe("CreateQuestRequest") {
                it("round-trips with defaults") {
                    val minimal =
                        CreateQuestRequest(
                            title = "My Quest",
                            energyCost = 3,
                        )
                    val full =
                        CreateQuestRequest(
                            title = "My Quest",
                            description = "Description here",
                            energyCost = 3,
                            epicId = "01HYQVK...",
                            initiativeId = "01HYQVK...",
                            dueDate = "2024-06-15",
                            scheduledDate = "2024-06-10",
                        )

                    json.decodeFromString<CreateQuestRequest>(json.encodeToString(minimal)) shouldBe minimal
                    json.decodeFromString<CreateQuestRequest>(json.encodeToString(full)) shouldBe full
                }
            }

            describe("QuestResponse") {
                it("round-trips through JSON") {
                    val response =
                        QuestResponse(
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
                    deserialized shouldBe response
                }
            }

            describe("EpicResponse") {
                it("round-trips with progress through JSON") {
                    val response =
                        EpicResponse(
                            id = "01HYQVK...",
                            title = "My Epic",
                            description = "Epic description",
                            status = "active",
                            initiativeId = null,
                            targetDate = "2024-12-31",
                            createdAt = "2024-01-01T00:00:00Z",
                            updatedAt = "2024-06-01T00:00:00Z",
                            completedAt = null,
                            progress =
                                EpicProgressResponse(
                                    totalQuests = 10,
                                    completedQuests = 3,
                                    totalEnergy = 35,
                                    spentEnergy = 12,
                                    completionPercent = 30,
                                ),
                        )
                    val serialized = json.encodeToString(response)
                    val deserialized = json.decodeFromString<EpicResponse>(serialized)
                    deserialized shouldBe response
                }
            }
        }

        describe("Knowledge DTOs") {
            describe("CreateNoteRequest") {
                it("round-trips with defaults") {
                    val minimal = CreateNoteRequest(title = "My Note")
                    val full =
                        CreateNoteRequest(
                            title = "My Note",
                            content = "# Content\n\nBody here",
                            folderId = "01HYQVK...",
                            initiativeId = "01HYQVK...",
                            isPinned = true,
                            tagIds = listOf("tag1", "tag2"),
                        )

                    json.decodeFromString<CreateNoteRequest>(json.encodeToString(minimal)) shouldBe minimal
                    json.decodeFromString<CreateNoteRequest>(json.encodeToString(full)) shouldBe full
                }
            }

            describe("NoteResponse") {
                it("round-trips through JSON") {
                    val response =
                        NoteResponse(
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
                    deserialized shouldBe response
                }
            }

            describe("LinkNotesRequest") {
                it("round-trips through JSON") {
                    val request =
                        LinkNotesRequest(
                            sourceNoteId = "01HYQVK1",
                            targetNoteId = "01HYQVK2",
                            context = "Related concept",
                        )
                    val serialized = json.encodeToString(request)
                    val deserialized = json.decodeFromString<LinkNotesRequest>(serialized)
                    deserialized shouldBe request
                }
            }
        }

        describe("Tracking DTOs") {
            describe("CreateItemRequest") {
                it("round-trips with custom fields") {
                    val request =
                        CreateItemRequest(
                            name = "Laptop",
                            description = "Work laptop",
                            templateId = "01HYQVK...",
                            locationId = "01HYQVK...",
                            quantity = 1,
                            customFields =
                                mapOf(
                                    "brand" to "Apple",
                                    "model" to "MacBook Pro 14",
                                ),
                        )
                    val serialized = json.encodeToString(request)
                    val deserialized = json.decodeFromString<CreateItemRequest>(serialized)
                    deserialized shouldBe request
                }
            }

            describe("ItemResponse") {
                it("round-trips through JSON") {
                    val response =
                        ItemResponse(
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
                    deserialized shouldBe response
                }
            }

            describe("CreateItemTemplateRequest") {
                it("round-trips with fields through JSON") {
                    val request =
                        CreateItemTemplateRequest(
                            name = "Book",
                            description = "Template for books",
                            icon = "book",
                            fields =
                                listOf(
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
                    deserialized shouldBe request
                }
            }
        }

        describe("System DTOs") {
            describe("CreateInitiativeRequest") {
                it("round-trips through JSON") {
                    val request =
                        CreateInitiativeRequest(
                            name = "Project Alpha",
                            description = "A test project",
                            color = "#FF5733",
                            icon = "rocket",
                        )
                    val serialized = json.encodeToString(request)
                    val deserialized = json.decodeFromString<CreateInitiativeRequest>(serialized)
                    deserialized shouldBe request
                }
            }

            describe("CaptureRequest") {
                it("round-trips through JSON") {
                    val request =
                        CaptureRequest(
                            content = "Remember to buy milk",
                            source = "voice",
                            attachmentIds = listOf("att1", "att2"),
                        )
                    val serialized = json.encodeToString(request)
                    val deserialized = json.decodeFromString<CaptureRequest>(serialized)
                    deserialized shouldBe request
                }
            }

            describe("CreateRoutineRequest") {
                it("round-trips with schedule through JSON") {
                    val request =
                        CreateRoutineRequest(
                            title = "Morning exercise",
                            description = "30 min workout",
                            energyCost = 3,
                            schedule =
                                ScheduleRequest(
                                    type = "weekly",
                                    daysOfWeek = listOf("monday", "wednesday", "friday"),
                                ),
                            scheduledTime = "07:00",
                        )
                    val serialized = json.encodeToString(request)
                    val deserialized = json.decodeFromString<CreateRoutineRequest>(serialized)
                    deserialized shouldBe request
                }
            }

            describe("RoutineResponse") {
                it("round-trips through JSON") {
                    val response =
                        RoutineResponse(
                            id = "01HYQVK...",
                            title = "Morning exercise",
                            description = "30 min workout",
                            energyCost = 3,
                            schedule =
                                ScheduleResponse(
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
                    deserialized shouldBe response
                }
            }
        }

        describe("Negative tests - malformed JSON") {
            it("AuthRequest fails on missing required field") {
                val malformed = """{"email": "test@example.com"}"""
                shouldThrow<SerializationException> {
                    json.decodeFromString<AuthRequest>(malformed)
                }
            }

            it("AuthRequest fails on wrong type for field") {
                val malformed = """{"email": 123, "password": "secret"}"""
                shouldThrow<SerializationException> {
                    json.decodeFromString<AuthRequest>(malformed)
                }
            }

            it("CreateQuestRequest fails on missing required energyCost") {
                val malformed = """{"title": "My Quest"}"""
                shouldThrow<SerializationException> {
                    json.decodeFromString<CreateQuestRequest>(malformed)
                }
            }

            it("ChangeOperation fails on invalid enum value") {
                val malformed = """"invalid_operation""""
                shouldThrow<SerializationException> {
                    json.decodeFromString<ChangeOperation>(malformed)
                }
            }

            it("SyncRequest fails on malformed changes array") {
                val malformed = """{
                "clientId": "client-123",
                "lastSyncVersion": 42,
                "changes": "not-an-array"
            }"""
                shouldThrow<SerializationException> {
                    json.decodeFromString<SyncRequest>(malformed)
                }
            }

            it("CreateItemRequest fails on invalid JSON syntax") {
                val malformed = """{"name": "Item", "quantity":}"""
                shouldThrow<SerializationException> {
                    json.decodeFromString<CreateItemRequest>(malformed)
                }
            }

            it("empty JSON object fails for required fields") {
                shouldThrow<SerializationException> {
                    json.decodeFromString<AuthRequest>("{}")
                }
            }
        }

        describe("Boundary value tests") {
            it("CreateQuestRequest handles empty title") {
                val request = CreateQuestRequest(title = "", energyCost = 3)
                val serialized = json.encodeToString(request)
                val deserialized = json.decodeFromString<CreateQuestRequest>(serialized)
                deserialized.title shouldBe ""
            }

            it("CreateItemRequest handles zero quantity") {
                val request = CreateItemRequest(name = "Item", quantity = 0)
                val serialized = json.encodeToString(request)
                val deserialized = json.decodeFromString<CreateItemRequest>(serialized)
                deserialized.quantity shouldBe 0
            }

            it("CreateItemRequest handles negative quantity") {
                val request = CreateItemRequest(name = "Item", quantity = -1)
                val serialized = json.encodeToString(request)
                val deserialized = json.decodeFromString<CreateItemRequest>(serialized)
                deserialized.quantity shouldBe -1
            }

            it("ItemResponse handles empty customFields map") {
                val response =
                    ItemResponse(
                        id = "01HYQVK...",
                        name = "Item",
                        description = null,
                        templateId = null,
                        locationId = null,
                        containerId = null,
                        quantity = 1,
                        photoAttachmentId = null,
                        initiativeId = null,
                        createdAt = "2024-01-01T00:00:00Z",
                        updatedAt = "2024-01-01T00:00:00Z",
                        customFields = emptyMap(),
                        locationPath = null,
                        containerPath = null,
                    )
                val serialized = json.encodeToString(response)
                val deserialized = json.decodeFromString<ItemResponse>(serialized)
                deserialized.customFields shouldBe emptyMap()
            }

            it("SyncResponse handles empty lists") {
                val response =
                    SyncResponse(
                        serverVersion = 1,
                        changes = emptyList(),
                        conflicts = emptyList(),
                        acknowledged = emptyList(),
                    )
                val serialized = json.encodeToString(response)
                val deserialized = json.decodeFromString<SyncResponse>(serialized)
                deserialized.changes shouldBe emptyList()
                deserialized.conflicts shouldBe emptyList()
                deserialized.acknowledged shouldBe emptyList()
            }
        }
    })
