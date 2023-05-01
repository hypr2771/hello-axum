use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use super::message::MessageWithAuthor;

#[derive(Serialize, Deserialize, Clone)]
pub struct Topic {
    pub _id: Option<ObjectId>,
    pub author: Option<ObjectId>,
    pub title: String,
    pub creation: Option<DateTime>,
}

impl Topic {
    pub fn with_messages(self, messages: Vec<MessageWithAuthor>) -> TopicWithMessages {
        TopicWithMessages {
            topic: self,
            messages: messages.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct TopicWithMessages {
    pub(crate) topic: Topic,
    pub(crate) messages: Vec<MessageWithAuthor>,
}
