package com.getaltair.altair.ui.inbox

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.domain.common.CaptureSource
import com.getaltair.altair.shared.dto.system.InboxItemResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.components.AltairCard
import com.getaltair.altair.ui.theme.components.AltairChip
/**
 * Card component for displaying a single inbox item.
 *
 * Shows the captured content, source badge, and timestamp. Clicking the card
 * triggers the triage flow. Optional swipe-to-delete functionality can be
 * added in future iterations.
 *
 * @param item The inbox item to display
 * @param onClick Callback when the card is clicked to initiate triage
 * @param onDelete Callback when the item should be deleted
 * @param modifier Modifier to be applied to the card
 */
@Composable
fun InboxItemCard(
    item: InboxItemResponse,
    onClick: () -> Unit,
    onDelete: () -> Unit,
    modifier: Modifier = Modifier
) {
    AltairCard(
        modifier = modifier.fillMaxWidth(),
        onClick = onClick
    ) {
        // Content text
        BasicText(
            text = item.content,
            style = AltairTheme.typography.bodyMedium.copy(
                color = AltairTheme.colors.textPrimary
            ),
            maxLines = 3
        )

        Spacer(modifier = Modifier.height(AltairTheme.spacing.md))

        // Metadata row: source badge and timestamp
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            // Source badge
            AltairChip(
                label = item.source.displayName(),
                selected = false
            )

            // Timestamp
            BasicText(
                text = formatTimestamp(item.createdAt),
                style = AltairTheme.typography.labelSmall.copy(
                    color = AltairTheme.colors.textSecondary
                )
            )
        }

        // Attachment count (if any)
        if (item.attachments.isNotEmpty()) {
            Spacer(modifier = Modifier.height(AltairTheme.spacing.sm))

            BasicText(
                text = "${item.attachments.size} attachment${if (item.attachments.size > 1) "s" else ""}",
                style = AltairTheme.typography.labelSmall.copy(
                    color = AltairTheme.colors.textTertiary
                )
            )
        }
    }
}

/**
 * Converts CaptureSource enum to user-friendly display name.
 */
private fun CaptureSource.displayName(): String = when (this) {
    CaptureSource.KEYBOARD -> "Keyboard"
    CaptureSource.VOICE -> "Voice"
    CaptureSource.CAMERA -> "Camera"
    CaptureSource.SHARE -> "Share"
    CaptureSource.WIDGET -> "Widget"
    CaptureSource.WATCH -> "Watch"
}

/**
 * Formats ISO 8601 timestamp to simple display.
 *
 * For MVP, returns a simplified date/time format.
 * Future enhancement: implement relative time ("5m ago", "Yesterday", etc.)
 * using kotlinx-datetime when added as a dependency.
 */
private fun formatTimestamp(isoTimestamp: String): String {
    return try {
        // Simple extraction: "2024-01-29T14:23:45Z" -> "Jan 29, 14:23"
        val parts = isoTimestamp.split("T")
        if (parts.size == 2) {
            val datePart = parts[0] // "2024-01-29"
            val timePart = parts[1].substringBefore("Z").substringBefore(".") // "14:23:45"

            val dateComponents = datePart.split("-")
            val timeComponents = timePart.split(":")

            if (dateComponents.size == 3 && timeComponents.size >= 2) {
                val month = when (dateComponents[1]) {
                    "01" -> "Jan"
                    "02" -> "Feb"
                    "03" -> "Mar"
                    "04" -> "Apr"
                    "05" -> "May"
                    "06" -> "Jun"
                    "07" -> "Jul"
                    "08" -> "Aug"
                    "09" -> "Sep"
                    "10" -> "Oct"
                    "11" -> "Nov"
                    "12" -> "Dec"
                    else -> dateComponents[1]
                }
                val day = dateComponents[2]
                val hour = timeComponents[0]
                val minute = timeComponents[1]

                return "$month $day, $hour:$minute"
            }
        }
        isoTimestamp // Fallback to raw timestamp
    } catch (e: Exception) {
        isoTimestamp // Fallback to raw timestamp on parse error
    }
}
