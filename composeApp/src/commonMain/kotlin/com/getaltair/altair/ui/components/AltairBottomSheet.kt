package com.getaltair.altair.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectVerticalDragGestures
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Altair Design System bottom sheet component.
 *
 * A modal bottom sheet with scrim background and drag-to-dismiss.
 *
 * @param visible Whether the bottom sheet is visible
 * @param onDismiss Callback when the sheet is dismissed
 * @param modifier Modifier to be applied to the sheet
 * @param showDragHandle Whether to show a drag handle at the top
 * @param content Content to display inside the sheet
 */
@Composable
fun AltairBottomSheet(
    visible: Boolean,
    onDismiss: () -> Unit,
    modifier: Modifier = Modifier,
    showDragHandle: Boolean = true,
    content: @Composable () -> Unit,
) {
    if (!visible) return

    val colors = AltairTheme.colors
    val shapes = AltairTheme.shapes
    val spacing = AltairTheme.spacing

    val interactionSource = remember { MutableInteractionSource() }

    Dialog(
        onDismissRequest = onDismiss,
        properties = DialogProperties(
            dismissOnClickOutside = true,
            dismissOnBackPress = true,
            usePlatformDefaultWidth = false,
        ),
    ) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(Color.Black.copy(alpha = 0.5f))
                .clickable(
                    interactionSource = interactionSource,
                    indication = null,
                    onClick = onDismiss,
                ),
            contentAlignment = Alignment.BottomCenter,
        ) {
            Column(
                modifier = modifier
                    .fillMaxWidth()
                    .heightIn(min = 200.dp)
                    .clip(RoundedCornerShape(topStart = 16.dp, topEnd = 16.dp))
                    .background(colors.surfaceElevated)
                    .clickable(
                        interactionSource = remember { MutableInteractionSource() },
                        indication = null,
                        onClick = { /* Consume click to prevent dismissal */ },
                    )
                    .pointerInput(Unit) {
                        detectVerticalDragGestures { change, dragAmount ->
                            if (dragAmount > 50) {
                                onDismiss()
                            }
                        }
                    }
                    .padding(spacing.lg),
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                if (showDragHandle) {
                    Box(
                        modifier = Modifier
                            .width(40.dp)
                            .height(4.dp)
                            .clip(shapes.full)
                            .background(colors.textTertiary),
                    )
                    Spacer(modifier = Modifier.height(spacing.md))
                }
                content()
            }
        }
    }
}
