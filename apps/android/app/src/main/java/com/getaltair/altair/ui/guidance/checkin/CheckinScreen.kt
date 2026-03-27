package com.getaltair.altair.ui.guidance.checkin

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.getaltair.altair.ui.components.AltairButton
import com.getaltair.altair.ui.components.AltairCard
import com.getaltair.altair.ui.components.AltairDetailScaffold
import com.getaltair.altair.ui.components.AltairErrorBox
import com.getaltair.altair.ui.components.AltairLoadingBox
import com.getaltair.altair.ui.components.AltairTextField
import org.koin.androidx.compose.koinViewModel

@Composable
fun CheckinScreen(
    onBack: () -> Unit,
    viewModel: CheckinViewModel = koinViewModel(),
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    AltairDetailScaffold(title = "Daily Check-in", onBack = onBack) { innerPadding ->
        when (val state = uiState) {
            is CheckinUiState.Loading -> {
                AltairLoadingBox(modifier = Modifier.padding(innerPadding))
            }

            is CheckinUiState.Error -> {
                AltairErrorBox(
                    message = state.message,
                    modifier = Modifier.padding(innerPadding),
                )
            }

            is CheckinUiState.Ready -> {
                CheckinContent(
                    state = state,
                    onEnergySelected = viewModel::setEnergyLevel,
                    onMoodChanged = viewModel::setMood,
                    onNotesChanged = viewModel::setNotes,
                    onSave = viewModel::save,
                    modifier = Modifier.padding(innerPadding),
                )
            }
        }
    }
}

@Composable
private fun CheckinContent(
    state: CheckinUiState.Ready,
    onEnergySelected: (Int) -> Unit,
    onMoodChanged: (String) -> Unit,
    onNotesChanged: (String) -> Unit,
    onSave: () -> Unit,
    modifier: Modifier = Modifier,
) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = 16.dp),
        verticalArrangement = Arrangement.spacedBy(24.dp),
    ) {
        AltairCard(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Energy Level",
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.onSurface,
                )
                Spacer(modifier = Modifier.height(12.dp))
                EnergySelector(
                    selected = state.energyLevel,
                    onSelect = onEnergySelected,
                    enabled = !state.isSaved,
                )
            }
        }

        AltairCard(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Mood",
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.onSurface,
                )
                Spacer(modifier = Modifier.height(8.dp))
                AltairTextField(
                    value = state.mood,
                    onValueChange = onMoodChanged,
                    placeholder = { Text("How are you feeling?") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )
            }
        }

        AltairCard(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Notes",
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.onSurface,
                )
                Spacer(modifier = Modifier.height(8.dp))
                AltairTextField(
                    value = state.notes,
                    onValueChange = onNotesChanged,
                    placeholder = { Text("Anything on your mind?") },
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(120.dp),
                )
            }
        }

        if (state.isSaved) {
            Text(
                text = "Check-in saved",
                style = MaterialTheme.typography.bodyLarge,
                color = MaterialTheme.colorScheme.tertiary,
                modifier = Modifier.align(Alignment.CenterHorizontally),
            )
        } else {
            AltairButton(
                onClick = onSave,
                enabled = !state.isSaving,
                modifier = Modifier.fillMaxWidth(),
            ) {
                if (state.isSaving) {
                    CircularProgressIndicator(
                        modifier = Modifier.size(20.dp),
                        color = MaterialTheme.colorScheme.onPrimary,
                        strokeWidth = 2.dp,
                    )
                } else {
                    Text("Save Check-in")
                }
            }
        }

        Spacer(modifier = Modifier.height(16.dp))
    }
}

@Composable
private fun EnergySelector(
    selected: Int?,
    onSelect: (Int) -> Unit,
    enabled: Boolean,
) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceEvenly,
    ) {
        for (level in 1..5) {
            val isSelected = selected == level
            Box(
                modifier = Modifier
                    .size(48.dp)
                    .clip(CircleShape)
                    .background(
                        if (isSelected) {
                            MaterialTheme.colorScheme.primary
                        } else {
                            MaterialTheme.colorScheme.surfaceContainerLow
                        },
                    )
                    .then(
                        if (enabled) {
                            Modifier.clickable { onSelect(level) }
                        } else {
                            Modifier
                        },
                    ),
                contentAlignment = Alignment.Center,
            ) {
                Text(
                    text = level.toString(),
                    style = MaterialTheme.typography.titleMedium,
                    color = if (isSelected) {
                        MaterialTheme.colorScheme.onPrimary
                    } else {
                        MaterialTheme.colorScheme.onSurfaceVariant
                    },
                )
            }
        }
    }
}
