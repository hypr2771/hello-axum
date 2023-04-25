use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Topic {
    pub _id: Option<ObjectId>,
    pub author: Option<ObjectId>,
    pub title: String,
    pub creation: Option<DateTime>,
}
