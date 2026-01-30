package com.getaltair.altair.ui.today

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.dto.guidance.TodayViewResponse
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Today View - Main screen for daily Guidance workflow.
 *
 * Displays energy budget, active quest (WIP=1), ready quests,
 * routine instances, and completed quests for the current day.
 *
 * @param todayView Today's data snapshot (nullable when loading)
 * @param isLoading Loading state indicator
 * @param onStartQuest Callback when user starts a quest (triggers WIP=1 check)
 * @param onCompleteQuest Callback when user completes the active quest
 * @param onAbandonQuest Callback when user abandons the active quest
 * @param onQuestClick Callback when user taps a quest for details
 * @param modifier Optional modifier
 */
@Composable
fun TodayScreen(
    todayView: TodayViewResponse?,
    isLoading: Boolean,
    onStartQuest: (String) -> Unit,
    onCompleteQuest: (String) -> Unit,
    onAbandonQuest: (String) -> Unit,
    onQuestClick: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    if (isLoading) {
        Box(
            modifier = modifier.fillMaxSize(),
            contentAlignment = Alignment.Center
        ) {
            BasicText(
                text = "Loading...",
                style = AltairTheme.typography.bodyLarge.copy(
                    color = AltairTheme.colors.accent
                )
            )
        }
        return
    }

    if (todayView == null) {
        Box(
            modifier = modifier.fillMaxSize(),
            contentAlignment = Alignment.Center
        ) {
            BasicText(
                text = "Unable to load today's view",
                style = AltairTheme.typography.bodyLarge.copy(
                    color = AltairTheme.colors.textSecondary
                )
            )
        }
        return
    }

    LazyColumn(
        modifier = modifier
            .fillMaxSize()
            .padding(horizontal = AltairTheme.spacing.md),
        verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.md)
    ) {
        // Energy Budget Card (top)
        item {
            EnergyBudgetCard(
                energyBudget = todayView.energyBudget,
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(top = AltairTheme.spacing.md)
            )
        }

        // Active Quest Card (WIP=1)
        item {
            ActiveQuestCard(
                activeQuest = todayView.activeQuest,
                onComplete = onCompleteQuest,
                onAbandon = onAbandonQuest,
                onQuestClick = onQuestClick,
                modifier = Modifier.fillMaxWidth()
            )
        }

        // Ready Quests Section
        item {
            ReadyQuestSection(
                readyQuests = todayView.readyQuests,
                routineInstances = todayView.routineInstances,
                onStartQuest = onStartQuest,
                onQuestClick = onQuestClick,
                modifier = Modifier.fillMaxWidth()
            )
        }

        // Completed Today Section
        item {
            CompletedTodaySection(
                completedQuests = todayView.completedToday,
                onQuestClick = onQuestClick,
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(bottom = AltairTheme.spacing.md)
            )
        }
    }
}
