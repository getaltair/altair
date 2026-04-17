package com.getaltair.altair.data.sync

import com.getaltair.altair.data.auth.TokenPreferences
import com.getaltair.altair.data.network.SyncApi
import com.getaltair.altair.data.network.UpsertRequest
import com.powersync.PowerSyncDatabase
import com.powersync.db.crud.CrudBatch
import com.powersync.db.crud.CrudEntry
import com.powersync.db.crud.UpdateType
import io.mockk.coEvery
import io.mockk.coVerify
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertThrows
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test

/**
 * Unit tests for [AltairPowerSyncConnector.uploadData].
 *
 * Covers FA-028 (PUT/PATCH routes to upsert), FA-029 (DELETE routes to delete),
 * and the invariant that batch.complete() is not called when an upload throws.
 */
class AltairPowerSyncConnectorTest {
    private lateinit var syncApi: SyncApi
    private lateinit var connector: AltairPowerSyncConnector
    private lateinit var database: PowerSyncDatabase

    @BeforeEach
    fun setUp() {
        syncApi = mockk()
        database = mockk()
        connector =
            AltairPowerSyncConnector(
                powerSyncUrl = "https://powersync.example.com",
                getToken = { "test-token" },
                syncApi = syncApi,
            )
    }

    /**
     * FA-028: A PUT CrudEntry must route to SyncApi.upsert() and not SyncApi.delete().
     * batch.complete(null) must be called exactly once on success.
     */
    @Test
    fun `uploadData_putOp_callsUpsertNotDelete`() =
        runTest {
            val entry =
                mockk<CrudEntry> {
                    every { op } returns UpdateType.PUT
                    every { table } returns "quests"
                    every { id } returns "quest-id-1"
                    every { opData } returns null
                }
            val batch =
                mockk<CrudBatch> {
                    every { crud } returns listOf(entry)
                    coEvery { complete(null) } returns Unit
                }
            coEvery { database.getCrudBatch(100) } returns batch
            coEvery { syncApi.upsert(any(), any(), any()) } returns Unit

            connector.uploadData(database)

            coVerify(exactly = 1) { syncApi.upsert("quests", "quest-id-1", UpsertRequest(emptyMap())) }
            coVerify(exactly = 0) { syncApi.delete(any(), any()) }
            coVerify(exactly = 1) { batch.complete(null) }
        }

    /**
     * FA-028: A DELETE CrudEntry must route to SyncApi.delete() and not SyncApi.upsert().
     * batch.complete(null) must be called exactly once on success.
     */
    @Test
    fun `uploadData_deleteOp_callsDeleteNotUpsert`() =
        runTest {
            val entry =
                mockk<CrudEntry> {
                    every { op } returns UpdateType.DELETE
                    every { table } returns "quests"
                    every { id } returns "quest-id-2"
                    every { opData } returns null
                }
            val batch =
                mockk<CrudBatch> {
                    every { crud } returns listOf(entry)
                    coEvery { complete(null) } returns Unit
                }
            coEvery { database.getCrudBatch(100) } returns batch
            coEvery { syncApi.delete(any(), any()) } returns Unit

            connector.uploadData(database)

            coVerify(exactly = 1) { syncApi.delete("quests", "quest-id-2") }
            coVerify(exactly = 0) { syncApi.upsert(any(), any(), any()) }
            coVerify(exactly = 1) { batch.complete(null) }
        }

    /**
     * FA-029: When SyncApi.upsert() throws, uploadData() must propagate the exception
     * and batch.complete() must NOT be called.
     */
    @Test
    fun `uploadData_upsertThrows_batchCompleteNotCalled`() =
        runTest {
            val entry =
                mockk<CrudEntry> {
                    every { op } returns UpdateType.PUT
                    every { table } returns "quests"
                    every { id } returns "quest-id-3"
                    every { opData } returns null
                }
            val batch =
                mockk<CrudBatch> {
                    every { crud } returns listOf(entry)
                    coEvery { complete(null) } returns Unit
                }
            coEvery { database.getCrudBatch(100) } returns batch
            coEvery { syncApi.upsert(any(), any(), any()) } throws RuntimeException("network error")

            assertThrows(RuntimeException::class.java) {
                kotlinx.coroutines.runBlocking { connector.uploadData(database) }
            }

            coVerify(exactly = 0) { batch.complete(any()) }
        }

