# Use the official Rust image as the base image
FROM rust:1.85.0-slim-bookworm as builder

# Install the protobuf compiler
RUN apt-get update && apt-get install -y protobuf-compiler

# create a new empty shell project
WORKDIR /rust_web_server

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies (downloaded but not compiled)
RUN cargo fetch

# Use cargo-build-dependencies plugin to pre-build dependencies
RUN cargo install cargo-build-dependencies
RUN cargo build-dependencies --release

# Copy the source code, build script and proto buffers
COPY src ./src
COPY proto ./proto
COPY build.rs .

# Build the project
RUN cargo build --release

# Use a base image with the required GLIBC version
FROM debian:bookworm-slim

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gcc \
        libc6-dev

# Set the working directory inside the container
WORKDIR /usr/src/rust_web_server

# Copy the built binary from the builder stage
COPY --from=builder /rust_web_server/target/release/server .

# Set the startup command to run the server binary
CMD ["./server"]