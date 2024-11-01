# Use the official Rust image as a builder
FROM rust:1.82.0 as builder

# Set the working directory
WORKDIR /usr/src/oec-server

# Copy the Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Create a new empty shell project
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build only the dependencies
RUN cargo build --release

# Copy the source code
COPY . .

# Build the actual project
RUN cargo build --release

# Use a minimal base image for the final container
FROM ubuntu:22.04

# Install required runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/oec-server/target/release/oec-server /usr/local/bin/oec-server

# Set the command to run the binary
CMD ["oec-server"]
