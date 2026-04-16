package com.getaltair.altair.navigation

import android.util.Log
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.navigation
import androidx.navigation.navArgument
import androidx.navigation.navDeepLink
import com.getaltair.altair.data.local.dao.TrackingItemDao
import com.getaltair.altair.ui.auth.LoginScreen
import com.getaltair.altair.ui.auth.RegisterScreen
import com.getaltair.altair.ui.guidance.EpicDetailScreen
import com.getaltair.altair.ui.guidance.FocusSessionScreen
import com.getaltair.altair.ui.guidance.GuidanceScreen
import com.getaltair.altair.ui.guidance.InitiativeDetailScreen
import com.getaltair.altair.ui.guidance.InitiativeListScreen
import com.getaltair.altair.ui.guidance.QuestDetailScreen
import com.getaltair.altair.ui.guidance.QuestListScreen
import com.getaltair.altair.ui.guidance.RoutineListScreen
import com.getaltair.altair.ui.knowledge.NoteDetailScreen
import com.getaltair.altair.ui.knowledge.NoteListScreen
import com.getaltair.altair.ui.knowledge.QuickNoteScreen
import com.getaltair.altair.ui.settings.SettingsScreen
import com.getaltair.altair.ui.today.TodayScreen
import com.getaltair.altair.ui.tracking.BarcodeScannerScreen
import com.getaltair.altair.ui.tracking.CategoryScreen
import com.getaltair.altair.ui.tracking.ItemCreationScreen
import com.getaltair.altair.ui.tracking.ItemDetailScreen
import com.getaltair.altair.ui.tracking.LocationScreen
import com.getaltair.altair.ui.tracking.ShoppingListScreen
import com.getaltair.altair.ui.tracking.TrackingScreen
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.launch
import org.koin.compose.koinInject

@Composable
fun NavGraph(
    navController: NavHostController,
    startDestination: String,
    modifier: Modifier = Modifier,
) {
    NavHost(
        navController = navController,
        startDestination = startDestination,
        modifier = modifier,
    ) {
        // Auth graph
        navigation(
            startDestination = Screen.Login.route,
            route = "auth",
        ) {
            composable(Screen.Login.route) {
                LoginScreen(navController = navController)
            }
            composable(Screen.Register.route) {
                RegisterScreen(navController = navController)
            }
        }

        // Today graph (tab 1) — Guidance detail screens nested here (Today-as-hub, OQ-001)
        navigation(
            startDestination = Screen.Today.route,
            route = "today_graph",
        ) {
            composable(Screen.Today.route) {
                TodayScreen(navController = navController)
            }
            composable(
                route = Screen.QuestDetail.route,
                arguments = listOf(navArgument("id") { type = NavType.StringType }),
                deepLinks = listOf(navDeepLink { uriPattern = "altair://quest/{id}" }),
            ) { backStackEntry ->
                val questId = backStackEntry.arguments?.getString("id") ?: return@composable
                QuestDetailScreen(questId = questId, navController = navController)
            }
            composable(
                route = Screen.DailyCheckin.route,
                deepLinks = listOf(navDeepLink { uriPattern = "altair://checkin" }),
            ) {
                TodayScreen(navController = navController)
            }
            composable(route = "today_graph/guidance") {
                GuidanceScreen(navController = navController)
            }
            composable(route = "today_graph/guidance/quests") {
                QuestListScreen(navController = navController)
            }
            composable(
                route = "today_graph/guidance/quests/{id}/focus",
                arguments = listOf(navArgument("id") { type = NavType.StringType }),
            ) { backStackEntry ->
                val questId = backStackEntry.arguments?.getString("id") ?: return@composable
                FocusSessionScreen(questId = questId, navController = navController)
            }
            composable(route = "today_graph/guidance/initiatives") {
                InitiativeListScreen(navController = navController)
            }
            composable(route = "today_graph/guidance/routines") {
                RoutineListScreen(navController = navController)
            }
            composable(
                route = Screen.InitiativeDetail.route,
                arguments = listOf(navArgument("id") { type = NavType.StringType }),
            ) { backStackEntry ->
                InitiativeDetailScreen(navController = navController, backStackEntry = backStackEntry)
            }
            composable(
                route = Screen.EpicDetail.route,
                arguments =
                    listOf(
                        navArgument("initiativeId") { type = NavType.StringType },
                        navArgument("id") { type = NavType.StringType },
                    ),
            ) { backStackEntry ->
                EpicDetailScreen(navController = navController, backStackEntry = backStackEntry)
            }
        }

        // Knowledge graph (tab 2)
        navigation(
            startDestination = Screen.Knowledge.route,
            route = "knowledge_graph",
        ) {
            composable(Screen.Knowledge.route) {
                NoteListScreen(navController = navController)
            }
            composable(
                route = "note/{id}",
                arguments = listOf(navArgument("id") { type = NavType.StringType }),
            ) { backStackEntry ->
                val noteId = backStackEntry.arguments?.getString("id") ?: return@composable
                NoteDetailScreen(noteId = noteId, navController = navController)
            }
            composable(route = "quick_note") {
                QuickNoteScreen(navController = navController)
            }
        }

        // Tracking graph (tab 3)
        navigation(
            startDestination = Screen.Tracking.route,
            route = "tracking_graph",
        ) {
            composable(Screen.Tracking.route) {
                TrackingScreen(navController = navController)
            }
            composable(
                route = Screen.ItemDetail.route,
                arguments = listOf(navArgument("id") { type = NavType.StringType }),
                deepLinks = listOf(navDeepLink { uriPattern = "altair://item/{id}" }),
            ) { backStackEntry ->
                val itemId = backStackEntry.arguments?.getString("id") ?: return@composable
                ItemDetailScreen(itemId = itemId, navController = navController)
            }
            composable(route = "tracking/items/new") {
                ItemCreationScreen(navController = navController)
            }
            composable(
                route = "tracking/shopping-lists/{id}",
                arguments = listOf(navArgument("id") { type = NavType.StringType }),
            ) { backStackEntry ->
                val listId = backStackEntry.arguments?.getString("id") ?: return@composable
                ShoppingListScreen(shoppingListId = listId, navController = navController)
            }
            composable(route = "tracking/locations") {
                LocationScreen(navController = navController)
            }
            composable(route = "tracking/categories") {
                CategoryScreen(navController = navController)
            }
            composable(route = Screen.BarcodeScanner.route) {
                val trackingItemDao = koinInject<TrackingItemDao>()
                val scope = rememberCoroutineScope()
                val handled = remember { mutableStateOf(false) }

                BarcodeScannerScreen(
                    onBarcodeScanned = { barcode ->
                        if (handled.value) return@BarcodeScannerScreen
                        handled.value = true
                        scope.launch {
                            try {
                                val found = trackingItemDao.findByBarcode(barcode)
                                if (found != null) {
                                    navController.popBackStack()
                                    navController.navigate(Screen.ItemDetail.route(found.id))
                                } else {
                                    navController.previousBackStackEntry
                                        ?.savedStateHandle
                                        ?.set("scanned_barcode", barcode)
                                    navController.popBackStack()
                                }
                            } catch (e: CancellationException) {
                                throw e
                            } catch (e: Exception) {
                                Log.e("NavGraph", "Barcode lookup failed", e)
                                handled.value = false
                            }
                        }
                    },
                    onDismiss = { navController.popBackStack() },
                )
            }
        }

        // Settings graph (tab 4)
        navigation(
            startDestination = Screen.Settings.route,
            route = "settings_graph",
        ) {
            composable(Screen.Settings.route) {
                SettingsScreen(navController = navController)
            }
        }
    }
}
