
FROM rust:1.58 as builder

RUN cargo new --bin server
WORKDIR ./server
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml

RUN cargo install --path .

ENTRYPOINT ["server", "cli"]