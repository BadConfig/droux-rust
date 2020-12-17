FROM rustlang/rust:nightly AS builder

RUN USER=root cargo new --bin droux
WORKDIR /droux

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./SQL ./SQL

# build for release
RUN rm ./target/release/deps/droux*
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && \
    apt-get --assume-yes install \
        make \
        libpq5 \
        libpq-dev \
        -qqy \
        --no-install-recommends
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /droux/target/release/droux /droux/droux
WORKDIR /droux/
EXPOSE 8000


CMD ["/droux/droux"]