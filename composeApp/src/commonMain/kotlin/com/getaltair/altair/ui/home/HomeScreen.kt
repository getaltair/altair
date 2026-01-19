package com.getaltair.altair.ui.home

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.getaltair.altair.ui.components.AltairSurface
import com.getaltair.altair.ui.components.AltairText
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Home screen displayed after successful authentication.
 *
 * This is a placeholder that will be expanded with actual app content
 * (Guidance, Knowledge, Tracking modules) as features are implemented.
 */
@Composable
fun HomeScreen(modifier: Modifier = Modifier) {
    AltairSurface(modifier = modifier.fillMaxSize()) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(AltairTheme.Spacing.lg),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
        ) {
            AltairText(
                text = "Welcome to Altair",
                style = AltairTheme.Typography.headlineLarge,
                color = AltairTheme.Colors.textPrimary,
            )
            AltairText(
                text = "You are signed in",
                style = AltairTheme.Typography.bodyLarge,
                color = AltairTheme.Colors.textSecondary,
                modifier = Modifier.padding(top = AltairTheme.Spacing.sm),
            )
        }
    }
}
