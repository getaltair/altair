//! Serde helper utilities for nuanced deserialization patterns.

use serde::{Deserialize, Deserializer};

/// Deserialize an `Option<Option<T>>` to distinguish three JSON states:
///
/// - Field absent -> outer `None` (do not touch the DB column)
/// - Field present as `null` -> `Some(None)` (set column to NULL)
/// - Field present with a value -> `Some(Some(val))` (set column to val)
///
/// Usage on struct fields:
/// ```ignore
/// #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_util::double_option")]
/// pub description: Option<Option<String>>,
/// ```
pub fn double_option<'de, T, D>(de: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(de).map(Some)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct TestStruct {
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "super::double_option"
        )]
        pub value: Option<Option<String>>,
    }

    #[test]
    fn absent_field_yields_outer_none() {
        let json = r#"{}"#;
        let parsed: TestStruct = serde_json::from_str(json).unwrap();
        assert!(parsed.value.is_none());
    }

    #[test]
    fn null_field_yields_some_none() {
        let json = r#"{"value": null}"#;
        let parsed: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.value, Some(None));
    }

    #[test]
    fn present_field_yields_some_some() {
        let json = r#"{"value": "hello"}"#;
        let parsed: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.value, Some(Some("hello".to_string())));
    }
}
