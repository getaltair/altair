package com.getaltair.altair.navigation

sealed class Screen(
    val route: String,
) {
    // Auth graph
    object Login : Screen("login")

    object Register : Screen("register")

    // Main graph — tab roots
    object Today : Screen("today")

    object Knowledge : Screen("knowledge")

    object Tracking : Screen("tracking")

    object Settings : Screen("settings")

    // Deep-linkable destinations
    object QuestDetail : Screen("quest/{id}") {
        fun route(id: String) = "quest/$id"
    }

    object ItemDetail : Screen("item/{id}") {
        fun route(id: String) = "item/$id"
    }

    object DailyCheckin : Screen("checkin")
}
