package com.getaltair.altair.navigation

import java.util.UUID

sealed class Screen(val route: String) {
    data object Today : Screen("today")
    data object InitiativeList : Screen("initiatives")
    data class InitiativeDetail(val id: UUID) : Screen("initiatives/${id}") {
        companion object {
            const val ROUTE = "initiatives/{id}"
            const val ARG_ID = "id"
        }
    }

    data class QuestDetail(val id: UUID) : Screen("quests/${id}") {
        companion object {
            const val ROUTE = "quests/{id}"
            const val ARG_ID = "id"
        }
    }

    data object RoutineList : Screen("routines")
    data class RoutineDetail(val id: UUID) : Screen("routines/${id}") {
        companion object {
            const val ROUTE = "routines/{id}"
            const val ARG_ID = "id"
        }
    }

    data object DailyCheckin : Screen("checkin")
    data object Settings : Screen("settings")
}
