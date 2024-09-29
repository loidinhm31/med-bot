use crate::models::appointment_model::{AppointmentApiResponse, Day, TimeSlot};
use crate::models::doctor_appointment::{AppointmentPicking, DoctorAppointment};
use crate::models::search_model::{ResultItem, SearchApiResponse};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use cron::TimeUnitSpec;
use reqwest::{Client, Error};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct MedService {
    bar: String,
}

impl MedService {}

impl MedService {
    pub fn builder() -> MedServiceBuilder {
        MedServiceBuilder::default()
    }

    pub async fn search_med(&self, client: &Client) -> Result<Vec<SearchApiResponse>, Box<Error>> {
        let mut map = HashMap::new();
        map.insert("search_key", "trần ngọc tài");
        map.insert("category", "doctor");
        map.insert("city_id", "medpro_79");
        map.insert("limit", "3");
        map.insert("offset", "1");
        map.insert("subject_ids", "medpro_thankinh");

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

    pub async fn check_appointment(&self, client: &Client) -> Result<AppointmentPicking, Box<dyn std::error::Error>> {
        let search_response = self.search_med(client).await?;

        if let Some(first_med) = search_response.first() {
            if let Some(first_doctor_item) = first_med.results.first() {
                let mut analyze_doctor = DoctorAppointment::default();

                // Validate doctor details
                if !self.validate_doctor(first_doctor_item, &mut analyze_doctor) {
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
                let target_date = NaiveDate::parse_from_str("2024-10-03", "%Y-%m-%d")?;

                // Process appointment and find available slot
                return doctor_appointment_result.days.iter()
                    .find_map(|appointment| self.find_available_shift(appointment, target_date))
                    .ok_or_else(|| "No appointment found".into());
            }
        }

        Err("Search failed".into())
    }

    fn validate_doctor(&self, doctor: &ResultItem, analyze_doctor: &mut DoctorAppointment) -> bool {
        // Check doctor's name
        let is_valid_doctor = doctor.title.as_deref() == Some("Trần Ngọc Tài");

        // Check subject
        if let Some(subjects) = &doctor.subjects {
            if let Some(target_subject) = subjects.iter().find(|subject| {
                subject.name.as_ref().map_or(false, |name| name.to_lowercase().contains("parkinson"))
            }) {
                analyze_doctor.subject_id = Some(target_subject.id.clone());
            } else {
                return false;
            }
        }

        // Check service
        if let Some(services) = &doctor.services {
            if let Some(target_service) = services.iter().find(|service| {
                service.name.as_ref().map_or(false, |name| name.to_lowercase() == "khám dịch vụ")
                    && service.subject_names.as_ref().map_or(false, |names| {
                    names.iter().any(|name| name.to_lowercase().contains("parkinson"))
                })
            }) {
                analyze_doctor.service_id = Some(target_service.id.clone());
            } else {
                return false;
            }
        }

        // Check partner and city ID
        let partner_valid = doctor.partner.as_ref().map_or(false, |partner| {
            partner.partner_id.as_deref() == Some("umc") && partner.city_id.as_deref() == Some("medpro_79")
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
                    })
                }
                None
            })
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct MedServiceBuilder {
    bar: String,
}

impl MedServiceBuilder {
    pub fn new() -> MedServiceBuilder {
        MedServiceBuilder {
            bar: String::from("X"),
        }
    }

    pub fn build(self) -> MedService {
        MedService { bar: self.bar }
    }
}
