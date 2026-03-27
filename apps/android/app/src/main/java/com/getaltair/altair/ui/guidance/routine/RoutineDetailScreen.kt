package com.getaltair.altair.ui.guidance.routine

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
import com.getaltair.altair.domain.entity.Routine
import com.getaltair.altair.ui.common.UiState
import com.getaltair.altair.ui.components.AltairCard
import com.getaltair.altair.ui.components.AltairDetailScaffold
import com.getaltair.altair.ui.components.AltairErrorBox
import com.getaltair.altair.ui.components.AltairLoadingBox
import com.getaltair.altair.util.capitalizeFirst
import org.koin.androidx.compose.koinViewModel

@Composable
fun RoutineDetailScreen(
    onBack: () -> Unit,
    viewModel: RoutineDetailViewModel = koinViewModel(),
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    val title = when (val state = uiState) {
        is UiState.Success -> state.data.name
        else -> "Routine"
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
                RoutineDetailContent(
                    routine = state.data,
                    modifier = Modifier.padding(innerPadding),
                )
            }
        }
    }
}

@Composable
private fun RoutineDetailContent(
    routine: Routine,
    modifier: Modifier = Modifier,
) {
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
                        text = routine.status.value.capitalizeFirst(),
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.primary,
                    )
                    Text(
                        text = routine.frequency.value.capitalizeFirst(),
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }

                if (routine.description != null) {
                    Spacer(modifier = Modifier.height(12.dp))
                    Text(
                        text = routine.description,
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.onSurface,
                    )
                }
            }
        }

        Spacer(modifier = Modifier.height(16.dp))
    }
}
