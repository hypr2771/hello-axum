use authorization::basic::BasicAuthorization;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, put},
    Error, Json, Router,
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
    ) -> Result<(StatusCode, Json<Option<User>>), StatusCode> {
    let result = UserRepository::using(client)
        .get_by_username(authorized.user.username)
        .await;

    match result {
        Ok(Some(user)) => Ok((StatusCode::OK, Json(Some(user)))),
        Ok(None) => Err(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_user(
    State(client): State<Client>,
    Json(to_create): Json<User>,
) -> Result<(StatusCode, Json<Option<User>>), StatusCode> {
    let result = UserRepository::using(client).create(to_create).await;

    match result {
        Ok(created) => Ok((StatusCode::CREATED, Json(Some(created)))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_topic(
    State(client): State<Client>,
    authorized: BasicAuthorization,
    Json(to_create): Json<Topic>,
) -> Result<(StatusCode, Json<Option<Topic>>), StatusCode> {
    let result = TopicRepository::using(client)
        .create(Topic {
            author: authorized.user._id,
            ..to_create
        })
        .await;

    match result {
        Ok(created) => Ok((StatusCode::CREATED, Json(Some(created)))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_message(
    State(client): State<Client>,
    authorized: BasicAuthorization,
    Path(topic): Path<String>,
    Json(to_create): Json<Message>,
) -> Result<(StatusCode, Json<Option<Message>>), StatusCode> {
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
                        Ok(created) => Ok((StatusCode::CREATED, Json(Some(created)))),
                        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                    }
                }
                Ok(None) => Err(StatusCode::NOT_FOUND),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
