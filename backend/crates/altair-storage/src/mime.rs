//! MIME type validation module
//!
//! This module provides validation and classification of MIME types for storage operations.
//! Only specific, allowed MIME types can be uploaded to prevent abuse and ensure
//! proper handling of different media types.

use crate::error::{StorageError, StorageResult};
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Allowed MIME types for upload
///
/// Per spec.md FR-002, the following categories are allowed:
/// - Images: JPEG, PNG, GIF, WebP
/// - Documents: PDF, plain text, markdown
/// - Audio: MP3, WAV, OGG, WebM audio
pub static ALLOWED_MIME_TYPES: &[&str] = &[
    // Images
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
    // Documents
    "application/pdf",
    "text/plain",
    "text/markdown",
    // Audio
    "audio/mpeg",
    "audio/wav",
    "audio/ogg",
    "audio/webm",
];

/// File extension to MIME type mapping
///
/// Used to validate that file extensions match their claimed MIME types
/// and to infer MIME types when not provided.
static EXTENSION_TO_MIME: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // Images
    map.insert("jpg", "image/jpeg");
    map.insert("jpeg", "image/jpeg");
    map.insert("png", "image/png");
    map.insert("gif", "image/gif");
    map.insert("webp", "image/webp");
    // Documents
    map.insert("pdf", "application/pdf");
    map.insert("txt", "text/plain");
    map.insert("md", "text/markdown");
    map.insert("markdown", "text/markdown");
    // Audio
    map.insert("mp3", "audio/mpeg");
    map.insert("wav", "audio/wav");
    map.insert("ogg", "audio/ogg");
    map.insert("webm", "audio/webm");
    map
});

/// MIME type to file extension mapping (canonical extensions)
static MIME_TO_EXTENSION: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // Images
    map.insert("image/jpeg", "jpg");
    map.insert("image/png", "png");
    map.insert("image/gif", "gif");
    map.insert("image/webp", "webp");
    // Documents
    map.insert("application/pdf", "pdf");
    map.insert("text/plain", "txt");
    map.insert("text/markdown", "md");
    // Audio
    map.insert("audio/mpeg", "mp3");
    map.insert("audio/wav", "wav");
    map.insert("audio/ogg", "ogg");
    map.insert("audio/webm", "webm");
    map
});

/// Media type classification for attachments
///
/// Used to categorize files for display, thumbnail generation, and feature support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaType {
    /// Image files (JPEG, PNG, GIF, WebP)
    Photo,
    /// Audio files (MP3, WAV, OGG, WebM)
    Audio,
    /// Video files (not currently allowed, reserved for future)
    Video,
    /// Document files (PDF, text, markdown)
    Document,
    /// Unknown or unsupported type
    Other,
}

impl MediaType {
    /// Returns true if this media type supports thumbnail generation
    pub fn supports_thumbnail(&self) -> bool {
        matches!(self, MediaType::Photo | MediaType::Video)
    }

    /// Returns the media type as a string for database storage
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaType::Photo => "photo",
            MediaType::Audio => "audio",
            MediaType::Video => "video",
            MediaType::Document => "document",
            MediaType::Other => "other",
        }
    }
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MediaType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "photo" | "image" => Ok(MediaType::Photo),
            "audio" => Ok(MediaType::Audio),
            "video" => Ok(MediaType::Video),
            "document" | "doc" => Ok(MediaType::Document),
            "other" => Ok(MediaType::Other),
            _ => Err(format!("Unknown media type: {}", s)),
        }
    }
}

/// Validate that a MIME type is allowed for upload
///
/// # Arguments
/// * `mime_type` - The MIME type to validate (e.g., "image/jpeg")
///
/// # Returns
/// `Ok(())` if the MIME type is allowed, `Err(StorageError::InvalidMimeType)` otherwise
///
/// # Example
/// ```
/// use altair_storage::mime::validate_mime_type;
///
/// assert!(validate_mime_type("image/jpeg").is_ok());
/// assert!(validate_mime_type("application/javascript").is_err());
/// ```
pub fn validate_mime_type(mime_type: &str) -> StorageResult<()> {
    // Normalize the MIME type (lowercase, trim whitespace)
    let normalized = mime_type.trim().to_lowercase();

    if ALLOWED_MIME_TYPES.contains(&normalized.as_str()) {
        Ok(())
    } else {
        Err(StorageError::invalid_mime_type(&normalized))
    }
}

/// Classify a MIME type into a MediaType category
///
/// # Arguments
/// * `mime_type` - The MIME type to classify (e.g., "image/jpeg")
///
/// # Returns
/// The `MediaType` classification for the given MIME type
///
/// # Example
/// ```
/// use altair_storage::mime::{classify_media_type, MediaType};
///
/// assert_eq!(classify_media_type("image/jpeg"), MediaType::Photo);
/// assert_eq!(classify_media_type("application/pdf"), MediaType::Document);
/// assert_eq!(classify_media_type("audio/mpeg"), MediaType::Audio);
/// ```
pub fn classify_media_type(mime_type: &str) -> MediaType {
    let normalized = mime_type.trim().to_lowercase();

    if normalized.starts_with("image/") {
        MediaType::Photo
    } else if normalized.starts_with("audio/") {
        MediaType::Audio
    } else if normalized.starts_with("video/") {
        MediaType::Video
    } else if normalized == "application/pdf" || normalized.starts_with("text/") {
        MediaType::Document
    } else {
        MediaType::Other
    }
}

