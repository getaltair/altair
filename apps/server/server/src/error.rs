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

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        // Inspect database-level error codes before consuming `e`.
        if let sqlx::Error::Database(ref db_err) = e {
            match db_err.code().as_deref() {
                // 23505: unique_violation — duplicate client-generated UUID (A-018 retry)
                Some("23505") => return AppError::Conflict("duplicate key".to_string()),
                // 23503: foreign_key_violation — invalid FK reference (e.g. initiative_id)
                Some("23503") => {
                    return AppError::BadRequest(
                        "foreign key constraint violation".to_string(),
                    )
                }
                _ => {}
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
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
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
}
