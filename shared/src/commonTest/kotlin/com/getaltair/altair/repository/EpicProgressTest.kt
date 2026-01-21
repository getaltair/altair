package com.getaltair.altair.repository

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.shouldBe

/**
 * Tests for [EpicProgress] validation and computed properties.
 */
class EpicProgressTest :
    BehaviorSpec({
        given("EpicProgress construction") {
            `when`("valid values are provided") {
                then("construction succeeds") {
                    val progress =
                        EpicProgress(
                            totalQuests = 10,
                            completedQuests = 5,
                            totalEnergy = 100,
                            spentEnergy = 50,
                        )

                    progress.totalQuests shouldBe 10
                    progress.completedQuests shouldBe 5
                    progress.totalEnergy shouldBe 100
                    progress.spentEnergy shouldBe 50
                }
            }

            `when`("all zeros are provided") {
                then("construction succeeds") {
                    val progress =
                        EpicProgress(
                            totalQuests = 0,
                            completedQuests = 0,
                            totalEnergy = 0,
                            spentEnergy = 0,
                        )

                    progress.totalQuests shouldBe 0
                    progress.completedQuests shouldBe 0
                }
            }

            `when`("completed equals total") {
                then("construction succeeds") {
                    val progress =
                        EpicProgress(
                            totalQuests = 5,
                            completedQuests = 5,
                            totalEnergy = 20,
                            spentEnergy = 20,
                        )

                    progress.completedQuests shouldBe 5
                    progress.spentEnergy shouldBe 20
                }
            }
        }

        given("EpicProgress validation") {
            `when`("totalQuests is negative") {
                then("construction throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        EpicProgress(
                            totalQuests = -1,
                            completedQuests = 0,
                            totalEnergy = 0,
                            spentEnergy = 0,
                        )
                    }
                }
            }

            `when`("completedQuests is negative") {
                then("construction throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        EpicProgress(
                            totalQuests = 10,
                            completedQuests = -1,
                            totalEnergy = 0,
                            spentEnergy = 0,
                        )
                    }
                }
            }

            `when`("completedQuests exceeds totalQuests") {
                then("construction throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        EpicProgress(
                            totalQuests = 5,
                            completedQuests = 6,
                            totalEnergy = 0,
                            spentEnergy = 0,
                        )
                    }
                }
            }

            `when`("totalEnergy is negative") {
                then("construction throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        EpicProgress(
                            totalQuests = 0,
                            completedQuests = 0,
                            totalEnergy = -1,
                            spentEnergy = 0,
                        )
                    }
                }
            }

            `when`("spentEnergy is negative") {
                then("construction throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        EpicProgress(
                            totalQuests = 0,
                            completedQuests = 0,
                            totalEnergy = 10,
                            spentEnergy = -1,
                        )
                    }
                }
            }

            `when`("spentEnergy exceeds totalEnergy") {
                then("construction throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        EpicProgress(
                            totalQuests = 0,
                            completedQuests = 0,
                            totalEnergy = 10,
                            spentEnergy = 11,
                        )
                    }
                }
            }
        }

        given("EpicProgress completionPercent") {
            `when`("there are no quests") {
                then("completionPercent is 0") {
                    val progress =
                        EpicProgress(
                            totalQuests = 0,
                            completedQuests = 0,
                            totalEnergy = 0,
                            spentEnergy = 0,
                        )

                    progress.completionPercent shouldBe 0
                }
            }

            `when`("no quests are completed") {
                then("completionPercent is 0") {
                    val progress =
                        EpicProgress(
                            totalQuests = 10,
                            completedQuests = 0,
                            totalEnergy = 100,
                            spentEnergy = 0,
                        )

                    progress.completionPercent shouldBe 0
                }
            }

            `when`("all quests are completed") {
                then("completionPercent is 100") {
                    val progress =
                        EpicProgress(
                            totalQuests = 10,
                            completedQuests = 10,
                            totalEnergy = 100,
                            spentEnergy = 100,
                        )

                    progress.completionPercent shouldBe 100
                }
            }

            `when`("quests are partially completed") {
                then("completionPercent is calculated correctly") {
                    val progress =
                        EpicProgress(
                            totalQuests = 10,
                            completedQuests = 5,
                            totalEnergy = 100,
                            spentEnergy = 50,
                        )

                    progress.completionPercent shouldBe 50
                }
            }

            `when`("completion results in decimal value") {
                then("completionPercent truncates to integer") {
                    val progress =
                        EpicProgress(
                            totalQuests = 3,
                            completedQuests = 1,
                            totalEnergy = 30,
                            spentEnergy = 10,
                        )

                    // 1/3 = 0.333... * 100 = 33.33... -> truncated to 33
                    progress.completionPercent shouldBe 33
                }
            }
        }

        given("EpicProgress data class features") {
            `when`("comparing two instances with same values") {
                then("they are equal") {
                    val progress1 =
                        EpicProgress(
                            totalQuests = 10,
                            completedQuests = 5,
                            totalEnergy = 100,
                            spentEnergy = 50,
                        )
                    val progress2 =
                        EpicProgress(
                            totalQuests = 10,
                            completedQuests = 5,
                            totalEnergy = 100,
                            spentEnergy = 50,
                        )

                    progress1 shouldBe progress2
                }
            }

            `when`("copying with modifications") {
                then("copy preserves unchanged fields") {
                    val original =
                        EpicProgress(
                            totalQuests = 10,
                            completedQuests = 5,
                            totalEnergy = 100,
                            spentEnergy = 50,
                        )
                    val copied = original.copy(completedQuests = 6, spentEnergy = 60)

                    copied.totalQuests shouldBe 10
                    copied.completedQuests shouldBe 6
                    copied.totalEnergy shouldBe 100
                    copied.spentEnergy shouldBe 60
                }
            }
        }
    })
