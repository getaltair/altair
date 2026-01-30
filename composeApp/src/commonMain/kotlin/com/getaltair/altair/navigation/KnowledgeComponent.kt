package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Knowledge module component - notes and PKM.
 * Handles notes, folders, and wikilinks.
 *
 * @param componentContext Decompose component context for lifecycle management
 * @param onOutput Callback for navigation outputs (default no-op for previews)
 */
class KnowledgeComponent(
    componentContext: ComponentContext,
    private val onOutput: (Output) -> Unit = {}
) : ComponentContext by componentContext {

    /**
     * Navigation outputs for the Knowledge module.
     * Will be expanded as we add note details, folder navigation, etc.
     */
    sealed interface Output {
        // Future: navigate to note detail, etc.
    }
}
