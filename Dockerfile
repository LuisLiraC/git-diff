FROM rust:1.78.0 AS builder

RUN USER=root cargo new --bin git-diff
WORKDIR /git-diff

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /git-diff/target/release/git-diff /usr/local/bin/git-diff

ENTRYPOINT ["git-diff"]
