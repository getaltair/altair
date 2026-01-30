package com.getaltair.altair.ui.guidance

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.getaltair.altair.shared.dto.guidance.CheckpointResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.headingSmall
import com.getaltair.altair.ui.theme.components.AltairButton
import com.getaltair.altair.ui.theme.components.ButtonVariant

/**
 * Checkpoint list component for quest detail view.
 *
 * Displays an ordered list of checkpoints with checkboxes for completion status.
 * Each checkpoint can be toggled on/off by clicking.
 * Includes an "Add Checkpoint" button at the bottom.
 *
 * @param checkpoints Ordered list of checkpoints
 * @param onCheckpointToggle Callback invoked when a checkpoint is toggled
 * @param onAddCheckpoint Callback invoked when "Add Checkpoint" is clicked
 * @param modifier Modifier to be applied to the column
 */
@Composable
fun CheckpointList(
    checkpoints: List<CheckpointResponse>,
    onCheckpointToggle: (String) -> Unit,
    onAddCheckpoint: () -> Unit,
    modifier: Modifier = Modifier
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography

    Column(
        modifier = modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        // Section header
        BasicText(
            text = "Checkpoints",
            style = typography.headingSmall.copy(
                color = colors.textPrimary
            ),
            modifier = Modifier.padding(bottom = 8.dp)
        )

        // Checkpoint items
        checkpoints.forEach { checkpoint ->
            CheckpointItem(
                checkpoint = checkpoint,
                onToggle = { onCheckpointToggle(checkpoint.id) }
            )
        }

        // Add checkpoint button
        AltairButton(
            onClick = onAddCheckpoint,
            variant = ButtonVariant.Ghost,
            modifier = Modifier.padding(top = 8.dp)
        ) {
            BasicText(
                text = "+ Add Checkpoint",
                style = TextStyle(
                    color = com.getaltair.altair.ui.theme.components.LocalAltairContentColor.current,
                    fontSize = typography.bodyMedium.fontSize
                )
            )
        }
    }
}

/**
 * Individual checkpoint item with checkbox and title.
 */
@Composable
private fun CheckpointItem(
    checkpoint: CheckpointResponse,
    onToggle: () -> Unit
) {
    val colors = AltairTheme.colors
    val typography = AltairTheme.typography

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onToggle)
            .padding(vertical = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        // Checkbox
        Checkbox(
            checked = checkpoint.completed,
            onCheckedChange = { onToggle() }
        )

        // Title
        BasicText(
            text = checkpoint.title,
            style = typography.bodyMedium.copy(
                color = if (checkpoint.completed) {
                    colors.textSecondary
                } else {
                    colors.textPrimary
                }
            ),
            modifier = Modifier.weight(1f)
        )
    }
}

/**
 * Simple checkbox component.
 */
@Composable
private fun Checkbox(
    checked: Boolean,
    onCheckedChange: () -> Unit
) {
    val colors = AltairTheme.colors
    val shape = RoundedCornerShape(4.dp)

    Box(
        modifier = Modifier
            .size(20.dp)
            .clip(shape)
            .background(
                if (checked) colors.accent else Color.Transparent
            )
            .border(
                width = 2.dp,
                color = if (checked) colors.accent else colors.border,
                shape = shape
            )
            .clickable(onClick = onCheckedChange),
        contentAlignment = Alignment.Center
    ) {
        if (checked) {
            // Checkmark using Unicode character
            BasicText(
                text = "✓",
                style = TextStyle(
                    color = Color.White,
                    fontSize = 12.dp.value.toInt().sp
                )
            )
        }
    }
}
