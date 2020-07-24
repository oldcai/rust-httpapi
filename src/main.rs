#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod models;
mod views;

use actix_web::{http, web, App, HttpRequest, HttpServer, Responder, Result};
use clap::Arg;

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
            .route("/", web::get().to(views::raw_ip_view))
            .route("/ip", web::get().to(views::raw_ip_view))
            .route("/remote_ip", web::get().to(views::remote_ip_view))
            .route("/headers", web::get().to(views::http_headers_view))
    })
    .bind(listen_addr)?
    .run()
    .await
}
