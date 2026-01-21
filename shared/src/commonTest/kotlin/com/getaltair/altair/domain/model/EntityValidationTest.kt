@file:Suppress("TooManyFunctions")

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
import com.getaltair.altair.generators.TestGenerators.displayName
import com.getaltair.altair.generators.TestGenerators.validEmail
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.shouldNotBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.long
import io.kotest.property.checkAll
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate

/**
 * Domain entity validation tests using BehaviorSpec with property-based testing.
 *
 * Tests are organized by entity type, validating constructor invariants and
 * business rules. Uses property-based testing for comprehensive coverage of
 * valid input ranges and data-driven tests for invalid edge cases.
 */
@Suppress("LargeClass")
class EntityValidationTest :
    BehaviorSpec({
        val now: Instant =
            Instant.fromEpochMilliseconds(
                kotlin.time.Clock.System
                    .now()
                    .toEpochMilliseconds(),
            )
        val userId = Ulid.generate()

        // ==================== User Validation ====================
        given("a User entity") {
            `when`("email is blank or malformed") {
                then("construction fails with IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        createUser(now, email = "")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createUser(now, email = "invalidemail")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createUser(now, email = "user@")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createUser(now, email = "user@domain")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createUser(now, email = "@")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createUser(now, email = "@@")
                    }
                }
            }

            `when`("email is valid") {
                then("user is created successfully") {
                    val user = createUser(now, email = "user@example.com")
                    user.email shouldBe "user@example.com"

                    val user2 = createUser(now, email = "user.name+tag@sub.domain.org")
                    user2.email shouldBe "user.name+tag@sub.domain.org"
                }
            }

            `when`("display name is blank") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createUser(now, displayName = "")
                    }
                }
            }

            `when`("display name exceeds 100 characters") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createUser(now, displayName = "a".repeat(101))
                    }
                }
            }

            `when`("storage values are negative") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createUser(now, storageUsedBytes = -1)
                    }
                    shouldThrow<IllegalArgumentException> {
                        createUser(now, storageQuotaBytes = -1)
                    }
                }
            }

            `when`("valid users are created") {
                then("storage calculations work correctly") {
                    checkAll(
                        Arb.validEmail(),
                        Arb.displayName(),
                        Arb.long(0L..10_000_000_000L),
                        Arb.long(0L..10_000_000_000L),
                    ) { email, name, used, quota ->
                        // Ensure used <= quota for valid construction
                        val actualUsed = minOf(used, quota)
                        val user =
                            User(
                                id = Ulid.generate(),
                                email = email,
                                displayName = name,
                                role = UserRole.MEMBER,
                                status = UserStatus.ACTIVE,
                                storageUsedBytes = actualUsed,
                                storageQuotaBytes = quota,
                                createdAt = now,
                                updatedAt = now,
                            )
                        user.storageRemainingBytes shouldBe quota - actualUsed
                        user.isOverQuota shouldBe false
                    }
                }
            }
        }

        // ==================== Quest Validation ====================
        given("a Quest entity") {
            `when`("title is blank or too long") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createQuest(now, userId, title = "")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createQuest(now, userId, title = "a".repeat(201))
                    }
                }
            }

            `when`("energy cost is out of range 1-5") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createQuest(now, userId, energyCost = 0)
                    }
                    shouldThrow<IllegalArgumentException> {
                        createQuest(now, userId, energyCost = 6)
                    }
                }
            }

            `when`("energy cost is valid (1-5)") {
                then("quest is created successfully") {
                    (1..5).forEach { cost ->
                        val quest = createQuest(now, userId, energyCost = cost)
                        quest.energyCost shouldBe cost
                    }
                }
            }

            `when`("status is ACTIVE without startedAt") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }

            `when`("status is COMPLETED without completedAt") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }

            `when`("startedAt is after completedAt") {
                then("construction fails") {
                    val earlier = Instant.fromEpochMilliseconds(1000)
                    val later = Instant.fromEpochMilliseconds(2000)
                    shouldThrow<IllegalArgumentException> {
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
            }
        }

        // ==================== Epic Validation ====================
        given("an Epic entity") {
            `when`("title is blank or too long") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createEpic(now, userId, title = "")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createEpic(now, userId, title = "a".repeat(201))
                    }
                }
            }

            `when`("status is COMPLETED without completedAt") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }

            `when`("status is COMPLETED with completedAt") {
                then("epic is created successfully") {
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
                    epic.status shouldBe EpicStatus.COMPLETED
                    epic.completedAt shouldNotBe null
                }
            }
        }

        // ==================== Initiative Validation ====================
        given("an Initiative entity") {
            `when`("color is invalid hex format") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createInitiative(now, userId, color = "red")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createInitiative(now, userId, color = "#GGG")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createInitiative(now, userId, color = "#12345")
                    }
                }
            }

            `when`("color is valid hex format") {
                then("initiative is created successfully") {
                    val init1 = createInitiative(now, userId, color = "#FF5733")
                    init1.color shouldBe "#FF5733"

                    val init2 = createInitiative(now, userId, color = "#F53")
                    init2.color shouldBe "#F53"

                    val init3 = createInitiative(now, userId, color = "#FF5733CC")
                    init3.color shouldBe "#FF5733CC"
                }
            }
        }

        // ==================== Note Validation ====================
        given("a Note entity") {
            `when`("title is blank") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createNote(now, userId, title = "")
                    }
                }
            }
        }

        // ==================== NoteLink Validation ====================
        given("a NoteLink entity") {
            `when`("source and target are the same") {
                then("construction fails") {
                    val noteId = Ulid.generate()
                    shouldThrow<IllegalArgumentException> {
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

            `when`("context exceeds 500 characters") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        NoteLink(
                            id = Ulid.generate(),
                            userId = userId,
                            sourceNoteId = Ulid.generate(),
                            targetNoteId = Ulid.generate(),
                            context = "a".repeat(501),
                            createdAt = now,
                            updatedAt = now,
                        )
                    }
                }
            }
        }

        // ==================== Folder Validation ====================
        given("a Folder entity") {
            `when`("parent is self") {
                then("construction fails") {
                    val folderId = Ulid.generate()
                    shouldThrow<IllegalArgumentException> {
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
        }

        // ==================== Tag Validation ====================
        given("a Tag entity") {
            `when`("name contains spaces") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }
        }

        // ==================== Attachment Validation ====================
        given("an Attachment entity") {
            `when`("neither noteId nor inboxItemId is provided") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }
        }

        // ==================== Location Validation ====================
        given("a Location entity") {
            `when`("parent is self") {
                then("construction fails") {
                    val locationId = Ulid.generate()
                    shouldThrow<IllegalArgumentException> {
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
        }

        // ==================== Container Validation ====================
        given("a Container entity") {
            `when`("parent is self") {
                then("construction fails") {
                    val containerId = Ulid.generate()
                    shouldThrow<IllegalArgumentException> {
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
        }

        // ==================== FieldDefinition Validation ====================
        given("a FieldDefinition entity") {
            `when`("type is ENUM without options") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
                    shouldThrow<IllegalArgumentException> {
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
            }

            `when`("type is not ENUM but has enumOptions") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }
        }

        // ==================== EnergyBudget Validation ====================
        given("an EnergyBudget entity") {
            `when`("budget values are negative") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
                    shouldThrow<IllegalArgumentException> {
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
            }

            `when`("budget is within limits") {
                then("remaining energy is calculated correctly") {
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
                    budget.remainingEnergy shouldBe 7
                    budget.isOverBudget shouldBe false
                }
            }

            `when`("spending exceeds budget") {
                then("over budget is detected") {
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
                    budget.remainingEnergy shouldBe 0
                    budget.isOverBudget shouldBe true
                }
            }
        }

        // ==================== Routine Validation ====================
        given("a Routine entity") {
            `when`("title is blank or too long") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createRoutine(now, userId, title = "")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createRoutine(now, userId, title = "a".repeat(201))
                    }
                }
            }

            `when`("energy cost is out of range") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createRoutine(now, userId, energyCost = 0)
                    }
                    shouldThrow<IllegalArgumentException> {
                        createRoutine(now, userId, energyCost = 6)
                    }
                }
            }
        }

        // ==================== Checkpoint Validation ====================
        given("a Checkpoint entity") {
            `when`("title is blank or too long") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createCheckpoint(now, userId, title = "")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createCheckpoint(now, userId, title = "a".repeat(201))
                    }
                }
            }

            `when`("sortOrder is negative") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createCheckpoint(now, userId, sortOrder = -1)
                    }
                }
            }

            `when`("isCompleted is true without completedAt") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }

            `when`("isCompleted is false with completedAt") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }
        }

        // ==================== InboxItem Validation ====================
        given("an InboxItem entity") {
            `when`("content is blank or too long") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createInboxItem(now, userId, content = "")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createInboxItem(now, userId, content = "a".repeat(5001))
                    }
                }
            }
        }

        // ==================== SourceDocument Validation ====================
        given("a SourceDocument entity") {
            `when`("title is blank or too long") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createSourceDocument(now, userId, title = "")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createSourceDocument(now, userId, title = "a".repeat(501))
                    }
                }
            }

            `when`("sourcePath is blank") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createSourceDocument(now, userId, sourcePath = "")
                    }
                }
            }

            `when`("fileSize or pageCount are negative") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createSourceDocument(now, userId, fileSizeBytes = -1)
                    }
                    shouldThrow<IllegalArgumentException> {
                        createSourceDocument(now, userId, pageCount = -1)
                    }
                }
            }
        }

        // ==================== Item Validation ====================
        given("an Item entity") {
            `when`("name is blank or too long") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createItem(now, userId, name = "")
                    }
                    shouldThrow<IllegalArgumentException> {
                        createItem(now, userId, name = "a".repeat(201))
                    }
                }
            }

            `when`("quantity is negative") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createItem(now, userId, quantity = -1)
                    }
                }
            }
        }

        // ==================== ExtractionJob Validation ====================
        given("an ExtractionJob entity") {
            `when`("progress is out of range 0-100") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
                        createExtractionJob(now, userId, progress = -1, status = JobStatus.PROCESSING)
                    }
                    shouldThrow<IllegalArgumentException> {
                        createExtractionJob(now, userId, progress = 101, status = JobStatus.PROCESSING)
                    }
                }
            }

            `when`("status is QUEUED with startedAt") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }

            `when`("status is PROCESSING without startedAt") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }

            `when`("status is COMPLETED without progress 100") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }

            `when`("status is FAILED without errorMessage") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }
        }

        // ==================== WatchedFolder Validation ====================
        given("a WatchedFolder entity") {
            `when`("path is blank") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
            }

            `when`("name is blank or too long") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
                    shouldThrow<IllegalArgumentException> {
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
            }
        }

        // ==================== ItemTemplate Validation ====================
        given("an ItemTemplate entity") {
            `when`("name is blank or too long") {
                then("construction fails") {
                    shouldThrow<IllegalArgumentException> {
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
                    shouldThrow<IllegalArgumentException> {
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
            }
        }
    })

// ==================== Helper Functions ====================
private fun createUser(
    now: Instant,
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
    now: Instant,
    userId: Ulid,
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

private fun createEpic(
    now: Instant,
    userId: Ulid,
    title: String = "Test Epic",
) = Epic(
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

private fun createInitiative(
    now: Instant,
    userId: Ulid,
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
    now: Instant,
    userId: Ulid,
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

private fun createRoutine(
    now: Instant,
    userId: Ulid,
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
    now: Instant,
    userId: Ulid,
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

private fun createInboxItem(
    now: Instant,
    userId: Ulid,
    content: String = "Test content",
) = InboxItem(
    id = Ulid.generate(),
    userId = userId,
    content = content,
    source = CaptureSource.KEYBOARD,
    attachmentIds = emptyList(),
    createdAt = now,
    updatedAt = now,
)

@Suppress("LongParameterList")
private fun createSourceDocument(
    now: Instant,
    userId: Ulid,
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
    now: Instant,
    userId: Ulid,
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
    now: Instant,
    userId: Ulid,
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
