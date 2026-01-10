package com.getaltair.altair.navigation

import com.arkivanov.decompose.ComponentContext

/**
 * Component for the Knowledge module (Notes, Folders, Tags, Wiki-links).
 */
interface KnowledgeComponent {
    // Add state and methods as features are implemented
}

/**
 * Default implementation of KnowledgeComponent.
 */
class DefaultKnowledgeComponent(componentContext: ComponentContext) :
    KnowledgeComponent,
    ComponentContext by componentContext
