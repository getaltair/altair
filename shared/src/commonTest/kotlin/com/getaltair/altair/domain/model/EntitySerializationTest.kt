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
import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe
import kotlinx.datetime.LocalDate
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlin.time.Instant

/**
 * Tests for entity serialization round-tripping through JSON.
 *
 * Validates that all domain entities can be serialized and deserialized correctly.
 */
class EntitySerializationTest :
    DescribeSpec({
        val json = Json { prettyPrint = false }
        val now: Instant =
            Instant.fromEpochMilliseconds(
                kotlin.time.Clock.System
                    .now()
                    .toEpochMilliseconds(),
            )
        val userId = Ulid.generate()

        describe("System entities") {
            describe("User") {
                it("round-trips through JSON") {
                    val user =
                        User(
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
                    deserialized shouldBe user
                }
            }

            describe("Initiative") {
                it("round-trips through JSON") {
                    val initiative =
                        Initiative(
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
                    deserialized shouldBe initiative
                }
            }

            describe("InboxItem") {
                it("round-trips through JSON") {
                    val item =
                        InboxItem(
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
                    deserialized shouldBe item
                }
            }

            describe("Routine") {
                it("round-trips through JSON with Schedule") {
                    val routine =
                        Routine(
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
                    deserialized shouldBe routine
                }
            }
        }

        describe("Guidance entities") {
            describe("Quest") {
                it("round-trips through JSON") {
                    val quest =
                        Quest(
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
                    deserialized shouldBe quest
                }
            }

            describe("Epic") {
                it("round-trips through JSON") {
                    val epic =
                        Epic(
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
                    deserialized shouldBe epic
                }
            }

            describe("Checkpoint") {
                it("round-trips through JSON") {
                    val checkpoint =
                        Checkpoint(
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
                    deserialized shouldBe checkpoint
                }
            }

            describe("EnergyBudget") {
                it("round-trips through JSON") {
                    val budget =
                        EnergyBudget(
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
                    deserialized shouldBe budget
                }
            }
        }

        describe("Knowledge entities") {
            describe("Note") {
                it("round-trips through JSON") {
                    val note =
                        Note(
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
                    deserialized shouldBe note
                }
            }

            describe("Tag") {
                it("round-trips through JSON") {
                    val tag =
                        Tag(
                            id = Ulid.generate(),
                            userId = userId,
                            name = "important",
                            color = "#FF0000",
                            createdAt = now,
                            updatedAt = now,
                        )
                    val serialized = json.encodeToString(tag)
                    val deserialized = json.decodeFromString<Tag>(serialized)
                    deserialized shouldBe tag
                }
            }

            describe("Folder") {
                it("round-trips through JSON") {
                    val folder =
                        Folder(
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
                    deserialized shouldBe folder
                }
            }

            describe("NoteLink") {
                it("round-trips through JSON") {
                    val noteLink =
                        NoteLink(
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
                    deserialized shouldBe noteLink
                }
            }

            describe("Attachment") {
                it("round-trips through JSON") {
                    val attachment =
                        Attachment(
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
                    deserialized shouldBe attachment
                }
            }
        }

        describe("Tracking entities") {
            describe("Item") {
                it("round-trips through JSON") {
                    val item =
                        Item(
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
                    deserialized shouldBe item
                }
            }

            describe("Container") {
                it("round-trips through JSON") {
                    val container =
                        Container(
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
                    deserialized shouldBe container
                }
            }

            describe("Location") {
                it("round-trips through JSON") {
                    val location =
                        Location(
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
                    deserialized shouldBe location
                }
            }

            describe("ItemTemplate") {
                it("round-trips through JSON") {
                    val template =
                        ItemTemplate(
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
                    deserialized shouldBe template
                }
            }

            describe("CustomField") {
                it("round-trips through JSON") {
                    val customField =
                        CustomField(
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
                    deserialized shouldBe customField
                }
            }

            describe("FieldDefinition") {
                it("round-trips through JSON with enum options") {
                    val fieldDef =
                        FieldDefinition(
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
                    deserialized shouldBe fieldDef
                }
            }
        }

        describe("Source/Document entities") {
            describe("SourceDocument") {
                it("round-trips through JSON") {
                    val doc =
                        SourceDocument(
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
                    deserialized shouldBe doc
                }
            }

            describe("SourceAnnotation") {
                it("round-trips through JSON") {
                    val annotation =
                        SourceAnnotation(
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
                    deserialized shouldBe annotation
                }
            }

            describe("WatchedFolder") {
                it("round-trips through JSON") {
                    val watchedFolder =
                        WatchedFolder(
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
                    deserialized shouldBe watchedFolder
                }
            }

            describe("ExtractionJob") {
                it("round-trips through JSON") {
                    val job =
                        ExtractionJob(
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
                    deserialized shouldBe job
                }
            }
        }

        describe("Enum serialization") {
            it("serializes all enum SerialNames to lowercase") {
                json.encodeToString(UserRole.ADMIN) shouldBe """"admin""""
                json.encodeToString(UserRole.MEMBER) shouldBe """"member""""
                json.encodeToString(UserStatus.ACTIVE) shouldBe """"active""""
                json.encodeToString(QuestStatus.BACKLOG) shouldBe """"backlog""""
                json.encodeToString(CaptureSource.KEYBOARD) shouldBe """"keyboard""""
            }
        }
    })
