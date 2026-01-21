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
import kotlinx.datetime.LocalDate
import kotlinx.datetime.TimeZone
import kotlinx.datetime.todayIn
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory
import kotlin.time.Clock
import kotlin.time.Instant

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
        private val logger = LoggerFactory.getLogger(SurrealEnergyBudgetRepository::class.java)
        private const val DEFAULT_BUDGET = 10
    }

    override suspend fun findById(id: Ulid): Either<DomainError, EnergyBudget> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM energy_budget WHERE id = energy_budget:\$id AND user_id = user:\$userId",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()
            parseBudget(result) ?: raise(DomainError.NotFoundError("EnergyBudget", id.value))
        }

    override suspend fun save(entity: EnergyBudget): Either<DomainError, EnergyBudget> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .executeBind(
                        """
                        UPDATE energy_budget:${'$'}id SET
                            date = ${'$'}date,
                            total_budget = ${'$'}totalBudget,
                            spent_energy = ${'$'}spentEnergy,
                            updated_at = time::now()
                        WHERE user_id = user:${'$'}userId;
                        """.trimIndent(),
                        mapOf(
                            "id" to entity.id.value,
                            "date" to entity.date.toString(),
                            "totalBudget" to entity.totalBudget,
                            "spentEnergy" to entity.spentEnergy,
                            "userId" to userId.value,
                        ),
                    ).bind()
            } else {
                db
                    .executeBind(
                        """
                        CREATE energy_budget:${'$'}id CONTENT {
                            user_id: user:${'$'}userId,
                            date: ${'$'}date,
                            total_budget: ${'$'}totalBudget,
                            spent_energy: ${'$'}spentEnergy
                        };
                        """.trimIndent(),
                        mapOf(
                            "id" to entity.id.value,
                            "userId" to userId.value,
                            "date" to entity.date.toString(),
                            "totalBudget" to entity.totalBudget,
                            "spentEnergy" to entity.spentEnergy,
                        ),
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            db
                .executeBind(
                    "DELETE energy_budget:${'$'}id WHERE user_id = user:${'$'}userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).bind()
        }

    override fun findAll(): Flow<List<EnergyBudget>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM energy_budget WHERE user_id = user:\$userId ORDER BY date DESC",
                    mapOf("userId" to userId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findAll: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findAll: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findAll: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findAll: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findAll: ERROR_MSG")

                            else -> logger.warn("Database error in findAll: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseBudgets(it) },
                ),
            )
        }

    override suspend fun findOrCreateByDate(date: LocalDate): Either<DomainError, EnergyBudget> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM energy_budget WHERE user_id = user:\$userId AND date = \$date",
                        mapOf("userId" to userId.value, "date" to date.toString()),
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
                db.queryBind(
                    "SELECT * FROM energy_budget WHERE user_id = user:\$userId AND date >= \$startDate AND date <= \$endDate ORDER BY date",
                    mapOf("userId" to userId.value, "startDate" to startDate.toString(), "endDate" to endDate.toString()),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findByDateRange: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findByDateRange: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findByDateRange: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findByDateRange: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findByDateRange: ERROR_MSG")

                            else -> logger.warn("Database error in findByDateRange: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseBudgets(it) },
                ),
            )
        }

    override suspend fun addSpentEnergy(
        date: LocalDate,
        energyToAdd: Int,
    ): Either<DomainError, EnergyBudget> =
        either {
            val budget = findOrCreateByDate(date).bind()
            val newSpent = budget.spentEnergy + energyToAdd
            db
                .executeBind(
                    "UPDATE energy_budget:${'$'}id SET spent_energy = ${'$'}spentEnergy, updated_at = time::now() WHERE user_id = user:${'$'}userId;",
                    mapOf("id" to budget.id.value, "spentEnergy" to newSpent, "userId" to userId.value),
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
                .executeBind(
                    "UPDATE energy_budget:${'$'}id SET total_budget = ${'$'}totalBudget, updated_at = time::now() WHERE user_id = user:${'$'}userId;",
                    mapOf("id" to budget.id.value, "totalBudget" to newBudget, "userId" to userId.value),
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
        } catch (e: SerializationException) {
            logger.warn("Failed to parse energy budget: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse energy budget: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse energy budget: ${e.message}", e)
            null
        }

    private fun parseBudgets(result: String): List<EnergyBudget> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToBudget(it.jsonObject)
                } catch (e: SerializationException) {
                    logger.warn("Failed to parse energy budget element: ${e.message}", e)
                    null
                } catch (e: IllegalStateException) {
                    logger.warn("Failed to parse energy budget element: ${e.message}", e)
                    null
                } catch (e: IllegalArgumentException) {
                    logger.warn("Failed to parse energy budget element: ${e.message}", e)
                    null
                }
            }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse energy budgets array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse energy budgets array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse energy budgets array: ${e.message}", e)
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
            } catch (e: IllegalArgumentException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
