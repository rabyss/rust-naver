use crate::services::NaverService;
use crate::cred::NaverCred;
use crate::services::map::model::{MapGeocodeInput, MapGeocodeOutput};

pub struct NaverMap {
    service: Box<NaverService>
}

impl NaverMap {
    pub fn new(cred: &NaverCred) -> Self {
        NaverMap {
            service: Box::new(NaverService::new(cred))
        }
    }

    pub fn geocode(&self, input: &MapGeocodeInput) -> Result<MapGeocodeOutput, failure::Error> {
        let api = "map-geocode/v2/geocode";
        self.service.http_get(api, input)
    }
}