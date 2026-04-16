package com.getaltair.altair.ui.today

import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.Orientation
import androidx.compose.foundation.gestures.draggable
import androidx.compose.foundation.gestures.rememberDraggableState
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.CloudOff
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.ElevatedCard
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExtendedFloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SmallFloatingActionButton
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableFloatStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.navigation.NavController
import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.data.local.entity.RoutineEntity
import com.getaltair.altair.ui.theme.WeatheredSlate
import org.koin.androidx.compose.koinViewModel
import java.time.LocalDate
import kotlin.math.roundToInt

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TodayScreen(
    navController: NavController,
    viewModel: TodayViewModel = koinViewModel(),
    modifier: Modifier = Modifier,
) {
    val currentUser by viewModel.currentUser.collectAsStateWithLifecycle()
    val todayQuests by viewModel.todayQuests.collectAsStateWithLifecycle()
    val dueRoutines by viewModel.dueRoutines.collectAsStateWithLifecycle()
    val isTodayCheckinDone by viewModel.isTodayCheckinDone.collectAsStateWithLifecycle()
    val checkinEnergy by viewModel.checkinEnergy.collectAsStateWithLifecycle()
    val checkinMood by viewModel.checkinMood.collectAsStateWithLifecycle()
    val syncStatus by viewModel.syncStatus.collectAsStateWithLifecycle()

    val displayName = currentUser?.displayName ?: "there"
    val greeting = greetingForHour()

    var fabExpanded by remember { mutableStateOf(false) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = {},
                actions = {
                    if (!syncStatus.connected || syncStatus.uploading) {
                        Icon(
                            imageVector = Icons.Filled.CloudOff,
                            contentDescription = "Offline",
                            tint = WeatheredSlate,
                        )
                    }
                },
            )
        },
        floatingActionButton = {
            Column(horizontalAlignment = Alignment.End) {
                if (fabExpanded) {
                    SmallFloatingActionButton(
                        onClick = {
                            fabExpanded = false
                            navController.navigate("new_note")
                        },
                        containerColor = MaterialTheme.colorScheme.secondaryContainer,
                        shape = RoundedCornerShape(50),
                    ) {
                        Text("Note", style = MaterialTheme.typography.labelMedium, modifier = Modifier.padding(horizontal = 8.dp))
                    }
                    Spacer(modifier = Modifier.height(8.dp))
                    SmallFloatingActionButton(
                        onClick = {
                            fabExpanded = false
                            navController.navigate("new_quest")
                        },
                        containerColor = MaterialTheme.colorScheme.secondaryContainer,
                        shape = RoundedCornerShape(50),
                    ) {
                        Text("Quest", style = MaterialTheme.typography.labelMedium, modifier = Modifier.padding(horizontal = 8.dp))
                    }
                    Spacer(modifier = Modifier.height(8.dp))
                }
                ExtendedFloatingActionButton(
                    onClick = { fabExpanded = !fabExpanded },
                    icon = { Icon(Icons.Filled.Add, contentDescription = "Add") },
                    text = { Text("New") },
                    shape = RoundedCornerShape(50),
                )
            }
        },
        modifier = modifier,
    ) { innerPadding ->
        LazyColumn(
            modifier =
                Modifier
                    .fillMaxSize()
                    .padding(innerPadding)
                    .padding(horizontal = 16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            item {
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "$greeting, $displayName",
                    style = MaterialTheme.typography.displayLarge,
                    color = MaterialTheme.colorScheme.onSurface,
                )
                Text(
                    text = LocalDate.now().toString(),
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }

            // Daily check-in card
            if (!isTodayCheckinDone) {
                item {
                    DailyCheckinCard(
                        energy = checkinEnergy,
                        mood = checkinMood,
                        onEnergyChange = { viewModel.setCheckinEnergy(it) },
                        onMoodChange = { viewModel.setCheckinMood(it) },
                        onSubmit = { viewModel.submitCheckin(checkinEnergy, checkinMood) },
                    )
                }
            }

            // All Guidance chip
            item {
                Button(
                    onClick = { navController.navigate("today_graph/guidance") },
                    shape = RoundedCornerShape(50),
                    colors =
                        ButtonDefaults.buttonColors(
                            containerColor = MaterialTheme.colorScheme.secondaryContainer,
                            contentColor = MaterialTheme.colorScheme.onSecondaryContainer,
                        ),
                ) {
                    Text("All Guidance")
                }
            }

            // Due routines
            if (dueRoutines.isNotEmpty()) {
                item {
                    Text(
                        "Routines",
                        style = MaterialTheme.typography.titleMedium,
                        color = MaterialTheme.colorScheme.onSurface,
                    )
                }
                items(dueRoutines) { routine ->
                    RoutineCard(routine = routine)
                }
            }

            // Today's quests
            item {
                Text(
                    "Today's Quests",
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.onSurface,
                )
            }

            if (todayQuests.isEmpty()) {
                item {
                    ElevatedCard(
                        modifier = Modifier.fillMaxWidth(),
                        shape = RoundedCornerShape(16.dp),
                    ) {
                        Column(
                            modifier = Modifier.padding(24.dp),
                            horizontalAlignment = Alignment.CenterHorizontally,
                        ) {
                            Text(
                                "Nothing on the horizon",
                                style = MaterialTheme.typography.bodyLarge,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                            )
                            Spacer(Modifier.height(8.dp))
                            Button(
                                onClick = { navController.navigate("new_quest") },
                                shape = RoundedCornerShape(50),
                            ) {
                                Text("Create Quest")
                            }
                        }
                    }
                }
            } else {
                items(todayQuests, key = { it.id }) { quest ->
                    SwipeableQuestCard(
                        quest = quest,
                        onComplete = { viewModel.completeQuest(quest.id) },
                        onStart = { viewModel.startQuest(quest.id) },
                        onCardClick = { navController.navigate("quest/${quest.id}") },
                    )
                }
            }

            item { Spacer(modifier = Modifier.height(80.dp)) }
        }
    }
}

