//! Quota management module
//!
//! This module provides per-user storage quota tracking and enforcement.
//! It manages bytes_used, bytes_limit, and provides reconciliation with
//! actual S3 usage.
//!
//! The quota system ensures users don't exceed their allocated storage
//! and provides accurate usage information for the UI.

use crate::client::S3Client;
use crate::error::{StorageError, StorageResult};
use crate::service::DEFAULT_QUOTA_BYTES;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::sql::Thing;
use tracing::instrument;

/// Default quota limit for new users (5GB)
pub const DEFAULT_BYTES_LIMIT: u64 = DEFAULT_QUOTA_BYTES;

/// Reconciliation threshold - if drift exceeds this percentage, update the quota
pub const RECONCILIATION_DRIFT_THRESHOLD: f64 = 0.01; // 1%

/// Quota information returned to users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaInfo {
    /// Current bytes used by the user
    pub bytes_used: u64,
    /// Maximum allowed bytes for the user
    pub bytes_limit: u64,
    /// Bytes available for additional uploads
    pub bytes_available: u64,
    /// Percentage of quota used (0.0 - 100.0)
    pub percentage_used: f64,
    /// Timestamp of last reconciliation with actual S3 usage
    pub last_reconciled: Option<DateTime<Utc>>,
}

impl QuotaInfo {
    /// Create a new QuotaInfo from bytes_used and bytes_limit
    pub fn new(bytes_used: u64, bytes_limit: u64, last_reconciled: Option<DateTime<Utc>>) -> Self {
        let bytes_available = bytes_limit.saturating_sub(bytes_used);
        let percentage_used = if bytes_limit > 0 {
            (bytes_used as f64 / bytes_limit as f64) * 100.0
        } else {
            100.0
        };

        Self {
            bytes_used,
            bytes_limit,
            bytes_available,
            percentage_used,
            last_reconciled,
        }
    }

    /// Create default quota info for a new user
    pub fn default_for_new_user() -> Self {
        Self::new(0, DEFAULT_BYTES_LIMIT, None)
    }

    /// Check if adding the specified bytes would exceed the quota
    pub fn would_exceed(&self, additional_bytes: u64) -> bool {
        self.bytes_used.saturating_add(additional_bytes) > self.bytes_limit
    }

    /// Check if user is at or above warning threshold (80%)
    pub fn is_at_warning_threshold(&self) -> bool {
        self.percentage_used >= 80.0
    }
}

/// Storage quota record from the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageQuota {
    /// Record ID
    pub id: Option<Thing>,
    /// Owner (user reference)
    pub owner: Thing,
    /// Current bytes used
    pub bytes_used: i64,
    /// Maximum allowed bytes
    pub bytes_limit: i64,
    /// Timestamp of last reconciliation
    pub last_reconciled: Option<DateTime<Utc>>,
    /// Device ID for sync
    pub device_id: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl StorageQuota {
    /// Convert to QuotaInfo for API responses
    pub fn to_info(&self) -> QuotaInfo {
        QuotaInfo::new(
            self.bytes_used.max(0) as u64,
            self.bytes_limit.max(0) as u64,
            self.last_reconciled,
        )
    }
}

/// Get quota for a user, creating default quota if none exists
///
/// # Arguments
/// * `db` - SurrealDB connection
/// * `user_id` - The user's record Thing
/// * `device_id` - Device ID for sync (used when creating new quota)
///
/// # Returns
/// `QuotaInfo` with current usage and limits
#[instrument(skip(db), fields(user_id = %user_id))]
pub async fn get_quota<C: surrealdb::Connection>(
    db: &Surreal<C>,
    user_id: &Thing,
    device_id: &str,
) -> StorageResult<QuotaInfo> {
    // Try to get existing quota
    let mut result = db
        .query("SELECT * FROM storage_quota WHERE owner = $owner")
        .bind(("owner", user_id.clone()))
        .await
        .map_err(|e| StorageError::s3("get_quota", format!("Database query failed: {}", e)))?;

    let quotas: Vec<StorageQuota> = result
        .take(0)
        .map_err(|e| StorageError::s3("get_quota", format!("Failed to deserialize: {}", e)))?;

    if let Some(quota) = quotas.into_iter().next() {
        tracing::debug!(
            bytes_used = quota.bytes_used,
            bytes_limit = quota.bytes_limit,
            "Found existing quota"
        );
        return Ok(quota.to_info());
    }

    // Create default quota for new user
    tracing::info!(user_id = %user_id, "Creating default quota for new user");
    create_default_quota(db, user_id, device_id).await
}

