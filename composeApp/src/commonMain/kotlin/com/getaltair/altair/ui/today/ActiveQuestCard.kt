package com.getaltair.altair.ui.today

import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.dto.guidance.QuestResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.bodySmall
import com.getaltair.altair.ui.theme.headingMedium
import com.getaltair.altair.ui.theme.labelMedium
import com.getaltair.altair.ui.theme.components.AltairButton
import com.getaltair.altair.ui.theme.components.ButtonVariant
import com.getaltair.altair.ui.theme.components.AltairCard

/**
 * Active Quest Card - Displays the WIP=1 in-progress quest.
 *
 * Shows a prominent card for the currently active quest with:
 * - Quest title and energy cost
 * - Checkpoint progress indicator
 * - Complete and Abandon action buttons
 *
 * If no quest is active, shows an empty state message.
 *
 * @param activeQuest Currently active quest (null if none)
 * @param onComplete Callback when user completes the quest
 * @param onAbandon Callback when user abandons the quest
 * @param onQuestClick Callback when user taps quest for details
 * @param modifier Optional modifier
 */
@Composable
fun ActiveQuestCard(
    activeQuest: QuestResponse?,
    onComplete: (String) -> Unit,
    onAbandon: (String) -> Unit,
    onQuestClick: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    val cardModifier = if (activeQuest != null) {
        modifier.border(
            width = 2.dp,
            color = AltairTheme.colors.accent,
            shape = RoundedCornerShape(8.dp)
        )
    } else {
        modifier
    }

    AltairCard(
        modifier = cardModifier,
        onClick = if (activeQuest != null) {
            { onQuestClick(activeQuest.id) }
        } else null
    ) {
        if (activeQuest == null) {
            // Empty state
            Column(
                modifier = Modifier.fillMaxWidth(),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
            ) {
                BasicText(
                    text = "No Active Quest",
                    style = AltairTheme.typography.headingMedium.copy(
                        color = AltairTheme.colors.textSecondary
                    )
                )
                BasicText(
                    text = "Start a quest from Ready below (WIP=1)",
                    style = AltairTheme.typography.bodyMedium.copy(
                        color = AltairTheme.colors.textTertiary
                    )
                )
            }
        } else {
            // Active quest content
            Column(
                verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.md)
            ) {
                // Header
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.Top
                ) {
                    Column(modifier = Modifier.weight(1f)) {
                        BasicText(
                            text = "Active Quest",
                            style = AltairTheme.typography.labelSmall.copy(
                                color = AltairTheme.colors.accent
                            )
                        )
                        Spacer(modifier = Modifier.height(4.dp))
                        BasicText(
                            text = activeQuest.title,
                            style = AltairTheme.typography.headingMedium.copy(
                                color = AltairTheme.colors.textPrimary
                            )
                        )
                    }

                    // Energy cost badge
                    BasicText(
                        text = "${activeQuest.energyCost} energy",
                        style = AltairTheme.typography.labelMedium.copy(
                            color = AltairTheme.colors.textSecondary
                        ),
                        modifier = Modifier.padding(top = 4.dp)
                    )
                }

                // Checkpoint progress
                if (activeQuest.checkpoints.isNotEmpty()) {
                    val completedCount = activeQuest.checkpoints.count { it.completed }
                    val totalCount = activeQuest.checkpoints.size

                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        horizontalArrangement = Arrangement.SpaceBetween,
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        BasicText(
                            text = "Progress",
                            style = AltairTheme.typography.bodySmall.copy(
                                color = AltairTheme.colors.textSecondary
                            )
                        )
                        BasicText(
                            text = "$completedCount / $totalCount checkpoints",
                            style = AltairTheme.typography.bodyMedium.copy(
                                color = AltairTheme.colors.textPrimary
                            )
                        )
                    }
                }

                // Action buttons
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
                ) {
                    AltairButton(
                        onClick = { onComplete(activeQuest.id) },
                        modifier = Modifier.weight(1f),
                        variant = ButtonVariant.Primary
                    ) {
                        BasicText(
                            text = "Complete",
                            style = TextStyle(color = androidx.compose.ui.graphics.Color.White)
                        )
                    }

                    AltairButton(
                        onClick = { onAbandon(activeQuest.id) },
                        modifier = Modifier.weight(1f),
                        variant = ButtonVariant.Secondary
                    ) {
                        BasicText(
                            text = "Abandon",
                            style = TextStyle(color = AltairTheme.colors.error)
                        )
                    }
                }
            }
        }
    }
}
