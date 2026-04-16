package com.getaltair.altair.ui.tracking

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.Badge
import androidx.compose.material3.ElevatedCard
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilterChip
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import com.getaltair.altair.data.local.entity.TrackingCategoryEntity
import com.getaltair.altair.data.local.entity.TrackingItemEntity
import com.getaltair.altair.data.local.entity.TrackingLocationEntity
import com.getaltair.altair.navigation.Screen
import com.getaltair.altair.ui.theme.SophisticatedTerracotta
import org.koin.androidx.compose.koinViewModel

private const val LOW_STOCK_THRESHOLD = 2.0

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TrackingScreen(
    navController: NavController,
    modifier: Modifier = Modifier,
    viewModel: TrackingViewModel = koinViewModel(),
) {
    val filteredItems by viewModel.filteredItems.collectAsState()
    val locations by viewModel.locations.collectAsState()
    val categories by viewModel.categories.collectAsState()
    val searchQuery by viewModel.searchQuery.collectAsState()
    val selectedLocation by viewModel.selectedLocation.collectAsState()
    val selectedCategory by viewModel.selectedCategory.collectAsState()

    Scaffold(
        topBar = {
            TopAppBar(title = { Text("Inventory") })
        },
        floatingActionButton = {
            FloatingActionButton(
                onClick = { navController.navigate("tracking/items/new") },
            ) {
                Icon(Icons.Default.Add, contentDescription = "Add Item")
            }
        },
        modifier = modifier,
    ) { innerPadding ->
        Column(
            modifier =
                Modifier
                    .fillMaxSize()
                    .padding(innerPadding),
        ) {
            OutlinedTextField(
                value = searchQuery,
                onValueChange = { viewModel.searchQuery.value = it },
                placeholder = { Text("Search items…") },
                leadingIcon = { Icon(Icons.Default.Search, contentDescription = null) },
                singleLine = true,
                modifier =
                    Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp, vertical = 8.dp),
            )

            LazyRow(
                contentPadding = PaddingValues(horizontal = 16.dp),
                horizontalArrangement = Arrangement.spacedBy(8.dp),
                modifier = Modifier.fillMaxWidth(),
            ) {
                item {
                    FilterChip(
                        selected = selectedLocation == null,
                        onClick = { viewModel.selectedLocation.value = null },
                        label = { Text("All locations") },
                    )
                }
                items(locations) { loc ->
                    FilterChip(
                        selected = selectedLocation == loc.id,
                        onClick = {
                            viewModel.selectedLocation.value =
                                if (selectedLocation == loc.id) null else loc.id
                        },
                        label = { Text(loc.name) },
                    )
                }
            }

            LazyRow(
                contentPadding = PaddingValues(horizontal = 16.dp, vertical = 4.dp),
                horizontalArrangement = Arrangement.spacedBy(8.dp),
                modifier = Modifier.fillMaxWidth(),
            ) {
                item {
                    FilterChip(
                        selected = selectedCategory == null,
                        onClick = { viewModel.selectedCategory.value = null },
                        label = { Text("All categories") },
                    )
                }
                items(categories) { cat ->
                    FilterChip(
                        selected = selectedCategory == cat.id,
                        onClick = {
                            viewModel.selectedCategory.value =
                                if (selectedCategory == cat.id) null else cat.id
                        },
                        label = { Text(cat.name) },
                    )
                }
            }

            if (filteredItems.isEmpty()) {
                Box(
                    modifier =
                        Modifier
                            .fillMaxSize()
                            .padding(32.dp),
                    contentAlignment = Alignment.Center,
                ) {
                    Text(
                        text = "No items found",
                        style = MaterialTheme.typography.bodyLarge,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            } else {
                LazyColumn(
                    contentPadding = PaddingValues(16.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    items(filteredItems, key = { it.id }) { item ->
                        TrackingItemCard(
                            item = item,
                            locationName = locations.find { it.id == item.locationId }?.name,
                            categoryName = categories.find { it.id == item.categoryId }?.name,
                            onClick = { navController.navigate(Screen.ItemDetail.route(item.id)) },
                        )
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun TrackingItemCard(
    item: TrackingItemEntity,
    locationName: String?,
    categoryName: String?,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val isLowStock = item.quantity <= LOW_STOCK_THRESHOLD

    ElevatedCard(
        onClick = onClick,
        shape =
            androidx.compose.foundation.shape
                .RoundedCornerShape(16.dp),
        modifier = modifier.fillMaxWidth(),
    ) {
        Row(
            modifier =
                Modifier
                    .fillMaxWidth()
                    .padding(16.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = item.name,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                )
                if (locationName != null || categoryName != null) {
                    Text(
                        text = listOfNotNull(locationName, categoryName).joinToString(" · "),
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }

            Spacer(modifier = Modifier.width(12.dp))

            Badge(
                containerColor = if (isLowStock) SophisticatedTerracotta else MaterialTheme.colorScheme.secondaryContainer,
                contentColor = if (isLowStock) Color.White else MaterialTheme.colorScheme.onSecondaryContainer,
            ) {
                Text(
                    text =
                        item.quantity.let {
                            if (it == it.toLong().toDouble()) it.toLong().toString() else it.toString()
                        },
                    modifier = Modifier.padding(horizontal = 6.dp, vertical = 2.dp),
                )
            }
        }
    }
}
