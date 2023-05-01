use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    error::Error,
    options::FindOptions,
    Client, Collection,
};

use crate::dto::topic::Topic;

const PAGE_SIZE: u64 = 25;

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

    pub async fn get(&self, page: u64) -> Result<Vec<Topic>, Error> {
        let cursor = self
            .collection
            .find(
                None,
                FindOptions::builder()
                    .skip(PAGE_SIZE * page)
                    .limit(PAGE_SIZE as i64)
                    .sort(doc! {
                        "creation": -1
                    })
                    .build(),
            )
            .await;

        match cursor {
            Ok(cursor) => {
                let topics: Vec<Result<Topic, Error>> = cursor.collect().await;
                Ok(topics
                    .into_iter()
                    .filter(|with_erroneous| with_erroneous.is_ok())
                    .map(|only_successes| only_successes.ok().unwrap())
                    .collect())
            }
            Err(e) => Err(e),
        }
    }
}
