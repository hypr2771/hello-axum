use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    error::Error,
    options::FindOptions,
    Client, Collection,
};

use crate::dto::message::Message;

const PAGE_SIZE: u64 = 25;

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

    pub async fn find_for_topic(&self, topic: ObjectId, page: u64) -> Result<Vec<Message>, Error> {
        let messages = self
            .collection
            .find(
                doc! {"topic": Some(topic)},
                FindOptions::builder()
                    .skip(PAGE_SIZE * page)
                    .limit(PAGE_SIZE as i64)
                    .sort(doc! {"publication": 1})
                    .build(),
            )
            .await;

        match messages {
            Ok(messages) => {
                let messages: Vec<Result<Message, Error>> = messages.collect().await;

                Ok(messages
                    .into_iter()
                    .filter(|with_erroneous| with_erroneous.is_ok())
                    .map(|only_successes| only_successes.ok().unwrap())
                    .collect())
            }
            Err(e) => Err(e),
        }
    }
}