@Composable
private fun DailyCheckinCard(
    energy: Int,
    mood: Int,
    onEnergyChange: (Int) -> Unit,
    onMoodChange: (Int) -> Unit,
    onSubmit: () -> Unit,
    modifier: Modifier = Modifier,
) {
    ElevatedCard(
        modifier = modifier.fillMaxWidth(),
        shape = RoundedCornerShape(16.dp),
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Text("Daily Check-in", style = MaterialTheme.typography.titleMedium)
            Spacer(Modifier.height(12.dp))
            Text("Energy (1–5)", style = MaterialTheme.typography.bodyMedium)
            Spacer(Modifier.height(4.dp))
            RatingRow(value = energy, onValueChange = onEnergyChange, max = 5)
            Spacer(Modifier.height(12.dp))
            Text("Mood (1–5)", style = MaterialTheme.typography.bodyMedium)
            Spacer(Modifier.height(4.dp))
            RatingRow(value = mood, onValueChange = onMoodChange, max = 5)
            Spacer(Modifier.height(16.dp))
            Button(
                onClick = onSubmit,
                shape = RoundedCornerShape(50),
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text("Submit")
            }
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun RatingRow(
    value: Int,
    onValueChange: (Int) -> Unit,
    max: Int,
    modifier: Modifier = Modifier,
) {
    FlowRow(modifier = modifier, horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        for (i in 1..max) {
            val selected = i == value
            Surface(
                onClick = { onValueChange(i) },
                shape = CircleShape,
                color = if (selected) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.surfaceContainerLow,
                contentColor = if (selected) MaterialTheme.colorScheme.onPrimary else MaterialTheme.colorScheme.onSurface,
                modifier = Modifier.size(40.dp),
            ) {
                Box(contentAlignment = Alignment.Center) {
                    Text("$i", style = MaterialTheme.typography.labelLarge)
                }
            }
        }
    }
}

@Composable
private fun RoutineCard(
    routine: RoutineEntity,
    modifier: Modifier = Modifier,
) {
    ElevatedCard(
        modifier = modifier.fillMaxWidth(),
        shape = RoundedCornerShape(16.dp),
    ) {
        Row(
            modifier = Modifier.padding(16.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(routine.title, style = MaterialTheme.typography.bodyLarge)
                Text(routine.frequencyType, style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
            }
        }
    }
}

@Composable
private fun SwipeableQuestCard(
    quest: QuestEntity,
    onComplete: () -> Unit,
    onStart: () -> Unit,
    onCardClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    var offsetX by remember { mutableFloatStateOf(0f) }
    val animatedOffset by animateFloatAsState(
        targetValue = offsetX,
        animationSpec = tween(durationMillis = 300),
        label = "swipe",
    )

    val draggableState =
        rememberDraggableState { delta ->
            if (quest.status == "in_progress" && delta > 0) {
                offsetX = (offsetX + delta).coerceIn(0f, 300f)
            }
        }

    Box(modifier = modifier.fillMaxWidth()) {
        // Complete hint behind card
        if (quest.status == "in_progress") {
            Box(
                modifier =
                    Modifier
                        .matchParentSize()
                        .background(MaterialTheme.colorScheme.primaryContainer, RoundedCornerShape(16.dp)),
                contentAlignment = Alignment.CenterStart,
            ) {
                Icon(
                    Icons.Filled.Check,
                    contentDescription = "Complete",
                    tint = MaterialTheme.colorScheme.onPrimaryContainer,
                    modifier = Modifier.padding(start = 16.dp),
                )
            }
        }

        ElevatedCard(
            onClick = onCardClick,
            modifier =
                Modifier
                    .fillMaxWidth()
                    .offset { IntOffset(animatedOffset.roundToInt(), 0) }
                    .draggable(
                        state = draggableState,
                        orientation = Orientation.Horizontal,
                        onDragStopped = {
                            if (offsetX > 150f && quest.status == "in_progress") {
                                onComplete()
                            }
                            offsetX = 0f
                        },
                    ),
            shape = RoundedCornerShape(16.dp),
        ) {
            Row(
                modifier = Modifier.padding(16.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Column(modifier = Modifier.weight(1f)) {
                    Text(quest.title, style = MaterialTheme.typography.bodyLarge)
                    if (!quest.description.isNullOrBlank()) {
                        Text(
                            quest.description,
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                            maxLines = 2,
                        )
                    }
                }
                Spacer(Modifier.width(8.dp))
                Column(horizontalAlignment = Alignment.End) {
                    StatusBadge(status = quest.status)
                    if (quest.priority.isNotBlank()) {
                        Spacer(Modifier.height(4.dp))
                        PriorityIndicator(priority = quest.priority)
                    }
                }
            }
            if (quest.status == "not_started") {
                Row(
                    modifier =
                        Modifier
                            .fillMaxWidth()
                            .padding(start = 16.dp, end = 16.dp, bottom = 12.dp),
                    horizontalArrangement = Arrangement.End,
                ) {
                    OutlinedButton(
                        onClick = onStart,
                        shape = RoundedCornerShape(50),
                    ) {
                        Text("Start")
                    }
                }
            }
        }
    }
}

@Composable
private fun StatusBadge(
    status: String,
    modifier: Modifier = Modifier,
) {
    val (label, color) =
        when (status) {
            "not_started" -> "Not Started" to MaterialTheme.colorScheme.surfaceVariant
            "in_progress" -> "In Progress" to MaterialTheme.colorScheme.primaryContainer
            "deferred" -> "Deferred" to MaterialTheme.colorScheme.surfaceVariant
            "completed" -> "Completed" to MaterialTheme.colorScheme.tertiaryContainer
            "cancelled" -> "Cancelled" to MaterialTheme.colorScheme.errorContainer
            else -> status to MaterialTheme.colorScheme.surfaceVariant
        }
    Surface(
        color = color,
        shape = RoundedCornerShape(50),
        modifier = modifier,
    ) {
        Text(
            label,
            style = MaterialTheme.typography.labelSmall,
            modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp),
        )
    }
}

@Composable
private fun PriorityIndicator(
    priority: String,
    modifier: Modifier = Modifier,
) {
    val color =
        when (priority.lowercase()) {
            "high" -> MaterialTheme.colorScheme.error
            "medium" -> MaterialTheme.colorScheme.tertiary
            else -> MaterialTheme.colorScheme.outlineVariant
        }
    Surface(
        color = Color.Transparent,
        modifier = modifier,
    ) {
        Text(
            priority.replaceFirstChar { it.uppercaseChar() },
            style = MaterialTheme.typography.labelSmall,
            color = color,
        )
    }
}

private fun greetingForHour(): String {
    val hour =
        java.time.LocalTime
            .now()
            .hour
    return when {
        hour < 12 -> "Good morning"
        hour < 17 -> "Good afternoon"
        else -> "Good evening"
    }
}
