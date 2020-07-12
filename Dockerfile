FROM rust:1.44.1-alpine as build
ADD . /code/
WORKDIR /code/

RUN cargo build --release

FROM rust:1.44.1-alpine

COPY --from=0 /code/target/release/httpapi /web/
WORKDIR /web/

CMD ["./httpapi", "0.0.0.0:8000"]
