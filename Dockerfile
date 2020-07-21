# You can override this `--build-arg BASE_IMAGE=...` to use different
# version of Rust or OpenSSL.
ARG VERSION=nightly-2020-07-12
ARG BASE_IMAGE=ekidd/rust-musl-builder:${VERSION}

FROM ${BASE_IMAGE} AS builder

WORKDIR /home/rust/

# Avoid having to install/build all dependencies by copying
# the Cargo files and making a dummy src/main.rs
COPY Cargo.toml .
COPY Cargo.lock .
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

# We need to touch our real main.rs file or else docker will use
# the cached one.
COPY --chown=rust:rust . .
RUN sudo touch src/main.rs

RUN cargo build --release

# Size optimization
RUN strip target/x86_64-unknown-linux-musl/release/main

# Start building the final image
FROM scratch
WORKDIR /home/rust/
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/main .
ENTRYPOINT ["./main"]
