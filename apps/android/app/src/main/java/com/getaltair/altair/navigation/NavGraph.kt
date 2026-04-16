package com.getaltair.altair.navigation

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.navigation
import androidx.navigation.navArgument
import androidx.navigation.navDeepLink
import com.getaltair.altair.ui.auth.LoginScreen
import com.getaltair.altair.ui.auth.RegisterScreen
import com.getaltair.altair.ui.guidance.FocusSessionScreen
import com.getaltair.altair.ui.guidance.GuidanceScreen
import com.getaltair.altair.ui.guidance.InitiativeListScreen
import com.getaltair.altair.ui.guidance.QuestDetailScreen
import com.getaltair.altair.ui.guidance.QuestListScreen
import com.getaltair.altair.ui.guidance.RoutineListScreen
import com.getaltair.altair.ui.knowledge.NoteDetailScreen
import com.getaltair.altair.ui.knowledge.NoteListScreen
import com.getaltair.altair.ui.knowledge.QuickNoteScreen
import com.getaltair.altair.ui.settings.SettingsScreen
import com.getaltair.altair.ui.today.TodayScreen
import com.getaltair.altair.ui.tracking.CategoryScreen
import com.getaltair.altair.ui.tracking.ItemCreationScreen
import com.getaltair.altair.ui.tracking.ItemDetailScreen
import com.getaltair.altair.ui.tracking.LocationScreen
import com.getaltair.altair.ui.tracking.ShoppingListScreen
import com.getaltair.altair.ui.tracking.TrackingScreen

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
