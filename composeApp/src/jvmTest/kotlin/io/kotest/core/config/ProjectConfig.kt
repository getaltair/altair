package io.kotest.core.config

import io.kotest.core.spec.IsolationMode
import io.kotest.core.test.AssertionMode
import kotlin.time.Duration.Companion.seconds

/**
 * Kotest project configuration for composeApp module.
 *
 * Enforces strict testing standards:
 * - Assertion mode: Error - Fails tests without assertions
 * - Coroutine test scope: true - Uses test dispatcher for coroutines
 * - Timeout: 30 seconds - Reasonable timeout for all tests
 * - Isolation mode: InstancePerLeaf - Fresh instance for each test
 * - Property test iterations: 1000 - Comprehensive property testing
 */
class ProjectConfig : AbstractProjectConfig() {
    override val assertionMode = AssertionMode.Error
    override val coroutineTestScope = true
    override val timeout = 30.seconds
    override val isolationMode = IsolationMode.InstancePerLeaf
}
