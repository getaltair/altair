package com.getaltair.altair.ui.tracking.item

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material.icons.filled.Favorite
import androidx.compose.material.icons.filled.Remove
import androidx.compose.material.icons.filled.Warning
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilledTonalIconButton
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.getaltair.altair.domain.entity.ItemEventType
import com.getaltair.altair.domain.entity.TrackingItem
import com.getaltair.altair.domain.entity.TrackingItemEvent
import com.getaltair.altair.domain.entity.TrackingItemStatus
import com.getaltair.altair.ui.components.AltairCard
import com.getaltair.altair.ui.components.AltairDetailScaffold
import com.getaltair.altair.ui.components.AltairLoadingBox
import com.getaltair.altair.ui.components.RelationsSection
import java.time.ZoneId
import java.time.format.DateTimeFormatter
import org.koin.androidx.compose.koinViewModel

private val dateFormatter = DateTimeFormatter.ofPattern("MMM d, yyyy h:mm a")

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ItemDetailScreen(
    onNavigateUp: () -> Unit,
    viewModel: ItemDetailViewModel = koinViewModel(),
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    val title = uiState.item?.name ?: "Item"

    AltairDetailScaffold(title = title, onBack = onNavigateUp) { innerPadding ->
        if (uiState.isLoading) {
            AltairLoadingBox(modifier = Modifier.padding(innerPadding))
            return@AltairDetailScaffold
        }

        val item = uiState.item
        if (item == null) {
            AltairLoadingBox(modifier = Modifier.padding(innerPadding))
            return@AltairDetailScaffold
        }

        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(innerPadding)
                .padding(horizontal = 16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            // Quantity section
            item {
                QuantitySection(
                    quantity = item.quantity,
                    unit = item.unit,
                    onIncrement = { viewModel.adjustQuantity(1) },
                    onDecrement = { viewModel.adjustQuantity(-1) },
                )
            }

            // Metadata section
            item {
                MetadataSection(
                    item = item,
                    onDelete = viewModel::onDeleteRequest,
                )
            }

            // Event history section
            if (uiState.events.isNotEmpty()) {
                item {
                    Text(
                        text = "Event History",
                        style = MaterialTheme.typography.titleSmall,
                    )
                }
                items(uiState.events, key = { it.id }) { event ->
                    EventRow(event = event)
                }
            }

            // Relations section
            if (uiState.relations.isNotEmpty()) {
                item {
                    RelationsSection(relations = uiState.relations)
                }
            }

            item { Spacer(modifier = Modifier.height(16.dp)) }
        }

        if (uiState.showDeleteConfirm) {
            AlertDialog(
                onDismissRequest = viewModel::onDeleteDismiss,
                title = { Text("Delete Item") },
                text = { Text("Are you sure you want to delete \"${item.name}\"? This cannot be undone.") },
                confirmButton = {
                    TextButton(onClick = {
                        viewModel.onDeleteConfirm()
                        onNavigateUp()
                    }) {
                        Text("Delete", color = MaterialTheme.colorScheme.error)
                    }
                },
                dismissButton = {
                    TextButton(onClick = viewModel::onDeleteDismiss) {
                        Text("Cancel")
                    }
                },
            )
        }
    }
}

@Composable
private fun QuantitySection(
    quantity: Int,
    unit: String?,
    onIncrement: () -> Unit,
    onDecrement: () -> Unit,
) {
    AltairCard(modifier = Modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalArrangement = Arrangement.SpaceEvenly,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            FilledTonalIconButton(onClick = onDecrement) {
                Icon(Icons.Default.Remove, contentDescription = "Decrease quantity")
            }
            Column(horizontalAlignment = Alignment.CenterHorizontally) {
                Text(
                    text = "$quantity",
                    style = MaterialTheme.typography.displaySmall,
                    textAlign = TextAlign.Center,
                )
                if (unit != null) {
                    Text(
                        text = unit,
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
            FilledTonalIconButton(onClick = onIncrement) {
                Icon(Icons.Default.Add, contentDescription = "Increase quantity")
            }
        }
    }
}

@Composable
private fun MetadataSection(
    item: TrackingItem,
    onDelete: () -> Unit,
) {
    AltairCard(modifier = Modifier.fillMaxWidth()) {
        Column(modifier = Modifier.padding(16.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = when (item.status) {
                        TrackingItemStatus.ACTIVE -> "Active"
                        TrackingItemStatus.ARCHIVED -> "Archived"
                    },
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.primary,
                )
                IconButton(onClick = onDelete, modifier = Modifier.size(24.dp)) {
                    Icon(
                        imageVector = Icons.Default.Delete,
                        contentDescription = "Delete item",
                        tint = MaterialTheme.colorScheme.error,
                    )
                }
            }

            if (item.description != null) {
                Spacer(modifier = Modifier.height(12.dp))
                Text(
                    text = item.description,
                    style = MaterialTheme.typography.bodyLarge,
                    color = MaterialTheme.colorScheme.onSurface,
                )
            }

            if (item.barcode != null) {
                Spacer(modifier = Modifier.height(8.dp))
                Text(
                    text = "Barcode: ${item.barcode}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }

            if (item.minQuantity != null) {
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = "Min quantity: ${item.minQuantity}",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun EventRow(event: TrackingItemEvent) {
    val (icon, label) = when (event.eventType) {
        ItemEventType.CONSUMED -> Icons.Default.Remove to "Consumed"
        ItemEventType.RESTOCKED -> Icons.Default.Add to "Restocked"
        ItemEventType.MOVED -> Icons.Default.Edit to "Moved"
        ItemEventType.ADJUSTED -> Icons.Default.Edit to "Adjusted"
        ItemEventType.EXPIRED -> Icons.Default.Warning to "Expired"
        ItemEventType.DONATED -> Icons.Default.Favorite to "Donated"
    }

    AltairCard(modifier = Modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Icon(
                imageVector = icon,
                contentDescription = null,
                modifier = Modifier.size(20.dp),
                tint = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = label,
                    style = MaterialTheme.typography.bodyMedium,
                )
                if (event.notes != null) {
                    Text(
                        text = event.notes,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
            Column(horizontalAlignment = Alignment.End) {
                val sign = if (event.quantityChange >= 0) "+" else ""
                Text(
                    text = "$sign${event.quantityChange}",
                    style = MaterialTheme.typography.labelMedium,
                    color = if (event.quantityChange >= 0) {
                        MaterialTheme.colorScheme.primary
                    } else {
                        MaterialTheme.colorScheme.error
                    },
                )
                Text(
                    text = event.createdAt
                        .atZone(ZoneId.systemDefault())
                        .format(dateFormatter),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}
