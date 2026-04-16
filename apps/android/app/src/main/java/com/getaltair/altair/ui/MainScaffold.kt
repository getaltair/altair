package com.getaltair.altair.ui

import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.MenuBook
import androidx.compose.material.icons.filled.CloudOff
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Inventory2
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.navigation.NavDestination.Companion.hierarchy
import androidx.navigation.NavGraph.Companion.findStartDestination
import androidx.navigation.NavHostController
import androidx.navigation.compose.currentBackStackEntryAsState
import com.getaltair.altair.navigation.NavGraph
import com.getaltair.altair.navigation.Screen
import com.getaltair.altair.ui.sync.SyncStatusViewModel
import org.koin.androidx.compose.koinViewModel

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainScaffold(navController: NavHostController) {
    val syncVm: SyncStatusViewModel = koinViewModel()
    val isPending by syncVm.isPending.collectAsStateWithLifecycle()

    val tabs =
        listOf(
            TabItem(Screen.Today.route, "today_graph", Icons.Default.Home, "Today"),
            TabItem(Screen.Knowledge.route, "knowledge_graph", Icons.AutoMirrored.Filled.MenuBook, "Knowledge"),
            TabItem(Screen.Tracking.route, "tracking_graph", Icons.Default.Inventory2, "Tracking"),
            TabItem(Screen.Settings.route, "settings_graph", Icons.Default.Settings, "Settings"),
        )

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Altair") },
                actions = {
                    if (isPending) {
                        Icon(
                            imageVector = Icons.Default.CloudOff,
                            contentDescription = "Syncing",
                            tint = Color(0xFF8A9EA2),
                        )
                    }
                },
            )
        },
        bottomBar = {
            NavigationBar {
                val navBackStackEntry by navController.currentBackStackEntryAsState()
                val currentDestination = navBackStackEntry?.destination
                tabs.forEach { tab ->
                    val selected = currentDestination?.hierarchy?.any { it.route == tab.graphRoute } == true
                    NavigationBarItem(
                        selected = selected,
                        onClick = {
                            navController.navigate(tab.graphRoute) {
                                popUpTo(navController.graph.findStartDestination().id) {
                                    saveState = true
                                }
                                launchSingleTop = true
                                restoreState = true
                            }
                        },
                        icon = { Icon(tab.icon, contentDescription = tab.label) },
                        label = { Text(tab.label) },
                    )
                }
            }
        },
    ) { innerPadding ->
        NavGraph(
            navController = navController,
            startDestination = "today_graph",
            modifier = Modifier.padding(innerPadding),
        )
    }
}

data class TabItem(
    val route: String,
    val graphRoute: String,
    val icon: ImageVector,
    val label: String,
)
