package com.getaltair.altair.viewmodel

import com.getaltair.altair.api.InboxApi
import com.getaltair.altair.shared.domain.common.CaptureSource
import com.getaltair.altair.shared.dto.system.CreateInboxItemRequest
import com.getaltair.altair.shared.dto.system.InboxItemResponse
import com.getaltair.altair.shared.dto.system.TriageRequest
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

/**
 * ViewModel for Universal Inbox screen.
 *
 * Manages inbox items, quick capture, and triage operations.
 * Handles all state management for inbox UI interactions.
 *
 * @property inboxApi API client for inbox operations
 * @property scope Coroutine scope for launching async operations
 */
class InboxViewModel(
    private val inboxApi: InboxApi,
    private val scope: CoroutineScope
) {
    // === State ===

    private val _inboxItems = MutableStateFlow<UiState<List<InboxItemResponse>>>(UiState.Loading)
    val inboxItems: StateFlow<UiState<List<InboxItemResponse>>> = _inboxItems.asStateFlow()

    private val _isCapturing = MutableStateFlow(false)
    val isCapturing: StateFlow<Boolean> = _isCapturing.asStateFlow()

    // === Actions ===

    /**
     * Load all inbox items for the current user.
     *
     * Sets state to Loading, then Success or Error based on API response.
     */
    fun loadInboxItems() {
        scope.launch {
            _inboxItems.value = UiState.Loading
            inboxApi.getInboxItems().fold(
                ifLeft = { error ->
                    _inboxItems.value = UiState.Error(error.toString())
                },
                ifRight = { items ->
                    _inboxItems.value = UiState.Success(items)
                }
            )
        }
    }

    /**
     * Quick capture a new inbox item.
     *
     * @param content The captured text content
     * @param source Source of the capture (default: KEYBOARD)
     */
    fun captureItem(content: String, source: CaptureSource = CaptureSource.KEYBOARD) {
        if (content.isBlank()) return

        scope.launch {
            _isCapturing.value = true
            val request = CreateInboxItemRequest(
                content = content,
                source = source
            )

            inboxApi.createInboxItem(request).fold(
                ifLeft = { error ->
                    _isCapturing.value = false
                    // On error, show error state but preserve existing items
                    val currentItems = (_inboxItems.value as? UiState.Success)?.data ?: emptyList()
                    _inboxItems.value = UiState.Error("Failed to capture: ${error}")
                    // Restore previous items after error
                    _inboxItems.value = UiState.Success(currentItems)
                },
                ifRight = { newItem ->
                    _isCapturing.value = false
                    // Add new item to the list
                    val currentItems = (_inboxItems.value as? UiState.Success)?.data ?: emptyList()
                    _inboxItems.value = UiState.Success(listOf(newItem) + currentItems)
                }
            )
        }
    }

    /**
     * Delete an inbox item.
     *
     * Removes the item from the UI immediately (optimistic update).
     *
     * @param id Inbox item identifier
     */
    fun deleteItem(id: String) {
        scope.launch {
            // Optimistic update: remove item immediately
            val currentItems = (_inboxItems.value as? UiState.Success)?.data ?: return@launch
            val updatedItems = currentItems.filter { it.id != id }
            _inboxItems.value = UiState.Success(updatedItems)

            inboxApi.deleteInboxItem(id).fold(
                ifLeft = { error ->
                    // On error, restore the item
                    _inboxItems.value = UiState.Success(currentItems)
                    _inboxItems.value = UiState.Error("Failed to delete: ${error}")
                    _inboxItems.value = UiState.Success(currentItems)
                },
                ifRight = {
                    // Success - item already removed
                }
            )
        }
    }

    /**
     * Triage an inbox item into a Quest, Note, Item, or SourceDocument.
     *
     * Creates the target entity and removes the inbox item atomically.
     *
     * @param id Inbox item identifier
     * @param request Triage parameters specifying target type
     */
    fun triageItem(id: String, request: TriageRequest) {
        scope.launch {
            inboxApi.triageItem(id, request).fold(
                ifLeft = { error ->
                    _inboxItems.value = UiState.Error("Failed to triage: ${error}")
                },
                ifRight = { response ->
                    // Remove triaged item from list
                    val currentItems = (_inboxItems.value as? UiState.Success)?.data ?: emptyList()
                    val updatedItems = currentItems.filter { it.id != id }
                    _inboxItems.value = UiState.Success(updatedItems)
                }
            )
        }
    }
}
