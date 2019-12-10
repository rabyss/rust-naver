use crate::services::map::api::NaverMap;
use crate::cred::NaverCred;
use crate::services::map::model::MapGeocodeInput;

#[test]
fn test_geocode() -> Result<(), failure::Error> {
    let key_id = dotenv::var("NAVER_API_KEY_ID")?;
    let key = dotenv::var("NAVER_API_KEY")?;
    let client = NaverMap::new(&NaverCred::new(&key_id, &key));
    let res = client.geocode(&MapGeocodeInput{
        query: "전북 삼성동 100".to_string()
    })?;

    println!("{:#?}", res);

    Ok(())
}