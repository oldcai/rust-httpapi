extern crate actix_web;

use actix_web::{web, App, HttpRequest, Responder, HttpServer};
use clap::Arg;

async fn index(_req: HttpRequest) -> impl Responder {
    let addr = _req.peer_addr().unwrap();
    let ip = addr.ip().to_string();
    format!("{}\n", ip)
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
            .route("/", web::get().to(index))
            .route("/ip", web::get().to(index))
    })
        .bind(listen_addr)?
        .run()
        .await
}

