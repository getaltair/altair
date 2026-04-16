package com.getaltair.altair.data.network

import kotlinx.serialization.Serializable
import retrofit2.http.Body
import retrofit2.http.DELETE
import retrofit2.http.PUT
import retrofit2.http.Path

@Serializable
data class UpsertRequest(
    val data: Map<String, String?>,
)

interface SyncApi {
    @PUT("api/sync/{table}/{id}")
    suspend fun upsert(
        @Path("table") table: String,
        @Path("id") id: String,
        @Body request: UpsertRequest,
    )

    @DELETE("api/sync/{table}/{id}")
    suspend fun delete(
        @Path("table") table: String,
        @Path("id") id: String,
    )
}
