use axum::{
    async_trait,
    extract::{rejection::TypedHeaderRejectionReason, FromRequestParts},
    headers::{authorization::Basic, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, RequestPartsExt, Router, TypedHeader,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    user: BasicAuthorization,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: user.user.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[async_trait]
impl<S> FromRequestParts<S> for BasicAuthorization
where
    S: Send + Sync,
{
    type Rejection = AuthorizationError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match parts.extract::<TypedHeader<Authorization<Basic>>>().await {
            Ok(header) => {
                let user = User {
                    id: 1337,
                    username: String::from(header.username()),
                };
                Ok(BasicAuthorization { user })
            }
            Err(error) => match error.reason() {
                TypedHeaderRejectionReason::Missing => {
                    Err(AuthorizationError::MissingAuthorization)
                }
                _ => Err(AuthorizationError::InvalidAuthorization),
            },
        }
    }
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

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

struct BasicAuthorization {
    user: User,
}

enum AuthorizationError {
    MissingAuthorization,
    InvalidAuthorization,
}
