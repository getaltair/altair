package com.getaltair.altair.ui.today

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
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.dto.guidance.QuestSummaryResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.bodySmall
import com.getaltair.altair.ui.theme.headingMedium
import com.getaltair.altair.ui.theme.components.AltairButton
import com.getaltair.altair.ui.theme.components.ButtonVariant
import com.getaltair.altair.ui.theme.components.AltairCard

/**
 * Ready Quest Section - Lists quests available to start.
 *
 * Displays ready quests and routine instances in a vertical list.
 * Each quest shows title, energy cost, and checkpoint progress.
 * Users can tap to start a quest (triggers WIP=1 validation).
 *
 * @param readyQuests Available ready quests
 * @param routineInstances Routine instances due today
 * @param onStartQuest Callback when user starts a quest
 * @param onQuestClick Callback when user taps quest for details
 * @param modifier Optional modifier
 */
@Composable
fun ReadyQuestSection(
    readyQuests: List<QuestSummaryResponse>,
    routineInstances: List<QuestSummaryResponse>,
    onStartQuest: (String) -> Unit,
    onQuestClick: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier,
        verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
    ) {
        // Section header
        BasicText(
            text = "Ready to Start",
            style = AltairTheme.typography.headingMedium.copy(
                color = AltairTheme.colors.textPrimary
            )
        )

        val allReadyQuests = readyQuests + routineInstances

        if (allReadyQuests.isEmpty()) {
            // Empty state
            AltairCard(modifier = Modifier.fillMaxWidth()) {
                BasicText(
                    text = "All caught up! No quests ready.",
                    style = AltairTheme.typography.bodyMedium.copy(
                        color = AltairTheme.colors.textSecondary
                    ),
                    modifier = Modifier.align(Alignment.CenterHorizontally)
                )
            }
        } else {
            // Quest list
            Column(
                verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
            ) {
                allReadyQuests.forEach { quest ->
                    ReadyQuestCard(
                        quest = quest,
                        onStartQuest = { onStartQuest(quest.id) },
                        onQuestClick = { onQuestClick(quest.id) },
                        modifier = Modifier.fillMaxWidth()
                    )
                }
            }
        }
    }
}

/**
 * Individual ready quest card.
 *
 * @param quest Quest summary data
 * @param onStartQuest Callback when start button clicked
 * @param onQuestClick Callback when card clicked for details
 * @param modifier Optional modifier
 */
@Composable
private fun ReadyQuestCard(
    quest: QuestSummaryResponse,
    onStartQuest: () -> Unit,
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

                Row(
                    horizontalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    // Energy cost
                    BasicText(
                        text = "${quest.energyCost} energy",
                        style = AltairTheme.typography.bodySmall.copy(
                            color = AltairTheme.colors.textSecondary
                        )
                    )

                    // Checkpoint progress (if any)
                    if (quest.checkpointCount > 0) {
                        BasicText(
                            text = "•",
                            style = AltairTheme.typography.bodySmall.copy(
                                color = AltairTheme.colors.textTertiary
                            )
                        )
                        BasicText(
                            text = "${quest.completedCheckpointCount}/${quest.checkpointCount} checkpoints",
                            style = AltairTheme.typography.bodySmall.copy(
                                color = AltairTheme.colors.textSecondary
                            )
                        )
                    }
                }
            }

            // Start button
            AltairButton(
                onClick = onStartQuest,
                variant = ButtonVariant.Primary,
                modifier = Modifier.padding(start = AltairTheme.spacing.sm)
            ) {
                BasicText(
                    text = "Start",
                    style = TextStyle(color = androidx.compose.ui.graphics.Color.White)
                )
            }
        }
    }
}
