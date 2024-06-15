FROM rust:1.78.0 AS build

RUN USER=root cargo new --bin git-diff
WORKDIR /git-diff

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/git_diff*
RUN cargo build --release

RUN pwd
RUN ls -la ./target/release
COPY ./target/release/git-diff .

ENTRYPOINT ["./git-diff"]