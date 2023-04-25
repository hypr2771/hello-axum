use authorization::basic::BasicAuthorization;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use dto::{message::Message, topic::Topic, user::User};
use mongodb::{
    bson::oid::ObjectId,
    options::{ClientOptions, Credential, ServerAddress},
    Client,
};
use repositories::{message_repository::MessageRepository, topic_repository::TopicRepository};

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
        .route("/topics", put(create_topic))
        .route("/topics/:topic/messages", put(create_message))
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

async fn create_topic(
    State(client): State<Client>,
    authorized: BasicAuthorization,
    Json(to_create): Json<Topic>,
) -> (StatusCode, Json<Option<Topic>>) {
    let result = TopicRepository::using(client)
        .create(Topic {
            author: authorized.user._id,
            ..to_create
        })
        .await;

    match result {
        Ok(created) => (StatusCode::CREATED, Json(Some(created))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

async fn create_message(
    State(client): State<Client>,
    authorized: BasicAuthorization,
    Path(topic): Path<String>,
    Json(to_create): Json<Message>,
) -> (StatusCode, Json<Option<Message>>) {
    let topic = ObjectId::parse_str(topic);

    match topic {
        Ok(topic) => {
            let topic = TopicRepository::using(client.clone())
                .get_by_id(topic)
                .await;

            match topic {
                Ok(Some(topic)) => {
                    let result = MessageRepository::using(client)
                        .create(Message {
                            author: authorized.user._id,
                            topic: topic._id,
                            ..to_create
                        })
                        .await;

                    match result {
                        Ok(created) => (StatusCode::CREATED, Json(Some(created))),
                        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
                    }
                }
                Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}
