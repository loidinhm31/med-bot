use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct MedTarget {
    pub appointment_api: String,
    pub search_med_api: String,
    pub origin_header: String,
    pub appid_header: String,
}

impl MedTarget {
    pub fn builder() -> MedTargetBuilder {
        MedTargetBuilder::new()
    }
}

pub struct MedTargetBuilder {
    pub appointment_api: String,
    pub search_med_api: String,
    pub origin_header: String,
    pub appid_header: String,
}

impl MedTargetBuilder {
    pub fn new() -> MedTargetBuilder {
        dotenv().ok();
        let appointment_api = match env::var("APPOINTMENT_API") {
            Ok(v) => v.to_string(),
            Err(_) => {
                eprintln!("Error loading APPOINTMENT_API from env");
                "UNKNOWN".to_string()
            }
        };

        let search_med_api = match env::var("SEARCH_MED_API") {
            Ok(v) => v.to_string(),
            Err(_) => {
                eprintln!("Error loading SEARCH_MED_API from env");
                "UNKNOWN".to_string()
            }
        };

        let origin_header = match env::var("ORIGIN_HEADER") {
            Ok(v) => v.to_string(),
            Err(_) => {
                eprintln!("Error loading ORIGIN_HEADER from env");
                "UNKNOWN".to_string()
            }
        };

        let appid_header = match env::var("APPID_HEADER") {
            Ok(v) => v.to_string(),
            Err(_) => {
                eprintln!("Error loading APPID_HEADER from env");
                "UNKNOWN".to_string()
            }
        };

        MedTargetBuilder {
            appointment_api,
            search_med_api,
            origin_header,
            appid_header: appid_header,
        }
    }


    pub fn build(self) -> MedTarget {
        MedTarget {
            appointment_api: self.appointment_api,
            search_med_api: self.search_med_api,
            origin_header: self.origin_header,
            appid_header: self.appid_header,
        }
    }
}