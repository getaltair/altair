package com.getaltair.altair.ui.guidance

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.BasicText
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.domain.common.QuestStatus
import com.getaltair.altair.shared.dto.guidance.QuestResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.components.AltairButton
import com.getaltair.altair.ui.theme.components.AltairDialog
import com.getaltair.altair.ui.theme.components.ButtonVariant
import com.getaltair.altair.ui.theme.components.LocalAltairContentColor

/**
 * Quest detail screen showing full quest information and actions.
 *
 * Features:
 * - Header with title, energy badge, and status
 * - Description section
 * - Checkpoint list with completion toggles
 * - Status-specific action buttons:
 *   - BACKLOG: "Start Quest"
 *   - ACTIVE: "Complete", "Abandon", "Back to Backlog"
 *   - COMPLETED/ABANDONED: "Reopen"
 * - Edit button in app bar
 * - WIP=1 conflict handling dialog
 *
 * @param quest Quest details
 * @param isLoading Whether data is loading
 * @param error Optional error message
 * @param hasActiveQuest Whether another quest is currently active (for WIP=1 check)
 * @param onNavigateBack Callback to navigate back
 * @param onStartQuest Callback to start the quest
 * @param onCompleteQuest Callback to complete the quest
 * @param onAbandonQuest Callback to abandon the quest
 * @param onBackToBacklog Callback to move quest back to backlog
 * @param onReopenQuest Callback to reopen a completed/abandoned quest
 * @param onCheckpointToggle Callback when a checkpoint is toggled
 * @param onAddCheckpoint Callback to add a new checkpoint
 * @param onEditQuest Callback to edit the quest
 * @param modifier Modifier to be applied to the screen
 */
@Composable
fun QuestDetailScreen(
    quest: QuestResponse?,
    isLoading: Boolean,
    error: String?,
    hasActiveQuest: Boolean,
    onNavigateBack: () -> Unit,
    onStartQuest: () -> Unit,
    onCompleteQuest: () -> Unit,
    onAbandonQuest: () -> Unit,
    onBackToBacklog: () -> Unit,
    onReopenQuest: () -> Unit,
    onCheckpointToggle: (String) -> Unit,
    onAddCheckpoint: () -> Unit,
    onEditQuest: () -> Unit,
    modifier: Modifier = Modifier
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography

    var showWipConflictDialog by remember { mutableStateOf(false) }

    Column(
        modifier = modifier.fillMaxSize()
    ) {
        // Header row (instead of TopAppBar)
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .background(colors.surfaceElevated)
                .padding(horizontal = AltairTheme.spacing.md, vertical = AltairTheme.spacing.sm),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            // Back button
            Box(
                modifier = Modifier
                    .clickable(onClick = onNavigateBack)
                    .padding(AltairTheme.spacing.sm)
            ) {
                BasicText(
                    text = "←",
                    style = typography.bodyLarge.copy(
                        color = colors.textPrimary
                    )
                )
            }

            // Edit button
            if (quest != null) {
                Box(
                    modifier = Modifier
                        .clickable(onClick = onEditQuest)
                        .padding(AltairTheme.spacing.sm)
                ) {
                    BasicText(
                        text = "✎",
                        style = typography.bodyLarge.copy(
                            color = colors.textPrimary
                        )
                    )
                }
            }
        }

        // Content area
        when {
            error != null -> {
                // Error state
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    BasicText(
                        text = error,
                        style = typography.bodyMedium.copy(
                            color = colors.error
                        )
                    )
                }
            }
            isLoading || quest == null -> {
                // Loading state
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    BasicText(
                        text = "Loading...",
                        style = typography.bodyMedium.copy(
                            color = colors.textSecondary
                        )
                    )
                }
            }
            else -> {
                // Content
                QuestDetailContent(
                    quest = quest,
                    hasActiveQuest = hasActiveQuest,
                    onStartQuest = {
                        if (hasActiveQuest && quest.status == QuestStatus.BACKLOG) {
                            showWipConflictDialog = true
                        } else {
                            onStartQuest()
                        }
                    },
                    onCompleteQuest = onCompleteQuest,
                    onAbandonQuest = onAbandonQuest,
                    onBackToBacklog = onBackToBacklog,
                    onReopenQuest = onReopenQuest,
                    onCheckpointToggle = onCheckpointToggle,
                    onAddCheckpoint = onAddCheckpoint,
                    modifier = Modifier.fillMaxSize()
                )
            }
        }
    }

    // WIP=1 conflict dialog
    if (showWipConflictDialog) {
        AltairDialog(
            onDismissRequest = { showWipConflictDialog = false },
            title = "Another Quest Active",
            content = {
                BasicText(
                    text = "You already have an active quest. Complete or abandon it before starting a new one.",
                    style = typography.bodyMedium.copy(
                        color = colors.textPrimary
                    )
                )
            },
            confirmButton = {
                AltairButton(
                    onClick = { showWipConflictDialog = false },
                    variant = ButtonVariant.Primary
                ) {
                    BasicText(
                        text = "OK",
                        style = TextStyle(
                            color = LocalAltairContentColor.current,
                            fontSize = typography.bodyMedium.fontSize
                        )
                    )
                }
            }
        )
    }
}

/**
 * Quest detail content layout.
 */
