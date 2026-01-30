package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext
import com.getaltair.altair.shared.domain.common.CaptureSource
import com.getaltair.altair.shared.dto.system.InboxItemResponse
import com.getaltair.altair.shared.dto.system.TriageRequest
import com.getaltair.altair.viewmodel.InboxViewModel

/**
 * Inbox component - handles Universal Inbox operations.
 *
 * Provides quick capture interface for:
 * - Text notes
 * - Voice memos
 * - Photos/screenshots
 * - Links/URLs
 *
 * Supports triage workflow to convert inbox items into:
 * - Quests (tasks to execute)
 * - Notes (knowledge items)
 * - Items (physical inventory)
 * - Source Documents (reference material)
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param inboxViewModel ViewModel managing inbox state and operations
 */
class InboxComponent(
    componentContext: ComponentContext,
    val inboxViewModel: InboxViewModel
) : ComponentContext by componentContext {

    /**
     * Initialize component - load inbox items.
     */
    init {
        inboxViewModel.loadInboxItems()
    }

    /**
     * Handle click on an inbox item.
     * Opens the item for viewing/triaging.
     *
     * @param item The inbox item that was clicked
     */
    fun onItemClick(item: InboxItemResponse) {
        // Future: open triage dialog or detail view
        // For now, this is a placeholder for UI implementation
    }

    /**
     * Handle quick capture action.
     * Opens the capture interface (keyboard, voice, camera, etc.).
     */
    fun onCaptureClick() {
        // Future: open capture sheet/dialog
        // For now, this is a placeholder for UI implementation
    }

    /**
     * Quick capture text content.
     *
     * @param content The text content to capture
     * @param source Source of the capture (default: KEYBOARD)
     */
    fun captureText(content: String, source: CaptureSource = CaptureSource.KEYBOARD) {
        inboxViewModel.captureItem(content, source)
    }

    /**
     * Delete an inbox item.
     *
     * @param id Inbox item identifier
     */
    fun deleteItem(id: String) {
        inboxViewModel.deleteItem(id)
    }

    /**
     * Triage an inbox item into a target entity.
     *
     * @param id Inbox item identifier
     * @param request Triage parameters specifying target type and fields
     */
    fun triageItem(id: String, request: TriageRequest) {
        inboxViewModel.triageItem(id, request)
    }

    /**
     * Handle triage completion.
     * Called after successfully triaging an item.
     */
    fun onTriageComplete() {
        // Refresh inbox items after triage
        inboxViewModel.loadInboxItems()
    }
}
