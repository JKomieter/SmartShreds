# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/smartshreds

# Copy the Cargo.toml and Cargo.lock files
COPY smartshreds/Cargo.toml smartshreds/Cargo.lock ./

# Copy the source code
COPY smartshreds/src ./src
COPY smartshreds/src/resources ./resources
COPY smartshreds/src/org.gtk_rs.SmartShreds.gschema.xml ./

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:buster-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libgtk-3-0 \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/local/bin

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/smartshreds/target/release/smartshreds .

# Copy the resources
COPY --from=builder /usr/src/smartshreds/resources ./resources
COPY --from=builder /usr/src/smartshreds/org.gtk_rs.SmartShreds.gschema.xml ./

# Set the entrypoint to the application binary
ENTRYPOINT ["./smartshreds"]