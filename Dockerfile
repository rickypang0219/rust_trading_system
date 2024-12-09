# Use the official Rust image as a builder
FROM rust:1.70 as builder

# Set the working directory inside the container
WORKDIR /usr/src/rust_trading_system

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new empty shell project to cache dependencies
RUN mkdir src && echo "fn main() { println!(\"Hello, world!\"); }" > src/main.rs

# Build dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a smaller base image to run the application
FROM debian:bullseye-slim

# Update package lists and install ca-certificates
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/rust_trading_system/target/release/rust_trading_system /usr/local/bin/rust_trading_system

# Command to run the application
CMD ["rust_trading_system"]
