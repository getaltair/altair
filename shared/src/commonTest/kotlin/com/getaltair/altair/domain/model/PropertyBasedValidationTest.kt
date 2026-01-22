package com.getaltair.altair.domain.model

import com.getaltair.altair.domain.common.ColorValidation
import com.getaltair.altair.domain.common.DomainConstants.MAX_ENERGY_COST
import com.getaltair.altair.domain.common.DomainConstants.MAX_NAME_LENGTH
import com.getaltair.altair.domain.common.DomainConstants.MAX_TAG_NAME_LENGTH
import com.getaltair.altair.domain.common.DomainConstants.MAX_TITLE_LENGTH
import com.getaltair.altair.domain.model.guidance.Quest
import com.getaltair.altair.domain.model.knowledge.Folder
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.model.knowledge.NoteLink
import com.getaltair.altair.domain.model.knowledge.Tag
import com.getaltair.altair.domain.model.system.Initiative
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.model.tracking.Container
import com.getaltair.altair.domain.model.tracking.Location
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.InitiativeStatus
import com.getaltair.altair.domain.types.enums.QuestStatus
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.longs.shouldBeExactly
import io.kotest.matchers.shouldBe
import io.kotest.matchers.shouldNotBe
import io.kotest.property.Arb
import io.kotest.property.arbitrary.Codepoint
import io.kotest.property.arbitrary.arbitrary
import io.kotest.property.arbitrary.az
import io.kotest.property.arbitrary.element
import io.kotest.property.arbitrary.filter
import io.kotest.property.arbitrary.filterNot
import io.kotest.property.arbitrary.hex
import io.kotest.property.arbitrary.int
import io.kotest.property.arbitrary.long
import io.kotest.property.arbitrary.map
import io.kotest.property.arbitrary.string
import io.kotest.property.checkAll
import kotlin.time.Instant

/**
 * Property-based tests for domain model validation using Kotest.
 *
 * These tests complement the example-based tests in EntityValidationTest
 * by generating random inputs to find edge cases and ensure validation
 * rules are comprehensive.
 */
