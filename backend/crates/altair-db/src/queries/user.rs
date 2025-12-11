//! User queries - Database operations for user accounts

use altair_core::{Error, Result};
use chrono::Utc;
use surrealdb::Surreal;
use surrealdb::sql::Thing;

use crate::schema::{User, UserPreferences};

/// Create a new user
pub async fn create_user<C: surrealdb::Connection>(
    db: &Surreal<C>,
    email: String,
    display_name: String,
    device_id: String,
) -> Result<User> {
    let user: User = db
        .create("user")
        .content(User {
            id: None,
            email,
            display_name,
            avatar_url: None,
            role: crate::schema::UserRole::Owner,
            preferences: UserPreferences::default(),
            device_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .await
        .map_err(|e| Error::Database(format!("Failed to create user: {}", e)))?
        .ok_or_else(|| Error::Database("User creation returned no result".to_string()))?;

    Ok(user)
}

/// Get user by email
pub async fn get_user_by_email<C: surrealdb::Connection>(
    db: &Surreal<C>,
    email: &str,
) -> Result<Option<User>> {
    let email_owned = email.to_string();
    let mut result = db
        .query("SELECT * FROM user WHERE email = $email")
        .bind(("email", email_owned))
        .await
        .map_err(|e| Error::Database(format!("Failed to query user by email: {}", e)))?;

    let users: Vec<User> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize user: {}", e)))?;

    Ok(users.into_iter().next())
}

/// Get user by ID
pub async fn get_user_by_id<C: surrealdb::Connection>(db: &Surreal<C>, id: Thing) -> Result<User> {
    let mut result = db
        .query("SELECT * FROM $id")
        .bind(("id", id.clone()))
        .await
        .map_err(|e| Error::Database(format!("Failed to get user by ID: {}", e)))?;

    let users: Vec<User> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize user: {}", e)))?;

    users.into_iter().next().ok_or_else(|| Error::NotFound {
        entity_type: "user".to_string(),
        id: format!("{:?}", id),
    })
}

/// Update user preferences (merge with existing)
pub async fn update_user_preferences<C: surrealdb::Connection>(
    db: &Surreal<C>,
    id: Thing,
    preferences: UserPreferences,
) -> Result<User> {
    let mut result = db
        .query("UPDATE $id MERGE $data RETURN AFTER")
        .bind(("id", id.clone()))
        .bind((
            "data",
            serde_json::json!({
                "preferences": preferences,
                "updated_at": Utc::now()
            }),
        ))
        .await
        .map_err(|e| Error::Database(format!("Failed to update user preferences: {}", e)))?;

    let users: Vec<User> = result
        .take(0)
        .map_err(|e| Error::Database(format!("Failed to deserialize user: {}", e)))?;

    users.into_iter().next().ok_or_else(|| Error::NotFound {
        entity_type: "user".to_string(),
        id: format!("{:?}", id),
    })
}

/// Check if any user exists (for first-launch detection)
pub async fn user_exists<C: surrealdb::Connection>(db: &Surreal<C>) -> Result<bool> {
    let mut result = db
        .query("SELECT count() FROM user GROUP ALL")
        .await
        .map_err(|e| Error::Database(format!("Failed to count users: {}", e)))?;

    let count: Option<i64> = result
        .take("count")
        .map_err(|e| Error::Database(format!("Failed to parse count: {}", e)))?;

    Ok(count.unwrap_or(0) > 0)
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
    async fn test_user_exists_empty_db() {
        let db = setup_test_db().await;
        let exists = user_exists(&db).await.unwrap();
        assert!(!exists);
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        let db = setup_test_db().await;

        let user = create_user(
            &db,
            "test@example.com".to_string(),
            "Test User".to_string(),
            "device123".to_string(),
        )
        .await
        .unwrap();

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.display_name, "Test User");

        // Test user_exists
        let exists = user_exists(&db).await.unwrap();
        assert!(exists);

        // Test get_by_email
        let found = get_user_by_email(&db, "test@example.com").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().email, "test@example.com");
    }

    #[tokio::test]
    async fn test_update_preferences() {
        let db = setup_test_db().await;

        let user = create_user(
            &db,
            "test@example.com".to_string(),
            "Test User".to_string(),
            "device123".to_string(),
        )
        .await
        .unwrap();

        let new_prefs = UserPreferences {
            theme: "dark".to_string(),
            gamification_enabled: false,
            ..Default::default()
        };

        let updated = update_user_preferences(&db, user.id.unwrap(), new_prefs.clone())
            .await
            .unwrap();

        assert_eq!(updated.preferences.theme, "dark");
        assert!(!updated.preferences.gamification_enabled);
    }
}