@Composable
private fun QuestDetailContent(
    quest: QuestResponse,
    hasActiveQuest: Boolean,
    onStartQuest: () -> Unit,
    onCompleteQuest: () -> Unit,
    onAbandonQuest: () -> Unit,
    onBackToBacklog: () -> Unit,
    onReopenQuest: () -> Unit,
    onCheckpointToggle: (String) -> Unit,
    onAddCheckpoint: () -> Unit,
    modifier: Modifier = Modifier
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography
    val scrollState = rememberScrollState()

    Column(
        modifier = modifier
            .fillMaxSize()
            .verticalScroll(scrollState)
            .padding(AltairTheme.spacing.md),
        verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.lg)
    ) {
        // Header section
        Column(
            verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
        ) {
            // Title and energy badge
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.Top
            ) {
                BasicText(
                    text = quest.title,
                    style = typography.headlineMedium.copy(
                        color = colors.textPrimary
                    ),
                    modifier = Modifier.weight(1f).padding(end = AltairTheme.spacing.sm)
                )

                EnergyBadge(energyCost = quest.energyCost)
            }

            // Status indicator
            val statusText = when (quest.status) {
                QuestStatus.BACKLOG -> "In Backlog"
                QuestStatus.ACTIVE -> "Active"
                QuestStatus.COMPLETED -> "Completed"
                QuestStatus.ABANDONED -> "Abandoned"
            }

            val statusColor = when (quest.status) {
                QuestStatus.BACKLOG -> colors.textTertiary
                QuestStatus.ACTIVE -> colors.accent
                QuestStatus.COMPLETED -> colors.success
                QuestStatus.ABANDONED -> colors.error
            }

            BasicText(
                text = statusText,
                style = typography.bodyMedium.copy(
                    color = statusColor
                )
            )
        }

        // Description section
        quest.description?.takeIf { it.isNotBlank() }?.let { description ->
            Column(
                verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
            ) {
                BasicText(
                    text = "Description",
                    style = typography.bodyMedium.copy(
                        color = colors.textSecondary
                    )
                )

                BasicText(
                    text = description,
                    style = typography.bodyMedium.copy(
                        color = colors.textPrimary
                    )
                )
            }
        }

        // Horizontal divider
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .height(1.dp)
                .background(colors.border)
        )

        // Checkpoints section
        if (quest.checkpoints.isNotEmpty() || quest.status != QuestStatus.COMPLETED) {
            CheckpointList(
                checkpoints = quest.checkpoints,
                onCheckpointToggle = onCheckpointToggle,
                onAddCheckpoint = onAddCheckpoint
            )

            // Horizontal divider
            Box(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(1.dp)
                    .background(colors.border)
            )
        }

        // Action buttons based on status
        ActionButtons(
            status = quest.status,
            hasActiveQuest = hasActiveQuest,
            onStartQuest = onStartQuest,
            onCompleteQuest = onCompleteQuest,
            onAbandonQuest = onAbandonQuest,
            onBackToBacklog = onBackToBacklog,
            onReopenQuest = onReopenQuest
        )
    }
}

/**
 * Action buttons section based on quest status.
 */
@Composable
private fun ActionButtons(
    status: QuestStatus,
    hasActiveQuest: Boolean,
    onStartQuest: () -> Unit,
    onCompleteQuest: () -> Unit,
    onAbandonQuest: () -> Unit,
    onBackToBacklog: () -> Unit,
    onReopenQuest: () -> Unit
) {
    val typography = AltairTheme.typography

    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
    ) {
        when (status) {
            QuestStatus.BACKLOG -> {
                AltairButton(
                    onClick = onStartQuest,
                    variant = ButtonVariant.Primary,
                    modifier = Modifier.fillMaxWidth()
                ) {
                    BasicText(
                        text = "Start Quest",
                        style = TextStyle(
                            color = LocalAltairContentColor.current,
                            fontSize = typography.bodyMedium.fontSize
                        )
                    )
                }
            }

            QuestStatus.ACTIVE -> {
                AltairButton(
                    onClick = onCompleteQuest,
                    variant = ButtonVariant.Primary,
                    modifier = Modifier.fillMaxWidth()
                ) {
                    BasicText(
                        text = "Complete Quest",
                        style = TextStyle(
                            color = LocalAltairContentColor.current,
                            fontSize = typography.bodyMedium.fontSize
                        )
                    )
                }

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
                ) {
                    AltairButton(
                        onClick = onBackToBacklog,
                        variant = ButtonVariant.Secondary,
                        modifier = Modifier.weight(1f)
                    ) {
                        BasicText(
                            text = "Back to Backlog",
                            style = TextStyle(
                                color = LocalAltairContentColor.current,
                                fontSize = typography.bodyMedium.fontSize
                            )
                        )
                    }

                    AltairButton(
                        onClick = onAbandonQuest,
                        variant = ButtonVariant.Secondary,
                        modifier = Modifier.weight(1f)
                    ) {
                        BasicText(
                            text = "Abandon",
                            style = TextStyle(
                                color = LocalAltairContentColor.current,
                                fontSize = typography.bodyMedium.fontSize
                            )
                        )
                    }
                }
            }

            QuestStatus.COMPLETED, QuestStatus.ABANDONED -> {
                AltairButton(
                    onClick = onReopenQuest,
                    variant = ButtonVariant.Secondary,
                    modifier = Modifier.fillMaxWidth()
                ) {
                    BasicText(
                        text = "Reopen Quest",
                        style = TextStyle(
                            color = LocalAltairContentColor.current,
                            fontSize = typography.bodyMedium.fontSize
                        )
                    )
                }
            }
        }
    }
}
