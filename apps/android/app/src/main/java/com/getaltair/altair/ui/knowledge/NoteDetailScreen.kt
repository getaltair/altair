package com.getaltair.altair.ui.knowledge

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Save
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ElevatedCard
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.navigation.NavController
import org.koin.androidx.compose.koinViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun NoteDetailScreen(
    noteId: String,
    navController: NavController,
    viewModel: NoteDetailViewModel = koinViewModel(),
    modifier: Modifier = Modifier,
) {
    val note by viewModel.note.collectAsStateWithLifecycle()
    val backlinks by viewModel.backlinks.collectAsStateWithLifecycle()
    val snapshots by viewModel.snapshots.collectAsStateWithLifecycle()

    var title by remember { mutableStateOf("") }
    var content by remember { mutableStateOf("") }
    var showLinkMenu by remember { mutableStateOf(false) }
    var linkSearchQuery by remember { mutableStateOf("") }

    // Populate fields when note loads
    LaunchedEffect(note) {
        note?.let {
            if (title.isEmpty() && content.isEmpty()) {
                title = it.title
                content = it.content ?: ""
            }
        }
    }

    BackHandler {
        viewModel.saveNote(title, content)
        navController.popBackStack()
    }

    // Detect [[ typed to trigger link menu
    val triggerIndex = content.lastIndexOf("[[")
    val afterTrigger = if (triggerIndex >= 0) content.substring(triggerIndex + 2) else ""
    val showingLinkSuggestions = triggerIndex >= 0 && !afterTrigger.contains("]]")

    // Build a simple notes list for link search — reuse KnowledgeViewModel
    val listViewModel: KnowledgeViewModel = koinViewModel()
    LaunchedEffect(afterTrigger, showingLinkSuggestions) {
        if (showingLinkSuggestions) {
            listViewModel.searchNotes(afterTrigger)
        }
    }
    val suggestedNotes by listViewModel.notes.collectAsStateWithLifecycle()

    Scaffold(
        modifier = modifier,
        topBar = {
            TopAppBar(
                title = { Text(title.ifBlank { "Note" }) },
                navigationIcon = {
                    IconButton(onClick = {
                        viewModel.saveNote(title, content)
                        navController.popBackStack()
                    }) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                    }
                },
            )
        },
        floatingActionButton = {
            FloatingActionButton(
                onClick = { viewModel.saveNote(title, content) },
                shape = RoundedCornerShape(50),
            ) {
                Icon(Icons.Default.Save, contentDescription = "Save")
            }
        },
    ) { innerPadding ->
        LazyColumn(
            modifier =
                Modifier
                    .fillMaxSize()
                    .padding(innerPadding),
            contentPadding = PaddingValues(horizontal = 16.dp, vertical = 8.dp),
        ) {
            item {
                OutlinedTextField(
                    value = title,
                    onValueChange = { title = it },
                    label = { Text("Title") },
                    singleLine = true,
                    modifier =
                        Modifier
                            .fillMaxWidth()
                            .padding(bottom = 8.dp),
                )
            }

            item {
                OutlinedTextField(
                    value = content,
                    onValueChange = { newContent ->
                        content = newContent
                    },
                    label = { Text("Content") },
                    modifier =
                        Modifier
                            .fillMaxWidth()
                            .padding(bottom = 16.dp),
                    minLines = 10,
                )

                if (showingLinkSuggestions && suggestedNotes.isNotEmpty()) {
                    DropdownMenu(
                        expanded = true,
                        onDismissRequest = {
                            // Remove the [[ trigger to dismiss
                            if (triggerIndex >= 0) {
                                content = content.substring(0, triggerIndex)
                            }
                        },
                    ) {
                        suggestedNotes.take(8).forEach { targetNote ->
                            DropdownMenuItem(
                                text = { Text(targetNote.title) },
                                onClick = {
                                    viewModel.linkNote(noteId, targetNote.id)
                                    // Replace [[query with [[title]]
                                    content = content.substring(0, triggerIndex) +
                                        "[[${targetNote.title}]]"
                                    listViewModel.searchNotes("")
                                },
                            )
                        }
                    }
                }
            }

            if (backlinks.isNotEmpty()) {
                item {
                    Text(
                        text = "Linked from",
                        style = MaterialTheme.typography.titleSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.padding(bottom = 4.dp),
                    )
                }
                items(backlinks, key = { it.id }) { relation ->
                    ElevatedCard(
                        modifier =
                            Modifier
                                .fillMaxWidth()
                                .padding(bottom = 8.dp),
                        shape = RoundedCornerShape(12.dp),
                    ) {
                        Column(modifier = Modifier.padding(12.dp)) {
                            Text(
                                text = relation.fromEntityId,
                                style = MaterialTheme.typography.bodySmall,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                            )
                        }
                    }
                }
            }

            if (snapshots.isNotEmpty()) {
                item {
                    Text(
                        text = "Snapshots",
                        style = MaterialTheme.typography.titleSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.padding(top = 8.dp, bottom = 4.dp),
                    )
                }
                items(snapshots, key = { it.id }) { snapshot ->
                    ElevatedCard(
                        modifier =
                            Modifier
                                .fillMaxWidth()
                                .padding(bottom = 8.dp),
                        shape = RoundedCornerShape(12.dp),
                    ) {
                        Column(modifier = Modifier.padding(12.dp)) {
                            if (!snapshot.capturedAt.isNullOrBlank()) {
                                Text(
                                    text = snapshot.capturedAt,
                                    style = MaterialTheme.typography.labelSmall,
                                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                                )
                            }
                            if (!snapshot.content.isNullOrBlank()) {
                                Text(
                                    text = snapshot.content.take(200),
                                    style = MaterialTheme.typography.bodySmall,
                                    modifier = Modifier.padding(top = 4.dp),
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
