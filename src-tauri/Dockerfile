FROM rust:1.64.0 as builder

RUN apt-get update && apt-get install -y libdbus-1-dev protobuf-compiler libgtk-3-dev webkit2gtk-4.0 libappindicator3-dev librsvg2-dev patchelf

WORKDIR /usr/src/app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir -p src/bin
RUN touch src/bin/server.rs
RUN touch src/bin/generate-key.rs
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY ./src ./src

RUN cargo install --path . --bin server

FROM debian:buster
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/
CMD ["server"]