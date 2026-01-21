package com.getaltair.altair.ui.util

import androidx.compose.ui.unit.dp
import io.kotest.core.spec.style.DescribeSpec
import io.kotest.datatest.withData
import io.kotest.matchers.shouldBe

/**
 * Tests for WindowSizeClass breakpoint calculations.
 */
class WindowSizeClassTest :
    DescribeSpec({
        describe("calculateWindowSizeClass") {
            describe("Compact breakpoint") {
                withData(
                    nameFn = { "${it.value}dp -> Compact" },
                    0.dp,
                    320.dp,
                    599.dp,
                ) { width ->
                    calculateWindowSizeClass(width) shouldBe WindowSizeClass.Compact
                }
            }

            describe("Medium breakpoint") {
                withData(
                    nameFn = { "${it.value}dp -> Medium" },
                    600.dp,
                    601.dp,
                    700.dp,
                    839.dp,
                ) { width ->
                    calculateWindowSizeClass(width) shouldBe WindowSizeClass.Medium
                }
            }

            describe("Expanded breakpoint") {
                withData(
                    nameFn = { "${it.value}dp -> Expanded" },
                    840.dp,
                    841.dp,
                    1024.dp,
                    1920.dp,
                ) { width ->
                    calculateWindowSizeClass(width) shouldBe WindowSizeClass.Expanded
                }
            }
        }

        describe("useBottomNavigation") {
            data class BottomNavTest(
                val sizeClass: WindowSizeClass,
                val expected: Boolean,
            )

            withData(
                nameFn = { "${it.sizeClass} -> ${it.expected}" },
                BottomNavTest(WindowSizeClass.Compact, true),
                BottomNavTest(WindowSizeClass.Medium, false),
                BottomNavTest(WindowSizeClass.Expanded, false),
            ) { test ->
                test.sizeClass.useBottomNavigation() shouldBe test.expected
            }
        }

        describe("WindowSizeBreakpoints") {
            it("has correct CompactMaxWidth") {
                WindowSizeBreakpoints.CompactMaxWidth shouldBe 600.dp
            }

            it("has correct MediumMaxWidth") {
                WindowSizeBreakpoints.MediumMaxWidth shouldBe 840.dp
            }
        }
    })
