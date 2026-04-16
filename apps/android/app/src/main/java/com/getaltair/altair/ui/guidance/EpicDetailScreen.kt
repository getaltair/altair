package com.getaltair.altair.ui.guidance

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.ElevatedCard
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SuggestionChip
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.navigation.NavBackStackEntry
import androidx.navigation.NavController
import com.getaltair.altair.navigation.Screen
import org.koin.androidx.compose.koinViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun EpicDetailScreen(
    navController: NavController,
    backStackEntry: NavBackStackEntry,
    modifier: Modifier = Modifier,
) {
    val vm: EpicDetailViewModel = koinViewModel()
    val epic by vm.epic.collectAsStateWithLifecycle()
    val quests by vm.quests.collectAsStateWithLifecycle()

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(epic?.title ?: "Epic") },
                navigationIcon = {
                    IconButton(onClick = { navController.popBackStack() }) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                    }
                },
            )
        },
        modifier = modifier,
    ) { innerPadding ->
        val e = epic
        if (e == null) {
            Column(modifier = Modifier.padding(innerPadding).padding(16.dp)) {
                Text("Loading…", style = MaterialTheme.typography.bodyLarge)
            }
            return@Scaffold
        }

        LazyColumn(
            modifier =
                Modifier
                    .fillMaxSize()
                    .padding(innerPadding)
                    .padding(horizontal = 16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            item {
                Column(
                    modifier = Modifier.padding(top = 16.dp),
                    verticalArrangement = Arrangement.spacedBy(12.dp),
                ) {
                    Text(e.title, style = MaterialTheme.typography.headlineLarge)

                    if (!e.description.isNullOrBlank()) {
                        Text(
                            e.description,
                            style = MaterialTheme.typography.bodyLarge,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }

                    SuggestionChip(
                        onClick = {},
                        label = {
                            Text(
                                e.status.replace('_', ' ').replaceFirstChar { it.uppercaseChar() },
                                style = MaterialTheme.typography.labelMedium,
                            )
                        },
                    )

                    if (quests.isNotEmpty()) {
                        Text("Quests", style = MaterialTheme.typography.titleMedium)
                    }
                }
            }

            items(quests, key = { it.id }) { quest ->
                ElevatedCard(
                    onClick = {
                        navController.navigate(Screen.QuestDetail.route(quest.id))
                    },
                    modifier = Modifier.fillMaxWidth(),
                    shape = RoundedCornerShape(16.dp),
                ) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(quest.title, style = MaterialTheme.typography.bodyLarge)
                        SuggestionChip(
                            onClick = {},
                            label = {
                                Text(
                                    quest.status.replace('_', ' ').replaceFirstChar { it.uppercaseChar() },
                                    style = MaterialTheme.typography.labelSmall,
                                )
                            },
                        )
                    }
                }
            }

            item {
                // bottom spacing
                Column(modifier = Modifier.padding(bottom = 16.dp)) {}
            }
        }
    }
}
