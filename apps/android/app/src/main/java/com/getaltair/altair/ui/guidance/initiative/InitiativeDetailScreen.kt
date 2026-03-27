package com.getaltair.altair.ui.guidance.initiative

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.getaltair.altair.domain.entity.Epic
import com.getaltair.altair.domain.entity.Initiative
import com.getaltair.altair.ui.common.UiState
import com.getaltair.altair.ui.components.AltairCard
import com.getaltair.altair.ui.components.AltairDetailScaffold
import com.getaltair.altair.ui.components.AltairErrorBox
import com.getaltair.altair.ui.components.AltairLoadingBox
import com.getaltair.altair.util.capitalizeFirst
import org.koin.androidx.compose.koinViewModel

@Composable
fun InitiativeDetailScreen(
    onBack: () -> Unit,
    viewModel: InitiativeDetailViewModel = koinViewModel(),
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    val title = when (val state = uiState) {
        is UiState.Success -> state.data.initiative.name
        else -> "Initiative"
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
                InitiativeDetailContent(
                    initiative = state.data.initiative,
                    epics = state.data.epics,
                    modifier = Modifier.padding(innerPadding),
                )
            }
        }
    }
}

@Composable
private fun InitiativeDetailContent(
    initiative: Initiative,
    epics: List<Epic>,
    modifier: Modifier = Modifier,
) {
    LazyColumn(
        modifier = modifier
            .fillMaxSize()
            .padding(horizontal = 16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        item {
            Column {
                Text(
                    text = initiative.status.value.capitalizeFirst(),
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.primary,
                )
                if (initiative.description != null) {
                    Spacer(modifier = Modifier.height(8.dp))
                    Text(
                        text = initiative.description,
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.onSurface,
                    )
                }
            }
        }

        if (epics.isNotEmpty()) {
            item {
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "EPICS",
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            items(epics, key = { it.id }) { epic ->
                EpicCard(epic = epic)
            }
        }

        item { Spacer(modifier = Modifier.height(16.dp)) }
    }
}

@Composable
private fun EpicCard(epic: Epic) {
    AltairCard(modifier = Modifier.fillMaxWidth()) {
        Column(modifier = Modifier.padding(16.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = epic.name,
                    style = MaterialTheme.typography.titleSmall,
                    color = MaterialTheme.colorScheme.onSurface,
                    modifier = Modifier.weight(1f),
                )
                Text(
                    text = epic.status.value.capitalizeFirst(),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.primary,
                )
            }
            if (epic.description != null) {
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = epic.description,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    maxLines = 2,
                )
            }
        }
    }
}
