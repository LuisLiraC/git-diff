FROM rust:1.78.0 AS build

RUN USER=root cargo new --bin git-diff
WORKDIR /git-diff

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# RUN rm ./target/release/deps/git_diff*
RUN cargo build --release

COPY --from=build /git-diff/target/release/git-diff /usr/local/bin/git-diff

ENTRYPOINT ["git-diff"]