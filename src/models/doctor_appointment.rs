use crate::dto::appointment_model::{DoctorChangeInfo, TimeSlot};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug)]
pub struct DoctorAppointment {
    pub subject_id: Option<String>,
    pub doctor_id: Option<String>,
    pub service_id: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppointmentPicking {
    pub doctor_name: Option<String>,
    pub appointment_day: Option<i64>,
    pub appointment_date: Option<i64>,
    pub available_slot: Option<Vec<TimeSlot>>,
    pub doctor_change_info: Option<DoctorChangeInfo>,
}