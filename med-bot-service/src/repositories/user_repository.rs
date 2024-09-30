extern crate dotenv;

use crate::models::documents::User;
use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult
    , Collection,
};

pub struct MongoUserRepository {
    col: Collection<User>,
}

impl MongoUserRepository {
    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
}