/// Get the MIME type for a file extension
///
/// # Arguments
/// * `extension` - The file extension without the dot (e.g., "jpg", "pdf")
///
/// # Returns
/// The MIME type for the extension, or `None` if unknown
///
/// # Example
/// ```
/// use altair_storage::mime::mime_type_for_extension;
///
/// assert_eq!(mime_type_for_extension("jpg"), Some("image/jpeg"));
/// assert_eq!(mime_type_for_extension("unknown"), None);
/// ```
pub fn mime_type_for_extension(extension: &str) -> Option<&'static str> {
    let ext = extension.trim().to_lowercase();
    EXTENSION_TO_MIME.get(ext.as_str()).copied()
}

/// Get the canonical file extension for a MIME type
///
/// # Arguments
/// * `mime_type` - The MIME type (e.g., "image/jpeg")
///
/// # Returns
/// The canonical extension for the MIME type, or `None` if unknown
///
/// # Example
/// ```
/// use altair_storage::mime::extension_for_mime_type;
///
/// assert_eq!(extension_for_mime_type("image/jpeg"), Some("jpg"));
/// assert_eq!(extension_for_mime_type("unknown/type"), None);
/// ```
pub fn extension_for_mime_type(mime_type: &str) -> Option<&'static str> {
    let normalized = mime_type.trim().to_lowercase();
    MIME_TO_EXTENSION.get(normalized.as_str()).copied()
}

/// Validate that a filename's extension matches the claimed MIME type
///
/// # Arguments
/// * `filename` - The filename including extension (e.g., "photo.jpg")
/// * `mime_type` - The claimed MIME type
///
/// # Returns
/// `true` if the extension matches the MIME type, `false` otherwise
///
/// # Example
/// ```
/// use altair_storage::mime::extension_matches_mime_type;
///
/// assert!(extension_matches_mime_type("photo.jpg", "image/jpeg"));
/// assert!(!extension_matches_mime_type("photo.jpg", "application/pdf"));
/// ```
pub fn extension_matches_mime_type(filename: &str, mime_type: &str) -> bool {
    // Extract extension from filename
    let extension = match filename.rsplit('.').next() {
        Some(ext) if ext != filename => ext,
        _ => return false, // No extension found
    };

    // Get expected MIME type for this extension
    let expected_mime = match mime_type_for_extension(extension) {
        Some(mime) => mime,
        None => return false, // Unknown extension
    };

    // Compare with claimed MIME type (case-insensitive)
    expected_mime.eq_ignore_ascii_case(mime_type.trim())
}

/// Get all allowed MIME types as a formatted string
///
/// Useful for error messages and documentation.
pub fn allowed_mime_types_string() -> String {
    ALLOWED_MIME_TYPES.join(", ")
}

