package com.getaltair.altair.ui.today

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.expandVertically
import androidx.compose.animation.shrinkVertically
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.rotate
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.dto.guidance.QuestSummaryResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.bodySmall
import com.getaltair.altair.ui.theme.headingMedium
import com.getaltair.altair.ui.theme.components.AltairCard

/**
 * Completed Today Section - Collapsible list of completed quests.
 *
 * Shows quests completed today with checkmarks.
 * Collapsed by default when there are many items.
 * Each item is clickable to view quest details.
 *
 * @param completedQuests List of completed quests for today
 * @param onQuestClick Callback when user taps a quest for details
 * @param modifier Optional modifier
 */
@Composable
fun CompletedTodaySection(
    completedQuests: List<QuestSummaryResponse>,
    onQuestClick: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    if (completedQuests.isEmpty()) {
        return
    }

    // Collapse by default if more than 3 items
    var isExpanded by remember { mutableStateOf(completedQuests.size <= 3) }

    Column(
        modifier = modifier,
        verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
    ) {
        // Section header (clickable to expand/collapse)
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clickable { isExpanded = !isExpanded },
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            BasicText(
                text = "Completed Today (${completedQuests.size})",
                style = AltairTheme.typography.headingMedium.copy(
                    color = AltairTheme.colors.textPrimary
                )
            )

            // Expand/collapse indicator (simple rotated triangle)
            val rotation by animateFloatAsState(
                targetValue = if (isExpanded) 180f else 0f
            )
            BasicText(
                text = "▼",
                style = AltairTheme.typography.bodyMedium.copy(
                    color = AltairTheme.colors.textSecondary
                ),
                modifier = Modifier.rotate(rotation)
            )
        }

        // Completed quest list
        AnimatedVisibility(
            visible = isExpanded,
            enter = expandVertically(),
            exit = shrinkVertically()
        ) {
            Column(
                verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
            ) {
                completedQuests.forEach { quest ->
                    CompletedQuestCard(
                        quest = quest,
                        onQuestClick = { onQuestClick(quest.id) },
                        modifier = Modifier.fillMaxWidth()
                    )
                }
            }
        }
    }
}

/**
 * Individual completed quest card.
 *
 * @param quest Quest summary data
 * @param onQuestClick Callback when card clicked
 * @param modifier Optional modifier
 */
@Composable
private fun CompletedQuestCard(
    quest: QuestSummaryResponse,
    onQuestClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    AltairCard(
        modifier = modifier,
        onClick = onQuestClick
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            // Checkmark icon (using a simple circle with background)
            Box(
                modifier = Modifier
                    .size(20.dp)
                    .clip(CircleShape)
                    .background(AltairTheme.colors.success),
                contentAlignment = Alignment.Center
            ) {
                BasicText(
                    text = "✓",
                    style = AltairTheme.typography.labelSmall.copy(
                        color = androidx.compose.ui.graphics.Color.White
                    )
                )
            }

            Spacer(modifier = Modifier.width(AltairTheme.spacing.sm))

            // Quest info
            Column(
                modifier = Modifier.weight(1f),
                verticalArrangement = Arrangement.spacedBy(4.dp)
            ) {
                BasicText(
                    text = quest.title,
                    style = AltairTheme.typography.bodyLarge.copy(
                        color = AltairTheme.colors.textPrimary
                    )
                )

                BasicText(
                    text = "${quest.energyCost} energy",
                    style = AltairTheme.typography.bodySmall.copy(
                        color = AltairTheme.colors.textSecondary
                    )
                )
            }
        }
    }
}
