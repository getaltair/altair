package com.getaltair.altair.ui.guidance

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.dto.guidance.QuestSummaryResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.bodySmall
import com.getaltair.altair.ui.theme.components.AltairCard

/**
 * Quest card component for list views.
 *
 * Displays quest title, energy badge, checkpoint progress, and status indicator.
 * Title is truncated to a single line with ellipsis.
 *
 * @param quest Quest summary data
 * @param onClick Callback invoked when the card is clicked
 * @param modifier Modifier to be applied to the card
 */
@Composable
fun QuestCard(
    quest: QuestSummaryResponse,
    onClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography

    // Status indicator color
    val statusColor = when (quest.status) {
        QuestStatus.BACKLOG -> colors.textTertiary
        QuestStatus.ACTIVE -> colors.accent
        QuestStatus.COMPLETED -> colors.success
        QuestStatus.ABANDONED -> colors.error
    }

    AltairCard(
        modifier = modifier.fillMaxWidth(),
        onClick = onClick
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.Top
        ) {
            // Status indicator dot
            Box(
                modifier = Modifier
                    .padding(top = 6.dp, end = 12.dp)
                    .size(8.dp)
                    .clip(CircleShape)
                    .background(statusColor)
            )

            // Content column
            Column(
                modifier = Modifier.weight(1f),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                // Title
                BasicText(
                    text = quest.title,
                    style = typography.bodyLarge.copy(
                        color = colors.textPrimary
                    ),
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )

                // Bottom row with progress and energy
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    // Checkpoint progress
                    if (quest.checkpointCount > 0) {
                        BasicText(
                            text = "${quest.completedCheckpointCount}/${quest.checkpointCount} completed",
                            style = TextStyle(
                                color = colors.textSecondary,
                                fontSize = typography.bodySmall.fontSize
                            )
                        )
                    }

                    // Energy badge
                    EnergyBadge(energyCost = quest.energyCost)
                }
            }
        }
    }
}
