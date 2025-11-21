# Builder stage
FROM rust:1.74-slim as builder

WORKDIR /app

# Install dependencies for building (including libpq for diesel)
RUN apt-get update && apt-get install -y libpq-dev pkg-config

COPY . .

# Build the release binary for the api package
RUN cargo build --release -p api

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libpq-5 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/api /app/hserver

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["./hserver"]
