FROM rust:latest as builder
RUN rustup component add clippy && \
    rustup component add rustfmt

# install build-/dev-dependencies here
# RUN apt-get update && \
#    apt-get -q install -y libpq-dev clang libsasl2-dev libsasl2-modules-gssapi-mit

WORKDIR /usr/src

# pre-compile dependencies for improved cacheability
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    # debug-build (to run tests/clippy)
    cargo build --locked && \
    # release-build (for install)
    cargo build --release --locked && \
    cargo clippy -- -D warnings -A dead-code && \
    rm -r src

COPY . .

# check formatting - should be done by intellij
RUN cargo fmt -- --check

# disallow all cargo clippy warnings, except for dead code
RUN cargo clippy -- -D warnings -A dead-code

RUN GIT_COMMIT=$(git rev-parse HEAD) cargo install --locked --path . --root /usr/local

FROM debian:latest
# enable JSON logging
ENV RUST_LOG_JSON=1

# install runtime-dependencies here
# RUN apt-get update && \
#    apt-get -q install -y libpq5 libclang1 && \
#    rm -rf /var/lib/apt/*

COPY --from=builder /usr/local/bin/* /usr/local/bin/

USER service

CMD my_app
