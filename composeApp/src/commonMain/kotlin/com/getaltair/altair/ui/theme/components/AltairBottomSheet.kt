package com.getaltair.altair.ui.theme.components

import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectVerticalDragGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import com.getaltair.altair.ui.theme.AltairColors

/**
 * Altair-styled bottom sheet component.
 *
 * Provides a modal sheet that slides up from the bottom with a drag handle,
 * elevated background, and swipe-to-dismiss gesture.
 *
 * @param onDismissRequest Callback invoked when the user tries to dismiss the sheet
 * @param modifier Modifier to be applied to the sheet container
 * @param content Sheet content laid out in a Column
 */
@Composable
fun AltairBottomSheet(
    onDismissRequest: () -> Unit,
    modifier: Modifier = Modifier,
    content: @Composable ColumnScope.() -> Unit
) {
    var dragOffset by remember { mutableStateOf(0f) }
    val dismissThreshold = 200f // Dismiss if dragged down more than 200dp

    Dialog(onDismissRequest = onDismissRequest) {
        Box(
            modifier = Modifier.fillMaxSize(),
            contentAlignment = Alignment.BottomCenter
        ) {
            // Scrim
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .background(androidx.compose.ui.graphics.Color.Black.copy(alpha = 0.5f))
            )

            // Bottom sheet
            Column(
                modifier = modifier
                    .fillMaxWidth()
                    .clip(RoundedCornerShape(topStart = 16.dp, topEnd = 16.dp))
                    .background(AltairColors.surfaceElevated)
                    .pointerInput(Unit) {
                        detectVerticalDragGestures(
                            onDragEnd = {
                                if (dragOffset > dismissThreshold) {
                                    onDismissRequest()
                                }
                                dragOffset = 0f
                            },
                            onVerticalDrag = { _, dragAmount ->
                                // Only allow dragging down
                                if (dragAmount > 0) {
                                    dragOffset += dragAmount
                                }
                            }
                        )
                    }
                    .padding(top = 8.dp)
            ) {
                // Drag handle
                Box(
                    modifier = Modifier
                        .width(32.dp)
                        .height(4.dp)
                        .clip(RoundedCornerShape(2.dp))
                        .background(AltairColors.border)
                        .align(Alignment.CenterHorizontally)
                )

                // Content
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp, vertical = 24.dp),
                    content = content
                )
            }
        }
    }
}