    /**
     * FA-028: A batch with two PUT entries must call SyncApi.upsert() exactly twice,
     * once for each entry with its specific args. batch.complete(null) must be called once.
     */
    @Test
    fun `uploadData_twoEntryPutBatch_callsUpsertTwice`() =
        runTest {
            val entry1 =
                mockk<CrudEntry> {
                    every { op } returns UpdateType.PUT
                    every { table } returns "quests"
                    every { id } returns "quest-id-a"
                    every { opData } returns null
                }
            val entry2 =
                mockk<CrudEntry> {
                    every { op } returns UpdateType.PUT
                    every { table } returns "notes"
                    every { id } returns "note-id-b"
                    every { opData } returns null
                }
            val batch =
                mockk<CrudBatch> {
                    every { crud } returns listOf(entry1, entry2)
                    coEvery { complete(null) } returns Unit
                }
            coEvery { database.getCrudBatch(100) } returns batch
            coEvery { syncApi.upsert(any(), any(), any()) } returns Unit

            connector.uploadData(database)

            coVerify(exactly = 1) { syncApi.upsert("quests", "quest-id-a", UpsertRequest(emptyMap())) }
            coVerify(exactly = 1) { syncApi.upsert("notes", "note-id-b", UpsertRequest(emptyMap())) }
            coVerify(exactly = 2) { syncApi.upsert(any(), any(), any()) }
            coVerify(exactly = 0) { syncApi.delete(any(), any()) }
            coVerify(exactly = 1) { batch.complete(null) }
        }

    /**
     * FA-028 / FA-029: A batch containing one PUT and one DELETE must call both
     * SyncApi.upsert() and SyncApi.delete() exactly once each.
     * batch.complete(null) must be called once on success.
     */
    @Test
    fun `uploadData_mixedPutDeleteBatch_callsBothOps`() =
        runTest {
            val putEntry =
                mockk<CrudEntry> {
                    every { op } returns UpdateType.PUT
                    every { table } returns "quests"
                    every { id } returns "quest-id-c"
                    every { opData } returns null
                }
            val deleteEntry =
                mockk<CrudEntry> {
                    every { op } returns UpdateType.DELETE
                    every { table } returns "quests"
                    every { id } returns "quest-id-d"
                    every { opData } returns null
                }
            val batch =
                mockk<CrudBatch> {
                    every { crud } returns listOf(putEntry, deleteEntry)
                    coEvery { complete(null) } returns Unit
                }
            coEvery { database.getCrudBatch(100) } returns batch
            coEvery { syncApi.upsert(any(), any(), any()) } returns Unit
            coEvery { syncApi.delete(any(), any()) } returns Unit

            connector.uploadData(database)

            coVerify(exactly = 1) { syncApi.upsert("quests", "quest-id-c", UpsertRequest(emptyMap())) }
            coVerify(exactly = 1) { syncApi.delete("quests", "quest-id-d") }
            coVerify(exactly = 1) { batch.complete(null) }
        }

    /**
     * FA-028: A PATCH CrudEntry is handled without silent fallthrough.
     *
     * Per the current implementation, PATCH and PUT share the same `when` branch in
     * [AltairPowerSyncConnector.uploadData] and both route to SyncApi.upsert(). This is
     * intentional — PATCH is treated identically to PUT. The test documents and verifies
     * this behaviour to catch any future regression where PATCH might be silently dropped.
     */
    @Test
    fun `uploadData_patchEntry_handledWithoutSilentFallthrough`() =
        runTest {
            val entry =
                mockk<CrudEntry> {
                    every { op } returns UpdateType.PATCH
                    every { table } returns "quests"
                    every { id } returns "quest-id-e"
                    every { opData } returns null
                }
            val batch =
                mockk<CrudBatch> {
                    every { crud } returns listOf(entry)
                    coEvery { complete(null) } returns Unit
                }
            coEvery { database.getCrudBatch(100) } returns batch
            coEvery { syncApi.upsert(any(), any(), any()) } returns Unit

            connector.uploadData(database)

            // PATCH routes to upsert — same branch as PUT in the when expression
            coVerify(exactly = 1) { syncApi.upsert("quests", "quest-id-e", UpsertRequest(emptyMap())) }
            coVerify(exactly = 0) { syncApi.delete(any(), any()) }
            coVerify(exactly = 1) { batch.complete(null) }
        }

    /**
     * Verifies that fetchCredentials() returns a PowerSyncCredentials whose token matches
     * the value provided by the getToken lambda (here wired to TokenPreferences.accessToken).
     *
     * Note: the task spec referred to getCredentials(); the actual override on
     * PowerSyncBackendConnector is fetchCredentials() — tested accordingly.
     */
    @Test
    fun `getCredentials_returnsTokenFromPreferences`() =
        runTest {
            val tokenPreferences = mockk<TokenPreferences>()
            every { tokenPreferences.accessToken } returns "user-access-token-xyz"

            val connectorWithPrefs =
                AltairPowerSyncConnector(
                    powerSyncUrl = "https://powersync.example.com",
                    getToken = { tokenPreferences.accessToken!! },
                    syncApi = syncApi,
                )

            val credentials = connectorWithPrefs.fetchCredentials()

            assertEquals("user-access-token-xyz", credentials.token)
            assertEquals("https://powersync.example.com", credentials.endpoint)
        }
}
