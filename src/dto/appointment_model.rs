use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentApiResponse {
    pub id: Option<String>,

    pub r#type: String,

    #[serde(rename = "subType")]
    pub sub_type: Option<String>,

    pub days: Vec<Day>,

    pub end: bool,

    pub detail: Detail,

    #[serde(rename = "waitingList")]
    pub waiting_list: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    pub shifts: Vec<Shift>,

    pub date: Option<i64>,

    #[serde(rename = "timeSlots")]
    pub time_slots: Option<Vec<TimeSlot>>,

    pub timemiliseconds: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shift {
    pub id: String,
    pub shift_name: Option<String>,
    pub shift_code: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub duration: Option<u32>,
    pub days: Option<String>,
    pub services: Option<Vec<Service>>,

    pub max_slot: Option<u32>,

    #[serde(rename = "doctorChange")]
    pub doctor_change: Option<bool>,

    #[serde(rename = "doctorChangeInfo")]
    pub doctor_change_info: Option<DoctorChangeInfo>,

    #[serde(rename = "roomId")]
    pub room_id: Option<String>,

    #[serde(rename = "priorityRoom")]
    pub priority_room: Option<u32>,

    #[serde(rename = "timeSlotInDay")]
    pub time_slot_in_day: Option<Vec<TimeSlot>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub subject_id: Option<String>,
    pub room_id: Option<String>,
    pub price: Option<u32>,
    pub advanced: Option<u32>,
    pub service_type: Option<String>,
    pub room_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorChangeInfo {
    #[serde(rename = "changeDoctorId")]
    pub change_doctor_id: Option<String>,

    #[serde(rename = "changeDoctorName")]
    pub change_doctor_name: Option<String>,

    pub role: Option<String>,

    #[serde(rename = "labelPrefix")]
    pub label_prefix: Option<String>,
    pub label: Option<String>,

    #[serde(rename = "reasonChangeDoctor")]
    pub reason_change_doctor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    #[serde(rename = "timeId")]
    pub time_id: String,

    #[serde(rename = "availableSlot")]
    pub available_slot: Option<u32>,

    #[serde(rename = "maxSlot")]
    pub max_slot: Option<u32>,

    #[serde(rename = "startTime")]
    pub start_time: String,

    #[serde(rename = "endTime")]
    pub end_time: String,

    #[serde(rename = "roomId")]
    pub room_id: String,

    #[serde(rename = "priorityRoom")]
    pub priority_room: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Detail {
    pub id: String,

    pub name: Option<String>,

    pub r#type: Option<String>,

    #[serde(rename = "displayDetail")]
    pub display_detail: Option<String>,

    pub description: Option<String>,

    #[serde(rename = "serviceType")]
    pub service_type: Option<String>,

    #[serde(rename = "serviceGroup")]
    pub service_group: Option<String>,

    pub price: Option<u32>,

    pub advanced: Option<u32>,

    pub rooms: Option<String>,

    #[serde(rename = "nextCombine")]
    pub next_combine: Option<bool>,

    pub days: Option<String>,

    #[serde(rename = "displaySchedule")]
    pub display_schedule: Option<String>,

    #[serde(rename = "bookingGroupName")]
    pub booking_group_name: Option<String>,

    #[serde(rename = "requiredCheckInsurance")]
    pub required_check_insurance: Option<bool>,
}