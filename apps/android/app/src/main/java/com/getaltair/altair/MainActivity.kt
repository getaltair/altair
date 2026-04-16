package com.getaltair.altair

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.navigation.compose.rememberNavController
import com.getaltair.altair.data.sync.SyncCoordinator
import com.getaltair.altair.navigation.NavGraph
import com.getaltair.altair.navigation.Screen
import com.getaltair.altair.ui.MainScaffold
import com.getaltair.altair.ui.auth.AuthViewModel
import com.getaltair.altair.ui.theme.AltairTheme
import org.koin.android.ext.android.inject
import org.koin.androidx.viewmodel.ext.android.viewModel

class MainActivity : ComponentActivity() {
    private val syncCoordinator: SyncCoordinator by inject()
    private val authViewModel: AuthViewModel by viewModel()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            AltairTheme {
                val navController = rememberNavController()
                val isAuthenticated by authViewModel.isAuthenticated.collectAsState()

                LaunchedEffect(isAuthenticated) {
                    if (isAuthenticated) {
                        syncCoordinator.startSync()
                    } else {
                        navController.navigate(Screen.Login.route) {
                            popUpTo(0) { inclusive = true }
                        }
                    }
                }

                if (isAuthenticated) {
                    MainScaffold(navController = navController)
                } else {
                    NavGraph(
                        navController = navController,
                        startDestination = Screen.Login.route,
                    )
                }
            }
        }
    }
}
