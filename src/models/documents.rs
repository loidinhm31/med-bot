use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Doctor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub doctor_ref_id: String,
    pub doctor_name: String,
    pub subject_ref_id: String,
    pub subject_name: String,
    pub service_name: String,
    pub hospital_id: String,
    pub city_id: String,
    pub target_date: String,
    pub active: bool,
}