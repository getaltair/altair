package com.getaltair.altair.ui.guidance

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.ui.theme.components.AltairChip

/**
 * Filter selection representing "All" statuses or a specific status.
 */
sealed class QuestFilter {
    data object All : QuestFilter()
    data class Status(val status: QuestStatus) : QuestFilter()
}

/**
 * Row of filter chips for quest status filtering.
 *
 * Displays chips for All, Backlog, Active, and Completed statuses.
 * Single selection with visual highlighting for the selected filter.
 *
 * @param selectedFilter Currently selected filter
 * @param onFilterSelected Callback invoked when a filter is selected
 * @param modifier Modifier to be applied to the row
 */
@Composable
fun StatusFilterChips(
    selectedFilter: QuestFilter,
    onFilterSelected: (QuestFilter) -> Unit,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        AltairChip(
            label = "All",
            selected = selectedFilter is QuestFilter.All,
            onClick = { onFilterSelected(QuestFilter.All) }
        )

        AltairChip(
            label = "Backlog",
            selected = selectedFilter is QuestFilter.Status &&
                    selectedFilter.status == QuestStatus.BACKLOG,
            onClick = { onFilterSelected(QuestFilter.Status(QuestStatus.BACKLOG)) }
        )

        AltairChip(
            label = "Active",
            selected = selectedFilter is QuestFilter.Status &&
                    selectedFilter.status == QuestStatus.ACTIVE,
            onClick = { onFilterSelected(QuestFilter.Status(QuestStatus.ACTIVE)) }
        )

        AltairChip(
            label = "Completed",
            selected = selectedFilter is QuestFilter.Status &&
                    selectedFilter.status == QuestStatus.COMPLETED,
            onClick = { onFilterSelected(QuestFilter.Status(QuestStatus.COMPLETED)) }
        )
    }
}
