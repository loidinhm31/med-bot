extern crate dotenv;

use crate::models::documents::Doctor;
use mongodb::bson::doc;
use mongodb::{
    error::Error,
    Collection,
};

#[derive(Debug)]
pub struct MongoDoctorRepository {
    col: Collection<Doctor>,
}

impl MongoDoctorRepository {
    pub fn builder(collection: Collection<Doctor>) -> MongoDoctorRepositoryBuilder {
        MongoDoctorRepositoryBuilder::new(collection)
    }

    pub async fn get_doctor_by_doctor_ref_id(&self, doctor_ref_id: String) -> Result<Option<Doctor>, Error> {
        let filter = doc! {"doctor_ref_id": doctor_ref_id, "active": true};
        match self.col
            .find_one(filter)
            .await {
            Ok(Some(doctor_detail)) => Ok(Some(doctor_detail)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn get_doctor_by_doctor_name(&self, doctor_name: String) -> Result<Option<Doctor>, Error> {
        let filter = doc! {
            "doctor_name": {
                "$regex": doctor_name,
                "$options": "i"  // Case-insensitive search
            },
            "active": true
        };

        match self.col
            .find_one(filter)
            .await {
            Ok(Some(doctor_detail)) => Ok(Some(doctor_detail)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn get_target_doctor(&self) -> Result<Option<Doctor>, Error> {
        let filter = doc! {
            "current_target": true,
            "active": true
        };

        match self.col
            .find_one(filter)
            .await {
            Ok(Some(doctor_detail)) => Ok(Some(doctor_detail)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}


pub struct MongoDoctorRepositoryBuilder {
    col: Option<Collection<Doctor>>,
}

impl MongoDoctorRepositoryBuilder {
    pub fn new(collection: Collection<Doctor>) -> MongoDoctorRepositoryBuilder {
        MongoDoctorRepositoryBuilder {
            col: Some(collection),
        }
    }

    pub fn build(self) -> MongoDoctorRepository {
        MongoDoctorRepository {
            col: self.col.expect("Doctor collection not initialized"),
        }
    }
}
