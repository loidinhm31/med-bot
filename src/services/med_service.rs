use crate::dto::appointment_model::{AppointmentApiResponse, Day, TimeSlot};
use crate::dto::search_model::{ResultItem, SearchApiResponse};
use crate::models::doctor_appointment::{AppointmentPicking, DoctorAppointment};
use crate::models::documents::Doctor;
use crate::repositories::doctor_repository::{MongoDoctorRepository, MongoDoctorRepositoryBuilder};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use cron::TimeUnitSpec;
use mongodb::Collection;
use reqwest::{Client, Error};
use std::collections::HashMap;

pub struct MedService {
    mongo_doctor_repository: MongoDoctorRepository,
}

impl MedService {}

impl MedService {
    pub fn builder(collection: Collection<Doctor>) -> MedServiceBuilder {
        MedServiceBuilder::new(collection)
    }

    pub async fn search_med(&self, client: &Client, search_key: String, city_id: String, subject_id: String) -> Result<Vec<SearchApiResponse>, Box<Error>> {
        let mut map = HashMap::new();
        map.insert("search_key", search_key);
        map.insert("category", String::from("doctor"));
        map.insert("city_id", city_id);
        map.insert("limit", String::from("3"));
        map.insert("offset", String::from("1"));
        map.insert("subject_ids", subject_id);

        let result = client.post("https://api-v2.medpro.com.vn/mongo/service/search")
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Accept-Encoding", "gzip, deflate, br, zstd")
            .header("Content-Type", "application/json;charset=utf-8")
            .header("locale", "vi")
            .header("platform", "web")
            .header("Origin", "https://medpro.vn")
            .header("Connection", "keep-alive")
            .header("Referer", "https://medpro.vn/")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "cross-site")
            .json(&map)
            .send()
            .await?;

        // Extract the response body as text
        let raw_json = result.text().await?;
        // println!("Raw JSON response: {}", raw_json);

        // Deserialize the JSON
        let deserialized_result: Vec<SearchApiResponse> = serde_json::from_str(&raw_json).unwrap();

