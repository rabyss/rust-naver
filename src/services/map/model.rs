use serde::{Serialize, Deserialize};
use crate::services::common_model::{Meta, AddressElement};

#[derive(Serialize, Deserialize, Default)]
pub struct MapGeocodeInput {
    pub query: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapGeocodeOutput {
    pub status: String,
    pub meta: Meta,
    pub addresses: Vec<MapGeocodeAddress>,
    pub error_message: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MapGeocodeAddress {
    pub road_address: String,
    pub jibun_address: String,
    pub english_address: String,
    pub address_elements: Vec<AddressElement>,
    pub x: String,
    pub y: String,
    pub distance: f64
}
