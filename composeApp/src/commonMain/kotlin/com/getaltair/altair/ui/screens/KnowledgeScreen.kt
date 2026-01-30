package com.getaltair.altair.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import com.getaltair.altair.navigation.KnowledgeComponent
import com.getaltair.altair.ui.theme.AltairColors
import com.getaltair.altair.ui.theme.AltairSpacing
import com.getaltair.altair.ui.theme.AltairTypography

/**
 * Knowledge module screen - notes and PKM.
 * MVP placeholder showing module purpose.
 */
@Composable
fun KnowledgeScreen(component: KnowledgeComponent) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(AltairColors.background)
            .padding(AltairSpacing.md),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        BasicText(
            text = "Knowledge",
            style = AltairTypography.displayLarge.copy(color = AltairColors.textPrimary)
        )
        Spacer(modifier = Modifier.height(AltairSpacing.sm))
        BasicText(
            text = "Personal Knowledge Management",
            style = AltairTypography.bodyLarge.copy(
                color = AltairColors.textSecondary,
                textAlign = TextAlign.Center
            )
        )
        Spacer(modifier = Modifier.height(AltairSpacing.xs))
        BasicText(
            text = "Markdown notes with wikilinks and backlinks",
            style = AltairTypography.bodyMedium.copy(
                color = AltairColors.textTertiary,
                textAlign = TextAlign.Center
            )
        )
    }
}
