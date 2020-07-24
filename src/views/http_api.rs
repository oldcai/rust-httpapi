extern crate actix_web;
extern crate serde;

use std::collections::HashMap;
use std::vec::Vec;

use actix_web::{App, http, HttpRequest, HttpServer, Responder, Result, web};
use clap::Arg;

use crate::models;

pub async fn raw_ip_view(_req: HttpRequest) -> impl Responder {
    let addr = _req.peer_addr().unwrap();
    let ip = addr.ip().to_string();
    format!("{}\n", ip)
}

pub async fn remote_ip_view(_req: HttpRequest) -> impl Responder {
    let connection_info = _req.connection_info();
    let ip = connection_info.remote().unwrap();
    format!("{}\n", ip)
}

pub async fn http_headers_view(_req: HttpRequest) -> Result<web::Json<models::HTTPInfo>> {
    let headers = _req.headers();
    let header_map = models::convert_http_headers_to_map(&headers);
    let obj = models::HTTPInfo {
        headers: header_map,
    };
    Ok(web::Json(obj))
}
