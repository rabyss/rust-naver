use reqwest::header::{HeaderMap, HeaderValue};

#[derive(Clone)]
pub struct NaverCred {
    pub api_key_id: String,
    pub api_key: String
}

impl NaverCred {
    pub fn new(api_key_id: &str, api_key: &str) -> Self {
        NaverCred {
            api_key_id: api_key_id.to_string(),
            api_key: api_key.to_string()
        }
    }

    pub fn authorization_header(&self) -> Result<HeaderMap, failure::Error> {
        let mut headers = HeaderMap::new();
        let key_id = HeaderValue::from_str(&self.api_key_id)?;
        let secret = HeaderValue::from_str(&self.api_key)?;

        headers.insert("X-NCP-APIGW-API-KEY-ID", key_id);
        headers.insert("X-NCP-APIGW-API-KEY", secret);

        Ok(headers)
    }
}