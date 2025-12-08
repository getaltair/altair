//! Custom serde serialization helpers for SurrealDB and chrono types
//!
//! This module provides custom serializers for types that need special handling
//! when converting from Rust to TypeScript:
//!
//! - `Thing`: SurrealDB record IDs serialize to `{ tb: string, id: string }`
//! - `NaiveTime`: chrono time-of-day values serialize to `"HH:MM"` strings

use chrono::NaiveTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use surrealdb::sql::Thing;

/// Custom serialization for SurrealDB Thing type
///
/// Converts Thing to a JSON object with `tb` (table) and `id` fields.
/// This ensures TypeScript receives a structured object instead of a string.
///
/// # Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use surrealdb::sql::Thing;
///
/// #[derive(Serialize, Deserialize)]
/// struct MyStruct {
///     #[serde(with = "altair_db::schema::serde_helpers::thing_serde")]
///     record_id: Thing,
/// }
/// ```
pub mod thing_serde {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct ThingHelper {
        tb: String,
        id: String,
    }

    pub fn serialize<S>(thing: &Thing, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use surrealdb::sql::Id;

        // Extract the inner string value from the ID enum
        let id_str = match &thing.id {
            Id::Number(n) => n.to_string(),
            Id::String(s) => s.clone(),
            Id::Uuid(u) => u.to_string(),
            _ => {
                // For complex types (Array, Object, Generate, Range), convert to string
                // This is a simple fallback - in practice most IDs will be String or Number
                format!("{:?}", thing.id)
            }
        };

        let helper = ThingHelper {
            tb: thing.tb.clone(),
            id: id_str,
        };
        helper.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Thing, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = ThingHelper::deserialize(deserializer)?;
        // Use the From trait to construct Thing from (String, String)
        Ok(Thing::from((helper.tb, helper.id)))
    }
}

/// specta type definition for Thing
///
/// Ensures TypeScript sees Thing as `{ tb: string; id: string }`
#[cfg(feature = "specta")]
impl specta::Type for Thing {
    fn inline(_type_map: &mut specta::TypeMap, _generics: specta::Generics) -> specta::DataType {
        specta::DataType::Object(specta::ObjectType {
            generics: vec![],
            fields: specta::ObjectFields::Named(vec![
                (
                    "tb".into(),
                    specta::ObjectField {
                        optional: false,
                        flatten: false,
                        deprecated: None,
                        docs: "Table name".into(),
                        ty: Box::new(specta::DataType::Primitive(specta::PrimitiveType::String)),
                    },
                ),
                (
                    "id".into(),
                    specta::ObjectField {
                        optional: false,
                        flatten: false,
                        deprecated: None,
                        docs: "Record ID".into(),
                        ty: Box::new(specta::DataType::Primitive(specta::PrimitiveType::String)),
                    },
                ),
            ]),
            tag: None,
        })
    }

    fn reference(
        _type_map: &mut specta::TypeMap,
        _generics: &[specta::DataType],
    ) -> specta::reference::Reference {
        // Use inline representation for Thing
        specta::reference::Reference::Inline
    }
}

/// Custom serialization for chrono NaiveTime type
///
/// Converts NaiveTime to a "HH:MM" string format for TypeScript.
/// This provides a simple, human-readable time representation.
///
/// # Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use chrono::NaiveTime;
///
/// #[derive(Serialize, Deserialize)]
/// struct MyStruct {
///     #[serde(with = "altair_db::schema::serde_helpers::naive_time_serde")]
///     time: NaiveTime,
/// }
/// ```
pub mod naive_time_serde {
    use super::*;

    pub fn serialize<S>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = time.format("%H:%M").to_string();
        serializer.serialize_str(&formatted)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveTime::parse_from_str(&s, "%H:%M").map_err(serde::de::Error::custom)
    }
}

/// specta type definition for NaiveTime
///
/// Ensures TypeScript sees NaiveTime as a string with HH:MM format
#[cfg(feature = "specta")]
impl specta::Type for NaiveTime {
    fn inline(_type_map: &mut specta::TypeMap, _generics: specta::Generics) -> specta::DataType {
        specta::DataType::Primitive(specta::PrimitiveType::String)
    }

    fn reference(
        _type_map: &mut specta::TypeMap,
        _generics: &[specta::DataType],
    ) -> specta::reference::Reference {
        // Use inline representation for NaiveTime
        specta::reference::Reference::Inline
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;
    use serde_json;

    #[derive(Serialize, Deserialize)]
    struct TestThingStruct {
        #[serde(with = "thing_serde")]
        record_id: Thing,
    }

    #[derive(Serialize, Deserialize)]
    struct TestTimeStruct {
        #[serde(with = "naive_time_serde")]
        time: NaiveTime,
    }

    #[test]
    fn test_thing_serialization() {
        let thing = Thing::from(("quest".to_string(), "123".to_string()));
        let test_struct = TestThingStruct { record_id: thing };

        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"record_id":{"tb":"quest","id":"123"}}"#);
    }

    #[test]
    fn test_thing_deserialization() {
        use surrealdb::sql::Id;

        let json = r#"{"record_id":{"tb":"quest","id":"456"}}"#;
        let test_struct: TestThingStruct = serde_json::from_str(json).unwrap();

        assert_eq!(test_struct.record_id.tb, "quest");
        // Check that the ID is a string type with value "456"
        if let Id::String(s) = &test_struct.record_id.id {
            assert_eq!(s, "456");
        } else {
            panic!("Expected Id::String, got {:?}", test_struct.record_id.id);
        }
    }

    #[test]
    fn test_thing_roundtrip() {
        let original = Thing::from(("campaign".to_string(), "abc-def-123".to_string()));
        let test_struct = TestThingStruct {
            record_id: original.clone(),
        };

        let json = serde_json::to_string(&test_struct).unwrap();
        let deserialized: TestThingStruct = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.record_id.tb, original.tb);
        assert_eq!(
            deserialized.record_id.id.to_string(),
            original.id.to_string()
        );
    }

    #[test]
    fn test_naive_time_serialization() {
        let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
        let test_struct = TestTimeStruct { time };

        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"time":"14:30"}"#);
    }

    #[test]
    fn test_naive_time_deserialization() {
        let json = r#"{"time":"09:15"}"#;
        let test_struct: TestTimeStruct = serde_json::from_str(json).unwrap();

        assert_eq!(test_struct.time.hour(), 9);
        assert_eq!(test_struct.time.minute(), 15);
    }

    #[test]
    fn test_naive_time_roundtrip() {
        let original = NaiveTime::from_hms_opt(23, 45, 0).unwrap();
        let test_struct = TestTimeStruct { time: original };

        let json = serde_json::to_string(&test_struct).unwrap();
        let deserialized: TestTimeStruct = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.time, original);
    }

    #[test]
    fn test_naive_time_midnight() {
        let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let test_struct = TestTimeStruct { time: midnight };

        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"time":"00:00"}"#);
    }

    #[test]
    fn test_naive_time_noon() {
        let noon = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let test_struct = TestTimeStruct { time: noon };

        let json = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(json, r#"{"time":"12:00"}"#);
    }
}
