package com.getaltair.altair.ui.guidance.today

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
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Checkbox
import androidx.compose.material3.CheckboxDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.getaltair.altair.R
import com.getaltair.altair.domain.entity.Quest
import com.getaltair.altair.domain.entity.Routine
import com.getaltair.altair.navigation.Screen
import com.getaltair.altair.ui.components.AltairButton
import com.getaltair.altair.ui.components.AltairCard
import com.getaltair.altair.ui.navigation.AltairBottomNavBar
import java.time.LocalDate
import java.time.format.DateTimeFormatter
import java.util.UUID
import org.koin.androidx.compose.koinViewModel

@Composable
fun TodayScreen(
    onNavigateToQuest: (UUID) -> Unit,
    onNavigateToCheckin: () -> Unit,
    onNavigateToTab: (String) -> Unit,
    viewModel: TodayViewModel = koinViewModel(),
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    Scaffold(
        containerColor = MaterialTheme.colorScheme.background,
        bottomBar = {
            AltairBottomNavBar(
                currentRoute = Screen.Today.route,
                onNavigate = onNavigateToTab,
            )
        },
    ) { innerPadding ->
        when (val state = uiState) {
            is TodayUiState.Loading -> {
                Box(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(innerPadding),
                    contentAlignment = Alignment.Center,
                ) {
                    CircularProgressIndicator(color = MaterialTheme.colorScheme.primary)
                }
            }

            is TodayUiState.Error -> {
                Box(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(innerPadding),
                    contentAlignment = Alignment.Center,
                ) {
                    Text(
                        text = state.message,
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.error,
                    )
                }
            }

            is TodayUiState.Success -> {
                TodayContent(
                    quests = state.quests,
                    routines = state.routines,
                    hasCheckin = state.checkin != null,
                    onQuestComplete = viewModel::completeQuest,
                    onNavigateToQuest = onNavigateToQuest,
                    onNavigateToCheckin = onNavigateToCheckin,
                    modifier = Modifier.padding(innerPadding),
                )
            }
        }
    }
}

@Composable
private fun TodayContent(
    quests: List<Quest>,
    routines: List<Routine>,
    hasCheckin: Boolean,
    onQuestComplete: (UUID) -> Unit,
    onNavigateToQuest: (UUID) -> Unit,
    onNavigateToCheckin: () -> Unit,
    modifier: Modifier = Modifier,
) {
    LazyColumn(
        modifier = modifier
            .fillMaxSize()
            .padding(horizontal = 16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        item {
            Spacer(modifier = Modifier.height(8.dp))
            GreetingHeader()
        }

        if (!hasCheckin) {
            item {
                CheckinPrompt(onNavigateToCheckin = onNavigateToCheckin)
            }
        }

        if (quests.isNotEmpty()) {
            item {
                Text(
                    text = "QUESTS",
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    letterSpacing = MaterialTheme.typography.labelSmall.letterSpacing,
                )
            }
            items(quests, key = { it.id }) { quest ->
                QuestCard(
                    quest = quest,
                    onComplete = { onQuestComplete(quest.id) },
                    onClick = { onNavigateToQuest(quest.id) },
                )
            }
        }

        if (routines.isNotEmpty()) {
            item {
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "ROUTINES",
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    letterSpacing = MaterialTheme.typography.labelSmall.letterSpacing,
                )
            }
            items(routines, key = { it.id }) { routine ->
                RoutineCard(routine = routine)
            }
        }

        item { Spacer(modifier = Modifier.height(16.dp)) }
    }
}

@Composable
private fun GreetingHeader() {
    val today = LocalDate.now()
    val formatter = DateTimeFormatter.ofPattern("EEEE, MMMM d")

    Column(modifier = Modifier.padding(vertical = 8.dp)) {
        Text(
            text = "Good day",
            style = MaterialTheme.typography.headlineLarge,
            color = MaterialTheme.colorScheme.onBackground,
        )
        Spacer(modifier = Modifier.height(4.dp))
        Text(
            text = today.format(formatter),
            style = MaterialTheme.typography.bodyLarge,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

@Composable
private fun CheckinPrompt(onNavigateToCheckin: () -> Unit) {
    AltairCard(
        modifier = Modifier.fillMaxWidth(),
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text(
                text = "How are you feeling today?",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.onSurface,
            )
            Spacer(modifier = Modifier.height(8.dp))
            Text(
                text = "Take a moment to check in with yourself.",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            Spacer(modifier = Modifier.height(12.dp))
            AltairButton(onClick = onNavigateToCheckin) {
                Text("Check in")
            }
        }
    }
}

@Composable
private fun QuestCard(
    quest: Quest,
    onComplete: () -> Unit,
    onClick: () -> Unit,
) {
    val isCompleted = quest.status == "completed"

    AltairCard(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Row(
            modifier = Modifier.padding(16.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Checkbox(
                checked = isCompleted,
                onCheckedChange = { if (!isCompleted) onComplete() },
                colors = CheckboxDefaults.colors(
                    checkedColor = MaterialTheme.colorScheme.primary,
                    uncheckedColor = MaterialTheme.colorScheme.outline,
                ),
            )
            Spacer(modifier = Modifier.width(12.dp))
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = quest.name,
                    style = MaterialTheme.typography.titleSmall,
                    color = if (isCompleted) {
                        MaterialTheme.colorScheme.onSurfaceVariant
                    } else {
                        MaterialTheme.colorScheme.onSurface
                    },
                    textDecoration = if (isCompleted) TextDecoration.LineThrough else null,
                )
                if (quest.description != null) {
                    Spacer(modifier = Modifier.height(2.dp))
                    Text(
                        text = quest.description,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        maxLines = 1,
                    )
                }
            }
            if (quest.priority == "high") {
                Spacer(modifier = Modifier.width(8.dp))
                Icon(
                    painter = painterResource(R.drawable.ic_priority_high),
                    contentDescription = "High priority",
                    modifier = Modifier.size(16.dp),
                    tint = MaterialTheme.colorScheme.error,
                )
            }
        }
    }
}

@Composable
private fun RoutineCard(routine: Routine) {
    AltairCard(
        modifier = Modifier.fillMaxWidth(),
    ) {
        Row(
            modifier = Modifier.padding(16.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = routine.name,
                    style = MaterialTheme.typography.titleSmall,
                    color = MaterialTheme.colorScheme.onSurface,
                )
                if (routine.description != null) {
                    Spacer(modifier = Modifier.height(2.dp))
                    Text(
                        text = routine.description,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        maxLines = 1,
                    )
                }
            }
            Text(
                text = routine.frequency.replaceFirstChar { it.uppercase() },
                style = MaterialTheme.typography.labelMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}
