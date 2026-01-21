package com.getaltair.altair.domain

import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.QuestStatus
import com.getaltair.altair.domain.types.enums.UserRole
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain
import io.kotest.matchers.string.shouldNotContain
import io.kotest.matchers.types.shouldBeInstanceOf
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

/**
 * Tests for module-specific error types.
 *
 * Verifies serialization round-trips and DomainError conformance.
 */
class ModuleErrorsTest :
    BehaviorSpec({
        val json = Json { prettyPrint = false }

        given("QuestError types") {
            `when`("NotFound is created") {
                then("it is a DomainError") {
                    val error: DomainError = QuestError.NotFound(Ulid.generate())
                    error.shouldBeInstanceOf<DomainError>()
                    error.shouldBeInstanceOf<QuestError>()
                }

                then("it round-trips through JSON") {
                    val id = Ulid.generate()
                    val error: DomainError = QuestError.NotFound(id)
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("EnergyBudgetExceeded is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = QuestError.EnergyBudgetExceeded(required = 5, available = 2)
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }

                then("toUserMessage contains values") {
                    val error = QuestError.EnergyBudgetExceeded(required = 5, available = 2)
                    val message = error.toUserMessage()
                    message shouldContain "5"
                    message shouldContain "2"
                }
            }

            `when`("InvalidStatusTransition is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        QuestError.InvalidStatusTransition(
                            questId = Ulid.generate(),
                            currentStatus = QuestStatus.BACKLOG,
                            targetStatus = QuestStatus.COMPLETED,
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("WipLimitExceeded is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = QuestError.WipLimitExceeded(currentWip = 3, maxWip = 3)
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("NoteError types") {
            `when`("NotFound is created") {
                then("it is a DomainError") {
                    val error: DomainError = NoteError.NotFound(Ulid.generate())
                    error.shouldBeInstanceOf<DomainError>()
                    error.shouldBeInstanceOf<NoteError>()
                }

                then("it round-trips through JSON") {
                    val error: DomainError = NoteError.NotFound(Ulid.generate())
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("TitleConflict with folderId is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        NoteError.TitleConflict(
                            title = "My Note",
                            folderId = Ulid.generate(),
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("TitleConflict with null folderId is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        NoteError.TitleConflict(
                            title = "My Note",
                            folderId = null,
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("InvalidWikiLink is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        NoteError.InvalidWikiLink(
                            linkText = "[[NonExistent]]",
                            noteId = Ulid.generate(),
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("CircularLink is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        NoteError.CircularLink(
                            sourceNoteId = Ulid.generate(),
                            targetNoteId = Ulid.generate(),
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("FolderNotFound is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = NoteError.FolderNotFound(Ulid.generate())
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("ItemError types") {
            `when`("NotFound is created") {
                then("it is a DomainError") {
                    val error: DomainError = ItemError.NotFound(Ulid.generate())
                    error.shouldBeInstanceOf<DomainError>()
                    error.shouldBeInstanceOf<ItemError>()
                }

                then("it round-trips through JSON") {
                    val error: DomainError = ItemError.NotFound(Ulid.generate())
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("InvalidQuantity is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        ItemError.InvalidQuantity(
                            itemId = Ulid.generate(),
                            quantity = -5,
                            reason = "Quantity must be non-negative",
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("ContainerCycle is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        ItemError.ContainerCycle(
                            containerId = Ulid.generate(),
                            targetContainerId = Ulid.generate(),
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("LocationNotFound is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = ItemError.LocationNotFound(Ulid.generate())
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("ContainerNotFound is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = ItemError.ContainerNotFound(Ulid.generate())
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("TemplateNotFound is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = ItemError.TemplateNotFound(Ulid.generate())
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("SyncError types") {
            `when`("ConflictDetected is created") {
                then("it is a DomainError") {
                    val error: DomainError =
                        SyncError.ConflictDetected(
                            entityType = "Quest",
                            entityId = Ulid.generate(),
                            clientVersion = 5,
                            serverVersion = 7,
                        )
                    error.shouldBeInstanceOf<DomainError>()
                    error.shouldBeInstanceOf<SyncError>()
                }

                then("it round-trips through JSON") {
                    val error: DomainError =
                        SyncError.ConflictDetected(
                            entityType = "Quest",
                            entityId = Ulid.generate(),
                            clientVersion = 5,
                            serverVersion = 7,
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("VersionMismatch is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        SyncError.VersionMismatch(
                            clientVersion = 10,
                            serverMinVersion = 15,
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("ServerUnreachable is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = SyncError.ServerUnreachable("Connection refused")
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("InvalidChangeSet is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = SyncError.InvalidChangeSet("Missing required field: entityId")
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("Timeout is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = SyncError.Timeout(elapsedMs = 30_000)
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("AuthError types") {
            `when`("InvalidCredentials is created") {
                then("it is a DomainError") {
                    val error: DomainError = AuthError.InvalidCredentials
                    error.shouldBeInstanceOf<DomainError>()
                    error.shouldBeInstanceOf<AuthError>()
                }

                then("it round-trips through JSON") {
                    val error: DomainError = AuthError.InvalidCredentials
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("TokenExpired is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = AuthError.TokenExpired(expiredAt = 1_700_000_000_000)
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("TokenInvalid is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = AuthError.TokenInvalid(reason = "Invalid signature")
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("AccountLocked with expiry is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        AuthError.AccountLocked(
                            reason = "Too many failed login attempts",
                            lockedUntil = 1_700_003_600_000,
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }

                then("toUserMessage mentions temporarily") {
                    val tempLock = AuthError.AccountLocked("reason", lockedUntil = 1_700_000_000_000)
                    tempLock.toUserMessage() shouldContain "temporarily"
                }
            }

            `when`("AccountLocked permanent is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        AuthError.AccountLocked(
                            reason = "Policy violation",
                            lockedUntil = null,
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }

                then("toUserMessage mentions contacting support") {
                    val permLock = AuthError.AccountLocked("reason", lockedUntil = null)
                    permLock.toUserMessage() shouldContain "contact support"
                }
            }

            `when`("InviteRequired is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = AuthError.InviteRequired
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("InvalidInviteCode is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = AuthError.InvalidInviteCode
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("EmailAlreadyExists is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = AuthError.EmailAlreadyExists
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("UserError types") {
            `when`("NotFound is created") {
                then("it is a DomainError") {
                    val error: DomainError = UserError.NotFound(Ulid.generate())
                    error.shouldBeInstanceOf<DomainError>()
                    error.shouldBeInstanceOf<UserError>()
                }

                then("it round-trips through JSON") {
                    val id = Ulid.generate()
                    val error: DomainError = UserError.NotFound(id)
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("EmailAlreadyExists is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = UserError.EmailAlreadyExists
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("EmailNotFound is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = UserError.EmailNotFound
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("StorageQuotaExceeded is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        UserError.StorageQuotaExceeded(
                            currentUsage = 1_000_000_000,
                            quota = 500_000_000,
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("InsufficientPermissions is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = UserError.InsufficientPermissions(requiredRole = UserRole.ADMIN)
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("EpicError types") {
            `when`("NotFound is created") {
                then("it is a DomainError") {
                    val error: DomainError = EpicError.NotFound(Ulid.generate())
                    error.shouldBeInstanceOf<DomainError>()
                    error.shouldBeInstanceOf<EpicError>()
                }

                then("it round-trips through JSON") {
                    val id = Ulid.generate()
                    val error: DomainError = EpicError.NotFound(id)
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("HasActiveQuests is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        EpicError.HasActiveQuests(
                            epicId = Ulid.generate(),
                            activeQuestCount = 3,
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }

                then("toUserMessage uses correct pluralization") {
                    val singleQuest = EpicError.HasActiveQuests(Ulid.generate(), 1)
                    val multipleQuests = EpicError.HasActiveQuests(Ulid.generate(), 3)

                    singleQuest.toUserMessage() shouldContain "1 active quest"
                    singleQuest.toUserMessage() shouldNotContain "quests"
                    multipleQuests.toUserMessage() shouldContain "3 active quests"
                }
            }

            `when`("InvalidStatusTransition is created") {
                then("it round-trips through JSON") {
                    val error: DomainError =
                        EpicError.InvalidStatusTransition(
                            epicId = Ulid.generate(),
                            currentStatus = "active",
                            targetStatus = "completed",
                        )
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }

            `when`("InitiativeNotFound is created") {
                then("it round-trips through JSON") {
                    val error: DomainError = EpicError.InitiativeNotFound(Ulid.generate())
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("module error pattern matching") {
            `when`("all module errors are in a list") {
                then("they can be matched in when expression") {
                    val errors: List<DomainError> =
                        listOf(
                            QuestError.NotFound(Ulid.generate()),
                            QuestError.EnergyBudgetExceeded(3, 2),
                            NoteError.NotFound(Ulid.generate()),
                            NoteError.TitleConflict("test", null),
                            ItemError.NotFound(Ulid.generate()),
                            ItemError.ContainerCycle(Ulid.generate(), Ulid.generate()),
                            SyncError.ServerUnreachable("test"),
                            AuthError.InvalidCredentials,
                            UserError.NotFound(Ulid.generate()),
                            UserError.EmailAlreadyExists,
                            EpicError.NotFound(Ulid.generate()),
                            EpicError.HasActiveQuests(Ulid.generate(), 2),
                        )

                    val types =
                        errors.map { error ->
                            when (error) {
                                is QuestError -> "quest"
                                is NoteError -> "note"
                                is ItemError -> "item"
                                is SyncError -> "sync"
                                is AuthError -> "auth"
                                is UserError -> "user"
                                is EpicError -> "epic"
                                else -> "other"
                            }
                        }

                    types shouldBe
                        listOf(
                            "quest",
                            "quest",
                            "note",
                            "note",
                            "item",
                            "item",
                            "sync",
                            "auth",
                            "user",
                            "user",
                            "epic",
                            "epic",
                        )
                }
            }
        }

        given("all module error types") {
            `when`("toUserMessage is called") {
                then("all errors provide non-empty user messages") {
                    val errors: List<DomainError> =
                        listOf(
                            QuestError.NotFound(Ulid.generate()),
                            QuestError.EnergyBudgetExceeded(3, 2),
                            QuestError.InvalidStatusTransition(
                                Ulid.generate(),
                                QuestStatus.BACKLOG,
                                QuestStatus.COMPLETED,
                            ),
                            QuestError.WipLimitExceeded(3, 3),
                            NoteError.NotFound(Ulid.generate()),
                            NoteError.TitleConflict("test", null),
                            NoteError.InvalidWikiLink("[[test]]", Ulid.generate()),
                            NoteError.CircularLink(Ulid.generate(), Ulid.generate()),
                            NoteError.FolderNotFound(Ulid.generate()),
                            ItemError.NotFound(Ulid.generate()),
                            ItemError.InvalidQuantity(Ulid.generate(), -1, "negative"),
                            ItemError.ContainerCycle(Ulid.generate(), Ulid.generate()),
                            ItemError.LocationNotFound(Ulid.generate()),
                            ItemError.ContainerNotFound(Ulid.generate()),
                            ItemError.TemplateNotFound(Ulid.generate()),
                            SyncError.ConflictDetected("Quest", Ulid.generate(), 1, 2),
                            SyncError.VersionMismatch(1, 2),
                            SyncError.ServerUnreachable("test"),
                            SyncError.InvalidChangeSet("test"),
                            SyncError.Timeout(1000),
                            AuthError.InvalidCredentials,
                            AuthError.TokenExpired(1000),
                            AuthError.TokenInvalid("test"),
                            AuthError.AccountLocked("test", 1000),
                            AuthError.InviteRequired,
                            AuthError.InvalidInviteCode,
                            AuthError.EmailAlreadyExists,
                            UserError.NotFound(Ulid.generate()),
                            UserError.EmailAlreadyExists,
                            UserError.EmailNotFound,
                            UserError.StorageQuotaExceeded(1000, 500),
                            UserError.InsufficientPermissions(UserRole.ADMIN),
                            EpicError.NotFound(Ulid.generate()),
                            EpicError.HasActiveQuests(Ulid.generate(), 2),
                            EpicError.InvalidStatusTransition(Ulid.generate(), "active", "completed"),
                            EpicError.InitiativeNotFound(Ulid.generate()),
                        )

                    errors.forEach { error ->
                        val message = error.toUserMessage()
                        message.isNotBlank() shouldBe true
                    }
                }
            }
        }
    })