/// Create default quota for a new user
///
/// # Arguments
/// * `db` - SurrealDB connection
/// * `user_id` - The user's record Thing
/// * `device_id` - Device ID for sync
#[instrument(skip(db), fields(user_id = %user_id))]
async fn create_default_quota<C: surrealdb::Connection>(
    db: &Surreal<C>,
    user_id: &Thing,
    device_id: &str,
) -> StorageResult<QuotaInfo> {
    let now = Utc::now();
    let quota = StorageQuota {
        id: None,
        owner: user_id.clone(),
        bytes_used: 0,
        bytes_limit: DEFAULT_BYTES_LIMIT as i64,
        last_reconciled: None,
        device_id: device_id.to_string(),
        created_at: now,
        updated_at: now,
    };

    let created: Option<StorageQuota> =
        db.create("storage_quota")
            .content(quota)
            .await
            .map_err(|e| {
                StorageError::s3(
                    "create_quota",
                    format!("Failed to create quota record: {}", e),
                )
            })?;

    match created {
        Some(q) => {
            tracing::info!(
                bytes_limit = q.bytes_limit,
                "Created default quota for user"
            );
            Ok(q.to_info())
        }
        None => {
            tracing::warn!("Quota creation returned None, returning default");
            Ok(QuotaInfo::default_for_new_user())
        }
    }
}

/// Check if a user has sufficient quota for an upload
///
/// # Arguments
/// * `db` - SurrealDB connection
/// * `user_id` - The user's record Thing
/// * `requested_bytes` - Size of the file to be uploaded
/// * `device_id` - Device ID for sync
///
/// # Returns
/// `Ok(())` if quota is available, `Err(StorageError::QuotaExceeded)` otherwise
#[instrument(skip(db), fields(user_id = %user_id, requested_bytes = requested_bytes))]
pub async fn check_quota<C: surrealdb::Connection>(
    db: &Surreal<C>,
    user_id: &Thing,
    requested_bytes: u64,
    device_id: &str,
) -> StorageResult<()> {
    let quota = get_quota(db, user_id, device_id).await?;

    if quota.would_exceed(requested_bytes) {
        tracing::warn!(
            bytes_used = quota.bytes_used,
            bytes_limit = quota.bytes_limit,
            requested_bytes = requested_bytes,
            "Quota exceeded"
        );
        return Err(StorageError::quota_exceeded(
            quota.bytes_used,
            quota.bytes_limit,
            requested_bytes,
        ));
    }

    tracing::debug!(
        bytes_available = quota.bytes_available,
        requested_bytes = requested_bytes,
        "Quota check passed"
    );
    Ok(())
}

