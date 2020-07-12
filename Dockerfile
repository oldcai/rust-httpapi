FROM alpine:latest

WORKDIR /web/
COPY ./target/x86_64-unknown-linux-musl/release/httpapi /web/

CMD ["./httpapi", "0.0.0.0:8000"]
