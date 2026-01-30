package com.getaltair.altair.ui.inbox

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.getaltair.altair.shared.dto.system.InboxItemResponse
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.components.AltairButton
import com.getaltair.altair.ui.theme.components.ButtonVariant

/**
 * Universal Inbox main screen.
 *
 * Displays a list of inbox items with capture and triage functionality.
 * Each item can be clicked to initiate triage, and new items can be captured
 * via the floating action button.
 *
 * @param items List of inbox items to display
 * @param isLoading Whether data is currently being loaded
 * @param onCaptureClick Callback when the capture FAB is clicked
 * @param onItemClick Callback when an inbox item is clicked for triage
 * @param onDeleteItem Callback when an item should be deleted
 * @param modifier Modifier to be applied to the screen container
 */
@Composable
fun InboxScreen(
    items: List<InboxItemResponse>,
    isLoading: Boolean,
    onCaptureClick: () -> Unit,
    onItemClick: (InboxItemResponse) -> Unit,
    onDeleteItem: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    Box(
        modifier = modifier.fillMaxSize()
    ) {
        when {
            isLoading -> {
                // Loading state
                Column(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(AltairTheme.spacing.lg),
                    horizontalAlignment = Alignment.CenterHorizontally,
                    verticalArrangement = Arrangement.Center
                ) {
                    BasicText(
                        text = "Loading inbox...",
                        style = AltairTheme.typography.bodyMedium.copy(
                            color = AltairTheme.colors.textSecondary
                        )
                    )
                }
            }

            items.isEmpty() -> {
                // Empty state
                Column(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(AltairTheme.spacing.lg),
                    horizontalAlignment = Alignment.CenterHorizontally,
                    verticalArrangement = Arrangement.Center
                ) {
                    BasicText(
                        text = "Inbox is empty",
                        style = AltairTheme.typography.headlineMedium.copy(
                            color = AltairTheme.colors.textPrimary
                        ),
                        modifier = Modifier.padding(bottom = AltairTheme.spacing.sm)
                    )
                    BasicText(
                        text = "Capture ideas, tasks, and notes here.\nTap + to get started.",
                        style = AltairTheme.typography.bodyMedium.copy(
                            color = AltairTheme.colors.textSecondary,
                            textAlign = TextAlign.Center
                        )
                    )
                }
            }

            else -> {
                // List of inbox items
                LazyColumn(
                    modifier = Modifier.fillMaxSize(),
                    contentPadding = PaddingValues(
                        horizontal = AltairTheme.spacing.md,
                        vertical = AltairTheme.spacing.md
                    ),
                    verticalArrangement = Arrangement.spacedBy(AltairTheme.spacing.sm)
                ) {
                    items(
                        items = items,
                        key = { it.id }
                    ) { item ->
                        InboxItemCard(
                            item = item,
                            onClick = { onItemClick(item) },
                            onDelete = { onDeleteItem(item.id) }
                        )
                    }

                    // Bottom padding for FAB
                    item {
                        Spacer(modifier = Modifier.height(80.dp))
                    }
                }
            }
        }

        // Floating Action Button (FAB)
        AltairButton(
            onClick = onCaptureClick,
            variant = ButtonVariant.Primary,
            modifier = Modifier
                .align(Alignment.BottomEnd)
                .padding(AltairTheme.spacing.lg)
        ) {
            BasicText(
                text = "+ Capture",
                style = TextStyle(color = androidx.compose.ui.graphics.Color.White)
            )
        }
    }
}
