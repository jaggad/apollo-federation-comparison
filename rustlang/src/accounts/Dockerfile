
FROM ekidd/rust-musl-builder AS build
WORKDIR /usr/src/
USER root

# Add compilation target for later scratch container
ENV RUST_TARGETS="x86_64-unknown-linux-musl"
RUN rustup target install x86_64-unknown-linux-musl

# Creating a placeholder project
RUN USER=root cargo new accounts
WORKDIR /usr/src/accounts

# moving deps info
COPY ./Cargo.toml ./Cargo.toml

# Caching deps
RUN cargo build --bin accounts --target x86_64-unknown-linux-musl --release
RUN rm -rf target/x86_64-unknown-linux-musl/release/deps/rust*

# Replacing with actual src
RUN rm src/*.rs
COPY ./src ./src

# Only code changes should need to compile
RUN cargo build --bin accounts --target x86_64-unknown-linux-musl --release

# This creates a TINY container with the executable! Like 4-5mb srsly
FROM scratch
COPY --from=build /usr/src/accounts/target/x86_64-unknown-linux-musl/release/accounts .
USER 1000
CMD ["./accounts"]