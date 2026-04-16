package com.getaltair.altair.ui.tracking

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.ElevatedCard
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.getaltair.altair.data.local.entity.TrackingItemEventEntity
import org.koin.androidx.compose.koinViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ItemDetailScreen(
    itemId: String,
    navController: NavController,
    modifier: Modifier = Modifier,
    viewModel: TrackingViewModel = koinViewModel(),
) {
    val items by viewModel.items.collectAsState()
    val item = items.find { it.id == itemId }
    val events by viewModel.watchItemEvents(itemId).collectAsState()
    val locations by viewModel.locations.collectAsState()
    val categories by viewModel.categories.collectAsState()
    val uiState by viewModel.uiState.collectAsState()

    var showConsumeDialog by remember { mutableStateOf(false) }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(item?.name ?: "Item") },
                navigationIcon = {
                    IconButton(onClick = { navController.popBackStack() }) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                    }
                },
            )
        },
        modifier = modifier,
    ) { innerPadding ->
        if (item == null) {
            Box(
                modifier =
                    Modifier
                        .fillMaxSize()
                        .padding(innerPadding),
                contentAlignment = Alignment.Center,
            ) {
                Text("Item not found")
            }
            return@Scaffold
        }

        LazyColumn(
            modifier =
                Modifier
                    .fillMaxSize()
                    .padding(innerPadding),
            contentPadding = PaddingValues(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                ElevatedCard(
                    shape = RoundedCornerShape(16.dp),
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Column(modifier = Modifier.padding(16.dp)) {
                        DetailRow(
                            label = "Quantity",
                            value =
                                item.quantity.let {
                                    if (it == it.toLong().toDouble()) it.toLong().toString() else it.toString()
                                },
                        )
                        val locationName = locations.find { it.id == item.locationId }?.name
                        if (locationName != null) {
                            DetailRow(label = "Location", value = locationName)
                        }
                        val categoryName = categories.find { it.id == item.categoryId }?.name
                        if (categoryName != null) {
                            DetailRow(label = "Category", value = categoryName)
                        }
                        if (item.description != null) {
                            DetailRow(label = "Description", value = item.description)
                        }
                    }
                }
            }

            item {
                Button(
                    onClick = { showConsumeDialog = true },
                    shape = RoundedCornerShape(50),
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text("Log Consumption")
                }
            }

            if (uiState is TrackingUiState.Error) {
                item {
                    Text(
                        text = (uiState as TrackingUiState.Error).message,
                        color = MaterialTheme.colorScheme.error,
                        style = MaterialTheme.typography.bodyMedium,
                    )
                }
            }

            item {
                Text(
                    text = "Event History",
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                )
            }

            if (events.isEmpty()) {
                item {
                    Text(
                        text = "No events recorded yet",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            } else {
                items(events, key = { it.id }) { event ->
                    EventRow(event = event)
                }
            }
        }
    }

    if (showConsumeDialog) {
        ConsumeDialog(
            currentQuantity = item?.quantity ?: 0.0,
            onDismiss = {
                showConsumeDialog = false
                viewModel.clearError()
            },
            onConfirm = { amount ->
                viewModel.logConsumption(itemId, amount)
                showConsumeDialog = false
            },
        )
    }
}

@Composable
private fun DetailRow(
    label: String,
    value: String,
    modifier: Modifier = Modifier,
) {
    Row(
        modifier =
            modifier
                .fillMaxWidth()
                .padding(vertical = 4.dp),
        horizontalArrangement = Arrangement.SpaceBetween,
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Text(
            text = value,
            style = MaterialTheme.typography.bodyMedium,
            fontWeight = FontWeight.Medium,
        )
    }
}

@Composable
private fun EventRow(
    event: TrackingItemEventEntity,
    modifier: Modifier = Modifier,
) {
    Column(modifier = modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text(
                text = event.eventType.replaceFirstChar { it.uppercase() },
                style = MaterialTheme.typography.bodyMedium,
            )
            if (event.quantityChange != null) {
                Text(
                    text =
                        if (event.quantityChange < 0) {
                            "−${-event.quantityChange}"
                        } else {
                            "+${event.quantityChange}"
                        },
                    style = MaterialTheme.typography.bodyMedium,
                    color =
                        if (event.quantityChange < 0) {
                            MaterialTheme.colorScheme.error
                        } else {
                            MaterialTheme.colorScheme.primary
                        },
                )
            }
        }
        val timestamp = event.occurredAt ?: event.createdAt
        Text(
            text = timestamp,
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Spacer(modifier = Modifier.height(4.dp))
        HorizontalDivider(color = MaterialTheme.colorScheme.outlineVariant)
    }
}

@Composable
private fun ConsumeDialog(
    currentQuantity: Double,
    onDismiss: () -> Unit,
    onConfirm: (Double) -> Unit,
    modifier: Modifier = Modifier,
) {
    var amountText by remember { mutableStateOf("") }
    val amount = amountText.toDoubleOrNull()
    val isValid = amount != null && amount > 0 && amount <= currentQuantity

    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Log Consumption") },
        text = {
            Column {
                Text(
                    text = "Current quantity: $currentQuantity",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                Spacer(modifier = Modifier.height(8.dp))
                OutlinedTextField(
                    value = amountText,
                    onValueChange = { amountText = it },
                    label = { Text("Amount to consume") },
                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
                    isError = amountText.isNotEmpty() && !isValid,
                    supportingText =
                        if (amountText.isNotEmpty() && !isValid) {
                            { Text("Must be > 0 and ≤ $currentQuantity") }
                        } else {
                            null
                        },
                    singleLine = true,
                )
            }
        },
        confirmButton = {
            Button(
                onClick = { if (isValid && amount != null) onConfirm(amount) },
                enabled = isValid,
                shape = RoundedCornerShape(50),
            ) {
                Text("Confirm")
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss) {
                Text("Cancel")
            }
        },
        modifier = modifier,
    )
}
