package com.getaltair.altair.ui.components

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair-styled dialog component.
 *
 * A modal dialog that appears above the content with a scrim overlay.
 * Use for confirmations, alerts, and small forms that require user attention.
 *
 * @param visible Whether the dialog is visible.
 * @param onDismissRequest Callback invoked when the dialog should be dismissed.
 * @param modifier Modifier to be applied to the dialog panel.
 * @param title Optional title displayed at the top of the dialog.
 * @param content The dialog body content.
 */
@Composable
fun AltairDialog(
    visible: Boolean,
    onDismissRequest: () -> Unit,
    modifier: Modifier = Modifier,
    title: String? = null,
    content: @Composable ColumnScope.() -> Unit,
) {
    if (visible) {
        Dialog(
            onDismissRequest = onDismissRequest,
            properties = DialogProperties(
                dismissOnBackPress = true,
                dismissOnClickOutside = true,
            ),
        ) {
            Column(
                modifier = modifier
                    .widthIn(min = 280.dp, max = 480.dp)
                    .shadow(
                        elevation = 8.dp,
                        shape = RoundedCornerShape(AltairTheme.Radii.lg),
                        ambientColor = Color.Black.copy(alpha = 0.3f),
                        spotColor = Color.Black.copy(alpha = 0.3f),
                    )
                    .clip(RoundedCornerShape(AltairTheme.Radii.lg))
                    .background(AltairTheme.Colors.backgroundElevated)
                    .border(
                        width = 1.dp,
                        color = AltairTheme.Colors.border,
                        shape = RoundedCornerShape(AltairTheme.Radii.lg),
                    )
                    .padding(AltairTheme.Spacing.lg),
            ) {
                if (title != null) {
                    AltairText(
                        text = title,
                        style = AltairTheme.Typography.headlineMedium,
                        color = AltairTheme.Colors.textPrimary,
                    )
                    Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
                }
                content()
            }
        }
    }
}

/**
 * Altair-styled confirmation dialog.
 *
 * A specialized dialog for confirmations with title, message,
 * and confirm/cancel action buttons.
 *
 * @param visible Whether the dialog is visible.
 * @param onDismissRequest Callback invoked when the dialog should be dismissed.
 * @param onConfirm Callback invoked when the confirm button is clicked.
 * @param title Dialog title.
 * @param message Dialog message or description.
 * @param confirmText Text for the confirm button.
 * @param cancelText Text for the cancel button.
 * @param isDestructive Whether the action is destructive (uses danger styling).
 * @param modifier Modifier to be applied to the dialog panel.
 */
@Composable
fun AltairConfirmDialog(
    visible: Boolean,
    onDismissRequest: () -> Unit,
    onConfirm: () -> Unit,
    title: String,
    message: String,
    confirmText: String = "Confirm",
    cancelText: String = "Cancel",
    isDestructive: Boolean = false,
    modifier: Modifier = Modifier,
) {
    AltairDialog(
        visible = visible,
        onDismissRequest = onDismissRequest,
        modifier = modifier,
        title = title,
    ) {
        AltairText(
            text = message,
            style = AltairTheme.Typography.bodyMedium,
            color = AltairTheme.Colors.textSecondary,
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.lg))
        Row(
            modifier = Modifier.align(Alignment.End),
            horizontalArrangement = Arrangement.End,
        ) {
            AltairButton(
                onClick = onDismissRequest,
                variant = ButtonVariant.Ghost,
            ) {
                AltairText(
                    text = cancelText,
                    style = AltairTheme.Typography.labelLarge,
                )
            }
            Spacer(modifier = Modifier.width(AltairTheme.Spacing.sm))
            AltairButton(
                onClick = {
                    onConfirm()
                    onDismissRequest()
                },
                variant = if (isDestructive) ButtonVariant.Danger else ButtonVariant.Primary,
            ) {
                AltairText(
                    text = confirmText,
                    style = AltairTheme.Typography.labelLarge,
                )
            }
        }
    }
}

/**
 * Altair-styled alert dialog.
 *
 * A simple alert dialog with a message and single acknowledgment button.
 *
 * @param visible Whether the dialog is visible.
 * @param onDismissRequest Callback invoked when the dialog should be dismissed.
 * @param title Dialog title.
 * @param message Dialog message.
 * @param buttonText Text for the acknowledgment button.
 * @param modifier Modifier to be applied to the dialog panel.
 */
@Composable
fun AltairAlertDialog(
    visible: Boolean,
    onDismissRequest: () -> Unit,
    title: String,
    message: String,
    buttonText: String = "OK",
    modifier: Modifier = Modifier,
) {
    AltairDialog(
        visible = visible,
        onDismissRequest = onDismissRequest,
        modifier = modifier,
        title = title,
    ) {
        AltairText(
            text = message,
            style = AltairTheme.Typography.bodyMedium,
            color = AltairTheme.Colors.textSecondary,
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.lg))
        Row(
            modifier = Modifier.align(Alignment.End),
            horizontalArrangement = Arrangement.End,
        ) {
            AltairButton(
                onClick = onDismissRequest,
                variant = ButtonVariant.Primary,
            ) {
                AltairText(
                    text = buttonText,
                    style = AltairTheme.Typography.labelLarge,
                )
            }
        }
    }
}
