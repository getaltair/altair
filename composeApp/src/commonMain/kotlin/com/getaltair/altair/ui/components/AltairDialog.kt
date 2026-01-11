package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.widthIn
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair Design System dialog component.
 *
 * A modal dialog with scrim background and centered content panel.
 *
 * @param visible Whether the dialog is visible
 * @param onDismiss Callback when the dialog is dismissed
 * @param modifier Modifier to be applied to the dialog panel
 * @param title Optional title for the dialog
 * @param dismissOnClickOutside Whether clicking outside dismisses the dialog
 * @param content Content to display inside the dialog
 */
@Composable
fun AltairDialog(
    visible: Boolean,
    onDismiss: () -> Unit,
    modifier: Modifier = Modifier,
    title: String? = null,
    dismissOnClickOutside: Boolean = true,
    content: @Composable () -> Unit,
) {
    if (!visible) return

    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes
    val spacing = AltairTheme.spacing
    val typography = AltairTheme.typography

    Dialog(
        onDismissRequest = onDismiss,
        properties = DialogProperties(
            dismissOnClickOutside = dismissOnClickOutside,
            dismissOnBackPress = true,
        ),
    ) {
        Column(
            modifier = modifier
                .widthIn(min = 280.dp, max = 560.dp)
                .clip(shapes.lg)
                .background(colors.surfaceElevated)
                .border(1.dp, colors.border, shapes.lg)
                .padding(spacing.lg),
        ) {
            if (title != null) {
                Text(
                    text = title,
                    style = typography.headlineMedium,
                    color = colors.textPrimary,
                )
                Spacer(modifier = Modifier.height(spacing.md))
            }
            content()
        }
    }
}

/**
 * Altair Design System confirmation dialog.
 *
 * A pre-built dialog for confirmation actions with title, message,
 * and confirm/cancel buttons.
 *
 * @param visible Whether the dialog is visible
 * @param title Dialog title
 * @param message Dialog message
 * @param confirmText Text for the confirm button
 * @param cancelText Text for the cancel button
 * @param onConfirm Callback when confirm is clicked
 * @param onDismiss Callback when dialog is dismissed or cancel is clicked
 * @param modifier Modifier to be applied to the dialog panel
 */
@Composable
fun AltairConfirmDialog(
    visible: Boolean,
    title: String,
    message: String,
    confirmText: String = "Confirm",
    cancelText: String = "Cancel",
    onConfirm: () -> Unit,
    onDismiss: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = AltairTheme.colors
    val spacing = AltairTheme.spacing
    val typography = AltairTheme.typography

    AltairDialog(
        visible = visible,
        onDismiss = onDismiss,
        modifier = modifier,
        title = title,
    ) {
        Text(
            text = message,
            style = typography.bodyLarge,
            color = colors.textSecondary,
        )
        Spacer(modifier = Modifier.height(spacing.lg))
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(spacing.sm, Alignment.End),
        ) {
            AltairButton(
                onClick = onDismiss,
                variant = ButtonVariant.Ghost,
            ) {
                Text(
                    text = cancelText,
                    style = typography.bodyMedium,
                    color = colors.textSecondary,
                )
            }
            AltairButton(
                onClick = {
                    onConfirm()
                    onDismiss()
                },
                variant = ButtonVariant.Primary,
            ) {
                Text(
                    text = confirmText,
                    style = typography.bodyMedium,
                    color = colors.textPrimary,
                )
            }
        }
    }
}
