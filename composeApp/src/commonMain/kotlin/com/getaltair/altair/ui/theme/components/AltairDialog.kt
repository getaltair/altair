package com.getaltair.altair.ui.theme.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.sp
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import com.getaltair.altair.ui.theme.AltairColors

/**
 * Altair-styled modal dialog component.
 *
 * Provides a centered modal overlay with scrim, title, content area,
 * and action buttons.
 *
 * @param onDismissRequest Callback invoked when the user tries to dismiss the dialog
 * @param title Dialog title displayed at the top
 * @param modifier Modifier to be applied to the dialog container
 * @param content Dialog content laid out in a Column
 * @param confirmButton Composable for the primary action button
 * @param dismissButton Optional composable for the secondary/cancel button
 */
@Composable
fun AltairDialog(
    onDismissRequest: () -> Unit,
    title: String,
    modifier: Modifier = Modifier,
    content: @Composable ColumnScope.() -> Unit,
    confirmButton: @Composable () -> Unit,
    dismissButton: (@Composable () -> Unit)? = null
) {
    Dialog(onDismissRequest = onDismissRequest) {
        Box(
            modifier = Modifier.fillMaxSize(),
            contentAlignment = Alignment.Center
        ) {
            // Scrim
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .background(androidx.compose.ui.graphics.Color.Black.copy(alpha = 0.5f))
            )

            // Dialog content
            val shape = RoundedCornerShape(12.dp)
            Column(
                modifier = modifier
                    .width(400.dp)
                    .clip(shape)
                    .background(AltairColors.surfaceElevated)
                    .border(
                        width = 1.dp,
                        color = AltairColors.border,
                        shape = shape
                    )
                    .padding(24.dp)
            ) {
                // Title
                BasicText(
                    text = title,
                    style = TextStyle(
                        color = AltairColors.textPrimary,
                        fontSize = 18.sp
                    ),
                    modifier = Modifier.padding(bottom = 16.dp)
                )

                // Content
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .weight(1f, fill = false),
                    content = content
                )

                Spacer(modifier = Modifier.height(24.dp))

                // Action buttons
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.End
                ) {
                    if (dismissButton != null) {
                        dismissButton()
                        Spacer(modifier = Modifier.width(8.dp))
                    }
                    confirmButton()
                }
            }
        }
    }
}
