// AUTO-GENERATED from registry/relation-types.json — do not edit
package com.altair.contracts

enum class RelationType(val value: String) {
    REFERENCES("references"),
    SUPPORTS("supports"),
    REQUIRES("requires"),
    RELATED_TO("related_to"),
    DEPENDS_ON("depends_on"),
    DUPLICATES("duplicates"),
    SIMILAR_TO("similar_to"),
    GENERATED_FROM("generated_from");

    companion object {
        fun fromValue(value: String): RelationType =
            entries.first { it.value == value }
    }
}
