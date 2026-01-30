package com.getaltair.altair.ui.guidance

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.dto.guidance.QuestSummaryResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.components.AltairButton
import com.getaltair.altair.ui.theme.components.ButtonVariant

/**
 * Quest list screen with filtering and FAB for creating quests.
 *
 * Features:
 * - Status filter chips at top (All, Backlog, Active, Completed)
 * - LazyColumn of quest cards
 * - Empty state per filter
 * - Loading and error states
 * - FAB for creating new quest (opens QuestEditSheet)
 *
 * @param quests List of all quests
 * @param isLoading Whether data is currently loading
 * @param error Optional error message
 * @param onQuestClick Callback when a quest card is clicked
 * @param onCreateQuest Callback when FAB is clicked to create new quest
 * @param modifier Modifier to be applied to the screen
 */
@Composable
fun QuestListScreen(
    quests: List<QuestSummaryResponse>,
    isLoading: Boolean,
    error: String?,
    onQuestClick: (String) -> Unit,
    onCreateQuest: () -> Unit,
    modifier: Modifier = Modifier
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography

    var selectedFilter by remember { mutableStateOf<QuestFilter>(QuestFilter.All) }

    // Filter quests based on selected filter
    val filteredQuests = when (selectedFilter) {
        is QuestFilter.All -> quests
        is QuestFilter.Status -> quests.filter {
            it.status == (selectedFilter as QuestFilter.Status).status
        }
    }

    Box(
        modifier = modifier.fillMaxSize()
    ) {
        Column(
            modifier = Modifier.fillMaxSize()
        ) {
            // Filter chips
            StatusFilterChips(
                selectedFilter = selectedFilter,
                onFilterSelected = { selectedFilter = it }
            )

            // Content area
            when {
                error != null -> {
                    // Error state
                    ErrorState(message = error)
                }
                isLoading -> {
                    // Loading state
                    LoadingState()
                }
                filteredQuests.isEmpty() -> {
                    // Empty state
                    EmptyState(filter = selectedFilter)
                }
                else -> {
                    // Quest list
                    LazyColumn(
                        modifier = Modifier.fillMaxSize(),
                        contentPadding = PaddingValues(
                            horizontal = AltairTheme.spacing.md,
                            vertical = AltairTheme.spacing.sm
                        ),
                        verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
                    ) {
                        items(
                            items = filteredQuests,
                            key = { it.id }
                        ) { quest ->
                            QuestCard(
                                quest = quest,
                                onClick = { onQuestClick(quest.id) }
                            )
                        }

                        // Bottom padding for FAB
                        item {
                            Spacer(modifier = Modifier.height(80.dp))
                        }
                    }
                }
            }
        }

        // Floating Action Button (FAB)
        AltairButton(
            onClick = onCreateQuest,
            variant = ButtonVariant.Primary,
            modifier = Modifier
                .align(Alignment.BottomEnd)
                .padding(AltairTheme.spacing.lg)
        ) {
            BasicText(
                text = "+ Create Quest",
                style = TextStyle(color = androidx.compose.ui.graphics.Color.White)
            )
        }
    }
}

/**
 * Empty state display based on current filter.
 */
@Composable
private fun EmptyState(filter: QuestFilter) {
    val typography = AltairTheme.typography
    val colors = AltairTheme.colors

    val message = when (filter) {
        is QuestFilter.All -> "No quests yet.\nTap + to create your first quest."
        is QuestFilter.Status -> when (filter.status) {
            QuestStatus.BACKLOG -> "No quests in backlog.\nTap + to create a new quest."
            QuestStatus.ACTIVE -> "No active quests.\nSelect a quest from backlog to start."
            QuestStatus.COMPLETED -> "No completed quests yet.\nComplete a quest to see it here."
            QuestStatus.ABANDONED -> "No abandoned quests."
        }
    }

    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        BasicText(
            text = message,
            style = typography.bodyMedium.copy(
                color = colors.textTertiary,
                textAlign = TextAlign.Center
            )
        )
    }
}

/**
 * Loading state display.
 */
@Composable
private fun LoadingState() {
    val typography = AltairTheme.typography
    val colors = AltairTheme.colors

    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        BasicText(
            text = "Loading quests...",
            style = typography.bodyMedium.copy(
                color = colors.textSecondary
            )
        )
    }
}

/**
 * Error state display.
 */
@Composable
private fun ErrorState(message: String) {
    val typography = AltairTheme.typography
    val colors = AltairTheme.colors

    Box(
        modifier = Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
        ) {
            BasicText(
                text = "Error",
                style = typography.bodyLarge.copy(
                    color = colors.error
                )
            )
            BasicText(
                text = message,
                style = typography.bodyMedium.copy(
                    color = colors.textSecondary,
                    textAlign = TextAlign.Center
                )
            )
        }
    }
}
