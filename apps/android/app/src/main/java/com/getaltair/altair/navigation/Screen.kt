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
        fun route(id: String): String {
            require(id.isNotBlank()) { "QuestDetail.route() requires a non-blank id" }
            return "quest/$id"
        }
    }

    object ItemDetail : Screen("item/{id}") {
        fun route(id: String): String {
            require(id.isNotBlank()) { "ItemDetail.route() requires a non-blank id" }
            return "item/$id"
        }
    }

    object DailyCheckin : Screen("checkin")

    object InitiativeDetail : Screen("today_graph/guidance/initiatives/{id}") {
        fun route(id: String): String {
            require(id.isNotBlank()) { "InitiativeDetail.route() requires a non-blank id" }
            return "today_graph/guidance/initiatives/$id"
        }
    }

    object EpicDetail : Screen("today_graph/guidance/initiatives/{initiativeId}/epics/{id}") {
        fun route(
            initiativeId: String,
            id: String,
        ): String {
            require(initiativeId.isNotBlank()) { "EpicDetail.route() requires a non-blank initiativeId" }
            require(id.isNotBlank()) { "EpicDetail.route() requires a non-blank id" }
            return "today_graph/guidance/initiatives/$initiativeId/epics/$id"
        }
    }

    object BarcodeScanner : Screen("barcode_scanner")
}
