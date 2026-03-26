// AUTO-GENERATED from registry/relation-sources.json — do not edit
package com.getaltair.altair.contracts

enum class RelationSource(val value: String) {
    USER("user"),
    AI("ai"),
    IMPORT("import"),
    RULE("rule"),
    MIGRATION("migration"),
    SYSTEM("system");

    companion object {
        fun fromValue(value: String): RelationSource =
            entries.first { it.value == value }
    }
}
