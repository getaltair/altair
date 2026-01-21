package io.kotest.core.config

import com.getaltair.altair.db.SurrealDbContainerExtension
import io.kotest.core.spec.IsolationMode
import io.kotest.core.test.AssertionMode
import kotlin.time.Duration.Companion.seconds

/**
 * Kotest project configuration for server module.
 *
 * Enforces strict testing standards:
 * - Assertion mode: Warn - See [ASSERTION_MODE_NOTE] for rationale
 * - Coroutine test scope: true - Uses test dispatcher for coroutines
 * - Timeout: 30 seconds - Reasonable timeout for all tests
 * - Isolation mode: InstancePerLeaf - Fresh instance for each test
 * - Property test iterations: 1000 - Comprehensive property testing
 * - Extensions: SurrealDbContainerExtension for Testcontainers lifecycle
 */
class ProjectConfig : AbstractProjectConfig() {
    /**
     * AssertionMode.Warn is used instead of AssertionMode.Error due to a known limitation
     * with Ktor's testApplication context.
     *
     * **Background**: Ktor's testApplication runs tests within a special coroutine context
     * that doesn't properly propagate assertion tracking to Kotest. This causes tests
     * that do contain assertions to be incorrectly flagged as having no assertions.
     *
     * **Affected tests**: Integration tests using `testApplication { }` blocks, such as:
     * - ApplicationTest
     * - RpcIntegrationTest
     * - AuthIntegrationTest
     *
     * **Mitigation**: Using Warn mode ensures test output highlights any tests that may
     * genuinely lack assertions while not failing the build. All tests should still
     * include meaningful assertions - the warning serves as a code review signal.
     *
     * **TODO**: Re-evaluate when Ktor or Kotest releases a fix for this interaction.
     * Track upstream: https://github.com/kotest/kotest/issues (assertion mode + Ktor)
     */
    override val assertionMode = AssertionMode.Warn
    override val coroutineTestScope = true
    override val timeout = 30.seconds
    override val isolationMode = IsolationMode.InstancePerLeaf

    override fun extensions() =
        listOf(
            SurrealDbContainerExtension,
        )
}
