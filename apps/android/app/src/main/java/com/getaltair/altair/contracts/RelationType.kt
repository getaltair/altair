package com.getaltair.altair.contracts

// Source of truth: packages/contracts/relation-types.json

enum class RelationType(
    val value: String,
) {
    REFERENCES("references"),
    SUPPORTS("supports"),
    REQUIRES("requires"),
    RELATED_TO("related_to"),
    DEPENDS_ON("depends_on"),
    DUPLICATES("duplicates"),
    SIMILAR_TO("similar_to"),
    GENERATED_FROM("generated_from"),
    ;

    companion object {
        fun fromValue(value: String): RelationType? = entries.find { it.value == value }
    }
}
