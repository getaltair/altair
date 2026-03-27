package com.getaltair.altair.data.local.converter

import androidx.room.TypeConverter
import java.time.Instant
import java.time.LocalDate
import java.time.format.DateTimeFormatter
import java.util.UUID

object RoomConverters {

    private val dateFormatter = DateTimeFormatter.ISO_LOCAL_DATE

    // UUID <-> String

    @TypeConverter
    fun uuidToString(uuid: UUID?): String? = uuid?.toString()

    @TypeConverter
    fun stringToUuid(value: String?): UUID? = value?.let { UUID.fromString(it) }

    // Instant <-> Long (epoch millis)

    @TypeConverter
    fun instantToLong(instant: Instant?): Long? = instant?.toEpochMilli()

    @TypeConverter
    fun longToInstant(timestamp: Long?): Instant? = timestamp?.let { Instant.ofEpochMilli(it) }

    // LocalDate <-> String (ISO format "yyyy-MM-dd")

    @TypeConverter
    fun localDateToString(date: LocalDate?): String? = date?.format(dateFormatter)

    @TypeConverter
    fun stringToLocalDate(dateString: String?): LocalDate? = dateString?.let {
        LocalDate.parse(it, dateFormatter)
    }
}
