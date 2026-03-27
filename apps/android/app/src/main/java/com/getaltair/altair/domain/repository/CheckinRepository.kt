package com.getaltair.altair.domain.repository

import com.getaltair.altair.domain.entity.DailyCheckin
import java.time.LocalDate
import java.util.UUID
import kotlinx.coroutines.flow.Flow

interface CheckinRepository {
    fun getForToday(userId: UUID): Flow<DailyCheckin?>
    fun getForDateRange(userId: UUID, start: LocalDate, end: LocalDate): Flow<List<DailyCheckin>>
    suspend fun save(checkin: DailyCheckin)
}
