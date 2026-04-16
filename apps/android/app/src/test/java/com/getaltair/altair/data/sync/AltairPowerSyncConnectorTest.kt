package com.getaltair.altair.data.sync

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
}
