package com.getaltair.altair.domain

import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.QuestStatus
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlin.test.assertTrue

/**
 * Tests for module-specific error types.
 * Verifies serialization round-trips and DomainError conformance.
 */
class ModuleErrorsTest {
    private val json = Json { prettyPrint = false }

    // QuestError tests

    @Test
    fun `QuestError NotFound is a DomainError`() {
        val error: DomainError = QuestError.NotFound(Ulid.generate())
        assertIs<DomainError>(error)
        assertIs<QuestError>(error)
    }

    @Test
    fun `QuestError NotFound round-trips through JSON`() {
        val id = Ulid.generate()
        val error: DomainError = QuestError.NotFound(id)
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `QuestError EnergyBudgetExceeded round-trips through JSON`() {
        val error: DomainError = QuestError.EnergyBudgetExceeded(required = 5, available = 2)
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `QuestError EnergyBudgetExceeded toUserMessage contains values`() {
        val error = QuestError.EnergyBudgetExceeded(required = 5, available = 2)
        val message = error.toUserMessage()
        assertTrue(message.contains("5"))
        assertTrue(message.contains("2"))
    }

    @Test
    fun `QuestError InvalidStatusTransition round-trips through JSON`() {
        val error: DomainError = QuestError.InvalidStatusTransition(
            questId = Ulid.generate(),
            currentStatus = QuestStatus.BACKLOG,
            targetStatus = QuestStatus.COMPLETED,
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `QuestError WipLimitExceeded round-trips through JSON`() {
        val error: DomainError = QuestError.WipLimitExceeded(currentWip = 3, maxWip = 3)
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    // NoteError tests

    @Test
    fun `NoteError NotFound is a DomainError`() {
        val error: DomainError = NoteError.NotFound(Ulid.generate())
        assertIs<DomainError>(error)
        assertIs<NoteError>(error)
    }

    @Test
    fun `NoteError NotFound round-trips through JSON`() {
        val error: DomainError = NoteError.NotFound(Ulid.generate())
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `NoteError TitleConflict round-trips through JSON`() {
        val error: DomainError = NoteError.TitleConflict(
            title = "My Note",
            folderId = Ulid.generate(),
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `NoteError TitleConflict with null folderId round-trips through JSON`() {
        val error: DomainError = NoteError.TitleConflict(
            title = "My Note",
            folderId = null,
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `NoteError InvalidWikiLink round-trips through JSON`() {
        val error: DomainError = NoteError.InvalidWikiLink(
            linkText = "[[NonExistent]]",
            noteId = Ulid.generate(),
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `NoteError CircularLink round-trips through JSON`() {
        val error: DomainError = NoteError.CircularLink(
            sourceNoteId = Ulid.generate(),
            targetNoteId = Ulid.generate(),
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `NoteError FolderNotFound round-trips through JSON`() {
        val error: DomainError = NoteError.FolderNotFound(Ulid.generate())
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    // ItemError tests

    @Test
    fun `ItemError NotFound is a DomainError`() {
        val error: DomainError = ItemError.NotFound(Ulid.generate())
        assertIs<DomainError>(error)
        assertIs<ItemError>(error)
    }

    @Test
    fun `ItemError NotFound round-trips through JSON`() {
        val error: DomainError = ItemError.NotFound(Ulid.generate())
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `ItemError InvalidQuantity round-trips through JSON`() {
        val error: DomainError = ItemError.InvalidQuantity(
            itemId = Ulid.generate(),
            quantity = -5,
            reason = "Quantity must be non-negative",
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `ItemError ContainerCycle round-trips through JSON`() {
        val error: DomainError = ItemError.ContainerCycle(
            containerId = Ulid.generate(),
            targetContainerId = Ulid.generate(),
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `ItemError LocationNotFound round-trips through JSON`() {
        val error: DomainError = ItemError.LocationNotFound(Ulid.generate())
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `ItemError ContainerNotFound round-trips through JSON`() {
        val error: DomainError = ItemError.ContainerNotFound(Ulid.generate())
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `ItemError TemplateNotFound round-trips through JSON`() {
        val error: DomainError = ItemError.TemplateNotFound(Ulid.generate())
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    // SyncError tests

    @Test
    fun `SyncError ConflictDetected is a DomainError`() {
        val error: DomainError = SyncError.ConflictDetected(
            entityType = "Quest",
            entityId = Ulid.generate(),
            clientVersion = 5,
            serverVersion = 7,
        )
        assertIs<DomainError>(error)
        assertIs<SyncError>(error)
    }

    @Test
    fun `SyncError ConflictDetected round-trips through JSON`() {
        val error: DomainError = SyncError.ConflictDetected(
            entityType = "Quest",
            entityId = Ulid.generate(),
            clientVersion = 5,
            serverVersion = 7,
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `SyncError VersionMismatch round-trips through JSON`() {
        val error: DomainError = SyncError.VersionMismatch(
            clientVersion = 10,
            serverMinVersion = 15,
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `SyncError ServerUnreachable round-trips through JSON`() {
        val error: DomainError = SyncError.ServerUnreachable("Connection refused")
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `SyncError InvalidChangeSet round-trips through JSON`() {
        val error: DomainError = SyncError.InvalidChangeSet("Missing required field: entityId")
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `SyncError Timeout round-trips through JSON`() {
        val error: DomainError = SyncError.Timeout(elapsedMs = 30000)
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    // AuthError tests

    @Test
    fun `AuthError InvalidCredentials is a DomainError`() {
        val error: DomainError = AuthError.InvalidCredentials
        assertIs<DomainError>(error)
        assertIs<AuthError>(error)
    }

    @Test
    fun `AuthError InvalidCredentials round-trips through JSON`() {
        val error: DomainError = AuthError.InvalidCredentials
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `AuthError TokenExpired round-trips through JSON`() {
        val error: DomainError = AuthError.TokenExpired(expiredAt = 1700000000000)
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `AuthError TokenInvalid round-trips through JSON`() {
        val error: DomainError = AuthError.TokenInvalid(reason = "Invalid signature")
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `AuthError AccountLocked with expiry round-trips through JSON`() {
        val error: DomainError = AuthError.AccountLocked(
            reason = "Too many failed login attempts",
            lockedUntil = 1700003600000,
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `AuthError AccountLocked permanent round-trips through JSON`() {
        val error: DomainError = AuthError.AccountLocked(
            reason = "Policy violation",
            lockedUntil = null,
        )
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `AuthError AccountLocked toUserMessage differs based on lockedUntil`() {
        val tempLock = AuthError.AccountLocked("reason", lockedUntil = 1700000000000)
        val permLock = AuthError.AccountLocked("reason", lockedUntil = null)

        assertTrue(tempLock.toUserMessage().contains("temporarily"))
        assertTrue(permLock.toUserMessage().contains("contact support"))
    }

    @Test
    fun `AuthError InviteRequired round-trips through JSON`() {
        val error: DomainError = AuthError.InviteRequired
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `AuthError InvalidInvite round-trips through JSON`() {
        val error: DomainError = AuthError.InvalidInvite(code = "ABC123")
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `AuthError EmailAlreadyExists round-trips through JSON`() {
        val error: DomainError = AuthError.EmailAlreadyExists(email = "test@example.com")
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    // Pattern matching tests

    @Test
    fun `all module errors can be matched in when expression`() {
        val errors: List<DomainError> = listOf(
            QuestError.NotFound(Ulid.generate()),
            QuestError.EnergyBudgetExceeded(3, 2),
            NoteError.NotFound(Ulid.generate()),
            NoteError.TitleConflict("test", null),
            ItemError.NotFound(Ulid.generate()),
            ItemError.ContainerCycle(Ulid.generate(), Ulid.generate()),
            SyncError.ServerUnreachable("test"),
            AuthError.InvalidCredentials,
        )

        val types = errors.map { error ->
            when (error) {
                is QuestError -> "quest"
                is NoteError -> "note"
                is ItemError -> "item"
                is SyncError -> "sync"
                is AuthError -> "auth"
                else -> "other"
            }
        }

        assertEquals(
            listOf("quest", "quest", "note", "note", "item", "item", "sync", "auth"),
            types,
        )
    }

    @Test
    fun `all error types provide non-empty user messages`() {
        val errors: List<DomainError> = listOf(
            QuestError.NotFound(Ulid.generate()),
            QuestError.EnergyBudgetExceeded(3, 2),
            QuestError.InvalidStatusTransition(Ulid.generate(), QuestStatus.BACKLOG, QuestStatus.COMPLETED),
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
            AuthError.InvalidInvite("test"),
            AuthError.EmailAlreadyExists("test@test.com"),
        )

        errors.forEach { error ->
            assertTrue(
                error.toUserMessage().isNotBlank(),
                "Error ${error::class.simpleName} should have a non-empty user message",
            )
        }
    }
}
