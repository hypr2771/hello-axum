use mongodb::{
    bson::{oid::ObjectId, DateTime},
    error::Error,
    Client, Collection,
};

use crate::dto::message::Message;

pub struct MessageRepository {
    collection: Collection<Message>,
}

impl MessageRepository {
    pub fn using(client: Client) -> Self {
        Self {
            collection: client.database("axum").collection("messages"),
        }
    }

    pub async fn create(&self, to_create: Message) -> Result<Message, Error> {
        let hydrated = Message {
            _id: Some(ObjectId::new()),
            publication: Some(DateTime::now()),
            ..to_create
        };

        self.collection
            .insert_one(hydrated.clone(), None)
            .await
            .map(|_| hydrated)
    }
}