/// Update quota after upload (increment bytes_used)
///
/// # Arguments
/// * `db` - SurrealDB connection
/// * `user_id` - The user's record Thing
/// * `bytes_added` - Size of the uploaded file
///
/// # Returns
/// Updated `QuotaInfo`
#[instrument(skip(db), fields(user_id = %user_id, bytes_added = bytes_added))]
pub async fn increment_quota<C: surrealdb::Connection>(
    db: &Surreal<C>,
    user_id: &Thing,
    bytes_added: u64,
) -> StorageResult<QuotaInfo> {
    let mut result = db
        .query(
            "UPDATE storage_quota SET bytes_used = bytes_used + $bytes, updated_at = time::now() WHERE owner = $owner RETURN AFTER"
        )
        .bind(("bytes", bytes_added as i64))
        .bind(("owner", user_id.clone()))
        .await
        .map_err(|e| StorageError::s3("increment_quota", format!("Database update failed: {}", e)))?;

    let quotas: Vec<StorageQuota> = result.take(0).map_err(|e| {
        StorageError::s3("increment_quota", format!("Failed to deserialize: {}", e))
    })?;

    match quotas.into_iter().next() {
        Some(quota) => {
            tracing::info!(
                bytes_added = bytes_added,
                new_bytes_used = quota.bytes_used,
                "Quota incremented"
            );
            Ok(quota.to_info())
        }
        None => Err(StorageError::s3(
            "increment_quota",
            "No quota record found for user",
        )),
    }
}

/// Update quota after delete (decrement bytes_used)
///
/// # Arguments
/// * `db` - SurrealDB connection
/// * `user_id` - The user's record Thing
/// * `bytes_removed` - Size of the deleted file
///
/// # Returns
/// Updated `QuotaInfo`
#[instrument(skip(db), fields(user_id = %user_id, bytes_removed = bytes_removed))]
pub async fn decrement_quota<C: surrealdb::Connection>(
    db: &Surreal<C>,
    user_id: &Thing,
    bytes_removed: u64,
) -> StorageResult<QuotaInfo> {
    // Use GREATEST to prevent negative values
    let mut result = db
        .query(
            "UPDATE storage_quota SET bytes_used = math::max([0, bytes_used - $bytes]), updated_at = time::now() WHERE owner = $owner RETURN AFTER"
        )
        .bind(("bytes", bytes_removed as i64))
        .bind(("owner", user_id.clone()))
        .await
        .map_err(|e| StorageError::s3("decrement_quota", format!("Database update failed: {}", e)))?;

    let quotas: Vec<StorageQuota> = result.take(0).map_err(|e| {
        StorageError::s3("decrement_quota", format!("Failed to deserialize: {}", e))
    })?;

    match quotas.into_iter().next() {
        Some(quota) => {
            tracing::info!(
                bytes_removed = bytes_removed,
                new_bytes_used = quota.bytes_used,
                "Quota decremented"
            );
            Ok(quota.to_info())
        }
        None => Err(StorageError::s3(
            "decrement_quota",
            "No quota record found for user",
        )),
    }
}

/// Update quota with an absolute bytes_used value
///
/// Used by reconciliation to correct drift between tracked and actual usage.
///
/// # Arguments
/// * `db` - SurrealDB connection
/// * `user_id` - The user's record Thing
/// * `bytes_used` - New absolute bytes_used value
///
/// # Returns
/// Updated `QuotaInfo`
#[instrument(skip(db), fields(user_id = %user_id, bytes_used = bytes_used))]
pub async fn set_quota_bytes_used<C: surrealdb::Connection>(
    db: &Surreal<C>,
    user_id: &Thing,
    bytes_used: u64,
) -> StorageResult<QuotaInfo> {
    let mut result = db
        .query(
            "UPDATE storage_quota SET bytes_used = $bytes, last_reconciled = time::now(), updated_at = time::now() WHERE owner = $owner RETURN AFTER"
        )
        .bind(("bytes", bytes_used as i64))
        .bind(("owner", user_id.clone()))
        .await
        .map_err(|e| StorageError::s3("set_quota_bytes_used", format!("Database update failed: {}", e)))?;

    let quotas: Vec<StorageQuota> = result.take(0).map_err(|e| {
        StorageError::s3(
            "set_quota_bytes_used",
            format!("Failed to deserialize: {}", e),
        )
    })?;

    match quotas.into_iter().next() {
        Some(quota) => {
            tracing::info!(bytes_used = bytes_used, "Quota bytes_used set directly");
            Ok(quota.to_info())
        }
        None => Err(StorageError::s3(
            "set_quota_bytes_used",
            "No quota record found for user",
        )),
    }
}