        // Return the deserialized result
        Ok(deserialized_result)
    }

    pub async fn get_appointments(&self, client: &Client, subject_id: &str, doctor_id: &str, service_id: &str) -> Result<AppointmentApiResponse, Box<Error>> {
        let mut map = HashMap::new();
        map.insert("subjectId", subject_id);
        map.insert("doctorId", doctor_id);
        map.insert("serviceId", service_id);
        map.insert("treeId", "DATE");

        let result = client.post("https://api-v2.medpro.com.vn/his-gateway/booking-tree-dynamic-current-node")
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Accept-Encoding", "gzip, deflate, br, zstd")
            .header("Content-Type", "application/json;charset=utf-8")
            .header("partnerid", "umc")
            .header("appid", "medpro")
            .header("locale", "vi")
            .header("platform", "pc")
            .header("Origin", "https://medpro.vn")
            .header("Connection", "keep-alive")
            .header("Referer", "https://medpro.vn/")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "cross-site")
            .json(&map)
            .send()
            .await?;

        // Extract the response body as text
        let raw_json = result.text().await?;
        // println!("Raw JSON response: {}", raw_json);

        // Deserialize the JSON
        let deserialized_result: AppointmentApiResponse = serde_json::from_str(&raw_json).unwrap();

        // Return the deserialized result
        Ok(deserialized_result)
    }

    pub async fn analyze_appointment(&self, client: &Client) -> Result<AppointmentPicking, Box<dyn std::error::Error>> {
        let doctor_detail = self.mongo_doctor_repository
            .get_doctor_by_doctor_name(String::from("trần ngọc tài")).await?;

        if let Some(doctor) = doctor_detail {
            let search_response = self.search_med(
                client,
                doctor.doctor_name.to_owned(),
                doctor.city_id.to_owned(),
                doctor.subject_ref_id.to_owned(),
            ).await?;

            if let Some(first_med) = search_response.first() {
                if let Some(first_doctor_item) = first_med.results.first() {
                    let mut analyze_doctor = DoctorAppointment::default();

                    // Validate doctor details
                    if !self.validate_doctor(first_doctor_item, &mut analyze_doctor, &doctor) {
                        return Err(Box::<dyn std::error::Error>::from("Invalid doctor"));
                    }

                    // Fetch doctor appointments
                    let doctor_appointment_result = self.get_appointments(
                        client,
                        analyze_doctor.subject_id.as_deref().unwrap(),
                        analyze_doctor.doctor_id.as_deref().unwrap(),
                        analyze_doctor.service_id.as_deref().unwrap(),
                    ).await?;

                    // Target date
                    let target_date = NaiveDate::parse_from_str(doctor.target_date.as_str(), "%Y-%m-%d")?;

                    // Process appointment and find available slot
                    return doctor_appointment_result.days.iter()
                        .find_map(|appointment| self.find_available_shift(appointment, target_date))
                        .ok_or_else(|| "No appointment found".into());
                }
            }
        }

        Err("Analyze appointment fail".into())
    }

    pub async fn get_doctor(&self) -> Result<Doctor, Box<dyn std::error::Error>> {
        let doctor_detail = self.mongo_doctor_repository
            .get_doctor_by_doctor_ref_id(String::from("umc_453"))
            .await?;
        Ok(doctor_detail.unwrap())
    }

    fn validate_doctor(&self, doctor: &ResultItem, analyze_doctor: &mut DoctorAppointment, doctor_detail: &Doctor) -> bool {
        // Check doctor's name
        let is_valid_doctor = doctor.title.as_deref() == Some(doctor_detail.doctor_name.as_str());

        // Check subject
        if let Some(subjects) = &doctor.subjects {
            if let Some(target_subject) = subjects.iter().find(|subject| {
                subject.name.as_ref().map_or(false, |name| name.to_lowercase().contains(doctor_detail.subject_name.as_str()))
            }) {
                analyze_doctor.subject_id = Some(target_subject.id.clone());
            } else {
                return false;
            }
        }

        // Check service
        if let Some(services) = &doctor.services {
            if let Some(target_service) = services.iter().find(|service| {
                service.name.as_ref().map_or(false, |name| name.to_lowercase() == doctor_detail.service_name.as_str())
                    && service.subject_names.as_ref().map_or(false, |names| {
                    names.iter().any(|name| name.to_lowercase().contains(doctor_detail.subject_name.as_str()))
                })
            }) {
                analyze_doctor.service_id = Some(target_service.id.clone());
            } else {
                return false;
            }
        }

        // Check partner and city ID
        let partner_valid = doctor.partner.as_ref().map_or(false, |partner| {
            partner.partner_id.as_deref() == Some(doctor_detail.hospital_id.as_str()) && partner.city_id.as_deref() == Some(doctor_detail.city_id.as_str())
        });

        analyze_doctor.doctor_id = doctor.id.clone();
        is_valid_doctor && partner_valid
    }

    fn find_available_shift(&self, appointment: &Day, target_date: NaiveDate) -> Option<AppointmentPicking> {
        let appointment_date = NaiveDateTime::from_timestamp_millis(appointment.date?).unwrap().date();

        if appointment_date == target_date {
            // Find a shift with available slots
            appointment.shifts.iter().find_map(|shift| {
                let available_slots: Vec<TimeSlot> = shift.time_slot_in_day.as_ref()?.iter()
                    .filter_map(|slot| {
                        if let (Some(available_slot), Some(max_slot)) = (slot.available_slot, slot.max_slot) {
                            if available_slot > 0 && available_slot <= max_slot {
                                return Some(TimeSlot {
                                    available_slot: Some(available_slot),
                                    max_slot: Some(max_slot),
                                    // Add other fields from the original TimeSlot struct
                                    ..slot.clone()  // Dereference `slot` and then clone it
                                });
                            }
                        }
                        None
                    }).collect();

                if !available_slots.is_empty() {
                    return Some(AppointmentPicking {
                        doctor_name: None,
                        appointment_day: Some(appointment.date.unwrap()),
                        appointment_date: Some(target_date.num_days_from_ce() as i64),
                        available_slot: Some(available_slots),
                        doctor_change_info: shift.doctor_change_info.clone(),
                    });
                }
                None
            })
        } else {
            None
        }
    }
}

pub struct MedServiceBuilder {
    mongo_doctor_repository: MongoDoctorRepository,
}

impl MedServiceBuilder {
    pub fn new(collection: Collection<Doctor>) -> MedServiceBuilder {
        let mongo_doctor_repository = MongoDoctorRepositoryBuilder::new(collection).build();
        MedServiceBuilder {
            mongo_doctor_repository,
        }
    }

    pub fn build(self) -> MedService {
        MedService {
            mongo_doctor_repository: self.mongo_doctor_repository,
        }
    }
}