/// Check if a MIME type is allowed (without returning an error)
///
/// # Arguments
/// * `mime_type` - The MIME type to check
///
/// # Returns
/// `true` if allowed, `false` otherwise
pub fn is_mime_type_allowed(mime_type: &str) -> bool {
    let normalized = mime_type.trim().to_lowercase();
    ALLOWED_MIME_TYPES.contains(&normalized.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_allowed_image_types() {
        assert!(validate_mime_type("image/jpeg").is_ok());
        assert!(validate_mime_type("image/png").is_ok());
        assert!(validate_mime_type("image/gif").is_ok());
        assert!(validate_mime_type("image/webp").is_ok());
    }

    #[test]
    fn test_validate_allowed_document_types() {
        assert!(validate_mime_type("application/pdf").is_ok());
        assert!(validate_mime_type("text/plain").is_ok());
        assert!(validate_mime_type("text/markdown").is_ok());
    }

    #[test]
    fn test_validate_allowed_audio_types() {
        assert!(validate_mime_type("audio/mpeg").is_ok());
        assert!(validate_mime_type("audio/wav").is_ok());
        assert!(validate_mime_type("audio/ogg").is_ok());
        assert!(validate_mime_type("audio/webm").is_ok());
    }

    #[test]
    fn test_validate_disallowed_types() {
        assert!(validate_mime_type("application/javascript").is_err());
        assert!(validate_mime_type("text/html").is_err());
        assert!(validate_mime_type("application/x-executable").is_err());
        assert!(validate_mime_type("video/mp4").is_err()); // video not allowed yet
    }

    #[test]
    fn test_validate_case_insensitive() {
        assert!(validate_mime_type("IMAGE/JPEG").is_ok());
        assert!(validate_mime_type("Image/Png").is_ok());
    }

    #[test]
    fn test_validate_trims_whitespace() {
        assert!(validate_mime_type("  image/jpeg  ").is_ok());
        assert!(validate_mime_type("\timage/png\n").is_ok());
    }

    #[test]
    fn test_classify_images() {
        assert_eq!(classify_media_type("image/jpeg"), MediaType::Photo);
        assert_eq!(classify_media_type("image/png"), MediaType::Photo);
        assert_eq!(classify_media_type("image/gif"), MediaType::Photo);
        assert_eq!(classify_media_type("image/webp"), MediaType::Photo);
    }

    #[test]
    fn test_classify_audio() {
        assert_eq!(classify_media_type("audio/mpeg"), MediaType::Audio);
        assert_eq!(classify_media_type("audio/wav"), MediaType::Audio);
        assert_eq!(classify_media_type("audio/ogg"), MediaType::Audio);
    }

    #[test]
    fn test_classify_video() {
        assert_eq!(classify_media_type("video/mp4"), MediaType::Video);
        assert_eq!(classify_media_type("video/webm"), MediaType::Video);
    }

    #[test]
    fn test_classify_documents() {
        assert_eq!(classify_media_type("application/pdf"), MediaType::Document);
        assert_eq!(classify_media_type("text/plain"), MediaType::Document);
        assert_eq!(classify_media_type("text/markdown"), MediaType::Document);
    }

    #[test]
    fn test_classify_other() {
        assert_eq!(classify_media_type("application/json"), MediaType::Other);
        assert_eq!(classify_media_type("unknown/type"), MediaType::Other);
    }

    #[test]
    fn test_mime_type_for_extension() {
        assert_eq!(mime_type_for_extension("jpg"), Some("image/jpeg"));
        assert_eq!(mime_type_for_extension("jpeg"), Some("image/jpeg"));
        assert_eq!(mime_type_for_extension("png"), Some("image/png"));
        assert_eq!(mime_type_for_extension("pdf"), Some("application/pdf"));
        assert_eq!(mime_type_for_extension("mp3"), Some("audio/mpeg"));
        assert_eq!(mime_type_for_extension("unknown"), None);
    }

    #[test]
    fn test_extension_for_mime_type() {
        assert_eq!(extension_for_mime_type("image/jpeg"), Some("jpg"));
        assert_eq!(extension_for_mime_type("image/png"), Some("png"));
        assert_eq!(extension_for_mime_type("application/pdf"), Some("pdf"));
        assert_eq!(extension_for_mime_type("audio/mpeg"), Some("mp3"));
        assert_eq!(extension_for_mime_type("unknown/type"), None);
    }

    #[test]
    fn test_extension_matches_mime_type() {
        assert!(extension_matches_mime_type("photo.jpg", "image/jpeg"));
        assert!(extension_matches_mime_type("photo.jpeg", "image/jpeg"));
        assert!(extension_matches_mime_type(
            "document.pdf",
            "application/pdf"
        ));
        assert!(extension_matches_mime_type("song.mp3", "audio/mpeg"));

        // Mismatches
        assert!(!extension_matches_mime_type("photo.jpg", "application/pdf"));
        assert!(!extension_matches_mime_type("document.pdf", "image/jpeg"));

        // No extension
        assert!(!extension_matches_mime_type("noextension", "image/jpeg"));
    }

    #[test]
    fn test_media_type_supports_thumbnail() {
        assert!(MediaType::Photo.supports_thumbnail());
        assert!(MediaType::Video.supports_thumbnail());
        assert!(!MediaType::Audio.supports_thumbnail());
        assert!(!MediaType::Document.supports_thumbnail());
        assert!(!MediaType::Other.supports_thumbnail());
    }

    #[test]
    fn test_media_type_as_str() {
        assert_eq!(MediaType::Photo.as_str(), "photo");
        assert_eq!(MediaType::Audio.as_str(), "audio");
        assert_eq!(MediaType::Video.as_str(), "video");
        assert_eq!(MediaType::Document.as_str(), "document");
        assert_eq!(MediaType::Other.as_str(), "other");
    }

    #[test]
    fn test_media_type_from_str() {
        assert_eq!("photo".parse::<MediaType>().unwrap(), MediaType::Photo);
        assert_eq!("image".parse::<MediaType>().unwrap(), MediaType::Photo);
        assert_eq!("audio".parse::<MediaType>().unwrap(), MediaType::Audio);
        assert_eq!("video".parse::<MediaType>().unwrap(), MediaType::Video);
        assert_eq!(
            "document".parse::<MediaType>().unwrap(),
            MediaType::Document
        );
        assert_eq!("doc".parse::<MediaType>().unwrap(), MediaType::Document);
        assert!("invalid".parse::<MediaType>().is_err());
    }

    #[test]
    fn test_is_mime_type_allowed() {
        assert!(is_mime_type_allowed("image/jpeg"));
        assert!(!is_mime_type_allowed("application/javascript"));
    }

    #[test]
    fn test_allowed_mime_types_string() {
        let types_str = allowed_mime_types_string();
        assert!(types_str.contains("image/jpeg"));
        assert!(types_str.contains("application/pdf"));
    }
}