class PropertyBasedValidationTest :
    FunSpec({
        val now: Instant = Instant.fromEpochMilliseconds(0)
        val userId = Ulid.generate()

        // ============================================================================
        // Custom Arbitraries (Generators)
        // ============================================================================

        /** Generates valid email addresses */
        val validEmailArb: Arb<String> =
            arbitrary {
                val localPart = Arb.string(1..20, Codepoint.az()).bind()
                val domain = Arb.string(1..10, Codepoint.az()).bind()
                val tld = Arb.element("com", "org", "net", "io", "dev").bind()
                "$localPart@$domain.$tld"
            }

        /** Generates invalid email addresses (missing @, domain, or tld) */
        val invalidEmailArb: Arb<String> =
            Arb.element(
                "plaintext",
                "missing@domain",
                "@nodomain.com",
                "no@tld",
                "spaces in@email.com",
                "@",
                "@@",
                "",
                "  ",
            )

        /** Generates valid hex colors (#RGB, #RRGGBB, #RRGGBBAA) */
        val validHexColorArb: Arb<String> =
            arbitrary {
                val length = Arb.element(3, 6, 8).bind()
                val hex = Arb.string(length, Codepoint.hex()).bind()
                "#$hex"
            }

        /** Generates invalid hex color strings */
        val invalidHexColorArb: Arb<String> =
            Arb.element(
                "red",
                "blue",
                "#GGG",
                "#ZZZZZZ",
                "#12345",
                "#1234567",
                "#123456789",
                "123456",
                "##AABBCC",
                "",
            )

        /** Generates valid energy costs (1-5) */
        val validEnergyCostArb: Arb<Int> = Arb.int(1..MAX_ENERGY_COST)

        /** Generates invalid energy costs (outside 1-5 range) */
        val invalidEnergyCostArb: Arb<Int> =
            Arb.element(
                -10,
                -1,
                0,
                6,
                7,
                10,
                100,
                Int.MIN_VALUE,
                Int.MAX_VALUE,
            )

        /** Generates non-negative longs for storage values */
        val nonNegativeLongArb: Arb<Long> = Arb.long(0L..Long.MAX_VALUE / 2)

        /** Generates negative longs for invalid storage values */
        val negativeLongArb: Arb<Long> = Arb.long(Long.MIN_VALUE..-1L)

        /** Generates tag names without spaces */
        val validTagNameArb: Arb<String> =
            Arb
                .string(1..MAX_TAG_NAME_LENGTH, Codepoint.az())
                .filter { it.isNotBlank() && !it.contains(' ') }

        /** Generates tag names with spaces (invalid) */
        val tagNameWithSpacesArb: Arb<String> =
            arbitrary {
                val before = Arb.string(1..10, Codepoint.az()).bind()
                val after = Arb.string(1..10, Codepoint.az()).bind()
                "$before $after"
            }

        // ============================================================================
        // Arbitrary Generator Functions
        // ============================================================================

        /** Generates non-blank strings within length limit */
        fun validStringArb(maxLength: Int): Arb<String> =
            Arb
                .string(1..maxLength, Codepoint.az())
                .filter { it.isNotBlank() }

        /** Generates strings that exceed the length limit */
        fun tooLongStringArb(maxLength: Int): Arb<String> =
            Arb.int(maxLength + 1..maxLength + 50).map { len ->
                "a".repeat(len)
            }

        // ============================================================================
        // Helper Functions
        // ============================================================================

        fun createUser(
            email: String = "test@example.com",
            displayName: String = "Test User",
            storageUsedBytes: Long = 0,
            storageQuotaBytes: Long = 1_000_000_000,
        ) = User(
            id = Ulid.generate(),
            email = email,
            displayName = displayName,
            role = UserRole.MEMBER,
            status = UserStatus.ACTIVE,
            storageUsedBytes = storageUsedBytes,
            storageQuotaBytes = storageQuotaBytes,
            createdAt = now,
            updatedAt = now,
        )

        fun createQuest(
            title: String = "Test Quest",
            energyCost: Int = 2,
        ) = Quest(
            id = Ulid.generate(),
            userId = userId,
            title = title,
            description = null,
            energyCost = energyCost,
            status = QuestStatus.BACKLOG,
            epicId = null,
            routineId = null,
            initiativeId = null,
            dueDate = null,
            scheduledDate = null,
            createdAt = now,
            updatedAt = now,
            startedAt = null,
            completedAt = null,
        )

        fun createInitiative(
            name: String = "Test Initiative",
            color: String? = null,
        ) = Initiative(
            id = Ulid.generate(),
            userId = userId,
            name = name,
            description = null,
            color = color,
            icon = null,
            status = InitiativeStatus.ACTIVE,
            createdAt = now,
            updatedAt = now,
        )

        fun createNote(title: String = "Test Note") =
            Note(
                id = Ulid.generate(),
                userId = userId,
                title = title,
                content = "Test content",
                folderId = null,
                initiativeId = null,
                isPinned = false,
                createdAt = now,
                updatedAt = now,
            )

        // ============================================================================
        // User Validation Tests
        // ============================================================================

        context("User validation") {
            test("accepts any valid email format") {
                checkAll(100, validEmailArb) { email ->
                    val user = createUser(email = email)
                    user.email shouldBe email
                }
            }

            test("rejects all invalid email formats") {
                checkAll(invalidEmailArb) { email ->
                    shouldThrow<IllegalArgumentException> {
                        createUser(email = email)
                    }
                }
            }

            test("accepts display names within length limit") {
                checkAll(100, validStringArb(MAX_NAME_LENGTH)) { name ->
                    val user = createUser(displayName = name)
                    user.displayName shouldBe name
                }
            }

            test("rejects display names exceeding length limit") {
                checkAll(20, tooLongStringArb(MAX_NAME_LENGTH)) { name ->
                    shouldThrow<IllegalArgumentException> {
                        createUser(displayName = name)
                    }
                }
            }

            test("accepts non-negative storage values") {
                checkAll(100, nonNegativeLongArb, nonNegativeLongArb) { used, quota ->
                    val user = createUser(storageUsedBytes = used, storageQuotaBytes = quota)
                    user.storageUsedBytes shouldBe used
                    user.storageQuotaBytes shouldBe quota
                }
            }

            test("rejects negative storage values") {
                checkAll(20, negativeLongArb) { negativeValue ->
                    shouldThrow<IllegalArgumentException> {
                        createUser(storageUsedBytes = negativeValue)
                    }
                    shouldThrow<IllegalArgumentException> {
                        createUser(storageQuotaBytes = negativeValue)
                    }
                }
            }

            test("correctly calculates storage remaining and over quota") {
                checkAll(100, nonNegativeLongArb, nonNegativeLongArb) { used, quota ->
                    val user = createUser(storageUsedBytes = used, storageQuotaBytes = quota)
                    val expectedRemaining = (quota - used).coerceAtLeast(0)
                    user.storageRemainingBytes shouldBe expectedRemaining
                    user.isOverQuota shouldBe (used > quota)
                }
            }
        }

        // ============================================================================
        // Quest Validation Tests
        // ============================================================================

        context("Quest validation") {
            test("accepts titles within length limit") {
                checkAll(100, validStringArb(MAX_TITLE_LENGTH)) { title ->
                    val quest = createQuest(title = title)
                    quest.title shouldBe title
                }
            }

            test("rejects titles exceeding length limit") {
                checkAll(20, tooLongStringArb(MAX_TITLE_LENGTH)) { title ->
                    shouldThrow<IllegalArgumentException> {
                        createQuest(title = title)
                    }
                }
            }

            test("accepts valid energy costs 1-5") {
                checkAll(validEnergyCostArb) { cost ->
                    val quest = createQuest(energyCost = cost)
                    quest.energyCost shouldBe cost
                }
            }

            test("rejects invalid energy costs") {
                checkAll(invalidEnergyCostArb) { cost ->
                    shouldThrow<IllegalArgumentException> {
                        createQuest(energyCost = cost)
                    }
                }
            }
        }

        // ============================================================================
        // Note Validation Tests
        // ============================================================================

        context("Note validation") {
            test("accepts titles within length limit") {
                checkAll(100, validStringArb(MAX_TITLE_LENGTH)) { title ->
                    val note = createNote(title = title)
                    note.title shouldBe title
                }
            }

            test("rejects titles exceeding length limit") {
                checkAll(20, tooLongStringArb(MAX_TITLE_LENGTH)) { title ->
                    shouldThrow<IllegalArgumentException> {
                        createNote(title = title)
                    }
                }
            }
        }

        // ============================================================================
        // Initiative Color Validation Tests
        // ============================================================================

        context("Initiative color validation") {
            test("accepts valid hex colors") {
                checkAll(100, validHexColorArb) { color ->
                    val initiative = createInitiative(color = color)
                    initiative.color shouldBe color
                }
            }

            test("rejects invalid hex colors") {
                checkAll(invalidHexColorArb.filterNot { it.isEmpty() }) { color ->
                    shouldThrow<IllegalArgumentException> {
                        createInitiative(color = color)
                    }
                }
            }

            test("accepts null color") {
                val initiative = createInitiative(color = null)
                initiative.color shouldBe null
            }

            test("ColorValidation regex matches expected patterns") {
                checkAll(100, validHexColorArb) { color ->
                    color.matches(ColorValidation.HEX_COLOR_REGEX).shouldBeTrue()
                }
            }

            test("ColorValidation regex rejects invalid patterns") {
                checkAll(invalidHexColorArb.filterNot { it.isEmpty() }) { color ->
                    color.matches(ColorValidation.HEX_COLOR_REGEX).shouldBeFalse()
                }
            }
        }

        // ============================================================================
        // Tag Validation Tests
        // ============================================================================

        context("Tag validation") {
            test("accepts names without spaces") {
                checkAll(100, validTagNameArb) { name ->
                    val tag =
                        Tag(
                            id = Ulid.generate(),
                            userId = userId,
                            name = name,
                            color = null,
                            createdAt = now,
                            updatedAt = now,
                        )
                    tag.name shouldBe name
                }
            }

            test("rejects names with spaces") {
                checkAll(50, tagNameWithSpacesArb) { name ->
                    shouldThrow<IllegalArgumentException> {
                        Tag(
                            id = Ulid.generate(),
                            userId = userId,
                            name = name,
                            color = null,
                            createdAt = now,
                            updatedAt = now,
                        )
                    }
                }
            }
        }

        // ============================================================================
        // Self-Reference Prevention Tests
        // ============================================================================

        context("Self-reference prevention") {
            test("NoteLink always prevents self-referencing for any ULID") {
                repeat(50) {
                    val noteId = Ulid.generate()
                    shouldThrow<IllegalArgumentException> {
                        NoteLink(
                            id = Ulid.generate(),
                            userId = userId,
                            sourceNoteId = noteId,
                            targetNoteId = noteId,
                            context = null,
                            createdAt = now,
                            updatedAt = now,
                        )
                    }
                }
            }

            test("NoteLink accepts different source and target") {
                repeat(50) {
                    val sourceId = Ulid.generate()
                    val targetId = Ulid.generate()
                    val link =
                        NoteLink(
                            id = Ulid.generate(),
                            userId = userId,
                            sourceNoteId = sourceId,
                            targetNoteId = targetId,
                            context = null,
                            createdAt = now,
                            updatedAt = now,
                        )
                    link.sourceNoteId shouldNotBe link.targetNoteId
                }
            }

            test("Folder always prevents self as parent for any ULID") {
                repeat(50) {
                    val folderId = Ulid.generate()
                    shouldThrow<IllegalArgumentException> {
                        Folder(
                            id = folderId,
                            userId = userId,
                            name = "Test",
                            parentId = folderId,
                            sortOrder = 0,
                            createdAt = now,
                            updatedAt = now,
                        )
                    }
                }
            }

            test("Location always prevents self as parent for any ULID") {
                repeat(50) {
                    val locationId = Ulid.generate()
                    shouldThrow<IllegalArgumentException> {
                        Location(
                            id = locationId,
                            userId = userId,
                            name = "Test",
                            description = null,
                            parentId = locationId,
                            address = null,
                            createdAt = now,
                            updatedAt = now,
                        )
                    }
                }
            }

            test("Container always prevents self as parent for any ULID") {
                repeat(50) {
                    val containerId = Ulid.generate()
                    shouldThrow<IllegalArgumentException> {
                        Container(
                            id = containerId,
                            userId = userId,
                            name = "Test",
                            description = null,
                            locationId = null,
                            parentContainerId = containerId,
                            label = null,
                            createdAt = now,
                            updatedAt = now,
                        )
                    }
                }
            }
        }

        // ============================================================================
        // Boundary Value Tests
        // ============================================================================

        context("Boundary value tests") {
            test("String fields accept exactly max length") {
                // User display name at exactly MAX_NAME_LENGTH
                val maxDisplayName = "a".repeat(MAX_NAME_LENGTH)
                val user = createUser(displayName = maxDisplayName)
                user.displayName.length shouldBe MAX_NAME_LENGTH

                // Quest title at exactly MAX_TITLE_LENGTH
                val maxTitle = "a".repeat(MAX_TITLE_LENGTH)
                val quest = createQuest(title = maxTitle)
                quest.title.length shouldBe MAX_TITLE_LENGTH

                // Tag name at exactly MAX_TAG_NAME_LENGTH
                val maxTagName = "a".repeat(MAX_TAG_NAME_LENGTH)
                val tag =
                    Tag(
                        id = Ulid.generate(),
                        userId = userId,
                        name = maxTagName,
                        color = null,
                        createdAt = now,
                        updatedAt = now,
                    )
                tag.name.length shouldBe MAX_TAG_NAME_LENGTH
            }

            test("String fields reject exactly max length plus one") {
                // User display name at MAX_NAME_LENGTH + 1
                shouldThrow<IllegalArgumentException> {
                    createUser(displayName = "a".repeat(MAX_NAME_LENGTH + 1))
                }

                // Quest title at MAX_TITLE_LENGTH + 1
                shouldThrow<IllegalArgumentException> {
                    createQuest(title = "a".repeat(MAX_TITLE_LENGTH + 1))
                }

                // Tag name at MAX_TAG_NAME_LENGTH + 1
                shouldThrow<IllegalArgumentException> {
                    Tag(
                        id = Ulid.generate(),
                        userId = userId,
                        name = "a".repeat(MAX_TAG_NAME_LENGTH + 1),
                        color = null,
                        createdAt = now,
                        updatedAt = now,
                    )
                }
            }

            test("Energy cost boundary values") {
                // Min valid (1)
                val questMin = createQuest(energyCost = 1)
                questMin.energyCost shouldBe 1

                // Max valid (5)
                val questMax = createQuest(energyCost = 5)
                questMax.energyCost shouldBe 5

                // Just below min (0)
                shouldThrow<IllegalArgumentException> {
                    createQuest(energyCost = 0)
                }

                // Just above max (6)
                shouldThrow<IllegalArgumentException> {
                    createQuest(energyCost = 6)
                }
            }

            test("Storage boundary at zero") {
                // Zero is valid
                val user = createUser(storageUsedBytes = 0, storageQuotaBytes = 0)
                user.storageUsedBytes shouldBeExactly 0L
                user.storageQuotaBytes shouldBeExactly 0L
                user.storageRemainingBytes shouldBeExactly 0L
                user.isOverQuota.shouldBeFalse() // 0 is not over 0
            }
        }
    })
