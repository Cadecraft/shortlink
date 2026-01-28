use std::collections::hash_map::HashMap;
use std::env;

pub async fn fetch() -> Option<HashMap<String, String>> {
    let data_url = env::var("DATA_URL").unwrap();
    let body = reqwest::get(data_url).await.ok()?.text().await.ok()?;
    let json: serde_json::Value = serde_json::from_str(&body).ok()?;
    let mut res = HashMap::new();
    for elem in json.as_object().unwrap().iter() {
        res.insert(elem.0.to_string(), elem.1.as_str()?.to_string());
    }
    Some(res)
}
