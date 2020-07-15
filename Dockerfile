FROM oldcai/rust-musl-builder:latest as build
ADD . /home/rust/src
WORKDIR /home/rust/src

#RUN apk add --no-cache ca-certificates gcc mingw-w64-gcc libc-dev musl-dev
#RUN rustup target add x86_64-unknown-linux-musl
#RUN cargo build --release
RUN cargo build --release

FROM alpine:latest

WORKDIR /web/
COPY --from=0 /home/rust/src/target/x86_64-unknown-linux-musl/release/httpapi /web/

CMD ["./httpapi", "0.0.0.0:8000"]