/// Reconcile quota with actual S3 usage
///
/// Lists all objects owned by the user in S3, sums their sizes, and updates
/// the quota if drift exceeds the threshold (1%).
///
/// # Arguments
/// * `db` - SurrealDB connection
/// * `client` - S3 client for listing objects
/// * `user_id` - The user's record Thing (table:id format used as S3 prefix)
/// * `user_prefix` - The S3 key prefix for this user's objects (e.g., "user:abc123/")
///
/// # Returns
/// Updated `QuotaInfo` after reconciliation
#[instrument(skip(db, client), fields(user_id = %user_id, user_prefix = user_prefix))]
pub async fn reconcile_quota<C: surrealdb::Connection>(
    db: &Surreal<C>,
    client: &S3Client,
    user_id: &Thing,
    user_prefix: &str,
) -> StorageResult<QuotaInfo> {
    // List all objects with user's prefix
    let actual_bytes = list_user_storage_usage(client, user_prefix).await?;

    // Get current quota
    let mut result = db
        .query("SELECT * FROM storage_quota WHERE owner = $owner")
        .bind(("owner", user_id.clone()))
        .await
        .map_err(|e| {
            StorageError::s3("reconcile_quota", format!("Database query failed: {}", e))
        })?;

    let quotas: Vec<StorageQuota> = result.take(0).map_err(|e| {
        StorageError::s3("reconcile_quota", format!("Failed to deserialize: {}", e))
    })?;

    let current_quota = quotas
        .into_iter()
        .next()
        .ok_or_else(|| StorageError::s3("reconcile_quota", "No quota record found for user"))?;

    let tracked_bytes = current_quota.bytes_used.max(0) as u64;

    // Calculate drift
    let drift = if tracked_bytes > 0 {
        ((actual_bytes as f64 - tracked_bytes as f64) / tracked_bytes as f64).abs()
    } else if actual_bytes > 0 {
        1.0 // 100% drift if we thought it was 0 but it's not
    } else {
        0.0 // Both are 0, no drift
    };

    tracing::info!(
        tracked_bytes = tracked_bytes,
        actual_bytes = actual_bytes,
        drift_percentage = drift * 100.0,
        "Reconciliation check"
    );

    // Update if drift exceeds threshold
    if drift > RECONCILIATION_DRIFT_THRESHOLD {
        tracing::warn!(
            tracked_bytes = tracked_bytes,
            actual_bytes = actual_bytes,
            drift = drift * 100.0,
            "Quota drift detected, updating"
        );
        set_quota_bytes_used(db, user_id, actual_bytes).await
    } else {
        // Just update the last_reconciled timestamp
        let mut result = db
            .query(
                "UPDATE storage_quota SET last_reconciled = time::now(), updated_at = time::now() WHERE owner = $owner RETURN AFTER"
            )
            .bind(("owner", user_id.clone()))
            .await
            .map_err(|e| StorageError::s3("reconcile_quota", format!("Failed to update timestamp: {}", e)))?;

        let quotas: Vec<StorageQuota> = result.take(0).map_err(|e| {
            StorageError::s3("reconcile_quota", format!("Failed to deserialize: {}", e))
        })?;

        quotas
            .into_iter()
            .next()
            .map(|q| q.to_info())
            .ok_or_else(|| StorageError::s3("reconcile_quota", "No quota record found for user"))
    }
}

