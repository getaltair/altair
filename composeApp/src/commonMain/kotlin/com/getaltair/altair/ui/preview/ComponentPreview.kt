package com.getaltair.altair.ui.preview

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.components.AltairButton
import com.getaltair.altair.ui.components.AltairCard
import com.getaltair.altair.ui.components.AltairTextField
import com.getaltair.altair.ui.components.ButtonVariant
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Component preview screen for visual verification of the Altair Design System.
 *
 * Displays all component variants and design tokens in a scrollable view
 * for development verification purposes.
 */
@Composable
internal fun ComponentPreview() {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography
    val spacing = AltairTheme.spacing

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(colors.background)
            .verticalScroll(rememberScrollState())
            .padding(spacing.lg),
        verticalArrangement = Arrangement.spacedBy(spacing.lg),
    ) {
        // Header
        Text(
            text = "Altair Design System",
            style = typography.displayLarge,
            color = colors.textPrimary,
        )

        Text(
            text = "Component Preview",
            style = typography.headlineMedium,
            color = colors.textSecondary,
        )

        Spacer(modifier = Modifier.height(spacing.md))

        // Button Variants Section
        SectionHeader("Button Variants")

        Row(
            horizontalArrangement = Arrangement.spacedBy(spacing.md),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            AltairButton(
                onClick = {},
                variant = ButtonVariant.Primary,
            ) {
                Text(
                    text = "Primary",
                    style = typography.bodyMedium,
                    color = colors.textPrimary,
                )
            }

            AltairButton(
                onClick = {},
                variant = ButtonVariant.Secondary,
            ) {
                Text(
                    text = "Secondary",
                    style = typography.bodyMedium,
                    color = colors.textPrimary,
                )
            }

            AltairButton(
                onClick = {},
                variant = ButtonVariant.Ghost,
            ) {
                Text(
                    text = "Ghost",
                    style = typography.bodyMedium,
                    color = colors.textSecondary,
                )
            }
        }

        // Disabled Button
        Row(
            horizontalArrangement = Arrangement.spacedBy(spacing.md),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            AltairButton(
                onClick = {},
                variant = ButtonVariant.Primary,
                enabled = false,
            ) {
                Text(
                    text = "Disabled",
                    style = typography.bodyMedium,
                    color = colors.textPrimary,
                )
            }
        }

        Spacer(modifier = Modifier.height(spacing.md))

        // TextField Section
        SectionHeader("Text Field")

        var textValue by remember { mutableStateOf("") }

        AltairTextField(
            value = textValue,
            onValueChange = { textValue = it },
            modifier = Modifier.fillMaxWidth(),
            placeholder = {
                Text(
                    text = "Enter text here...",
                    style = typography.bodyMedium,
                    color = colors.textTertiary,
                )
            },
        )

        var filledValue by remember { mutableStateOf("Sample input text") }

        AltairTextField(
            value = filledValue,
            onValueChange = { filledValue = it },
            modifier = Modifier.fillMaxWidth(),
        )

        Spacer(modifier = Modifier.height(spacing.md))

        // Card Section
        SectionHeader("Card")

        AltairCard(
            modifier = Modifier.fillMaxWidth(),
        ) {
            Column(
                verticalArrangement = Arrangement.spacedBy(spacing.sm),
            ) {
                Text(
                    text = "Card Title",
                    style = typography.headlineMedium,
                    color = colors.textPrimary,
                )
                Text(
                    text = "This is a card component with elevated surface background. " +
                        "Hover to see the hover state effect on desktop.",
                    style = typography.bodyMedium,
                    color = colors.textSecondary,
                )
            }
        }

        Spacer(modifier = Modifier.height(spacing.lg))

        // Token Gallery Section
        SectionHeader("Color Tokens")

        ColorTokenGallery()

        Spacer(modifier = Modifier.height(spacing.md))

        SectionHeader("Typography Scale")

        TypographyGallery()

        Spacer(modifier = Modifier.height(spacing.md))

        SectionHeader("Spacing Scale")

        SpacingGallery()
    }
}

@Composable
private fun SectionHeader(title: String) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography

    Text(
        text = title,
        style = typography.headlineMedium,
        color = colors.textPrimary,
    )
}

@Composable
private fun ColorTokenGallery() {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography
    val spacing = AltairTheme.spacing

    Column(
        verticalArrangement = Arrangement.spacedBy(spacing.sm),
    ) {
        ColorSwatch("background", colors.background)
        ColorSwatch("surface", colors.surface)
        ColorSwatch("surfaceElevated", colors.surfaceElevated)
        ColorSwatch("surfaceHover", colors.surfaceHover)
        ColorSwatch("border", colors.border)
        ColorSwatch("borderFocused", colors.borderFocused)
        ColorSwatch("textPrimary", colors.textPrimary)
        ColorSwatch("textSecondary", colors.textSecondary)
        ColorSwatch("textTertiary", colors.textTertiary)
        ColorSwatch("accent", colors.accent)
        ColorSwatch("accentHover", colors.accentHover)
        ColorSwatch("success", colors.success)
        ColorSwatch("warning", colors.warning)
        ColorSwatch("error", colors.error)
    }
}

@Composable
private fun ColorSwatch(name: String, color: androidx.compose.ui.graphics.Color) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography
    val spacing = AltairTheme.spacing
    val shapes = AltairTheme.shapes

    Row(
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(spacing.sm),
    ) {
        Box(
            modifier = Modifier
                .size(32.dp)
                .background(color, shapes.sm),
        )
        Text(
            text = name,
            style = typography.bodyMedium,
            color = colors.textSecondary,
        )
    }
}

@Composable
private fun TypographyGallery() {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography
    val spacing = AltairTheme.spacing

    Column(
        verticalArrangement = Arrangement.spacedBy(spacing.sm),
    ) {
        Text(
            text = "displayLarge (32sp, SemiBold)",
            style = typography.displayLarge,
            color = colors.textPrimary,
        )
        Text(
            text = "headlineMedium (20sp, Medium)",
            style = typography.headlineMedium,
            color = colors.textPrimary,
        )
        Text(
            text = "bodyLarge (16sp, Normal)",
            style = typography.bodyLarge,
            color = colors.textPrimary,
        )
        Text(
            text = "bodyMedium (14sp, Normal)",
            style = typography.bodyMedium,
            color = colors.textPrimary,
        )
        Text(
            text = "labelSmall (12sp, Medium)",
            style = typography.labelSmall,
            color = colors.textPrimary,
        )
    }
}

@Composable
private fun SpacingGallery() {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography
    val spacing = AltairTheme.spacing
    val shapes = AltairTheme.shapes

    Column(
        verticalArrangement = Arrangement.spacedBy(spacing.sm),
    ) {
        SpacingRow("xs (4dp)", spacing.xs)
        SpacingRow("sm (8dp)", spacing.sm)
        SpacingRow("md (16dp)", spacing.md)
        SpacingRow("lg (24dp)", spacing.lg)
        SpacingRow("xl (32dp)", spacing.xl)
    }
}

@Composable
private fun SpacingRow(label: String, width: androidx.compose.ui.unit.Dp) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography
    val spacing = AltairTheme.spacing
    val shapes = AltairTheme.shapes

    Row(
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(spacing.sm),
    ) {
        Box(
            modifier = Modifier
                .width(width)
                .height(16.dp)
                .background(colors.accent, shapes.sm),
        )
        Text(
            text = label,
            style = typography.bodyMedium,
            color = colors.textSecondary,
        )
    }
}
