use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn without_password(self) -> UserWithoutPassword {
        UserWithoutPassword {
            _id: self._id.unwrap(),
            username: self.username,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct UserWithoutPassword {
    pub _id: ObjectId,
    pub username: String,
}
