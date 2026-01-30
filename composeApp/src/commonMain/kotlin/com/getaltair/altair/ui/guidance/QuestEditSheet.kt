package com.getaltair.altair.ui.guidance

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.getaltair.altair.shared.dto.guidance.QuestResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.headingMedium
import com.getaltair.altair.ui.theme.components.AltairBottomSheet
import com.getaltair.altair.ui.theme.components.AltairButton
import com.getaltair.altair.ui.theme.components.AltairChip
import com.getaltair.altair.ui.theme.components.AltairTextField
import com.getaltair.altair.ui.theme.components.ButtonVariant
import com.getaltair.altair.ui.theme.components.LocalAltairContentColor

/**
 * Bottom sheet for creating or editing a quest.
 *
 * Provides input fields for:
 * - Title (required)
 * - Description (optional, multiline)
 * - Energy cost (1-5 scale, chip picker)
 *
 * Works for both create and edit modes. When editing, initializes with existing quest data.
 *
 * @param existingQuest Quest to edit (null for create mode)
 * @param onDismiss Callback invoked when sheet is dismissed without saving
 * @param onSave Callback invoked with title, description, and energy cost when saved
 * @param modifier Modifier to be applied to the sheet
 */
@Composable
fun QuestEditSheet(
    existingQuest: QuestResponse? = null,
    onDismiss: () -> Unit,
    onSave: (title: String, description: String?, energyCost: Int) -> Unit,
    modifier: Modifier = Modifier
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography

    var title by remember { mutableStateOf(existingQuest?.title ?: "") }
    var description by remember { mutableStateOf(existingQuest?.description ?: "") }
    var energyCost by remember { mutableStateOf(existingQuest?.energyCost ?: 3) }

    val isValid = title.isNotBlank() && energyCost in 1..5

    AltairBottomSheet(
        onDismissRequest = onDismiss,
        modifier = modifier
    ) {
        Column(
            modifier = Modifier.fillMaxWidth(),
            verticalArrangement = Arrangement.spacedBy(20.dp)
        ) {
            // Header
            BasicText(
                text = if (existingQuest != null) "Edit Quest" else "New Quest",
                style = typography.headingMedium.copy(
                    color = colors.textPrimary
                )
            )

            // Title field
            AltairTextField(
                value = title,
                onValueChange = { title = it },
                label = "Title",
                placeholder = "What needs to be done?",
                singleLine = true,
                modifier = Modifier.fillMaxWidth()
            )

            // Description field
            AltairTextField(
                value = description,
                onValueChange = { description = it },
                label = "Description (optional)",
                placeholder = "Add more details...",
                singleLine = false,
                modifier = Modifier
                    .fillMaxWidth()
                    .height(120.dp)
            )

            // Energy cost picker
            Column(
                modifier = Modifier.fillMaxWidth(),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                BasicText(
                    text = "Energy Cost",
                    style = TextStyle(
                        color = colors.textSecondary,
                        fontSize = 12.dp.value.toInt().sp
                    )
                )

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    (1..5).forEach { level ->
                        AltairChip(
                            label = level.toString(),
                            selected = energyCost == level,
                            onClick = { energyCost = level },
                            modifier = Modifier.weight(1f)
                        )
                    }
                }
            }

            Spacer(modifier = Modifier.height(8.dp))

            // Action buttons
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                AltairButton(
                    onClick = onDismiss,
                    variant = ButtonVariant.Secondary,
                    modifier = Modifier.weight(1f)
                ) {
                    BasicText(
                        text = "Cancel",
                        style = TextStyle(
                            color = LocalAltairContentColor.current,
                            fontSize = typography.bodyMedium.fontSize
                        )
                    )
                }

                AltairButton(
                    onClick = {
                        if (isValid) {
                            onSave(
                                title.trim(),
                                description.takeIf { it.isNotBlank() },
                                energyCost
                            )
                        }
                    },
                    variant = ButtonVariant.Primary,
                    enabled = isValid,
                    modifier = Modifier.weight(1f)
                ) {
                    BasicText(
                        text = "Save",
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
