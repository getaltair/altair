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

    data object NoteList : Screen("notes")
    data class NoteDetail(val noteId: String) : Screen("notes/${noteId}") {
        companion object {
            const val ROUTE = "notes/{noteId}"
            const val ARG_NOTE_ID = "noteId"
        }
    }
    data class NoteEditor(val noteId: String? = null) : Screen(
        if (noteId != null) "note_editor/${noteId}" else "note_editor"
    ) {
        companion object {
            const val ROUTE = "note_editor?noteId={noteId}"
            const val ARG_NOTE_ID = "noteId"
        }
    }

    data object ItemList : Screen("items")
    data class ItemDetail(val itemId: String) : Screen("items/${itemId}") {
        companion object {
            const val ROUTE = "items/{itemId}"
            const val ARG_ITEM_ID = "itemId"
        }
    }

    data object ShoppingLists : Screen("shopping_lists")
    data class ShoppingListDetail(val listId: String) : Screen("shopping_lists/${listId}") {
        companion object {
            const val ROUTE = "shopping_lists/{listId}"
            const val ARG_LIST_ID = "listId"
        }
    }

    data object CameraCapture : Screen("camera_capture")
    data object BarcodeScanner : Screen("barcode_scanner")
    data object VoiceNote : Screen("voice_note")
}
