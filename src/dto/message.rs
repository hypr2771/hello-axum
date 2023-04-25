use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub _id: Option<ObjectId>,
    pub topic: Option<ObjectId>,
    pub author: Option<ObjectId>,
    pub content: String,
    pub publication: Option<DateTime>,
}
