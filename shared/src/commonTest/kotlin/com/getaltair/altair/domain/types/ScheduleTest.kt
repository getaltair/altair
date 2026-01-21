package com.getaltair.altair.domain.types

import com.getaltair.altair.domain.types.enums.WeekOfMonth
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import kotlinx.datetime.DayOfWeek
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

/**
 * Tests for Schedule sealed class hierarchy.
 *
 * Validates serialization, deserialization, and validation rules
 * for all schedule types (Daily, Weekly, Monthly, Interval).
 */
class ScheduleTest :
    FunSpec({
        val json = Json { prettyPrint = false }

        context("Daily schedule") {
            test("serializes correctly") {
                val schedule: Schedule = Schedule.Daily
                val serialized = json.encodeToString(schedule)
                serialized shouldBe """{"type":"daily"}"""
            }

            test("deserializes correctly") {
                val deserialized = json.decodeFromString<Schedule>("""{"type":"daily"}""")
                deserialized shouldBe Schedule.Daily
            }
        }

        context("Weekly schedule") {
            test("serializes with sorted days") {
                val schedule = Schedule.Weekly(setOf(DayOfWeek.WEDNESDAY, DayOfWeek.MONDAY))
                val serialized = json.encodeToString<Schedule>(schedule)
                // Days should be sorted (Monday before Wednesday) for deterministic output
                serialized shouldBe """{"type":"weekly","sortedDays":["MONDAY","WEDNESDAY"]}"""

                val deserialized = json.decodeFromString<Schedule>(serialized) as Schedule.Weekly
                deserialized.days shouldBe schedule.days
            }

            test("rejects empty days") {
                shouldThrow<IllegalArgumentException> {
                    Schedule.Weekly(emptySet())
                }
            }
        }

        context("MonthlyDate schedule") {
            test("serializes correctly") {
                val schedule: Schedule = Schedule.MonthlyDate(15)
                val serialized = json.encodeToString(schedule)
                serialized shouldBe """{"type":"monthly_date","dayOfMonth":15}"""
            }

            test("rejects invalid day") {
                shouldThrow<IllegalArgumentException> {
                    Schedule.MonthlyDate(0)
                }
                shouldThrow<IllegalArgumentException> {
                    Schedule.MonthlyDate(32)
                }
            }

            test("accepts edge values") {
                val day1 = Schedule.MonthlyDate(1)
                val day31 = Schedule.MonthlyDate(31)
                day1.dayOfMonth shouldBe 1
                day31.dayOfMonth shouldBe 31
            }
        }

        context("MonthlyRelative schedule") {
            test("serializes correctly") {
                val schedule: Schedule = Schedule.MonthlyRelative(WeekOfMonth.SECOND, DayOfWeek.TUESDAY)
                val serialized = json.encodeToString(schedule)
                val deserialized = json.decodeFromString<Schedule>(serialized)
                deserialized shouldBe schedule
            }
        }

        context("Interval schedule") {
            test("serializes correctly") {
                val schedule: Schedule = Schedule.Interval(7)
                val serialized = json.encodeToString(schedule)
                serialized shouldBe """{"type":"interval","days":7}"""
            }

            test("rejects zero or negative days") {
                shouldThrow<IllegalArgumentException> {
                    Schedule.Interval(0)
                }
                shouldThrow<IllegalArgumentException> {
                    Schedule.Interval(-1)
                }
            }

            test("accepts minimum value") {
                val schedule = Schedule.Interval(1)
                schedule.days shouldBe 1
            }
        }

        context("Schedule round-trip serialization") {
            test("all schedule types round-trip through serialization") {
                val schedules: List<Schedule> =
                    listOf(
                        Schedule.Daily,
                        Schedule.Weekly(setOf(DayOfWeek.FRIDAY)),
                        Schedule.MonthlyDate(28),
                        Schedule.MonthlyRelative(WeekOfMonth.LAST, DayOfWeek.FRIDAY),
                        Schedule.Interval(14),
                    )

                schedules.forEach { original ->
                    val serialized = json.encodeToString(original)
                    val deserialized = json.decodeFromString<Schedule>(serialized)
                    deserialized shouldBe original
                }
            }
        }
    })
