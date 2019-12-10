use crate::cred::NaverCred;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub mod map;
pub mod common_model;

struct NaverService {
    cred: NaverCred,
    host: &'static str
}

impl NaverService {
    fn new(cred: &NaverCred) -> Self {
        const HOST: &'static str = "https://naveropenapi.apigw.ntruss.com";

        NaverService {
            cred: cred.clone(),
            host: HOST
        }
    }

    fn http_get<Input: Serialize, Output: DeserializeOwned>(&self, api_path: &str, input: &Input) -> Result<Output, failure::Error> {
        let url = format!("{}/{}", self.host, api_path);

        let client = reqwest::Client::new();
        let response = client.get(&url)
            .headers(self.cred.authorization_header()?)
            .query(input)
            .send()?;

        let output = response.error_for_status()
            .and_then(|mut r| r.json::<Output>())?;

        Ok(output)
    }
}