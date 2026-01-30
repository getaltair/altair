package com.getaltair.altair.ui.inbox

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.dto.system.InboxItemResponse
import com.getaltair.altair.shared.dto.system.TriageRequest
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.components.AltairButton
import com.getaltair.altair.ui.theme.components.AltairDialog
import com.getaltair.altair.ui.theme.components.AltairTextField
import com.getaltair.altair.ui.theme.components.ButtonVariant

/**
 * Dialog for triaging an inbox item into a specific entity type.
 *
 * Provides radio button selection for target type (Quest, Note, Item) and
 * displays relevant fields based on the selection. The title field is
 * pre-filled from the inbox item content.
 *
 * @param item The inbox item being triaged
 * @param onDismiss Callback when the dialog should be dismissed
 * @param onTriage Callback with the completed triage request
 * @param modifier Modifier to be applied to the dialog
 */
@Composable
fun TriageDialog(
    item: InboxItemResponse,
    onDismiss: () -> Unit,
    onTriage: (TriageRequest) -> Unit,
    modifier: Modifier = Modifier
) {
    var selectedType by remember { mutableStateOf("quest") }
    var title by remember { mutableStateOf(item.content.take(200)) }
    var energyCost by remember { mutableStateOf(3) }

    AltairDialog(
        onDismissRequest = onDismiss,
        title = "Triage Item",
        modifier = modifier,
        content = {
        // Target type selection
        BasicText(
            text = "Convert to:",
            style = AltairTheme.typography.labelSmall.copy(
                color = AltairTheme.colors.textSecondary
            ),
            modifier = Modifier.padding(bottom = AltairTheme.spacing.sm)
        )

        Column(
            verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
        ) {
            RadioOption(
                label = "Quest",
                description = "An actionable task",
                selected = selectedType == "quest",
                onClick = { selectedType = "quest" }
            )

            RadioOption(
                label = "Note",
                description = "A knowledge entry",
                selected = selectedType == "note",
                onClick = { selectedType = "note" }
            )

            RadioOption(
                label = "Item",
                description = "A physical item",
                selected = selectedType == "item",
                onClick = { selectedType = "item" }
            )
        }

        Spacer(modifier = Modifier.height(AltairTheme.spacing.lg))

        // Title field (always visible)
        AltairTextField(
            value = title,
            onValueChange = { title = it },
            label = "Title",
            modifier = Modifier.fillMaxWidth(),
            placeholder = "Enter title..."
        )

        // Type-specific fields
        when (selectedType) {
            "quest" -> {
                Spacer(modifier = Modifier.height(AltairTheme.spacing.md))

                // Energy cost picker
                BasicText(
                    text = "Energy Cost",
                    style = AltairTheme.typography.labelSmall.copy(
                        color = AltairTheme.colors.textSecondary
                    ),
                    modifier = Modifier.padding(bottom = AltairTheme.spacing.sm)
                )

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
                ) {
                    for (level in 1..5) {
                        EnergyLevelButton(
                            level = level,
                            selected = energyCost == level,
                            onClick = { energyCost = level }
                        )
                    }
                }
            }

            "note" -> {
                // Folder selection can be added in future iterations
                // For MVP, folderId remains null
            }

            "item" -> {
                // Location selection can be added in future iterations
                // For MVP, locationId remains null
            }
        }
        },
        confirmButton = {
            AltairButton(
                onClick = {
                    val request = TriageRequest(
                        targetType = selectedType,
                        title = title,
                        energyCost = if (selectedType == "quest") energyCost else null,
                        folderId = null, // Optional for MVP
                        locationId = null // Optional for MVP
                    )
                    onTriage(request)
                },
                variant = ButtonVariant.Primary,
                enabled = title.isNotBlank()
            ) {
                BasicText(
                    text = "Confirm",
                    style = TextStyle(color = androidx.compose.ui.graphics.Color.White)
                )
            }
        },
        dismissButton = {
            AltairButton(
                onClick = onDismiss,
                variant = ButtonVariant.Secondary
            ) {
                BasicText(
                    text = "Cancel",
                    style = TextStyle(color = AltairTheme.colors.textPrimary)
                )
            }
        }
    )
}

/**
 * Radio button option with label and description.
 */
@Composable
private fun RadioOption(
    label: String,
    description: String,
    selected: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(8.dp))
            .background(
                if (selected) AltairTheme.colors.surface else AltairTheme.colors.background
            )
            .border(
                width = 1.dp,
                color = if (selected) AltairTheme.colors.accent else AltairTheme.colors.border,
                shape = RoundedCornerShape(8.dp)
            )
            .clickable(onClick = onClick)
            .padding(AltairTheme.spacing.md),
        verticalAlignment = Alignment.CenterVertically
    ) {
        // Radio circle
        Box(
            modifier = Modifier
                .size(20.dp)
                .clip(CircleShape)
                .border(
                    width = 2.dp,
                    color = if (selected) AltairTheme.colors.accent else AltairTheme.colors.border,
                    shape = CircleShape
                ),
            contentAlignment = Alignment.Center
        ) {
            if (selected) {
                Box(
                    modifier = Modifier
                        .size(10.dp)
                        .clip(CircleShape)
                        .background(AltairTheme.colors.accent)
                )
            }
        }

        Spacer(modifier = Modifier.width(AltairTheme.spacing.md))

        Column {
            BasicText(
                text = label,
                style = AltairTheme.typography.bodyMedium.copy(
                    color = AltairTheme.colors.textPrimary
                )
            )
            BasicText(
                text = description,
                style = AltairTheme.typography.labelSmall.copy(
                    color = AltairTheme.colors.textSecondary
                )
            )
        }
    }
}

/**
 * Energy level button for Quest triage (1-5 scale).
 */
@Composable
private fun EnergyLevelButton(
    level: Int,
    selected: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    val backgroundColor = when {
        selected -> when (level) {
            1 -> AltairTheme.colors.energy1
            5 -> AltairTheme.colors.energy5
            else -> AltairTheme.colors.accent
        }
        else -> AltairTheme.colors.surface
    }

    Box(
        modifier = modifier
            .size(48.dp)
            .clip(RoundedCornerShape(8.dp))
            .background(backgroundColor)
            .border(
                width = 1.dp,
                color = if (selected) backgroundColor else AltairTheme.colors.border,
                shape = RoundedCornerShape(8.dp)
            )
            .clickable(onClick = onClick),
        contentAlignment = Alignment.Center
    ) {
        BasicText(
            text = level.toString(),
            style = AltairTheme.typography.bodyMedium.copy(
                color = if (selected) androidx.compose.ui.graphics.Color.White else AltairTheme.colors.textPrimary
            )
        )
    }
}
