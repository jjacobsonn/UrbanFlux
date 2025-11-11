# Multi-stage Dockerfile for UrbanFlux ETL
# Stage 1: Builder
FROM rust:1.83-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/urbanflux /usr/local/bin/urbanflux

# Create necessary directories
RUN mkdir -p /app/testdata /app/runs /app/bad_rows

# Create non-root user
RUN useradd -m -u 1000 urbanflux && \
    chown -R urbanflux:urbanflux /app

USER urbanflux

ENTRYPOINT ["urbanflux"]
CMD ["--help"]
