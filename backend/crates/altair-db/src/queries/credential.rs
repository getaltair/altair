//! Credential queries - Database operations for user credentials (password hashes)

use altair_core::{Error, Result};
use chrono::Utc;
use surrealdb::Surreal;
use surrealdb::sql::Thing;

use crate::schema::UserCredential;

/// Create credential for a user
pub async fn create_credential<C: surrealdb::Connection>(
    db: &Surreal<C>,
    user_id: Thing,
    password_hash: String,
) -> Result<UserCredential> {
    let credential: UserCredential = db
        .create("user_credential")
        .content(UserCredential {
            id: None,
            user: user_id,
            password_hash,
            updated_at: Utc::now(),
        })
        .await
        .map_err(|e| Error::Database(format!("Failed to create credential: {}", e)))?
        .ok_or_else(|| Error::Database("Credential creation returned no result".to_string()))?;

    Ok(credential)
}

/// Get credential by user ID
pub async fn get_credential_by_user<C: surrealdb::Connection>(
    db: &Surreal<C>,
    user_id: &Thing,
) -> Result<Option<UserCredential>> {
    let user_id_owned = user_id.clone();
    let mut result = db
        .query("SELECT * FROM user_credential WHERE user = $user")
        .bind(("user", user_id_owned))
        .await
        .map_err(|e| Error::Database(format!("Failed to query credential by user: {}", e)))?;

    let credentials: Vec<UserCredential> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize credential: {}", e)))?;

    Ok(credentials.into_iter().next())
}

/// Update credential (change password)
pub async fn update_credential<C: surrealdb::Connection>(
    db: &Surreal<C>,
    user_id: &Thing,
    new_password_hash: String,
) -> Result<UserCredential> {
    let user_id_owned = user_id.clone();
    let mut result = db
        .query(
            "UPDATE user_credential SET password_hash = $hash, updated_at = $time WHERE user = $user RETURN AFTER",
        )
        .bind(("user", user_id_owned))
        .bind(("hash", new_password_hash))
        .bind(("time", Utc::now()))
        .await
        .map_err(|e| Error::Database(format!("Failed to update credential: {}", e)))?;

    let credentials: Vec<UserCredential> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize credential: {}", e)))?;

    credentials
        .into_iter()
        .next()
        .ok_or_else(|| Error::NotFound {
            entity_type: "credential".to_string(),
            id: format!("user: {:?}", user_id),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> surrealdb::Surreal<surrealdb::engine::local::Db> {
        let db = Surreal::new::<surrealdb::engine::local::Mem>(())
            .await
            .unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        db
    }

    #[tokio::test]
    async fn test_create_and_get_credential() {
        let db = setup_test_db().await;
        let user_id = Thing::from(("user", "test_user"));

        let credential = create_credential(&db, user_id.clone(), "hashed_password_123".to_string())
            .await
            .unwrap();

        assert_eq!(credential.user, user_id);
        assert_eq!(credential.password_hash, "hashed_password_123");

        let found = get_credential_by_user(&db, &user_id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().password_hash, "hashed_password_123");
    }

    #[tokio::test]
    async fn test_update_credential() {
        let db = setup_test_db().await;
        let user_id = Thing::from(("user", "test_user"));

        create_credential(&db, user_id.clone(), "old_hashed_password".to_string())
            .await
            .unwrap();

        let updated = update_credential(&db, &user_id, "new_hashed_password".to_string())
            .await
            .unwrap();

        assert_eq!(updated.password_hash, "new_hashed_password");

        let found = get_credential_by_user(&db, &user_id).await.unwrap();
        assert_eq!(found.unwrap().password_hash, "new_hashed_password");
    }

    #[tokio::test]
    async fn test_get_nonexistent_credential() {
        let db = setup_test_db().await;
        let user_id = Thing::from(("user", "nonexistent_user"));

        let found = get_credential_by_user(&db, &user_id).await.unwrap();
        assert!(found.is_none());
    }
}
