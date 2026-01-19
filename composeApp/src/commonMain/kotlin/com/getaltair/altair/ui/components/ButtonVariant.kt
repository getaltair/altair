package com.getaltair.altair.ui.components

import androidx.compose.ui.graphics.Color
import com.getaltair.altair.ui.theme.AltairColors

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
internal fun ButtonVariant.backgroundColor(colors: AltairColors): Color =
    when (this) {
        ButtonVariant.Primary -> colors.accent
        ButtonVariant.Secondary -> colors.backgroundSubtle
        ButtonVariant.Ghost -> Color.Transparent
        ButtonVariant.Danger -> colors.error
    }

/**
 * Returns the background color when the button is hovered.
 */
internal fun ButtonVariant.backgroundColorHover(colors: AltairColors): Color =
    when (this) {
        ButtonVariant.Primary -> colors.accentHover
        ButtonVariant.Secondary -> colors.backgroundHover
        ButtonVariant.Ghost -> colors.backgroundHover
        ButtonVariant.Danger -> colors.errorHover
    }

/**
 * Returns the background color when the button is pressed.
 */
internal fun ButtonVariant.backgroundColorPressed(colors: AltairColors): Color =
    when (this) {
        ButtonVariant.Primary -> colors.accentPressed
        ButtonVariant.Secondary -> colors.backgroundPressed
        ButtonVariant.Ghost -> colors.backgroundPressed
        ButtonVariant.Danger -> colors.errorPressed
    }

/**
 * Returns the text/content color for the button variant.
 */
internal fun ButtonVariant.contentColor(colors: AltairColors): Color =
    when (this) {
        ButtonVariant.Primary -> colors.textPrimary
        ButtonVariant.Secondary -> colors.textPrimary
        ButtonVariant.Ghost -> colors.textSecondary
        ButtonVariant.Danger -> colors.textPrimary
    }

/**
 * Returns the border color for the button variant (if any).
 */
internal fun ButtonVariant.borderColor(colors: AltairColors): Color =
    when (this) {
        ButtonVariant.Primary -> Color.Transparent
        ButtonVariant.Secondary -> colors.border
        ButtonVariant.Ghost -> Color.Transparent
        ButtonVariant.Danger -> Color.Transparent
    }
