FROM rust:1.78.0 AS builder

WORKDIR /git-diff

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN cargo build --release
RUN pwd
RUN ls -la
COPY src ./src

RUN pwd
RUN ls -la

RUN cargo install --path .

FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/git-diff /usr/local/bin/git-diff

ENTRYPOINT ["git-diff"]
