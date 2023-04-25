use axum::{
    async_trait,
    extract::{rejection::TypedHeaderRejectionReason, FromRef, FromRequestParts},
    headers::{authorization::Basic, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt, TypedHeader,
};
use mongodb::Client;
use serde_json::json;

use crate::{dto::user::User, repositories::user_repository::UserRepository};

pub struct BasicAuthorization {
    pub user: User,
}

pub enum AuthorizationError {
    MissingAuthorization,
    InvalidAuthorization,
}

impl IntoResponse for AuthorizationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthorizationError::MissingAuthorization => {
                (StatusCode::FORBIDDEN, "Missing authorization header")
            }
            AuthorizationError::InvalidAuthorization => {
                (StatusCode::UNAUTHORIZED, "Invalid authorization header")
            }
        };

        let body = Json(json!({ "error": error_message }));

        (status, body).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for BasicAuthorization
where
    Client: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthorizationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let client = Client::from_ref(state);

        match parts.extract::<TypedHeader<Authorization<Basic>>>().await {
            // Basic is valid
            Ok(header) => {
                let db_user = UserRepository::using(client)
                    .get_by_username(String::from(header.username()))
                    .await;

                match db_user {
                    // User exists
                    Ok(Some(existing)) => {
                        if existing.password == header.password() {
                            Ok(BasicAuthorization { user: existing })
                        } else {
                            Err(AuthorizationError::InvalidAuthorization)
                        }
                    }
                    // User does not exists
                    _ => Err(AuthorizationError::InvalidAuthorization),
                }
            }
            // Basic is invalid
            Err(error) => match error.reason() {
                TypedHeaderRejectionReason::Missing => {
                    Err(AuthorizationError::MissingAuthorization)
                }
                _ => Err(AuthorizationError::InvalidAuthorization),
            },
        }
    }
}
