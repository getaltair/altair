package com.getaltair.altair.navigation

import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import com.getaltair.altair.ui.guidance.checkin.CheckinScreen
import com.getaltair.altair.ui.guidance.initiative.InitiativeDetailScreen
import com.getaltair.altair.ui.guidance.initiative.InitiativeListScreen
import com.getaltair.altair.ui.guidance.quest.QuestDetailScreen
import com.getaltair.altair.ui.guidance.routine.RoutineDetailScreen
import com.getaltair.altair.ui.guidance.routine.RoutineListScreen
import com.getaltair.altair.ui.guidance.today.TodayScreen
import com.getaltair.altair.ui.settings.SettingsScreen
import java.util.UUID

@Composable
fun AltairNavGraph(
    navController: NavHostController,
    startDestination: String = Screen.Today.route,
) {
    NavHost(
        navController = navController,
        startDestination = startDestination,
    ) {
        composable(Screen.Today.route) {
            TodayScreen(
                onNavigateToQuest = { id ->
                    navController.navigate(Screen.QuestDetail(id).route)
                },
                onNavigateToCheckin = {
                    navController.navigate(Screen.DailyCheckin.route)
                },
                onNavigateToTab = { route -> navigateToTab(navController, route) },
            )
        }

        composable(Screen.InitiativeList.route) {
            InitiativeListScreen(
                onNavigateToDetail = { id ->
                    navController.navigate(Screen.InitiativeDetail(id).route)
                },
                onNavigateToTab = { route -> navigateToTab(navController, route) },
            )
        }

        composable(
            route = Screen.InitiativeDetail.ROUTE,
            arguments = listOf(
                navArgument(Screen.InitiativeDetail.ARG_ID) { type = NavType.StringType },
            ),
        ) {
            InitiativeDetailScreen(
                onBack = { navController.popBackStack() },
            )
        }

        composable(
            route = Screen.QuestDetail.ROUTE,
            arguments = listOf(
                navArgument(Screen.QuestDetail.ARG_ID) { type = NavType.StringType },
            ),
        ) {
            QuestDetailScreen(
                onBack = { navController.popBackStack() },
            )
        }

        composable(Screen.RoutineList.route) {
            RoutineListScreen(
                onNavigateToDetail = { id ->
                    navController.navigate(Screen.RoutineDetail(id).route)
                },
                onNavigateToTab = { route -> navigateToTab(navController, route) },
            )
        }

        composable(
            route = Screen.RoutineDetail.ROUTE,
            arguments = listOf(
                navArgument(Screen.RoutineDetail.ARG_ID) { type = NavType.StringType },
            ),
        ) {
            RoutineDetailScreen(
                onBack = { navController.popBackStack() },
            )
        }

        composable(Screen.DailyCheckin.route) {
            CheckinScreen(
                onBack = { navController.popBackStack() },
            )
        }

        composable(Screen.Settings.route) {
            SettingsScreen(
                onNavigateToTab = { route -> navigateToTab(navController, route) },
            )
        }
    }
}

private fun navigateToTab(navController: NavHostController, route: String) {
    navController.navigate(route) {
        popUpTo(Screen.Today.route) { saveState = true }
        launchSingleTop = true
        restoreState = true
    }
}
