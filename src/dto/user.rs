use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}
