package com.getaltair.altair

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.navigation.compose.rememberNavController
import com.getaltair.altair.navigation.AltairNavGraph
import com.getaltair.altair.ui.theme.AltairTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            AltairTheme {
                val navController = rememberNavController()
                AltairNavGraph(navController = navController)
            }
        }
    }
}
