// AUTO-GENERATED from registry/entity-types.json — do not edit
package com.altair.contracts

enum class EntityType(val value: String) {
    USER("user"),
    HOUSEHOLD("household"),
    INITIATIVE("initiative"),
    TAG("tag"),
    ATTACHMENT("attachment"),
    GUIDANCE_EPIC("guidance_epic"),
    GUIDANCE_QUEST("guidance_quest"),
    GUIDANCE_ROUTINE("guidance_routine"),
    GUIDANCE_FOCUS_SESSION("guidance_focus_session"),
    GUIDANCE_DAILY_CHECKIN("guidance_daily_checkin"),
    KNOWLEDGE_NOTE("knowledge_note"),
    KNOWLEDGE_NOTE_SNAPSHOT("knowledge_note_snapshot"),
    TRACKING_LOCATION("tracking_location"),
    TRACKING_CATEGORY("tracking_category"),
    TRACKING_ITEM("tracking_item"),
    TRACKING_ITEM_EVENT("tracking_item_event"),
    TRACKING_SHOPPING_LIST("tracking_shopping_list"),
    TRACKING_SHOPPING_LIST_ITEM("tracking_shopping_list_item");

    companion object {
        fun fromValue(value: String): EntityType =
            entries.first { it.value == value }
    }
}
