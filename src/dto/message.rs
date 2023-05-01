use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use super::user::UserWithoutPassword;

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub _id: Option<ObjectId>,
    pub topic: Option<ObjectId>,
    pub author: Option<ObjectId>,
    pub content: String,
    pub publication: Option<DateTime>,
}

impl Message {
    pub fn with_author(self, author: &UserWithoutPassword) -> MessageWithAuthor {
        MessageWithAuthor {
            message: self,
            author: author.clone(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct MessageWithAuthor {
    pub message: Message,
    pub author: UserWithoutPassword,
}
