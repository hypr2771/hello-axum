use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    Client, Collection,
};

use crate::dto::user::{User, UserWithoutPassword};

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn using(client: Client) -> UserRepository {
        let database = client.database("axum");

        let collection = database.collection("users");

        Self { collection }
    }

    pub async fn get_by_username(self, username: String) -> Result<Option<User>, Error> {
        self.collection
            .find_one(doc! {"username": username}, None)
            .await
    }

    pub async fn create(self, user: User) -> Result<User, Error> {
        let to_create = User {
            _id: Some(ObjectId::new()),
            ..user
        };

        self.collection
            .insert_one(to_create.clone(), None)
            .await
            .map(|_| to_create)
    }

    pub async fn find_all_by_id(
        &self,
        authors: Vec<ObjectId>,
    ) -> Result<Vec<UserWithoutPassword>, Error> {
        let users = self
            .collection
            .find(doc! {"_id": {"$in":authors.clone()}}, None)
            .await;

        match users {
            Ok(users) => {
                let users: Vec<Result<User, Error>> = users.collect().await;

                Ok(users
                    .into_iter()
                    .filter(|with_erroneous| with_erroneous.is_ok())
                    .map(|only_successes| only_successes.ok().unwrap())
                    .map(|users| users.without_password())
                    .collect())
            }
            Err(e) => Err(e),
        }
    }
}
