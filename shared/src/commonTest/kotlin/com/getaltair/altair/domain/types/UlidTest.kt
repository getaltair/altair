package com.getaltair.altair.domain.types

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.comparables.shouldBeLessThan
import io.kotest.matchers.shouldBe
import io.kotest.matchers.shouldNotBe

/**
 * Tests for ULID value object.
 *
 * Validates ULID generation, parsing, and normalization rules.
 */
class UlidTest :
    FunSpec({
        context("ULID generation") {
            test("generate creates valid 26-character ULID") {
                val ulid = Ulid.generate()
                ulid.value.length shouldBe 26
            }

            test("generated ULIDs are unique") {
                val ulids = (1..100).map { Ulid.generate() }
                val uniqueValues = ulids.map { it.value }.toSet()
                uniqueValues.size shouldBe 100
            }

            test("ULIDs generated later have greater values") {
                val ulid1 = Ulid.generate(timestamp = 1000)
                val ulid2 = Ulid.generate(timestamp = 2000)
                ulid1.value shouldBeLessThan ulid2.value
            }
        }

        context("ULID parsing and validation") {
            test("constructor accepts valid ULID string") {
                val validUlid = "01ARZ3NDEKTSV4RRFFQ69G5FAV"
                val ulid = Ulid(validUlid)
                ulid.value shouldBe validUlid
            }

            test("constructor rejects too short string") {
                shouldThrow<IllegalArgumentException> {
                    Ulid("01ARZ3NDEKTSV4RRFFQ69G5FA")
                }
            }

            test("constructor rejects too long string") {
                shouldThrow<IllegalArgumentException> {
                    Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAVX")
                }
            }

            test("constructor rejects invalid characters") {
                shouldThrow<IllegalArgumentException> {
                    Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAI") // I is not valid
                }
                shouldThrow<IllegalArgumentException> {
                    Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAL") // L is not valid
                }
                shouldThrow<IllegalArgumentException> {
                    Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAO") // O is not valid
                }
                shouldThrow<IllegalArgumentException> {
                    Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAU") // U is not valid
                }
            }

            test("constructor normalizes lowercase to uppercase") {
                val lowerUlid = "01arz3ndektsv4rrffq69g5fav"
                val ulid = Ulid(lowerUlid)
                // The value is normalized to uppercase
                ulid.value shouldBe "01ARZ3NDEKTSV4RRFFQ69G5FAV"
            }

            test("lowercase and uppercase ULIDs are equal") {
                val ulid1 = Ulid("01arz3ndektsv4rrffq69g5fav")
                val ulid2 = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")
                ulid1 shouldBe ulid2
            }
        }

        context("ULID properties") {
            test("toString returns the value") {
                val validUlid = "01ARZ3NDEKTSV4RRFFQ69G5FAV"
                val ulid = Ulid(validUlid)
                ulid.toString() shouldBe validUlid
            }

            test("equality works for same values") {
                val ulid1 = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")
                val ulid2 = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")
                ulid1 shouldBe ulid2
            }

            test("inequality works for different values") {
                val ulid1 = Ulid.generate()
                val ulid2 = Ulid.generate()
                ulid1 shouldNotBe ulid2
            }
        }

        context("ULID round-trip") {
            test("all generated ULIDs are parseable") {
                repeat(100) {
                    val ulid = Ulid.generate()
                    val parsed = Ulid(ulid.value)
                    parsed shouldBe ulid
                }
            }
        }
    })
