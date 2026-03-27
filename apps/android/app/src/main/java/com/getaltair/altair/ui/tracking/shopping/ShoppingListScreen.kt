package com.getaltair.altair.ui.tracking.shopping

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Check
import androidx.compose.material3.Checkbox
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextDecoration
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.getaltair.altair.domain.entity.TrackingShoppingListItem
import com.getaltair.altair.ui.components.AltairCard
import com.getaltair.altair.ui.components.AltairDetailScaffold
import com.getaltair.altair.ui.components.AltairEmptyState
import com.getaltair.altair.ui.components.AltairLoadingBox
import org.koin.androidx.compose.koinViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ShoppingListScreen(
    onNavigateUp: () -> Unit,
    viewModel: ShoppingListViewModel = koinViewModel(),
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    val title = uiState.list?.name ?: "Shopping List"

    AltairDetailScaffold(title = title, onBack = onNavigateUp) { innerPadding ->
        if (uiState.isLoading) {
            AltairLoadingBox(modifier = Modifier.padding(innerPadding))
            return@AltairDetailScaffold
        }

        if (uiState.items.isEmpty()) {
            AltairEmptyState(
                icon = Icons.Default.Add,
                title = "No items in this list",
                modifier = Modifier.padding(innerPadding),
            )
            return@AltairDetailScaffold
        }

        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(innerPadding)
                .padding(horizontal = 16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            // Mark Complete button at top
            item {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.End,
                ) {
                    IconButton(onClick = viewModel::markComplete) {
                        Icon(
                            imageVector = Icons.Default.Check,
                            contentDescription = "Mark list complete",
                            tint = MaterialTheme.colorScheme.primary,
                        )
                    }
                }
            }

            items(uiState.items, key = { it.id }) { item ->
                ShoppingItemRow(
                    item = item,
                    onToggle = { viewModel.toggleItem(item.id) },
                )
            }

            item { Spacer(modifier = Modifier.height(16.dp)) }
        }
    }
}

@Composable
private fun ShoppingItemRow(
    item: TrackingShoppingListItem,
    onToggle: () -> Unit,
) {
    AltairCard(modifier = Modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 12.dp, vertical = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Checkbox(
                checked = item.isChecked,
                onCheckedChange = { onToggle() },
            )
            Text(
                text = item.name,
                style = MaterialTheme.typography.bodyLarge,
                color = if (item.isChecked) {
                    MaterialTheme.colorScheme.onSurfaceVariant
                } else {
                    MaterialTheme.colorScheme.onSurface
                },
                textDecoration = if (item.isChecked) TextDecoration.LineThrough else null,
                modifier = Modifier.weight(1f),
            )
            if (item.unit != null) {
                Text(
                    text = "${item.quantity} ${item.unit}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            } else if (item.quantity > 1) {
                Text(
                    text = "${item.quantity}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}
