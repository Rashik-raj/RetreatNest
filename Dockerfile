# Stage 1: Build the Axum application
FROM rust:latest AS builder

WORKDIR /app

# Copy Cargo.toml and Cargo.lock first to leverage Docker layer caching
COPY . .

RUN apt-get update && apt-get install -y cmake libclang-dev && rm -rf /var/lib/apt/lists/*


# Build the Axum application in release mode
RUN cargo build --release --workspace

# Stage 2: Create the final runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install necessary runtime dependencies (e.g., OpenSSL if used)
# RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/my_retreat_nest /app/my_retreat_nest
COPY --from=builder /app/target/release/migration /app/migration

# Set the entrypoint to run the application
CMD ["/app/my_retreat_nest"]

# Expose the port your Axum application listens on
EXPOSE 8000