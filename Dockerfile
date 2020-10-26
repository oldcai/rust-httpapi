FROM oldcai/rust-musl-builder:latest as build
ADD . /home/rust/src
WORKDIR /home/rust/src

#RUN apk add --no-cache ca-certificates gcc mingw-w64-gcc libc-dev musl-dev
#RUN rustup target add x86_64-unknown-linux-musl
#RUN export CARGO_HOME="~/.cargo" \
#    && export SCCACHE_DIR="~/.cache/sccache" \
#    && export RUSTC_WRAPPER="$(dirname $(whereis cargo | cut -d ' ' -f 2))/sccache"

ADD user_cargo_config_in_docker.toml /home/rust/.cargo/config

RUN cargo build --release

FROM alpine:latest

WORKDIR /web/
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/httpapi /web/

CMD ["./httpapi", "0.0.0.0:80"]
