FROM rust:1.49.0

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release
