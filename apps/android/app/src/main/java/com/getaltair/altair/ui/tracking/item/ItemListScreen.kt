package com.getaltair.altair.ui.tracking.item

import androidx.compose.foundation.clickable
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
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.List
import androidx.compose.material.icons.filled.QrCodeScanner
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.getaltair.altair.domain.entity.TrackingItem
import com.getaltair.altair.domain.entity.TrackingItemStatus
import com.getaltair.altair.navigation.Screen
import com.getaltair.altair.ui.components.AltairCard
import com.getaltair.altair.ui.components.AltairEmptyState
import com.getaltair.altair.ui.components.AltairFilterChips
import com.getaltair.altair.ui.components.AltairLoadingBox
import com.getaltair.altair.ui.components.AltairSearchBar
import com.getaltair.altair.ui.components.FilterChipOption
import com.getaltair.altair.ui.navigation.AltairBottomNavBar
import org.koin.androidx.compose.koinViewModel

private val filterOptions = listOf(
    FilterChipOption("All", ItemFilter.ALL),
    FilterChipOption("Active", ItemFilter.ACTIVE),
    FilterChipOption("Archived", ItemFilter.ARCHIVED),
    FilterChipOption("Low Stock", ItemFilter.LOW_STOCK),
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ItemListScreen(
    onNavigateToDetail: (String) -> Unit,
    onNavigateToBarcodeScanner: () -> Unit,
    onNavigateToTab: (String) -> Unit,
    viewModel: ItemListViewModel = koinViewModel(),
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    Scaffold(
        containerColor = MaterialTheme.colorScheme.background,
        topBar = {
            TopAppBar(
                title = {
                    Text(
                        text = "Items",
                        style = MaterialTheme.typography.headlineMedium,
                    )
                },
                actions = {
                    IconButton(onClick = onNavigateToBarcodeScanner) {
                        Icon(
                            imageVector = Icons.Default.QrCodeScanner,
                            contentDescription = "Scan barcode",
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.background,
                    titleContentColor = MaterialTheme.colorScheme.onBackground,
                ),
            )
        },
        bottomBar = {
            AltairBottomNavBar(
                currentRoute = Screen.ItemList.route,
                onNavigate = onNavigateToTab,
            )
        },
    ) { innerPadding ->
        if (uiState.isLoading) {
            AltairLoadingBox(modifier = Modifier.padding(innerPadding))
        } else if (uiState.items.isEmpty() && uiState.searchQuery.isBlank()) {
            AltairEmptyState(
                icon = Icons.Default.List,
                title = "No items yet",
                modifier = Modifier.padding(innerPadding),
            )
        } else {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(innerPadding),
            ) {
                AltairSearchBar(
                    query = uiState.searchQuery,
                    onQueryChange = viewModel::onSearchQueryChange,
                    placeholder = "Search items...",
                )
                AltairFilterChips(
                    options = filterOptions,
                    selectedValue = uiState.activeFilter,
                    onSelect = viewModel::onFilterChange,
                )
                LazyColumn(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(horizontal = 16.dp),
                    verticalArrangement = Arrangement.spacedBy(16.dp),
                ) {
                    items(uiState.items, key = { it.id }) { item ->
                        ItemCard(
                            item = item,
                            onClick = { onNavigateToDetail(item.id.toString()) },
                        )
                    }
                    item { Spacer(modifier = Modifier.height(16.dp)) }
                }
            }
        }
    }
}

@Composable
private fun ItemCard(
    item: TrackingItem,
    onClick: () -> Unit,
) {
    AltairCard(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Column(modifier = Modifier.padding(16.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = item.name,
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.onSurface,
                    modifier = Modifier.weight(1f),
                )
                ItemStatusBadge(status = item.status)
            }
            Spacer(modifier = Modifier.height(4.dp))
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                val quantityText = if (item.unit != null) {
                    "${item.quantity} ${item.unit}"
                } else {
                    "${item.quantity}"
                }
                Text(
                    text = quantityText,
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                if (item.minQuantity != null && item.quantity <= item.minQuantity) {
                    Text(
                        text = "Low Stock",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.error,
                    )
                }
            }
        }
    }
}

@Composable
private fun ItemStatusBadge(status: TrackingItemStatus) {
    val (text, color) = when (status) {
        TrackingItemStatus.ACTIVE -> "Active" to MaterialTheme.colorScheme.primary
        TrackingItemStatus.ARCHIVED -> "Archived" to MaterialTheme.colorScheme.onSurfaceVariant
    }
    Text(
        text = text,
        style = MaterialTheme.typography.labelSmall,
        color = color,
    )
}
