package com.getaltair.altair.data.repository

import com.getaltair.altair.data.local.dao.CheckinDao
import com.getaltair.altair.data.local.mapper.toDomain
import com.getaltair.altair.data.local.mapper.toEntity
import com.getaltair.altair.domain.entity.DailyCheckin
import com.getaltair.altair.domain.repository.CheckinRepository
import java.time.LocalDate
import java.util.UUID
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

class CheckinRepositoryImpl(
    private val checkinDao: CheckinDao,
) : CheckinRepository {

    override fun getForToday(userId: UUID): Flow<DailyCheckin?> =
        checkinDao.getByUserAndDate(userId, LocalDate.now().toString()).map { it?.toDomain() }

    override fun getForDateRange(
        userId: UUID,
        start: LocalDate,
        end: LocalDate,
    ): Flow<List<DailyCheckin>> =
        checkinDao.getByUserInRange(userId, start.toString(), end.toString()).map { entities ->
            entities.map { it.toDomain() }
        }

    override suspend fun save(checkin: DailyCheckin) {
        checkinDao.insert(checkin.toEntity())
    }
}
