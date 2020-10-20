#FROM liuchong/rustup:nightly as builder
## Copied from the rust official image, since they don't have a nightly image
FROM buildpack-deps:stretch

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
	case "${dpkgArch##*-}" in \
		amd64) rustArch='x86_64-unknown-linux-gnu'; \
        rustupSha256='7b5ce33a881992b285e2aa6cbc785da4138c5bab7c8c9b55c06918bfb1ba0efa' ;; \
		armhf) rustArch='armv7-unknown-linux-gnueabihf'; \
        rustupSha256='a92b003a15b2e4bd240c0f1d46232958c173f5605814e19961fc8a4d99a25b3e' ;; \
		i386) rustArch='i686-unknown-linux-gnu'; \
        rustupSha256='4a478c977b7b4900456c2d4dd165019e7c923ebdaba3f47316717d1690387d9a' ;; \
		*) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
	esac; \
    \
    url="https://static.rust-lang.org/rustup/archive/1.5.0/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    # changed to nightly version here
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

## end of the official image



# diesel dependencies
RUN apt-get update && \
    apt-get install \
        make \
        libpq5 \
        libpq-dev \
        -qqy \
        --no-install-recommends

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch
EXPOSE 5555
COPY . /app/
WORKDIR /app/
RUN rustup default nightly


RUN cargo clean
RUN cargo build
RUN cargo run --bin webapp
