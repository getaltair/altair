package com.getaltair.altair.ui.knowledge

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.MenuBook
import androidx.compose.material3.Icon
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.getaltair.altair.ui.components.AltairSurface
import com.getaltair.altair.ui.components.AltairText
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.LocalAltairColors

/**
 * Knowledge screen - note and information management.
 *
 * This is a placeholder that will be expanded with:
 * - Note list and search
 * - Note creation and editing
 * - Tag organization
 * - Knowledge graph visualization
 */
@Composable
fun KnowledgeScreen(modifier: Modifier = Modifier) {
    val colors = LocalAltairColors.current

    AltairSurface(modifier = modifier.fillMaxSize()) {
        Column(
            modifier =
                Modifier
                    .fillMaxSize()
                    .padding(AltairTheme.Spacing.lg),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
        ) {
            Icon(
                imageVector = Icons.AutoMirrored.Filled.MenuBook,
                contentDescription = null,
                tint = colors.accent,
                modifier = Modifier.padding(bottom = AltairTheme.Spacing.md),
            )
            AltairText(
                text = "Knowledge",
                style = AltairTheme.Typography.headlineLarge,
                color = colors.textPrimary,
            )
            AltairText(
                text = "Notes and information at your fingertips",
                style = AltairTheme.Typography.bodyLarge,
                color = colors.textSecondary,
                modifier = Modifier.padding(top = AltairTheme.Spacing.sm),
            )
            AltairText(
                text = "Coming soon",
                style = AltairTheme.Typography.labelMedium,
                color = colors.textTertiary,
                modifier = Modifier.padding(top = AltairTheme.Spacing.lg),
            )
        }
    }
}
