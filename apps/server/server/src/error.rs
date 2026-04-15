use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sqlx;

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    Conflict(String),

    #[error("{0}")]
    UnprocessableEntity(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

/// Maps a PostgreSQL error code to an AppError where a specific HTTP status is warranted.
/// Returns None for unrecognized codes; the caller falls through to AppError::Internal.
///
/// Extracted as a pure function so the mapping logic is unit-testable without a live DB
/// connection (sqlx::Error::Database cannot be constructed outside sqlx internals).
fn pg_code_to_app_error(code: &str) -> Option<AppError> {
    match code {
        // 23505: unique_violation — duplicate client-generated UUID (A-018 retry)
        "23505" => Some(AppError::Conflict("duplicate key".to_string())),
        // 23503: foreign_key_violation — invalid FK reference (e.g. initiative_id)
        "23503" => Some(AppError::BadRequest(
            "foreign key constraint violation".to_string(),
        )),
        _ => None,
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        // Inspect database-level error codes before consuming `e`.
        if let sqlx::Error::Database(ref db_err) = e {
            if let Some(app_err) = db_err.code().as_deref().and_then(pg_code_to_app_error) {
                return app_err;
            }
        }
        match e {
            // NOTE: RowNotFound maps to NotFound. All row lookups use fetch_optional +
            // .ok_or(AppError::NotFound); fetch_one on a query returning zero rows would
            // silently produce 404 instead of 500 — use fetch_optional to preserve intent.
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => AppError::Internal(anyhow::Error::from(e)),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::Unauthorized => {
                tracing::debug!("Unauthorized response");
                (StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
            }
            AppError::Forbidden => {
                tracing::debug!("Forbidden response");
                (StatusCode::FORBIDDEN, "Forbidden".to_string())
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::Internal(e) => {
                tracing::error!("Internal error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        routing::get,
    };
    use tower::ServiceExt;

    async fn not_found_handler() -> Result<(), AppError> {
        Err(AppError::NotFound)
    }

    async fn internal_error_handler() -> Result<(), AppError> {
        Err(AppError::Internal(anyhow::anyhow!(
            "secret internal detail"
        )))
    }

    #[tokio::test]
    async fn not_found_returns_404() {
        let app = Router::new().route("/", get(not_found_handler));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn internal_error_returns_500_without_leaking_details() {
        let app = Router::new().route("/", get(internal_error_handler));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let text = String::from_utf8_lossy(&body);
        assert!(
            !text.contains("secret internal detail"),
            "Internal error details must not be leaked"
        );
    }

    // Helper: drive an AppError through IntoResponse and return (status, body_text)
    async fn status_and_body(err: AppError) -> (StatusCode, String) {
        let response = err.into_response();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        (status, String::from_utf8_lossy(&bytes).into_owned())
    }

    #[tokio::test]
    async fn unauthorized_returns_401() {
        let (status, body) = status_and_body(AppError::Unauthorized).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(body.contains("\"Unauthorized\""), "body was: {body}");
    }

    #[tokio::test]
    async fn forbidden_returns_403() {
        let (status, _body) = status_and_body(AppError::Forbidden).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn bad_request_returns_400_with_message() {
        let (status, body) = status_and_body(AppError::BadRequest("bad input".to_string())).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(body.contains("\"bad input\""), "body was: {body}");
    }

    #[tokio::test]
    async fn conflict_returns_409() {
        let (status, body) = status_and_body(AppError::Conflict("duplicate".to_string())).await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert!(body.contains("\"duplicate\""), "body was: {body}");
    }

    #[tokio::test]
    async fn unprocessable_entity_returns_422() {
        let (status, body) =
            status_and_body(AppError::UnprocessableEntity("invalid type".to_string())).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(body.contains("\"invalid type\""), "body was: {body}");
    }

    #[tokio::test]
    async fn internal_error_does_not_leak_inner_detail() {
        let (status, body) =
            status_and_body(AppError::Internal(anyhow::anyhow!("super secret detail"))).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(
            !body.contains("super secret detail"),
            "Internal error must not leak details; body was: {body}"
        );
    }

    // --- From<sqlx::Error> conversion tests ---

    // From<sqlx::Error>: non-RowNotFound variants must produce AppError::Internal.
    #[test]
    fn sqlx_error_converts_to_internal() {
        let err: AppError = sqlx::Error::PoolTimedOut.into();
        assert!(
            matches!(err, AppError::Internal(_)),
            "sqlx::Error must convert to AppError::Internal"
        );
    }

    // From<sqlx::Error>: non-RowNotFound variant must yield HTTP 500.
    #[tokio::test]
    async fn sqlx_error_returns_500() {
        let (status, _) = status_and_body(sqlx::Error::PoolTimedOut.into()).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    // From<sqlx::Error>: sqlx error details (e.g. table/constraint names) must not
    // appear in the HTTP response body.
    #[tokio::test]
    async fn sqlx_error_does_not_leak_schema_detail() {
        // Use a Protocol error that embeds realistic schema detail (constraint names, table names).
        let detail = "duplicate key value violates unique constraint \"pk_users\"";
        let sqlx_err = sqlx::Error::Protocol(detail.into());
        let (status, body) = status_and_body(AppError::from(sqlx_err)).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(
            !body.contains(detail),
            "Schema detail must not appear in the response body; body was: {body}"
        );
        assert!(
            !body.contains("pk_users"),
            "Table/constraint names must not be leaked; body was: {body}"
        );
    }

    // From<sqlx::Error>: RowNotFound must map to AppError::NotFound (404), not Internal.
    // All service-layer lookups use fetch_optional + .ok_or(AppError::NotFound), so this
    // mapping is safe and intentional — see error.rs impl note.
    #[tokio::test]
    async fn sqlx_row_not_found_converts_to_not_found() {
        let (status, _) = status_and_body(sqlx::Error::RowNotFound.into()).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    // --- pg_code_to_app_error unit tests ---
    // These test the extracted pure function directly, avoiding the need for a live DB
    // connection to construct sqlx::Error::Database.

    #[test]
    fn pg_23505_maps_to_conflict() {
        assert!(
            matches!(pg_code_to_app_error("23505"), Some(AppError::Conflict(_))),
            "23505 (unique_violation) must map to Conflict"
        );
    }

    #[test]
    fn pg_23503_maps_to_bad_request() {
        assert!(
            matches!(pg_code_to_app_error("23503"), Some(AppError::BadRequest(_))),
            "23503 (foreign_key_violation) must map to BadRequest"
        );
    }

    #[test]
    fn pg_unknown_code_returns_none() {
        assert!(
            pg_code_to_app_error("99999").is_none(),
            "Unrecognized PG codes must return None (caller produces Internal)"
        );
    }
}
