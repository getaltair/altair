package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.collections.shouldBeEmpty
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain
import io.kotest.matchers.types.shouldBeInstanceOf

/**
 * Tests for pagination types: [PageRequest] and [PageResult].
 */
class PaginationTest :
    BehaviorSpec({
        given("PageRequest with valid parameters") {
            `when`("created with custom limit and offset") {
                then("returns Right with correct values") {
                    val result = PageRequest(limit = 50, offset = 0)
                    result.shouldBeInstanceOf<Either.Right<PageRequest>>()
                    result.value.limit shouldBe 50
                    result.value.offset shouldBe 0
                }
            }

            `when`("created with default parameters") {
                then("returns Right with defaults") {
                    val result = PageRequest()
                    result.shouldBeInstanceOf<Either.Right<PageRequest>>()
                    result.value.limit shouldBe PageRequest.DEFAULT_PAGE_SIZE
                    result.value.offset shouldBe 0
                }
            }

            `when`("created with minimum valid limit") {
                then("returns Right") {
                    val result = PageRequest(limit = 1)
                    result.shouldBeInstanceOf<Either.Right<PageRequest>>()
                    result.value.limit shouldBe 1
                }
            }

            `when`("created with maximum valid limit") {
                then("returns Right") {
                    val result = PageRequest(limit = PageRequest.MAX_PAGE_SIZE)
                    result.shouldBeInstanceOf<Either.Right<PageRequest>>()
                    result.value.limit shouldBe PageRequest.MAX_PAGE_SIZE
                }
            }

            `when`("created with valid offset") {
                then("returns Right") {
                    val result = PageRequest(offset = 100)
                    result.shouldBeInstanceOf<Either.Right<PageRequest>>()
                    result.value.offset shouldBe 100
                }
            }
        }

        given("PageRequest with invalid parameters") {
            `when`("limit is zero") {
                then("returns Left with validation error") {
                    val result = PageRequest(limit = 0)
                    result.shouldBeInstanceOf<Either.Left<DomainError.ValidationError>>()
                    result.value.field shouldBe "limit"
                    result.value.message shouldContain "1"
                    result.value.message shouldContain "${PageRequest.MAX_PAGE_SIZE}"
                }
            }

            `when`("limit is negative") {
                then("returns Left with validation error") {
                    val result = PageRequest(limit = -1)
                    result.shouldBeInstanceOf<Either.Left<DomainError.ValidationError>>()
                    result.value.field shouldBe "limit"
                }
            }

            `when`("limit exceeds maximum") {
                then("returns Left with validation error") {
                    val result = PageRequest(limit = PageRequest.MAX_PAGE_SIZE + 1)
                    result.shouldBeInstanceOf<Either.Left<DomainError.ValidationError>>()
                    result.value.field shouldBe "limit"
                }
            }

            `when`("offset is negative") {
                then("returns Left with validation error") {
                    val result = PageRequest(limit = 50, offset = -1)
                    result.shouldBeInstanceOf<Either.Left<DomainError.ValidationError>>()
                    result.value.field shouldBe "offset"
                    result.value.message shouldContain "non-negative"
                }
            }

            `when`("both limit and offset are invalid") {
                then("validates limit first") {
                    val result = PageRequest(limit = 0, offset = -1)
                    result.shouldBeInstanceOf<Either.Left<DomainError.ValidationError>>()
                    result.value.field shouldBe "limit"
                }
            }
        }

        given("PageRequest.unsafeCreate") {
            `when`("called with valid parameters") {
                then("succeeds") {
                    val request = PageRequest.unsafeCreate(limit = 25, offset = 50)
                    request.limit shouldBe 25
                    request.offset shouldBe 50
                }
            }

            `when`("called with default parameters") {
                then("uses defaults") {
                    val request = PageRequest.unsafeCreate()
                    request.limit shouldBe PageRequest.DEFAULT_PAGE_SIZE
                    request.offset shouldBe 0
                }
            }

            `when`("called with zero limit") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        PageRequest.unsafeCreate(limit = 0)
                    }
                }
            }

            `when`("called with negative limit") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        PageRequest.unsafeCreate(limit = -1)
                    }
                }
            }

            `when`("called with limit exceeding maximum") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        PageRequest.unsafeCreate(limit = PageRequest.MAX_PAGE_SIZE + 1)
                    }
                }
            }

            `when`("called with negative offset") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        PageRequest.unsafeCreate(offset = -1)
                    }
                }
            }
        }

        given("PageRequest constants") {
            `when`("checking DEFAULT_PAGE_SIZE") {
                then("it is 50") {
                    PageRequest.DEFAULT_PAGE_SIZE shouldBe 50
                }
            }

            `when`("checking MAX_PAGE_SIZE") {
                then("it is 100") {
                    PageRequest.MAX_PAGE_SIZE shouldBe 100
                }
            }
        }

        given("PageResult construction") {
            `when`("created with items and metadata") {
                then("stores all values correctly") {
                    val items = listOf("a", "b", "c")
                    val result = PageResult(items = items, totalCount = 10, hasMore = true)

                    result.items shouldBe items
                    result.totalCount shouldBe 10
                    result.hasMore shouldBe true
                }
            }

            `when`("created with empty items list") {
                then("succeeds") {
                    val result = PageResult(items = emptyList<String>(), totalCount = 0, hasMore = false)

                    result.items.shouldBeEmpty()
                    result.totalCount shouldBe 0
                }
            }

            `when`("hasMore is false") {
                then("stores hasMore correctly") {
                    val result = PageResult(items = listOf(1, 2, 3), totalCount = 3, hasMore = false)
                    result.hasMore shouldBe false
                }
            }
        }

        given("PageResult data class features") {
            `when`("comparing two instances with same values") {
                then("they are equal") {
                    val result1 = PageResult(items = listOf(1, 2), totalCount = 5, hasMore = true)
                    val result2 = PageResult(items = listOf(1, 2), totalCount = 5, hasMore = true)

                    result1 shouldBe result2
                }
            }

            `when`("copying with modifications") {
                then("copy preserves unchanged fields") {
                    val original = PageResult(items = listOf("a"), totalCount = 100, hasMore = true)
                    val copied = original.copy(hasMore = false)

                    copied.items shouldBe listOf("a")
                    copied.totalCount shouldBe 100
                    copied.hasMore shouldBe false
                }
            }
        }

        given("PageResult validation") {
            `when`("totalCount is negative") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        PageResult(items = listOf(1, 2, 3), totalCount = -1, hasMore = false)
                    }
                }
            }

            `when`("totalCount is less than items size") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        PageResult(items = listOf(1, 2, 3), totalCount = 2, hasMore = false)
                    }
                }
            }

            `when`("totalCount equals items size") {
                then("succeeds") {
                    val result = PageResult(items = listOf(1, 2, 3), totalCount = 3, hasMore = false)
                    result.totalCount shouldBe 3
                }
            }

            `when`("totalCount is greater than items size") {
                then("succeeds") {
                    val result = PageResult(items = listOf(1, 2, 3), totalCount = 100, hasMore = true)
                    result.totalCount shouldBe 100
                }
            }

            `when`("totalCount is zero with empty items") {
                then("succeeds") {
                    val result = PageResult(items = emptyList<Int>(), totalCount = 0, hasMore = false)
                    result.totalCount shouldBe 0
                    result.items.shouldBeEmpty()
                }
            }
        }
    })
