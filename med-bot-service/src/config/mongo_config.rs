use crate::models::documents::{Doctor, User};
use dotenv::dotenv;
use mongodb::bson::Document;
use mongodb::{
    Client, Collection,
};
use std::env;

#[derive(Debug, Clone)]
pub struct MongoClient {
    pub dynamic_collection: Collection<Document>,
    pub user_collection: Collection<User>,
    pub doctor_collection: Collection<Doctor>,
}

impl MongoClient {
    pub async fn builder() -> MongoClientBuilder {
        MongoClientBuilder::new().await
    }
}

pub struct MongoClientBuilder {
    pub dynamic_collection: Option<Collection<Document>>,
    pub user_collection: Option<Collection<User>>,
    pub doctor_collection: Option<Collection<Doctor>>,
    client: Client,
}

impl MongoClientBuilder {
    pub async fn new() -> MongoClientBuilder {
        dotenv().ok();
        let uri = match env::var("MONGODB_URI") {
            Ok(v) => {
                log::info!("Using MONGODB_URI: {}", &v);
                v.to_string()
            },
            Err(_) => {
                log::error!("Error loading MONGODB_URI from env, using default");
                "mongodb://localhost:27017".to_string()  // Default to localhost
            }
        };

        let client = Client::with_uri_str(uri).await.expect("failed to connect");

        log::info!("Connected to mongodb");

        MongoClientBuilder {
            client,
            dynamic_collection: None,
            user_collection: None,
            doctor_collection: None,
        }
    }

    pub fn with_dynamic_collection(mut self) -> MongoClientBuilder {
        let db = self.client.database("med_tool");
        let col: Collection<Document> = db.collection("Dynamic");
        self.dynamic_collection = Some(col);
        self
    }

    pub fn with_user_collection(mut self) -> MongoClientBuilder {
        let db = self.client.database("med_tool");
        let col: Collection<User> = db.collection("user");
        self.user_collection = Some(col);
        self
    }

    pub fn with_doctor_collection(mut self) -> MongoClientBuilder {
        let db = self.client.database("med_tool");
        let col: Collection<Doctor> = db.collection("doctor");
        self.doctor_collection = Some(col);
        self
    }

    pub fn build(self) -> MongoClient {
        MongoClient {
            dynamic_collection: self.dynamic_collection.expect("Dynamic collection not initialized"),
            user_collection: self.user_collection.expect("User collection not initialized"),
            doctor_collection: self.doctor_collection.expect("Doctor collection not initialized"),
        }
    }
}