/// List all objects for a user prefix and sum their sizes
///
/// # Arguments
/// * `client` - S3 client
/// * `prefix` - S3 key prefix for the user's objects
///
/// # Returns
/// Total bytes used by objects with the given prefix
#[instrument(skip(client), fields(prefix = prefix))]
async fn list_user_storage_usage(client: &S3Client, prefix: &str) -> StorageResult<u64> {
    let mut total_bytes: u64 = 0;
    let mut continuation_token: Option<String> = None;

    loop {
        let mut request = client
            .inner()
            .list_objects_v2()
            .bucket(client.bucket())
            .prefix(prefix);

        if let Some(token) = &continuation_token {
            request = request.continuation_token(token);
        }

        let response = request.send().await.map_err(|e| {
            StorageError::s3("list_objects", format!("Failed to list objects: {}", e))
        })?;

        // Sum sizes of all objects
        for object in response.contents() {
            if let Some(size) = object.size() {
                total_bytes += size as u64;
            }
        }

        // Check for more pages
        if response.is_truncated() == Some(true) {
            continuation_token = response.next_continuation_token().map(|s| s.to_string());
        } else {
            break;
        }
    }

    tracing::debug!(
        prefix = prefix,
        total_bytes = total_bytes,
        "Listed user storage"
    );
    Ok(total_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quota_info_new() {
        let info = QuotaInfo::new(1000, 5000, None);
        assert_eq!(info.bytes_used, 1000);
        assert_eq!(info.bytes_limit, 5000);
        assert_eq!(info.bytes_available, 4000);
        assert!((info.percentage_used - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_quota_info_default() {
        let info = QuotaInfo::default_for_new_user();
        assert_eq!(info.bytes_used, 0);
        assert_eq!(info.bytes_limit, DEFAULT_BYTES_LIMIT);
        assert_eq!(info.bytes_available, DEFAULT_BYTES_LIMIT);
        assert!((info.percentage_used - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_quota_would_exceed() {
        let info = QuotaInfo::new(4500, 5000, None);
        assert!(!info.would_exceed(500)); // Exactly at limit is OK
        assert!(info.would_exceed(501)); // Over limit
        assert!(!info.would_exceed(0)); // Zero bytes
    }

    #[test]
    fn test_quota_warning_threshold() {
        let below_warning = QuotaInfo::new(3999, 5000, None); // 79.98%
        let at_warning = QuotaInfo::new(4000, 5000, None); // 80%
        let above_warning = QuotaInfo::new(4500, 5000, None); // 90%

        assert!(!below_warning.is_at_warning_threshold());
        assert!(at_warning.is_at_warning_threshold());
        assert!(above_warning.is_at_warning_threshold());
    }

    #[test]
    fn test_quota_zero_limit() {
        // Edge case: zero limit should show 100% used
        let info = QuotaInfo::new(0, 0, None);
        assert!((info.percentage_used - 100.0).abs() < 0.01);
        assert_eq!(info.bytes_available, 0);
    }

    #[test]
    fn test_quota_saturating_sub() {
        // If bytes_used > bytes_limit somehow (shouldn't happen but handle gracefully)
        let info = QuotaInfo::new(6000, 5000, None);
        assert_eq!(info.bytes_available, 0); // Saturates to 0
        assert!(info.percentage_used > 100.0); // Over 100%
    }

    #[test]
    fn test_storage_quota_to_info() {
        let quota = StorageQuota {
            id: None,
            owner: Thing::from(("user", "test123")),
            bytes_used: 1024 * 1024 * 100,       // 100MB
            bytes_limit: 5 * 1024 * 1024 * 1024, // 5GB
            last_reconciled: Some(Utc::now()),
            device_id: "device1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let info = quota.to_info();
        assert_eq!(info.bytes_used, 100 * 1024 * 1024);
        assert_eq!(info.bytes_limit, 5 * 1024 * 1024 * 1024);
        assert!(info.last_reconciled.is_some());
    }

    #[test]
    fn test_storage_quota_negative_bytes_handled() {
        // Handle negative bytes gracefully (shouldn't happen but be safe)
        let quota = StorageQuota {
            id: None,
            owner: Thing::from(("user", "test123")),
            bytes_used: -100,
            bytes_limit: 5000,
            last_reconciled: None,
            device_id: "device1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let info = quota.to_info();
        assert_eq!(info.bytes_used, 0); // Negative becomes 0
    }
}
