package com.getaltair.altair.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.getaltair.altair.navigation.HomeComponent

/**
 * Home screen UI.
 * Entry point showing Altair branding and navigation to other areas.
 */
@Composable
fun HomeScreen(component: HomeComponent) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Text(
            text = "Altair",
            style = MaterialTheme.typography.headlineLarge
        )
        Text(
            text = "Life Management Ecosystem",
            style = MaterialTheme.typography.bodyLarge,
            modifier = Modifier.padding(bottom = 32.dp)
        )
        Button(onClick = { component.onSettingsClicked() }) {
            Text("Settings")
        }
    }
}
