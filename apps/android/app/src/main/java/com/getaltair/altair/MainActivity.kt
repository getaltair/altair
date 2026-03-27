package com.getaltair.altair

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.navigation.compose.rememberNavController
import com.getaltair.altair.data.preferences.ThemePreferences
import com.getaltair.altair.navigation.AltairNavGraph
import com.getaltair.altair.ui.theme.AltairTheme
import org.koin.android.ext.android.inject

class MainActivity : ComponentActivity() {

    private val themePreferences: ThemePreferences by inject()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            val isDarkMode by themePreferences.isDarkMode.collectAsState(initial = isSystemInDarkTheme())
            AltairTheme(darkTheme = isDarkMode) {
                val navController = rememberNavController()
                AltairNavGraph(navController = navController)
            }
        }
    }
}
