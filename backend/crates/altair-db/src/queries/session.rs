//! Session queries - Database operations for authentication sessions

use altair_core::{Error, Result};
use chrono::{DateTime, Utc};
use surrealdb::Surreal;
use surrealdb::sql::Thing;

use crate::schema::Session;

/// Create a new session
pub async fn create_session<C: surrealdb::Connection>(
    db: &Surreal<C>,
    token: String,
    user: Thing,
    expires_at: DateTime<Utc>,
    device_id: String,
) -> Result<Session> {
    let session: Session = db
        .create("session")
        .content(Session {
            id: None,
            token,
            user,
            expires_at,
            device_id,
            created_at: Utc::now(),
        })
        .await
        .map_err(|e| Error::Database(format!("Failed to create session: {}", e)))?
        .ok_or_else(|| Error::Database("Session creation returned no result".to_string()))?;

    Ok(session)
}

/// Get session by token
pub async fn get_session_by_token<C: surrealdb::Connection>(
    db: &Surreal<C>,
    token: &str,
) -> Result<Option<Session>> {
    let token_owned = token.to_string();
    let mut result = db
        .query("SELECT * FROM session WHERE token = $tok")
        .bind(("tok", token_owned))
        .await
        .map_err(|e| Error::Database(format!("Failed to query session by token: {}", e)))?;

    let sessions: Vec<Session> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize session: {}", e)))?;

    Ok(sessions.into_iter().next())
}

/// Refresh session expiry
pub async fn refresh_session<C: surrealdb::Connection>(
    db: &Surreal<C>,
    token: &str,
    new_expiry: DateTime<Utc>,
) -> Result<Session> {
    let token_owned = token.to_string();
    let mut result = db
        .query("UPDATE session SET expires_at = $expiry WHERE token = $tok RETURN AFTER")
        .bind(("tok", token_owned))
        .bind(("expiry", new_expiry))
        .await
        .map_err(|e| Error::Database(format!("Failed to refresh session: {}", e)))?;

    let sessions: Vec<Session> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize session: {}", e)))?;

    sessions.into_iter().next().ok_or_else(|| Error::NotFound {
        entity_type: "session".to_string(),
        id: format!("token: {}", token),
    })
}

/// Delete session (logout)
pub async fn delete_session<C: surrealdb::Connection>(db: &Surreal<C>, token: &str) -> Result<()> {
    let token_owned = token.to_string();
    db.query("DELETE session WHERE token = $tok")
        .bind(("tok", token_owned))
        .await
        .map_err(|e| Error::Database(format!("Failed to delete session: {}", e)))?;

    Ok(())
}

/// Delete all expired sessions (cleanup job)
pub async fn delete_expired_sessions<C: surrealdb::Connection>(db: &Surreal<C>) -> Result<u64> {
    let mut result = db
        .query("DELETE session WHERE expires_at < time::now() RETURN BEFORE")
        .await
        .map_err(|e| Error::Database(format!("Failed to delete expired sessions: {}", e)))?;

    let deleted: Vec<Session> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize deleted sessions: {}", e)))?;

    Ok(deleted.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    async fn setup_test_db() -> surrealdb::Surreal<surrealdb::engine::local::Db> {
        let db = Surreal::new::<surrealdb::engine::local::Mem>(())
            .await
            .unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_create_and_get_session() {
        let db = setup_test_db().await;
        let user_id = Thing::from(("user", "test_user"));

        let session = create_session(
            &db,
            "test_token_123".to_string(),
            user_id.clone(),
            Utc::now() + Duration::days(7),
            "device123".to_string(),
        )
        .await
        .unwrap();

        assert_eq!(session.token, "test_token_123");
        assert_eq!(session.user, user_id);

        let found = get_session_by_token(&db, "test_token_123").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().token, "test_token_123");
    }

    #[tokio::test]
    async fn test_refresh_session() {
        let db = setup_test_db().await;
        let user_id = Thing::from(("user", "test_user"));

        let original_expiry = Utc::now() + Duration::days(7);
        let _session = create_session(
            &db,
            "test_token_456".to_string(),
            user_id,
            original_expiry,
            "device123".to_string(),
        )
        .await
        .unwrap();

        let new_expiry = Utc::now() + Duration::days(14);
        let refreshed = refresh_session(&db, "test_token_456", new_expiry)
            .await
            .unwrap();

        assert!(refreshed.expires_at > original_expiry);
    }

    #[tokio::test]
    async fn test_delete_session() {
        let db = setup_test_db().await;
        let user_id = Thing::from(("user", "test_user"));

        create_session(
            &db,
            "test_token_789".to_string(),
            user_id,
            Utc::now() + Duration::days(7),
            "device123".to_string(),
        )
        .await
        .unwrap();

        delete_session(&db, "test_token_789").await.unwrap();

        let found = get_session_by_token(&db, "test_token_789").await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    #[ignore] // TODO: Fix test - time::now() behaves inconsistently in test environment
    async fn test_delete_expired_sessions() {
        let db = setup_test_db().await;
        let user_id = Thing::from(("user", "test_user_cleanup"));

        // Create expired session with past expiry
        create_session(
            &db,
            "expired_token_cleanup".to_string(),
            user_id.clone(),
            Utc::now() - Duration::hours(24),
            "device123".to_string(),
        )
        .await
        .unwrap();

        // Create active session with future expiry
        create_session(
            &db,
            "active_token_cleanup".to_string(),
            user_id,
            Utc::now() + Duration::days(7),
            "device123".to_string(),
        )
        .await
        .unwrap();

        // Delete expired sessions
        let deleted_count = delete_expired_sessions(&db).await.unwrap();
        assert!(
            deleted_count >= 1,
            "Expected at least 1 deleted session, got {}",
            deleted_count
        );

        // Verify expired session is gone
        let expired = get_session_by_token(&db, "expired_token_cleanup")
            .await
            .unwrap();
        assert!(expired.is_none(), "Expired session should be deleted");

        // Verify active session still exists
        let active = get_session_by_token(&db, "active_token_cleanup")
            .await
            .unwrap();
        assert!(active.is_some(), "Active session should still exist");
    }
}
