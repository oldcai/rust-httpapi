FROM rust:1.44.1-alpine

WORKDIR /web/
COPY ./target/release/httpapi /web/

CMD ["./httpapi", "0.0.0.0:8000"]
