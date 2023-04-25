use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    error::Error,
    Client, Collection,
};

use crate::dto::topic::Topic;

pub struct TopicRepository {
    collection: Collection<Topic>,
}

impl TopicRepository {
    pub fn using(client: Client) -> Self {
        Self {
            collection: client.database("axum").collection("topics"),
        }
    }

    pub async fn get_by_id(&self, to_create: ObjectId) -> Result<Option<Topic>, Error> {
        self.collection
            .find_one(doc! {"_id": to_create}, None)
            .await
    }

    pub async fn create(&self, to_create: Topic) -> Result<Topic, Error> {
        let hydrated = Topic {
            _id: Some(ObjectId::new()),
            creation: Some(DateTime::now()),
            ..to_create
        };

        self.collection
            .insert_one(hydrated.clone(), None)
            .await
            .map(|_| hydrated)
    }
}
