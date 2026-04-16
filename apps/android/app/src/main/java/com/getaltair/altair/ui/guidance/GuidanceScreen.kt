package com.getaltair.altair.ui.guidance

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
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.ElevatedCard
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Tab
import androidx.compose.material3.TabRow
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.navigation.NavController
import org.koin.androidx.compose.koinViewModel

@Composable
fun GuidanceScreen(
    navController: NavController,
    viewModel: GuidanceViewModel = koinViewModel(),
    modifier: Modifier = Modifier,
) {
    val initiatives by viewModel.initiatives.collectAsStateWithLifecycle()
    val quests by viewModel.quests.collectAsStateWithLifecycle()
    val routines by viewModel.routines.collectAsStateWithLifecycle()

    var selectedTab by remember { mutableIntStateOf(0) }
    val tabs = listOf("Quests", "Initiatives", "Routines")

    Column(modifier = modifier.fillMaxSize()) {
        Text(
            "Guidance",
            style = MaterialTheme.typography.headlineLarge,
            modifier = Modifier.padding(horizontal = 16.dp, vertical = 12.dp),
        )

        TabRow(selectedTabIndex = selectedTab) {
            tabs.forEachIndexed { index, title ->
                Tab(
                    selected = selectedTab == index,
                    onClick = { selectedTab = index },
                    text = { Text(title) },
                )
            }
        }

        when (selectedTab) {
            0 -> {
                QuestListTab(
                    quests = quests,
                    onQuestClick = { id -> navController.navigate("quest/$id") },
                )
            }

            1 -> {
                InitiativeListTab(
                    initiatives = initiatives,
                    onInitiativeClick = { id -> navController.navigate("today_graph/guidance/initiatives/$id") },
                )
            }

            2 -> {
                RoutineListTab(
                    routines = routines,
                    onMarkDone = { id -> viewModel.markRoutineDone(id) },
                )
            }
        }
    }
}

@Composable
private fun QuestListTab(
    quests: List<com.getaltair.altair.data.local.entity.QuestEntity>,
    onQuestClick: (String) -> Unit,
    modifier: Modifier = Modifier,
) {
    if (quests.isEmpty()) {
        Column(
            modifier = modifier.fillMaxSize().padding(32.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
        ) {
            Text("No quests yet", style = MaterialTheme.typography.bodyLarge)
        }
        return
    }
    LazyColumn(
        modifier = modifier.fillMaxSize().padding(horizontal = 16.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        item { Spacer(Modifier.height(8.dp)) }
        items(quests, key = { it.id }) { quest ->
            ElevatedCard(
                onClick = { onQuestClick(quest.id) },
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(16.dp),
            ) {
                Row(
                    modifier = Modifier.padding(16.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Column(modifier = Modifier.weight(1f)) {
                        Text(quest.title, style = MaterialTheme.typography.bodyLarge)
                        Text(
                            quest.status.replace('_', ' ').replaceFirstChar { it.uppercaseChar() },
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            }
        }
        item { Spacer(Modifier.height(16.dp)) }
    }
}

@Composable
private fun InitiativeListTab(
    initiatives: List<com.getaltair.altair.data.local.entity.InitiativeEntity>,
    onInitiativeClick: (String) -> Unit,
    modifier: Modifier = Modifier,
) {
    LazyColumn(
        modifier = modifier.fillMaxSize().padding(horizontal = 16.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        item { Spacer(Modifier.height(8.dp)) }
        if (initiatives.isEmpty()) {
            item {
                Text(
                    "No initiatives yet",
                    style = MaterialTheme.typography.bodyLarge,
                    modifier = Modifier.padding(32.dp),
                )
            }
        }
        items(initiatives, key = { it.id }) { initiative ->
            ElevatedCard(
                onClick = { onInitiativeClick(initiative.id) },
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(16.dp),
            ) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text(initiative.title, style = MaterialTheme.typography.bodyLarge)
                    Text(
                        initiative.status.replace('_', ' ').replaceFirstChar { it.uppercaseChar() },
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
        }
        item { Spacer(Modifier.height(16.dp)) }
    }
}

@Composable
private fun RoutineListTab(
    routines: List<com.getaltair.altair.data.local.entity.RoutineEntity>,
    onMarkDone: (String) -> Unit,
    modifier: Modifier = Modifier,
) {
    LazyColumn(
        modifier = modifier.fillMaxSize().padding(horizontal = 16.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        item { Spacer(Modifier.height(8.dp)) }
        if (routines.isEmpty()) {
            item {
                Text(
                    "No routines yet",
                    style = MaterialTheme.typography.bodyLarge,
                    modifier = Modifier.padding(32.dp),
                )
            }
        }
        items(routines, key = { it.id }) { routine ->
            ElevatedCard(
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(16.dp),
            ) {
                Row(
                    modifier = Modifier.padding(16.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Column(modifier = Modifier.weight(1f)) {
                        Text(routine.title, style = MaterialTheme.typography.bodyLarge)
                        Text(
                            routine.frequencyType,
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                    TextButton(onClick = { onMarkDone(routine.id) }) {
                        Text("Done")
                    }
                }
            }
        }
        item { Spacer(Modifier.height(16.dp)) }
    }
}
