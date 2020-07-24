use std::collections::HashMap;
use actix_web::http;

#[derive(Serialize, Deserialize, Debug)]
pub struct HTTPInfo {
    pub(crate) headers: std::collections::HashMap<String, Vec<String>>,
}

pub fn convert_http_headers_to_map(headers: &http::HeaderMap) -> HashMap<String, Vec<String>> {
    let mut header_hashmap = HashMap::new();
    for (k, v) in headers {
        let k = k.as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(Vec::new).push(v)
    }
    header_hashmap
}

