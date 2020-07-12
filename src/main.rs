extern crate actix_web;
extern crate serde;
extern crate serde_json;

use actix_web::{http, web, App, HttpRequest, HttpServer, Responder, Result};
use clap::Arg;
use std::collections::HashMap;
use std::vec::Vec;

#[macro_use]
extern crate serde_derive;

async fn raw_ip_view(_req: HttpRequest) -> impl Responder {
    let addr = _req.peer_addr().unwrap();
    let ip = addr.ip().to_string();
    format!("{}\n", ip)
}

#[derive(Serialize, Deserialize, Debug)]
struct HTTPInfo {
    headers: std::collections::HashMap<String, Vec<String>>,
}

fn convert_http_headers_to_map(headers: &http::HeaderMap) -> HashMap<String, Vec<String>> {
    let mut header_hashmap = HashMap::new();
    for (k, v) in headers {
        let k = k.as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(Vec::new).push(v)
    }
    header_hashmap
}

async fn http_headers_view(_req: HttpRequest) -> Result<web::Json<HTTPInfo>> {
    let headers = _req.headers();
    let header_map = convert_http_headers_to_map(&headers);
    let obj: HTTPInfo = HTTPInfo {
        headers: header_map,
    };
    Ok(web::Json(obj))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = clap::App::new("HTTP Proxy")
        .arg(
            Arg::with_name("listen_addr")
                .takes_value(true)
                .value_name("LISTEN ADDR")
                .index(1)
                .required(true),
        )
        .get_matches();
    let listen_addr = matches.value_of("listen_addr").unwrap();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(raw_ip_view))
            .route("/ip", web::get().to(raw_ip_view))
            .route("/headers", web::get().to(http_headers_view))
    })
    .bind(listen_addr)?
    .run()
    .await
}
