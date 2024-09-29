use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchApiResponse {
    pub category: String,
    pub search_key: String,
    pub hospitals: Vec<Hospital>,
    pub cities: Vec<City>,
    pub total: Option<u32>,
    pub results: Vec<ResultItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hospital {
    pub id: String,
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub ctas: Option<Vec<Cta>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub id: Option<String>,
    pub r#type: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultItem {
    pub id: Option<String>,

    #[serde(rename = "partnerId")]
    pub partner_id: Option<String>,

    pub title: Option<String>,

    pub role: Option<String>,

    pub gender: Option<String>,

    pub category: Option<String>,

    pub desc: Option<String>,

    pub tags: Option<Vec<Tag>>,

    pub desc2: Option<String>,

    pub price: Option<String>,

    #[serde(rename = "partnerId")]
    pub price_description: Option<String>,

    #[serde(rename = "partnerId")]
    pub tree_id: Option<String>,

    pub trees: Option<Vec<Tree>>,

    pub days: Option<String>,

    #[serde(rename = "hospitalAddress")]
    pub hospital_address: Option<String>,

    pub hospitals: Option<Vec<Hospital>>,

    pub subjects: Option<Vec<Subject>>,

    pub services: Option<Vec<Service>>,

    pub data: Option<String>,

    #[serde(rename = "originalPrice")]
    pub original_price: Option<String>,

    pub cta: Option<Cta>,

    pub description: Option<Description>,

    pub partner: Option<Partner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<String>,
    pub r#type: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    pub tree_id: Option<String>,
    pub detail_shift_id: Option<String>,
    pub doctor_id: Option<String>,
    pub days: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub id: String,
    pub r#type: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: String,

    pub r#type: Option<String>,

    pub name: Option<String>,

    pub price: Option<u32>,

    #[serde(rename = "displayDetail")]
    pub display_detail: Option<String>,

    #[serde(rename = "subjectNames")]
    pub subject_names: Option<Vec<String>>,

    pub ctas: Option<Vec<Cta>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cta {
    pub name: Option<String>,

    #[serde(rename = "partnerId")]
    pub partner_id: Option<String>,

    #[serde(rename = "treeId")]
    pub tree_id: Option<String>,

    #[serde(rename = "subjectId")]
    pub subject_id: Option<String>,

    #[serde(rename = "serviceId")]
    pub service_id: Option<String>,

    #[serde(rename = "doctorId")]
    pub doctor_id: Option<String>,

    #[serde(rename = "roomId")]
    pub room_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    pub rating: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Partner {
    #[serde(rename = "isCashBack")]
    pub is_cash_back: Option<bool>,

    pub _id: String,

    #[serde(rename = "partnerId")]
    pub partner_id: Option<String>,

    pub name: Option<String>,

    pub address: Option<String>,

    pub city_id: Option<String>,

    pub slug: Option<String>,

    #[serde(rename = "newHospitalTypes")]
    pub new_hospital_types: Option<Vec<u32>>,
}

