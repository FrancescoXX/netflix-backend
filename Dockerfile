# STAGE 1: Builder
FROM rust:1.83-slim-bookworm as builder

WORKDIR /app

# Install build dependencies (needed for reqwest/OpenSSL)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy all source code
COPY . .

# Build in release mode
RUN cargo build --release

# STAGE 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies (OpenSSL/CA certs)
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/netflix_backend /usr/local/bin/app

# --- NEW: Copy the video file correctly ---
# We copy from "assets/video.mp4" (local) to "." (root of container)
COPY assets ./assets

# Expose port 8080
ENV PORT=8080
EXPOSE 8080

CMD ["app"]