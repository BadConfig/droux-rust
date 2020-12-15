FROM rustlang/rust:nightly AS builder

WORKDIR /code
COPY ./ /code

RUN mkdir .cargo
RUN cargo vendor > .cargo/config
RUN cargo build --release
RUN cargo install --path . --verbose

FROM debian:buster-slim

RUN apt-get update && \
    apt-get --assume-yes install \
        make \
        libpq5 \
        libpq-dev \
        -qqy \
        --no-install-recommends
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /code/target/release/droux /droux/droux
WORKDIR /droux/
EXPOSE 8000


CMD ["/droux/droux"]