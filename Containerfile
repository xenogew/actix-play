FROM rust:1.61-slim as builder

# create a new empty shell project
RUN USER=root cargo new --bin actix-play
WORKDIR /actix-play

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm -rf ./target/release/.fingerprint/actix-play*
RUN cargo build --release

FROM rust:1.61-slim
LABEL org.opencontainers.image.authors="Natta Wang <xenogew@gmail.com>"

COPY --from=builder /actix-play/target/release/actix-play .

EXPOSE 8080

CMD ["./actix-play"]
