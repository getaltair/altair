package com.getaltair.altair.navigation

import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import com.getaltair.altair.ui.capture.BarcodeScannerScreen
import com.getaltair.altair.ui.capture.CameraCaptureScreen
import com.getaltair.altair.ui.capture.VoiceNoteScreen
import com.getaltair.altair.ui.guidance.checkin.CheckinScreen
import com.getaltair.altair.ui.guidance.initiative.InitiativeDetailScreen
import com.getaltair.altair.ui.guidance.initiative.InitiativeListScreen
import com.getaltair.altair.ui.guidance.quest.QuestDetailScreen
import com.getaltair.altair.ui.guidance.routine.RoutineDetailScreen
import com.getaltair.altair.ui.guidance.routine.RoutineListScreen
import com.getaltair.altair.ui.guidance.today.TodayScreen
import com.getaltair.altair.ui.knowledge.NoteDetailScreen
import com.getaltair.altair.ui.knowledge.NoteEditorScreen
import com.getaltair.altair.ui.knowledge.NoteListScreen
import com.getaltair.altair.ui.settings.SettingsScreen
import com.getaltair.altair.ui.tracking.item.ItemDetailScreen
import com.getaltair.altair.ui.tracking.item.ItemListScreen
import com.getaltair.altair.ui.tracking.shopping.ShoppingListScreen
import com.getaltair.altair.ui.tracking.shopping.ShoppingListsScreen
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

        composable(Screen.NoteList.route) {
            NoteListScreen(
                onNavigateToDetail = { noteId ->
                    navController.navigate(Screen.NoteDetail(noteId).route)
                },
                onNavigateToEditor = { noteId ->
                    if (noteId != null) {
                        navController.navigate(Screen.NoteEditorEdit(noteId).route)
                    } else {
                        navController.navigate(Screen.NoteEditorCreate.route)
                    }
                },
                onNavigateToCamera = {
                    navController.navigate(Screen.CameraCapture.route)
                },
                onNavigateToTab = { route -> navigateToTab(navController, route) },
            )
        }

        composable(
            route = Screen.NoteDetail.ROUTE,
            arguments = listOf(
                navArgument(Screen.NoteDetail.ARG_NOTE_ID) { type = NavType.StringType },
            ),
        ) {
            NoteDetailScreen(
                onNavigateUp = { navController.popBackStack() },
                onNavigateToEditor = { noteId ->
                    navController.navigate(Screen.NoteEditorEdit(noteId).route)
                },
            )
        }

        composable(Screen.NoteEditorCreate.route) {
            NoteEditorScreen(
                noteId = null,
                onNavigateUp = { navController.popBackStack() },
            )
        }

        composable(
            route = Screen.NoteEditorEdit.ROUTE,
            arguments = listOf(
                navArgument(Screen.NoteEditorEdit.ARG_NOTE_ID) { type = NavType.StringType },
            ),
        ) { backStackEntry ->
            val noteId = backStackEntry.arguments?.getString(Screen.NoteEditorEdit.ARG_NOTE_ID)
            NoteEditorScreen(
                noteId = noteId,
                onNavigateUp = { navController.popBackStack() },
            )
        }

        composable(Screen.ItemList.route) {
            ItemListScreen(
                onNavigateToDetail = { itemId ->
                    navController.navigate(Screen.ItemDetail(itemId).route)
                },
                onNavigateToBarcodeScanner = {
                    navController.navigate(Screen.BarcodeScanner.route)
                },
                onNavigateToTab = { route -> navigateToTab(navController, route) },
            )
        }

        composable(
            route = Screen.ItemDetail.ROUTE,
            arguments = listOf(
                navArgument(Screen.ItemDetail.ARG_ITEM_ID) { type = NavType.StringType },
            ),
        ) {
            ItemDetailScreen(
                onNavigateUp = { navController.popBackStack() },
            )
        }

        composable(Screen.ShoppingLists.route) {
            ShoppingListsScreen(
                onNavigateToList = { listId ->
                    navController.navigate(Screen.ShoppingListDetail(listId).route)
                },
                onNavigateToTab = { route -> navigateToTab(navController, route) },
            )
        }

        composable(
            route = Screen.ShoppingListDetail.ROUTE,
            arguments = listOf(
                navArgument(Screen.ShoppingListDetail.ARG_LIST_ID) { type = NavType.StringType },
            ),
        ) {
            ShoppingListScreen(
                onNavigateUp = { navController.popBackStack() },
            )
        }

        composable(Screen.CameraCapture.route) {
            CameraCaptureScreen(
                onNavigateUp = { navController.popBackStack() },
                onPhotoCaptured = { navController.popBackStack() },
            )
        }

        composable(Screen.BarcodeScanner.route) {
            BarcodeScannerScreen(
                onBarcodeFound = { navController.popBackStack() },
                onNavigateUp = { navController.popBackStack() },
            )
        }

        composable(Screen.VoiceNote.route) {
            VoiceNoteScreen(
                onRecordingSaved = { navController.popBackStack() },
                onNavigateUp = { navController.popBackStack() },
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
