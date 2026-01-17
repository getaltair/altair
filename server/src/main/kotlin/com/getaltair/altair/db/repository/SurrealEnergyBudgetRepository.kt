@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.guidance.EnergyBudget
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.EnergyBudgetRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate
import kotlinx.datetime.TimeZone
import kotlinx.datetime.todayIn
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlin.time.Clock

class SurrealEnergyBudgetRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : EnergyBudgetRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    companion object {
        private const val DEFAULT_BUDGET = 10
    }

    override suspend fun findById(id: Ulid): Either<DomainError, EnergyBudget> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM energy_budget:${id.value} WHERE user_id = user:${userId.value}",
                    ).bind()
            parseBudget(result) ?: raise(DomainError.NotFoundError("EnergyBudget", id.value))
        }

    override suspend fun save(entity: EnergyBudget): Either<DomainError, EnergyBudget> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .execute(
                        """
                        UPDATE energy_budget:${entity.id.value} SET
                            date = '${entity.date}',
                            total_budget = ${entity.totalBudget},
                            spent_energy = ${entity.spentEnergy},
                            updated_at = time::now()
                        WHERE user_id = user:${userId.value};
                        """.trimIndent(),
                    ).bind()
            } else {
                db
                    .execute(
                        """
                        CREATE energy_budget:${entity.id.value} CONTENT {
                            user_id: user:${userId.value},
                            date: '${entity.date}',
                            total_budget: ${entity.totalBudget},
                            spent_energy: ${entity.spentEnergy}
                        };
                        """.trimIndent(),
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            db.execute("DELETE energy_budget:${id.value} WHERE user_id = user:${userId.value};").bind()
        }

    override fun findAll(): Flow<List<EnergyBudget>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM energy_budget WHERE user_id = user:${userId.value} ORDER BY date DESC",
                )
            emit(result.fold({ emptyList() }, { parseBudgets(it) }))
        }

    override suspend fun findOrCreateByDate(date: LocalDate): Either<DomainError, EnergyBudget> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM energy_budget WHERE user_id = user:${userId.value} AND date = '$date'",
                    ).bind()
            val existing = parseBudget(result)
            if (existing != null) {
                existing
            } else {
                val now = Clock.System.now()
                val newBudget =
                    EnergyBudget(
                        id = Ulid.generate(),
                        userId = userId,
                        date = date,
                        totalBudget = DEFAULT_BUDGET,
                        spentEnergy = 0,
                        createdAt = now,
                        updatedAt = now,
                    )
                save(newBudget).bind()
            }
        }

    override suspend fun findToday(): Either<DomainError, EnergyBudget> =
        either {
            val today = Clock.System.todayIn(TimeZone.currentSystemDefault())
            findOrCreateByDate(today).bind()
        }

    override fun findByDateRange(
        startDate: LocalDate,
        endDate: LocalDate,
    ): Flow<List<EnergyBudget>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM energy_budget WHERE user_id = user:${userId.value} AND date >= '$startDate' AND date <= '$endDate' ORDER BY date",
                )
            emit(result.fold({ emptyList() }, { parseBudgets(it) }))
        }

    override suspend fun addSpentEnergy(
        date: LocalDate,
        energyToAdd: Int,
    ): Either<DomainError, EnergyBudget> =
        either {
            val budget = findOrCreateByDate(date).bind()
            val newSpent = budget.spentEnergy + energyToAdd
            db
                .execute(
                    "UPDATE energy_budget:${budget.id.value} SET spent_energy = $newSpent, updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).bind()
            findById(budget.id).bind()
        }

    override suspend fun updateTotalBudget(
        date: LocalDate,
        newBudget: Int,
    ): Either<DomainError, EnergyBudget> =
        either {
            val budget = findOrCreateByDate(date).bind()
            db
                .execute(
                    "UPDATE energy_budget:${budget.id.value} SET total_budget = $newBudget, updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).bind()
            findById(budget.id).bind()
        }

    private fun parseBudget(result: String): EnergyBudget? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToBudget(it) }
        } catch (e: Exception) {
            null
        }

    private fun parseBudgets(result: String): List<EnergyBudget> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToBudget(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
            emptyList()
        }

    private fun mapToBudget(obj: kotlinx.serialization.json.JsonObject): EnergyBudget {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return EnergyBudget(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            date = LocalDate.parse(obj["date"]?.jsonPrimitive?.content ?: throw IllegalStateException()),
            totalBudget = obj["total_budget"]?.jsonPrimitive?.content?.toIntOrNull() ?: DEFAULT_BUDGET,
            spentEnergy = obj["spent_energy"]?.jsonPrimitive?.content?.toIntOrNull() ?: 0,
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
        )
    }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: Exception) {
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
