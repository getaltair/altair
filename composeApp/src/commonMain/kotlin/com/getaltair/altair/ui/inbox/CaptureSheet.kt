package com.getaltair.altair.ui.inbox

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.domain.common.CaptureSource
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.components.AltairBottomSheet
import com.getaltair.altair.ui.theme.components.AltairButton
import com.getaltair.altair.ui.theme.components.AltairTextField
import com.getaltair.altair.ui.theme.components.ButtonVariant

/**
 * Bottom sheet for quick capture to the Universal Inbox.
 *
 * Provides a simple text input field and capture button. The capture source
 * defaults to KEYBOARD for manual text entry.
 *
 * @param onDismiss Callback when the sheet should be dismissed
 * @param onCapture Callback with the captured text when user confirms
 * @param modifier Modifier to be applied to the bottom sheet
 */
@Composable
fun CaptureSheet(
    onDismiss: () -> Unit,
    onCapture: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    var captureText by remember { mutableStateOf("") }

    AltairBottomSheet(
        onDismissRequest = onDismiss,
        modifier = modifier
    ) {
        // Title
        BasicText(
            text = "Quick Capture",
            style = AltairTheme.typography.headlineMedium.copy(
                color = AltairTheme.colors.textPrimary
            )
        )

        Spacer(modifier = Modifier.height(AltairTheme.spacing.md))

        // Text input
        AltairTextField(
            value = captureText,
            onValueChange = { captureText = it },
            modifier = Modifier.fillMaxWidth(),
            placeholder = "What's on your mind?",
            singleLine = false
        )

        Spacer(modifier = Modifier.height(AltairTheme.spacing.lg))

        // Capture button
        AltairButton(
            onClick = {
                if (captureText.isNotBlank()) {
                    onCapture(captureText.trim())
                    onDismiss()
                }
            },
            variant = ButtonVariant.Primary,
            enabled = captureText.isNotBlank(),
            modifier = Modifier.fillMaxWidth()
        ) {
            BasicText(
                text = "Capture",
                style = TextStyle(color = androidx.compose.ui.graphics.Color.White)
            )
        }
    }
}
