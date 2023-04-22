use authorization::basic::BasicAuthorization;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use dto::user::User;
use mongodb::{
    options::{ClientOptions, Credential, ServerAddress},
    Client,
};

use std::net::SocketAddr;

use crate::repositories::user_repository::UserRepository;

mod authorization;
mod dto;
mod repositories;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let client = Client::with_options(
        ClientOptions::builder()
            .hosts(vec![ServerAddress::Tcp {
                host: String::from("localhost"),
                port: Some(27017),
            }])
            .credential(
                Credential::builder()
                    .username(String::from("root"))
                    .password(String::from("example"))
                    .build(),
            )
            .build(),
    )
    .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/users/me", get(who_am_i))
        .route("/users", put(create_user))
        .with_state(client);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello Axum!"
}

async fn who_am_i(
    State(client): State<Client>,
    authorized: BasicAuthorization,
) -> (StatusCode, Json<Option<User>>) {
    let result = UserRepository::using(client)
        .get_by_username(authorized.user.username)
        .await;

    match result {
        Ok(Some(user)) => (StatusCode::OK, Json(Some(user))),
        Ok(None) => (StatusCode::NO_CONTENT, Json(None)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

async fn create_user(
    State(client): State<Client>,
    Json(to_create): Json<User>,
) -> (StatusCode, Json<Option<User>>) {
    let result = UserRepository::using(client).create(to_create).await;

    match result {
        Ok(created) => (StatusCode::CREATED, Json(Some(created))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}
