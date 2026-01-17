package com.getaltair.altair.domain.types

import com.getaltair.altair.domain.types.enums.WeekOfMonth
import kotlinx.datetime.DayOfWeek
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class ScheduleTest {
    private val json = Json { prettyPrint = false }

    @Test
    fun `Daily serializes correctly`() {
        val schedule: Schedule = Schedule.Daily
        val serialized = json.encodeToString(schedule)
        assertEquals("""{"type":"daily"}""", serialized)
    }

    @Test
    fun `Daily deserializes correctly`() {
        val deserialized = json.decodeFromString<Schedule>("""{"type":"daily"}""")
        assertEquals(Schedule.Daily, deserialized)
    }

    @Test
    fun `Weekly serializes with sorted days`() {
        val schedule = Schedule.Weekly(setOf(DayOfWeek.WEDNESDAY, DayOfWeek.MONDAY))
        val serialized = json.encodeToString<Schedule>(schedule)
        // Days should be sorted (Monday before Wednesday) for deterministic output
        assertEquals("""{"type":"weekly","sortedDays":["MONDAY","WEDNESDAY"]}""", serialized)
        val deserialized = json.decodeFromString<Schedule>(serialized) as Schedule.Weekly
        assertEquals(schedule.days, deserialized.days)
    }

    @Test
    fun `Weekly rejects empty days`() {
        assertFailsWith<IllegalArgumentException> {
            Schedule.Weekly(emptySet())
        }
    }

    @Test
    fun `MonthlyDate serializes correctly`() {
        val schedule: Schedule = Schedule.MonthlyDate(15)
        val serialized = json.encodeToString(schedule)
        assertEquals("""{"type":"monthly_date","dayOfMonth":15}""", serialized)
    }

    @Test
    fun `MonthlyDate rejects invalid day`() {
        assertFailsWith<IllegalArgumentException> {
            Schedule.MonthlyDate(0)
        }
        assertFailsWith<IllegalArgumentException> {
            Schedule.MonthlyDate(32)
        }
    }

    @Test
    fun `MonthlyDate accepts edge values`() {
        val day1 = Schedule.MonthlyDate(1)
        val day31 = Schedule.MonthlyDate(31)
        assertEquals(1, day1.dayOfMonth)
        assertEquals(31, day31.dayOfMonth)
    }

    @Test
    fun `MonthlyRelative serializes correctly`() {
        val schedule: Schedule = Schedule.MonthlyRelative(WeekOfMonth.SECOND, DayOfWeek.TUESDAY)
        val serialized = json.encodeToString(schedule)
        val deserialized = json.decodeFromString<Schedule>(serialized)
        assertEquals(schedule, deserialized)
    }

    @Test
    fun `Interval serializes correctly`() {
        val schedule: Schedule = Schedule.Interval(7)
        val serialized = json.encodeToString(schedule)
        assertEquals("""{"type":"interval","days":7}""", serialized)
    }

    @Test
    fun `Interval rejects zero or negative days`() {
        assertFailsWith<IllegalArgumentException> {
            Schedule.Interval(0)
        }
        assertFailsWith<IllegalArgumentException> {
            Schedule.Interval(-1)
        }
    }

    @Test
    fun `Interval accepts minimum value`() {
        val schedule = Schedule.Interval(1)
        assertEquals(1, schedule.days)
    }

    @Test
    fun `all schedule types round-trip through serialization`() {
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
            assertEquals(original, deserialized)
        }
    }
}
