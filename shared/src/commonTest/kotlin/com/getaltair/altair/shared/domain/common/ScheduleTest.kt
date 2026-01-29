package com.getaltair.altair.shared.domain.common

import kotlinx.datetime.DayOfWeek
import kotlinx.datetime.LocalDate
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull

class ScheduleTest {

    @Test
    fun testDailySchedule() {
        val schedule = Schedule.Daily
        val from = LocalDate(2026, 1, 28)
        val next = schedule.nextOccurrence(from)
        assertEquals(LocalDate(2026, 1, 29), next)
        assertEquals("daily", schedule.toSerializedString())
    }

    @Test
    fun testDailyScheduleParsing() {
        val parsed = Schedule.parse("daily")
        assertNotNull(parsed)
        assertEquals(Schedule.Daily, parsed)
    }

    @Test
    fun testWeeklySchedule() {
        val schedule = Schedule.Weekly(setOf(DayOfWeek.MONDAY, DayOfWeek.WEDNESDAY, DayOfWeek.FRIDAY))

        // From Tuesday, should get Wednesday
        val from1 = LocalDate(2026, 1, 27) // Tuesday
        assertEquals(LocalDate(2026, 1, 28), schedule.nextOccurrence(from1)) // Wednesday

        // From Friday, should get Monday
        val from2 = LocalDate(2026, 1, 30) // Friday
        assertEquals(LocalDate(2026, 2, 2), schedule.nextOccurrence(from2)) // Monday

        assertEquals("weekly:1,3,5", schedule.toSerializedString())
    }

    @Test
    fun testWeeklyScheduleParsing() {
        val parsed = Schedule.parse("weekly:1,3,5")
        assertNotNull(parsed)
        val weekly = parsed as Schedule.Weekly
        assertEquals(setOf(DayOfWeek.MONDAY, DayOfWeek.WEDNESDAY, DayOfWeek.FRIDAY), weekly.daysOfWeek)
    }

    @Test
    fun testMonthlyDateSchedule() {
        val schedule = Schedule.MonthlyDate(15)

        // Before the 15th
        val from1 = LocalDate(2026, 1, 10)
        assertEquals(LocalDate(2026, 1, 15), schedule.nextOccurrence(from1))

        // After the 15th
        val from2 = LocalDate(2026, 1, 20)
        assertEquals(LocalDate(2026, 2, 15), schedule.nextOccurrence(from2))

        assertEquals("monthly:15", schedule.toSerializedString())
    }

    @Test
    fun testMonthlyDateScheduleParsing() {
        val parsed = Schedule.parse("monthly:15")
        assertNotNull(parsed)
        val monthly = parsed as Schedule.MonthlyDate
        assertEquals(15, monthly.dayOfMonth)
    }

    @Test
    fun testMonthlyRelativeSchedule() {
        val schedule = Schedule.MonthlyRelative(RelativeWeek.FIRST, DayOfWeek.MONDAY)

        // January 2026 starts on Thursday
        // First Monday is January 5
        val from1 = LocalDate(2026, 1, 1)
        assertEquals(LocalDate(2026, 1, 5), schedule.nextOccurrence(from1))

        // After first Monday, should get February's first Monday
        val from2 = LocalDate(2026, 1, 10)
        assertEquals(LocalDate(2026, 2, 2), schedule.nextOccurrence(from2))

        assertEquals("monthly:first:1", schedule.toSerializedString())
    }

    @Test
    fun testMonthlyRelativeLastSchedule() {
        val schedule = Schedule.MonthlyRelative(RelativeWeek.LAST, DayOfWeek.FRIDAY)

        // January 2026: last Friday is January 30
        val from = LocalDate(2026, 1, 1)
        assertEquals(LocalDate(2026, 1, 30), schedule.nextOccurrence(from))

        assertEquals("monthly:last:5", schedule.toSerializedString())
    }

    @Test
    fun testMonthlyRelativeScheduleParsing() {
        val parsed = Schedule.parse("monthly:first:1")
        assertNotNull(parsed)
        val monthly = parsed as Schedule.MonthlyRelative
        assertEquals(RelativeWeek.FIRST, monthly.week)
        assertEquals(DayOfWeek.MONDAY, monthly.dayOfWeek)
    }

    @Test
    fun testIntervalSchedule() {
        val schedule = Schedule.Interval(3)
        val from = LocalDate(2026, 1, 28)
        val next = schedule.nextOccurrence(from)
        assertEquals(LocalDate(2026, 1, 31), next)
        assertEquals("interval:3", schedule.toSerializedString())
    }

    @Test
    fun testIntervalScheduleParsing() {
        val parsed = Schedule.parse("interval:3")
        assertNotNull(parsed)
        val interval = parsed as Schedule.Interval
        assertEquals(3, interval.days)
    }

    @Test
    fun testInvalidParsing() {
        assertNull(Schedule.parse("invalid"))
        assertNull(Schedule.parse(""))
        assertNull(Schedule.parse("weekly:"))
        assertNull(Schedule.parse("monthly:invalid"))
    }

    @Test
    fun testWeeklyScheduleRequiresAtLeastOneDay() {
        try {
            Schedule.Weekly(emptySet())
            kotlin.test.fail("Should throw IllegalArgumentException")
        } catch (e: IllegalArgumentException) {
            assertEquals("Weekly schedule must have at least one day", e.message)
        }
    }

    @Test
    fun testMonthlyDateValidation() {
        try {
            Schedule.MonthlyDate(0)
            kotlin.test.fail("Should throw IllegalArgumentException")
        } catch (e: IllegalArgumentException) {
            assertEquals("Day must be 1-31", e.message)
        }

        try {
            Schedule.MonthlyDate(32)
            kotlin.test.fail("Should throw IllegalArgumentException")
        } catch (e: IllegalArgumentException) {
            assertEquals("Day must be 1-31", e.message)
        }
    }

    @Test
    fun testIntervalValidation() {
        try {
            Schedule.Interval(0)
            kotlin.test.fail("Should throw IllegalArgumentException")
        } catch (e: IllegalArgumentException) {
            assertEquals("Interval must be at least 1 day", e.message)
        }
    }
}
