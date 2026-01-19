package com.getaltair.altair.ui.components

import androidx.compose.ui.graphics.Color
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Button style variants for [AltairButton].
 *
 * Each variant provides a distinct visual style for different semantic purposes:
 * - [Primary]: Main call-to-action buttons
 * - [Secondary]: Secondary actions, less prominent
 * - [Ghost]: Minimal style for tertiary actions
 * - [Danger]: Destructive or warning actions
 */
enum class ButtonVariant {
    /** Primary action button with accent background. */
    Primary,

    /** Secondary action button with subtle background. */
    Secondary,

    /** Ghost button with transparent background. */
    Ghost,

    /** Danger button for destructive actions. */
    Danger,
}

/**
 * Returns the background color for the button variant.
 */
internal fun ButtonVariant.backgroundColor(): Color = when (this) {
    ButtonVariant.Primary -> AltairTheme.Colors.accent
    ButtonVariant.Secondary -> AltairTheme.Colors.backgroundSubtle
    ButtonVariant.Ghost -> Color.Transparent
    ButtonVariant.Danger -> AltairTheme.Colors.error
}

/**
 * Returns the background color when the button is hovered.
 */
internal fun ButtonVariant.backgroundColorHover(): Color = when (this) {
    ButtonVariant.Primary -> AltairTheme.Colors.accentHover
    ButtonVariant.Secondary -> AltairTheme.Colors.backgroundHover
    ButtonVariant.Ghost -> AltairTheme.Colors.backgroundHover
    ButtonVariant.Danger -> Color(0xFFF87171) // Lighter red on hover
}

/**
 * Returns the background color when the button is pressed.
 */
internal fun ButtonVariant.backgroundColorPressed(): Color = when (this) {
    ButtonVariant.Primary -> AltairTheme.Colors.accentPressed
    ButtonVariant.Secondary -> AltairTheme.Colors.backgroundPressed
    ButtonVariant.Ghost -> AltairTheme.Colors.backgroundPressed
    ButtonVariant.Danger -> Color(0xFFB91C1C) // Darker red on press
}

/**
 * Returns the text/content color for the button variant.
 */
internal fun ButtonVariant.contentColor(): Color = when (this) {
    ButtonVariant.Primary -> AltairTheme.Colors.textPrimary
    ButtonVariant.Secondary -> AltairTheme.Colors.textPrimary
    ButtonVariant.Ghost -> AltairTheme.Colors.textSecondary
    ButtonVariant.Danger -> AltairTheme.Colors.textPrimary
}

/**
 * Returns the border color for the button variant (if any).
 */
internal fun ButtonVariant.borderColor(): Color = when (this) {
    ButtonVariant.Primary -> Color.Transparent
    ButtonVariant.Secondary -> AltairTheme.Colors.border
    ButtonVariant.Ghost -> Color.Transparent
    ButtonVariant.Danger -> Color.Transparent
}
