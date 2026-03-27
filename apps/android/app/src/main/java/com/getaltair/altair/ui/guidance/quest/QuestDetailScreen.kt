package com.getaltair.altair.ui.guidance.quest

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.getaltair.altair.domain.entity.Priority
import com.getaltair.altair.domain.entity.Quest
import com.getaltair.altair.domain.entity.QuestStatus
import com.getaltair.altair.ui.common.UiState
import com.getaltair.altair.ui.components.AltairButton
import com.getaltair.altair.ui.components.AltairCard
import com.getaltair.altair.ui.components.AltairDetailScaffold
import com.getaltair.altair.ui.components.AltairErrorBox
import com.getaltair.altair.ui.components.AltairLoadingBox
import com.getaltair.altair.util.capitalizeFirst
import java.time.format.DateTimeFormatter
import org.koin.androidx.compose.koinViewModel

@Composable
fun QuestDetailScreen(
    onBack: () -> Unit,
    viewModel: QuestDetailViewModel = koinViewModel(),
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    val title = when (val state = uiState) {
        is UiState.Success -> state.data.name
        else -> "Quest"
    }

    AltairDetailScaffold(title = title, onBack = onBack) { innerPadding ->
        when (val state = uiState) {
            is UiState.Loading -> {
                AltairLoadingBox(modifier = Modifier.padding(innerPadding))
            }

            is UiState.Error -> {
                AltairErrorBox(
                    message = state.message,
                    modifier = Modifier.padding(innerPadding),
                )
            }

            is UiState.Success -> {
                QuestDetailContent(
                    quest = state.data,
                    onComplete = viewModel::completeQuest,
                    modifier = Modifier.padding(innerPadding),
                )
            }
        }
    }
}

@Composable
private fun QuestDetailContent(
    quest: Quest,
    onComplete: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val isCompleted = quest.status == QuestStatus.COMPLETED
    val isInProgress = quest.status == QuestStatus.IN_PROGRESS

    Column(
        modifier = modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = 16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        AltairCard(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Text(
                        text = quest.status.value.capitalizeFirst(),
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.primary,
                    )
                    Text(
                        text = quest.priority.value.capitalizeFirst(),
                        style = MaterialTheme.typography.labelMedium,
                        color = if (quest.priority == Priority.HIGH) {
                            MaterialTheme.colorScheme.error
                        } else {
                            MaterialTheme.colorScheme.onSurfaceVariant
                        },
                    )
                }

                if (quest.description != null) {
                    Spacer(modifier = Modifier.height(12.dp))
                    Text(
                        text = quest.description,
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.onSurface,
                    )
                }

                if (quest.dueDate != null) {
                    Spacer(modifier = Modifier.height(12.dp))
                    Text(
                        text = "Due ${quest.dueDate.format(DateTimeFormatter.ofPattern("MMMM d, yyyy"))}",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }

                if (quest.estimatedMinutes != null) {
                    Spacer(modifier = Modifier.height(4.dp))
                    Text(
                        text = "Estimated: ${quest.estimatedMinutes} minutes",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
        }

        if (!isCompleted) {
            AltairButton(
                onClick = onComplete,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(
                    text = if (isInProgress) "Complete Quest" else "Start Quest",
                )
            }
        }

        Spacer(modifier = Modifier.height(16.dp))
    }
}
