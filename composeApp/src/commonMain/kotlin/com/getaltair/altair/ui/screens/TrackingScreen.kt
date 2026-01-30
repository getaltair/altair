package com.getaltair.altair.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import com.getaltair.altair.navigation.TrackingComponent
import com.getaltair.altair.ui.theme.AltairColors
import com.getaltair.altair.ui.theme.AltairSpacing
import com.getaltair.altair.ui.theme.AltairTypography

/**
 * Tracking module screen - inventory management.
 * MVP placeholder showing module purpose.
 */
@Composable
fun TrackingScreen(component: TrackingComponent) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(AltairColors.background)
            .padding(AltairSpacing.md),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        BasicText(
            text = "Tracking",
            style = AltairTypography.displayLarge.copy(color = AltairColors.textPrimary)
        )
        Spacer(modifier = Modifier.height(AltairSpacing.sm))
        BasicText(
            text = "Inventory Management",
            style = AltairTypography.bodyLarge.copy(
                color = AltairColors.textSecondary,
                textAlign = TextAlign.Center
            )
        )
        Spacer(modifier = Modifier.height(AltairSpacing.xs))
        BasicText(
            text = "Know what you own and where it is",
            style = AltairTypography.bodyMedium.copy(
                color = AltairColors.textTertiary,
                textAlign = TextAlign.Center
            )
        )
    }
}
