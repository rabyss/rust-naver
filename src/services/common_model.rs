use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub total_count: i32,
    pub page: Option<i32>,
    pub count: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddressElement {
    pub types: Vec<String>,
    pub long_name: String,
    pub short_name: String,
    pub code: String
}