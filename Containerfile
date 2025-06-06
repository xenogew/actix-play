FROM rust:1.87-slim AS builder

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

FROM debian:bookworm-slim
LABEL org.opencontainers.image.authors="Natta Wang <xenogew@gmail.com>"

WORKDIR /work/bin

COPY --from=builder /actix-play/target/release/actix-play .

RUN groupadd -g 10001 natta-santa
RUN useradd -u 10001 -g 10001 -s /sbin/nologin natta-santa

RUN chown -R natta-santa:natta-santa /work/bin && \
    chmod 744 /work/bin/actix-play

USER natta-santa

EXPOSE 8080

CMD ["./actix-play"]
