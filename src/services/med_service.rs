use crate::dto::appointment_model::{AppointmentApiResponse, Day, TimeSlot};
use crate::dto::search_model::{ResultItem, SearchApiResponse};
use crate::models::doctor_appointment::{AppointmentPicking, DoctorAppointment};
use crate::models::documents::Doctor;
use crate::repositories::doctor_repository::{MongoDoctorRepository, MongoDoctorRepositoryBuilder};
use crate::services::mail_service::MailService;
use chrono::{NaiveDate, NaiveDateTime};
use cron::TimeUnitSpec;
use mongodb::Collection;
use reqwest::{Client, Error};
use std::collections::HashMap;
use crate::config::med_target_config::MedTarget;

pub struct MedService {
    med_target: MedTarget,
    mongo_doctor_repository: MongoDoctorRepository,
    mail_service: MailService,
}

impl MedService {}

impl MedService {
    pub fn builder(med_target: MedTarget, collection: Collection<Doctor>, mail_service: MailService) -> MedServiceBuilder {
        MedServiceBuilder::new(med_target, collection, mail_service)
    }

    pub async fn search_med(&self, client: &Client, search_key: String, city_id: String, subject_id: String) -> Result<Vec<SearchApiResponse>, Box<Error>> {
        let mut map = HashMap::new();
        map.insert("search_key", search_key);
        map.insert("category", String::from("doctor"));
        map.insert("city_id", city_id);
        map.insert("limit", String::from("3"));
        map.insert("offset", String::from("1"));
        map.insert("subject_ids", subject_id);

        let result = client.post(self.med_target.search_med_api.clone())
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Accept-Encoding", "gzip, deflate, br, zstd")
            .header("Content-Type", "application/json;charset=utf-8")
            .header("locale", "vi")
            .header("platform", "web")
            .header("Origin", self.med_target.origin_header.clone())
            .header("Connection", "keep-alive")
            .header("Referer", self.med_target.origin_header.clone())
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

    pub async fn get_appointments(&self, client: &Client, subject_id: String, doctor_id: String, service_id: String, partner_id: String) -> Result<AppointmentApiResponse, Box<Error>> {
        let mut map = HashMap::new();
        map.insert("subjectId", subject_id);
        map.insert("doctorId", doctor_id);
        map.insert("serviceId", service_id);
        map.insert("treeId", "DATE".parse().unwrap());

        let result = client.post(self.med_target.appointment_api.clone())
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0")
            .header("Accept", "application/json, text/plain, */*")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Accept-Encoding", "gzip, deflate, br, zstd")
            .header("Content-Type", "application/json;charset=utf-8")
            .header("partnerid", partner_id)
            .header("appid", self.med_target.appid_header.clone())
            .header("locale", "vi")
            .header("platform", "pc")
            .header("Origin", self.med_target.origin_header.clone())
            .header("Connection", "keep-alive")
            .header("Referer", self.med_target.origin_header.clone())
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
            .get_target_doctor().await?;

        if let Some(doctor) = doctor_detail {
            log::info!("Got doctor");
            let search_response = self.search_med(
                client,
                doctor.doctor_name.to_owned(),
                doctor.city_id.to_owned(),
                doctor.subject_ref_id.to_owned(),
            ).await?;
            log::info!("Got search response");
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
                        analyze_doctor.subject_id.as_deref().unwrap().to_string(),
                        analyze_doctor.doctor_id.as_deref().unwrap().to_string(),
                        analyze_doctor.service_id.as_deref().unwrap().to_string(),
                        analyze_doctor.partner_id.as_deref().unwrap().to_string(),
                    ).await?;
                    log::info!("Got appointments");

                    // Process appointment and find available slot
                    let checked_appointments = doctor_appointment_result.days.iter()
                        .find_map(|appointment| {
                            log::info!("Checking appointment: {:?}", appointment);
                            self.find_available_shift(
                                appointment,
                                doctor.doctor_name.clone(),
                                doctor.target_date.clone(),
                            )
                        });

                    if let Some(checked_appointments) = checked_appointments {
                        self.mail_service.send_email(&checked_appointments)?;
                        return Ok(checked_appointments)
                    } else {
                        let result_appointment = AppointmentPicking {
                            doctor_name: Some(doctor.doctor_name.clone()),
                            appointment_day: None,
                            appointment_date: Some(doctor.target_date.clone()),
                            available_slot: None,
                            doctor_change_info: None,
                        };

                        self.mail_service.send_email(&result_appointment)?;
                        return Ok(result_appointment);
                    }
                }
            }
        }

        Err("Analyze appointment fail".into())
    }

    pub async fn get_doctor(&self) -> Result<Doctor, Box<dyn std::error::Error>> {
        let doctor_detail = self.mongo_doctor_repository
            .get_doctor_by_doctor_ref_id(String::from("test_ref_id"))
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
            if partner.partner_id.as_deref() == Some(doctor_detail.hospital_id.as_str()) &&
                partner.city_id.as_deref() == Some(doctor_detail.city_id.as_str()) {
                analyze_doctor.partner_id = partner.partner_id.clone();
                return true;
            }
            false
        });

        analyze_doctor.doctor_id = doctor.id.clone();
        is_valid_doctor && partner_valid
    }

    fn find_available_shift(&self, appointment: &Day, doctor_name: String, target_date: String) -> Option<AppointmentPicking> {
        // Target date
        let naive_target_date = NaiveDate::parse_from_str(target_date.as_str(), "%Y-%m-%d").unwrap();
        log::info!("Checking for target date: {}", naive_target_date.clone());

        let appointment_date = NaiveDateTime::from_timestamp_millis(appointment.date?).unwrap().date();
        log::info!("Compare for appointment date: {}", appointment_date.clone());

        if appointment_date == naive_target_date {
            log::info!("Found available items for target date");
            // Find a shift with available slots
            appointment.shifts.iter().find_map(|shift| {
                log::info!("Shift: {:?}", shift.shift_code);
                let available_slots: Vec<TimeSlot> = shift.time_slot_in_day.as_ref()?.iter()
                    .filter_map(|slot| {
                        log::info!("Available Slot {}", slot.available_slot.clone().unwrap());
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
                        doctor_name: Some(doctor_name.clone()),
                        appointment_date: Some(target_date.clone()),
                        appointment_day: shift.days.clone(),
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
    med_target: MedTarget,
    mongo_doctor_repository: MongoDoctorRepository,
    mail_service: MailService,
}

impl MedServiceBuilder {
    pub fn new(med_target: MedTarget, collection: Collection<Doctor>, mail_service: MailService) -> MedServiceBuilder {
        let mongo_doctor_repository = MongoDoctorRepositoryBuilder::new(collection).build();
        MedServiceBuilder {
            med_target,
            mongo_doctor_repository,
            mail_service,
        }
    }

    pub fn build(self) -> MedService {
        MedService {
            med_target: self.med_target,
            mongo_doctor_repository: self.mongo_doctor_repository,
            mail_service: self.mail_service,
        }
    }
}
