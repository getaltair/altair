package com.getaltair.altair

import androidx.compose.runtime.Composable
import com.getaltair.altair.ui.preview.ComponentPreview
import com.getaltair.altair.ui.theme.AltairTheme
import org.jetbrains.compose.ui.tooling.preview.Preview

/**
 * Main application composable.
 *
 * Wraps the application content with AltairTheme to provide design tokens
 * throughout the composition tree. Currently displays the ComponentPreview
 * for design system verification.
 */
@Composable
@Preview
fun App() {
    AltairTheme {
        ComponentPreview()
    }
}
