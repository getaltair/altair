package com.getaltair.altair.ui.tracking

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.ShoppingListDao
import com.getaltair.altair.data.local.dao.ShoppingListItemDao
import com.getaltair.altair.data.local.dao.TrackingCategoryDao
import com.getaltair.altair.data.local.dao.TrackingItemDao
import com.getaltair.altair.data.local.dao.TrackingItemEventDao
import com.getaltair.altair.data.local.dao.TrackingLocationDao
import com.getaltair.altair.data.local.entity.ShoppingListEntity
import com.getaltair.altair.data.local.entity.ShoppingListItemEntity
import com.getaltair.altair.data.local.entity.TrackingCategoryEntity
import com.getaltair.altair.data.local.entity.TrackingItemEntity
import com.getaltair.altair.data.local.entity.TrackingItemEventEntity
import com.getaltair.altair.data.local.entity.TrackingLocationEntity
import com.powersync.PowerSyncDatabase
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch
import kotlinx.datetime.Clock
import java.util.UUID

private const val TAG = "TrackingViewModel"

sealed class TrackingUiState {
    object Idle : TrackingUiState()

    object Loading : TrackingUiState()

    data class Error(
        val message: String,
    ) : TrackingUiState()
}

@OptIn(ExperimentalCoroutinesApi::class)
class TrackingViewModel(
    private val trackingItemDao: TrackingItemDao,
    private val trackingItemEventDao: TrackingItemEventDao,
    private val trackingLocationDao: TrackingLocationDao,
    private val trackingCategoryDao: TrackingCategoryDao,
    private val shoppingListDao: ShoppingListDao,
    private val shoppingListItemDao: ShoppingListItemDao,
    private val db: PowerSyncDatabase,
) : ViewModel() {
    val searchQuery = MutableStateFlow("")
    val selectedLocation = MutableStateFlow<String?>(null)
    val selectedCategory = MutableStateFlow<String?>(null)

    private val _uiState = MutableStateFlow<TrackingUiState>(TrackingUiState.Idle)
    val uiState: StateFlow<TrackingUiState> = _uiState

    private val currentHouseholdId: StateFlow<String?> =
        db
            .watch<String>(
                sql =
                    """
                    SELECT hm.household_id FROM household_memberships hm
                    INNER JOIN users u ON hm.user_id = u.id
                    WHERE u.deleted_at IS NULL AND hm.deleted_at IS NULL
                    LIMIT 1
                    """.trimIndent(),
                parameters = emptyList(),
            ) { cursor -> cursor.getString(0) ?: "" }
            .map { list -> list.firstOrNull()?.takeIf { it.isNotEmpty() } }
            .stateIn(viewModelScope, SharingStarted.Eagerly, null)

    // Eagerly started so items.value is available in command handlers (logConsumption, etc.)
    val items: StateFlow<List<TrackingItemEntity>> =
        currentHouseholdId
            .flatMapLatest { hid ->
                if (hid == null) flowOf(emptyList()) else trackingItemDao.watchAll(hid)
            }.stateIn(viewModelScope, SharingStarted.Eagerly, emptyList())

    val filteredItems: StateFlow<List<TrackingItemEntity>> =
        combine(items, searchQuery, selectedLocation, selectedCategory) { list, query, loc, cat ->
            list.filter { item ->
                (query.isBlank() || item.name.contains(query, ignoreCase = true)) &&
                    (loc == null || item.locationId == loc) &&
                    (cat == null || item.categoryId == cat)
            }
        }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    val locations: StateFlow<List<TrackingLocationEntity>> =
        currentHouseholdId
            .flatMapLatest { hid ->
                if (hid == null) flowOf(emptyList()) else trackingLocationDao.watchAll(hid)
            }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    val categories: StateFlow<List<TrackingCategoryEntity>> =
        currentHouseholdId
            .flatMapLatest { hid ->
                if (hid == null) flowOf(emptyList()) else trackingCategoryDao.watchAll(hid)
            }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    val shoppingLists: StateFlow<List<ShoppingListEntity>> =
        currentHouseholdId
            .flatMapLatest { hid ->
                if (hid == null) flowOf(emptyList()) else shoppingListDao.watchAll(hid)
            }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    fun watchItemEvents(itemId: String): StateFlow<List<TrackingItemEventEntity>> =
        trackingItemEventDao
            .watchAll(itemId)
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    fun watchShoppingListItems(shoppingListId: String): StateFlow<List<ShoppingListItemEntity>> =
        shoppingListItemDao
            .watchAll(shoppingListId)
            .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), emptyList())

    fun createItem(
        name: String,
        quantity: Double,
        locationId: String?,
        categoryId: String?,
        barcode: String? = null,
    ) {
        viewModelScope.launch {
            val hid =
                currentHouseholdId.value ?: run {
                    _uiState.value = TrackingUiState.Error("Not in a household")
                    return@launch
                }
            _uiState.value = TrackingUiState.Loading
            try {
                val now = Clock.System.now().toString()
                val id = UUID.randomUUID().toString()
                db.execute(
                    "INSERT INTO tracking_items (id, name, quantity, barcode, location_id, category_id, user_id, household_id, created_at, updated_at) " +
                        "VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                    listOf(id, name, quantity, barcode, locationId, categoryId, "", hid, now, now),
                )
                _uiState.value = TrackingUiState.Idle
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to create item", e)
                _uiState.value = TrackingUiState.Error(e.message ?: "Failed to create item")
            }
        }
    }

    fun logConsumption(
        itemId: String,
        amount: Double,
    ) {
        viewModelScope.launch {
            val current = items.value.find { it.id == itemId }
            if (current == null) {
                _uiState.value = TrackingUiState.Error("Item not found")
                return@launch
            }
            // FA-008: validate before any write
            if (amount <= 0 || amount > current.quantity) {
                _uiState.value =
                    TrackingUiState.Error(
                        "Invalid amount: must be > 0 and <= current quantity (${current.quantity})",
                    )
                return@launch
            }
            _uiState.value = TrackingUiState.Loading
            try {
                val now = Clock.System.now().toString()
                val eventId = UUID.randomUUID().toString()
                db.execute(
                    "INSERT INTO tracking_item_events (id, item_id, event_type, quantity_change, occurred_at, created_at) " +
                        "VALUES (?, ?, ?, ?, ?, ?)",
                    listOf(eventId, itemId, "consumption", -amount, now, now),
                )
                db.execute(
                    "UPDATE tracking_items SET quantity = quantity - ?, updated_at = ? WHERE id = ?",
                    listOf(amount, now, itemId),
                )
                _uiState.value = TrackingUiState.Idle
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to log consumption", e)
                _uiState.value = TrackingUiState.Error(e.message ?: "Failed to log consumption")
            }
        }
    }

    fun createLocation(name: String) {
        viewModelScope.launch {
            val hid =
                currentHouseholdId.value ?: run {
                    _uiState.value = TrackingUiState.Error("Not in a household")
                    return@launch
                }
            _uiState.value = TrackingUiState.Loading
            try {
                val now = Clock.System.now().toString()
                val id = UUID.randomUUID().toString()
                db.execute(
                    "INSERT INTO tracking_locations (id, name, household_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                    listOf(id, name, hid, now, now),
                )
                _uiState.value = TrackingUiState.Idle
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to create location", e)
                _uiState.value = TrackingUiState.Error(e.message ?: "Failed to create location")
            }
        }
    }

    fun createCategory(name: String) {
        viewModelScope.launch {
            val hid =
                currentHouseholdId.value ?: run {
                    _uiState.value = TrackingUiState.Error("Not in a household")
                    return@launch
                }
            _uiState.value = TrackingUiState.Loading
            try {
                val now = Clock.System.now().toString()
                val id = UUID.randomUUID().toString()
                db.execute(
                    "INSERT INTO tracking_categories (id, name, household_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                    listOf(id, name, hid, now, now),
                )
                _uiState.value = TrackingUiState.Idle
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to create category", e)
                _uiState.value = TrackingUiState.Error(e.message ?: "Failed to create category")
            }
        }
    }

    fun createShoppingList(name: String) {
        viewModelScope.launch {
            val hid =
                currentHouseholdId.value ?: run {
                    _uiState.value = TrackingUiState.Error("Not in a household")
                    return@launch
                }
            _uiState.value = TrackingUiState.Loading
            try {
                val now = Clock.System.now().toString()
                val id = UUID.randomUUID().toString()
                db.execute(
                    "INSERT INTO shopping_lists (id, name, household_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                    listOf(id, name, hid, now, now),
                )
                _uiState.value = TrackingUiState.Idle
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to create shopping list", e)
                _uiState.value = TrackingUiState.Error(e.message ?: "Failed to create shopping list")
            }
        }
    }

    fun addShoppingListItem(
        shoppingListId: String,
        name: String,
        quantity: Double = 1.0,
    ) {
        viewModelScope.launch {
            _uiState.value = TrackingUiState.Loading
            try {
                val now = Clock.System.now().toString()
                val id = UUID.randomUUID().toString()
                db.execute(
                    "INSERT INTO shopping_list_items (id, shopping_list_id, name, quantity, status, created_at, updated_at) " +
                        "VALUES (?, ?, ?, ?, ?, ?, ?)",
                    listOf(id, shoppingListId, name, quantity, "pending", now, now),
                )
                _uiState.value = TrackingUiState.Idle
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to add shopping list item", e)
                _uiState.value = TrackingUiState.Error(e.message ?: "Failed to add shopping list item")
            }
        }
    }

    fun toggleShoppingListItem(item: ShoppingListItemEntity) {
        viewModelScope.launch {
            _uiState.value = TrackingUiState.Loading
            try {
                val newStatus = if (item.status == "completed") "pending" else "completed"
                val now = Clock.System.now().toString()
                db.execute(
                    "UPDATE shopping_list_items SET status = ?, updated_at = ? WHERE id = ?",
                    listOf(newStatus, now, item.id),
                )
                _uiState.value = TrackingUiState.Idle
            } catch (e: CancellationException) {
                throw e
            } catch (e: Exception) {
                Log.e(TAG, "Failed to toggle shopping list item", e)
                _uiState.value = TrackingUiState.Error(e.message ?: "Failed to update item")
            }
        }
    }

    fun clearError() {
        _uiState.value = TrackingUiState.Idle
    }
}
