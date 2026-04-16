package com.getaltair.altair.ui.guidance

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.Button
import androidx.compose.material3.ElevatedCard
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.navigation.NavController
import org.koin.androidx.compose.koinViewModel

@OptIn(ExperimentalMaterial3Api::class, ExperimentalLayoutApi::class)
@Composable
fun QuestDetailScreen(
    questId: String,
    navController: NavController,
    viewModel: QuestDetailViewModel = koinViewModel(),
    modifier: Modifier = Modifier,
) {
    LaunchedEffect(questId) { viewModel.init(questId) }

    val quest by viewModel.quest.collectAsStateWithLifecycle()

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(quest?.title ?: "Quest") },
                navigationIcon = {
                    IconButton(onClick = { navController.popBackStack() }) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                    }
                },
            )
        },
        modifier = modifier,
    ) { innerPadding ->
        val q = quest
        if (q == null) {
            Column(modifier = Modifier.padding(innerPadding).padding(16.dp)) {
                Text("Loading…", style = MaterialTheme.typography.bodyLarge)
            }
            return@Scaffold
        }

        Column(
            modifier =
                Modifier
                    .fillMaxSize()
                    .padding(innerPadding)
                    .verticalScroll(rememberScrollState())
                    .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            // Title + meta
            Text(q.title, style = MaterialTheme.typography.headlineLarge)

            if (!q.description.isNullOrBlank()) {
                Text(
                    q.description,
                    style = MaterialTheme.typography.bodyLarge,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }

            ElevatedCard(
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(16.dp),
            ) {
                Column(modifier = Modifier.padding(16.dp)) {
                    LabelValue("Status", q.status.replace('_', ' ').replaceFirstChar { it.uppercaseChar() })
                    Spacer(Modifier.height(8.dp))
                    LabelValue("Priority", q.priority.replaceFirstChar { it.uppercaseChar() })
                    if (q.dueDate != null) {
                        Spacer(Modifier.height(8.dp))
                        LabelValue("Due", q.dueDate)
                    }
                }
            }

            // Valid transition buttons
            val transitions = viewModel.validTransitions(q.status)
            if (transitions.isNotEmpty()) {
                Text("Transition", style = MaterialTheme.typography.titleSmall)
                FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    transitions.forEach { target ->
                        if (target == "completed") {
                            Button(
                                onClick = { viewModel.transitionStatus(target) },
                                shape = RoundedCornerShape(50),
                            ) {
                                Text(target.replace('_', ' ').replaceFirstChar { it.uppercaseChar() })
                            }
                        } else {
                            OutlinedButton(
                                onClick = { viewModel.transitionStatus(target) },
                                shape = RoundedCornerShape(50),
                            ) {
                                Text(target.replace('_', ' ').replaceFirstChar { it.uppercaseChar() })
                            }
                        }
                    }
                }
            }

            // Focus session button
            if (q.status == "in_progress") {
                Button(
                    onClick = {
                        navController.navigate("today_graph/guidance/quests/${q.id}/focus")
                    },
                    shape = RoundedCornerShape(50),
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text("Start Focus Session")
                }
            }
        }
    }
}

@Composable
private fun LabelValue(
    label: String,
    value: String,
) {
    Column {
        Text(label, style = MaterialTheme.typography.labelMedium, color = MaterialTheme.colorScheme.onSurfaceVariant)
        Text(value, style = MaterialTheme.typography.bodyLarge)
    }
}
