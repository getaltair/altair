package com.getaltair.altair.data.local.converter

import androidx.room.TypeConverter
import java.util.UUID
import timber.log.Timber

object RoomConverters {

    @TypeConverter
    fun uuidToString(uuid: UUID?): String? = uuid?.toString()

    @TypeConverter
    fun stringToUuid(value: String?): UUID? = value?.let {
        try {
            UUID.fromString(it)
        } catch (e: IllegalArgumentException) {
            Timber.w(e, "Malformed UUID in database: $it")
            null
        }
    }
}
