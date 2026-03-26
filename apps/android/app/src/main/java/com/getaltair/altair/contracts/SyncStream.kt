// AUTO-GENERATED from registry/sync-streams.json — do not edit
package com.getaltair.altair.contracts

enum class SyncStream(val value: String) {
    MY_PROFILE("my_profile"),
    MY_MEMBERSHIPS("my_memberships"),
    MY_PERSONAL_DATA("my_personal_data"),
    MY_HOUSEHOLD_DATA("my_household_data"),
    MY_RELATIONS("my_relations"),
    MY_ATTACHMENT_METADATA("my_attachment_metadata"),
    INITIATIVE_DETAIL("initiative_detail"),
    NOTE_DETAIL("note_detail"),
    ITEM_HISTORY("item_history"),
    QUEST_DETAIL("quest_detail");

    companion object {
        val AUTO_SUBSCRIBED = listOf(
            MY_PROFILE,
            MY_MEMBERSHIPS,
            MY_PERSONAL_DATA,
            MY_HOUSEHOLD_DATA,
            MY_RELATIONS,
            MY_ATTACHMENT_METADATA,
        )
        val ON_DEMAND = listOf(
            INITIATIVE_DETAIL,
            NOTE_DETAIL,
            ITEM_HISTORY,
            QUEST_DETAIL,
        )
    }
}
