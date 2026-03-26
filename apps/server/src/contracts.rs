//! Canonical registry constants and validation helpers for Altair domain types.
//! These values are the single source of truth for all entity types, relation types,
//! source types, and status values used across the system.

pub const ENTITY_TYPES: &[&str] = &[
    "user",
    "household",
    "initiative",
    "tag",
    "attachment",
    "guidance_epic",
    "guidance_quest",
    "guidance_routine",
    "guidance_focus_session",
    "guidance_daily_checkin",
    "knowledge_note",
    "knowledge_note_snapshot",
    "tracking_location",
    "tracking_category",
    "tracking_item",
    "tracking_item_event",
    "tracking_shopping_list",
    "tracking_shopping_list_item",
];

pub const RELATION_TYPES: &[&str] = &[
    "references",
    "supports",
    "requires",
    "related_to",
    "depends_on",
    "duplicates",
    "similar_to",
    "generated_from",
];

pub const SOURCE_TYPES: &[&str] = &["user", "ai", "import", "rule", "migration", "system"];

pub const RELATION_STATUSES: &[&str] = &["accepted", "suggested", "dismissed", "rejected", "expired"];

pub const INITIATIVE_STATUSES: &[&str] = &["active", "paused", "completed", "archived"];

#[allow(dead_code)]
pub const ATTACHMENT_STATES: &[&str] = &["pending", "processing", "ready", "failed"];

pub fn is_valid_entity_type(s: &str) -> bool {
    ENTITY_TYPES.contains(&s)
}

pub fn is_valid_relation_type(s: &str) -> bool {
    RELATION_TYPES.contains(&s)
}

pub fn is_valid_source_type(s: &str) -> bool {
    SOURCE_TYPES.contains(&s)
}

pub fn is_valid_relation_status(s: &str) -> bool {
    RELATION_STATUSES.contains(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_entity_types_are_accepted() {
        assert!(is_valid_entity_type("user"));
        assert!(is_valid_entity_type("initiative"));
        assert!(is_valid_entity_type("knowledge_note"));
        assert!(is_valid_entity_type("tracking_item_event"));
    }

    #[test]
    fn invalid_entity_type_is_rejected() {
        assert!(!is_valid_entity_type("unknown_type"));
        assert!(!is_valid_entity_type(""));
        assert!(!is_valid_entity_type("USER"));
    }

    #[test]
    fn valid_relation_types_are_accepted() {
        assert!(is_valid_relation_type("references"));
        assert!(is_valid_relation_type("supports"));
        assert!(is_valid_relation_type("generated_from"));
    }

    #[test]
    fn invalid_relation_type_is_rejected() {
        assert!(!is_valid_relation_type("links_to"));
        assert!(!is_valid_relation_type(""));
    }

    #[test]
    fn valid_source_types_are_accepted() {
        assert!(is_valid_source_type("user"));
        assert!(is_valid_source_type("ai"));
        assert!(is_valid_source_type("system"));
    }

    #[test]
    fn invalid_source_type_is_rejected() {
        assert!(!is_valid_source_type("robot"));
        assert!(!is_valid_source_type(""));
    }

    #[test]
    fn valid_relation_statuses_are_accepted() {
        assert!(is_valid_relation_status("accepted"));
        assert!(is_valid_relation_status("suggested"));
        assert!(is_valid_relation_status("dismissed"));
        assert!(is_valid_relation_status("rejected"));
        assert!(is_valid_relation_status("expired"));
    }

    #[test]
    fn invalid_relation_status_is_rejected() {
        assert!(!is_valid_relation_status("pending"));
        assert!(!is_valid_relation_status(""));
    }
}
