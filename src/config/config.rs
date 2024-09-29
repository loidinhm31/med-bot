use crate::models::documents::User;
use dotenv::dotenv;
use mongodb::bson::Document;
use mongodb::{
    Client, Collection,
};
use std::env;

#[derive(Debug)]
pub struct MongoClient {
    pub dynamic_collection: Collection<Document>,
    pub user_collection: Collection<User>,
}

impl MongoClient {
    pub async fn builder() -> MongoClientBuilder {
        MongoClientBuilder::new().await
    }
}

pub struct MongoClientBuilder {
    pub dynamic_collection: Option<Collection<Document>>,
    pub user_collection: Option<Collection<User>>,
    client: Client
}

impl MongoClientBuilder {

    pub async fn new() -> MongoClientBuilder {
        dotenv().ok();
        let uri = match env::var("MONGODB_URI") {
            Ok(v) => v.to_string(),
            Err(_) => {
                eprintln!("Error loading MongoDB URI from env, using default");
                "mongodb://localhost:27017".to_string()  // Default to localhost
            }
        };

        let client = Client::with_uri_str(uri).await.expect("failed to connect");

        MongoClientBuilder {
            client,
            user_collection: None,
            dynamic_collection: None,
        }
    }

    pub fn with_dynamic_collection(mut self) -> MongoClientBuilder {
        let db = self.client.database("sched_tool");
        let col: Collection<Document> = db.collection("Dynamic");
        self.dynamic_collection = Some(col);
        self
    }

    pub fn with_user_collection(mut self) -> MongoClientBuilder {
        let db = self.client.database("sched_tool");
        let col: Collection<User> = db.collection("User");
        self.user_collection = Some(col);
        self
    }

    pub fn build(self) -> MongoClient
    {
        MongoClient {
            user_collection: self.user_collection.expect("User collection not initialized"),
            dynamic_collection: self.dynamic_collection.expect("Dynamic collection not initialized"),
        }
    }
}