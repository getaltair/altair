package com.getaltair.altair.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import com.getaltair.altair.ui.components.AltairSurface
import com.getaltair.altair.ui.components.AltairText
import com.getaltair.altair.ui.theme.AltairTheme
import com.getaltair.altair.ui.theme.AltairThemeProvider
import com.getaltair.altair.ui.theme.LocalAltairColors

/**
 * Error screen displayed when app initialization fails.
 * Provides user-friendly error message with technical details for debugging.
 */
@Composable
fun ErrorScreen(
    error: Throwable,
    modifier: Modifier = Modifier,
) {
    // Log the full error for debugging
    LaunchedEffect(error) {
        System.err.println("[ERROR] ErrorScreen displayed for: ${error::class.simpleName}: ${error.message}")
        error.printStackTrace(System.err)
    }

    AltairThemeProvider {
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
                AltairText(
                    text = "Failed to start Altair",
                    style = AltairTheme.Typography.headlineMedium,
                    color = colors.error,
                    textAlign = TextAlign.Center,
                )
                AltairText(
                    text = buildErrorDescription(error),
                    style = AltairTheme.Typography.bodyMedium,
                    color = colors.textPrimary,
                    textAlign = TextAlign.Center,
                    modifier = Modifier.padding(top = AltairTheme.Spacing.md),
                )
                AltairText(
                    text = "Please restart the app. If this persists, contact support.",
                    style = AltairTheme.Typography.bodySmall,
                    color = colors.textSecondary,
                    textAlign = TextAlign.Center,
                    modifier = Modifier.padding(top = AltairTheme.Spacing.lg),
                )
            }
        }
    }
}

/**
 * Builds a user-friendly error description that includes the error type
 * and message for debugging purposes.
 */
private fun buildErrorDescription(error: Throwable): String =
    buildString {
        // Include error type for technical users/support
        val errorType = error::class.simpleName ?: "Error"
        append(errorType)

        // Include message if available
        error.message?.let { message ->
            append(": ")
            append(message)
        }
    }
