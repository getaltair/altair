package com.getaltair.altair.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp

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
        println("ErrorScreen displayed for: ${error::class.simpleName}: ${error.message}")
        error.printStackTrace()
    }

    MaterialTheme {
        Column(
            modifier =
                modifier
                    .fillMaxSize()
                    .padding(24.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
        ) {
            Text(
                text = "Failed to start Altair",
                style = MaterialTheme.typography.headlineMedium,
                color = MaterialTheme.colorScheme.error,
                textAlign = TextAlign.Center,
            )
            Text(
                text = buildErrorDescription(error),
                style = MaterialTheme.typography.bodyMedium,
                modifier = Modifier.padding(top = 16.dp),
                textAlign = TextAlign.Center,
            )
            Text(
                text = "Please restart the app. If this persists, contact support.",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                modifier = Modifier.padding(top = 24.dp),
                textAlign = TextAlign.Center,
            )
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
