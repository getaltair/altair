// Generated from registry JSON. Do not edit by hand.

package com.altair.contracts

enum class EntityType(val wire: String) {
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
        fun fromWire(value: String): EntityType =
            entries.firstOrNull { it.wire == value }
                ?: error("Unknown EntityType: $value")
    }
}

enum class RelationType(val wire: String) {
    REFERENCES("references"),
    SUPPORTS("supports"),
    REQUIRES("requires"),
    RELATED_TO("related_to"),
    DEPENDS_ON("depends_on"),
    DUPLICATES("duplicates"),
    SIMILAR_TO("similar_to"),
    GENERATED_FROM("generated_from");

    companion object {
        fun fromWire(value: String): RelationType =
            entries.firstOrNull { it.wire == value }
                ?: error("Unknown RelationType: $value")
    }
}

enum class RelationSourceType(val wire: String) {
    USER("user"),
    AI("ai"),
    IMPORT("import"),
    RULE("rule"),
    MIGRATION("migration"),
    SYSTEM("system");

    companion object {
        fun fromWire(value: String): RelationSourceType =
            entries.firstOrNull { it.wire == value }
                ?: error("Unknown RelationSourceType: $value")
    }
}

enum class RelationStatusType(val wire: String) {
    ACCEPTED("accepted"),
    SUGGESTED("suggested"),
    DISMISSED("dismissed"),
    REJECTED("rejected"),
    EXPIRED("expired");

    companion object {
        fun fromWire(value: String): RelationStatusType =
            entries.firstOrNull { it.wire == value }
                ?: error("Unknown RelationStatusType: $value")
    }
}

enum class SyncStream(val wire: String) {
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
        fun fromWire(value: String): SyncStream =
            entries.firstOrNull { it.wire == value }
                ?: error("Unknown SyncStream: $value")
    }
}



// DTO data classes

data class EntityRef(

    val entityType: EntityType,

    val entityId: String

)



data class RelationRecord(

    val id: String,

    val from: EntityRef,

    val to: EntityRef,

    val relationType: RelationType,

    val sourceType: RelationSourceType,

    val status: RelationStatusType,

    val confidence: Double? = null,

    val evidence: Map<String, Any?> = emptyMap(),

    val createdAt: String,

    val updatedAt: String

)
