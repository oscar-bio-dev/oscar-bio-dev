# Builder stage for Frontend and Backend
FROM rust:1.80-alpine AS builder

# Install build dependencies, musl-tools, and Node.js (if needed for any JS tooling, but Trunk handles wasm)
RUN apk add --no-cache musl-dev pkgconfig openssl-dev perl make

# Add Wasm target for Leptos frontend
RUN rustup target add wasm32-unknown-unknown
# Install Trunk for building the Leptos SPA
RUN wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.20.1/trunk-x86_64-unknown-linux-musl.tar.gz | tar -xzf- -C /usr/local/bin

WORKDIR /app

# Copy the entire workspace
COPY . .

# Build the Frontend (Wasm)
WORKDIR /app/frontend
# Note: Since the backend serves `../frontend/dist`, we need to make sure the paths match in the final container.
# We will copy the `dist` folder to `/app/frontend/dist` in the final container.
RUN trunk build --release

# Build the Backend (Musl static binary)
WORKDIR /app
RUN cargo build --bin backend --release --target x86_64-unknown-linux-musl

# Final Stage: Scratch (Zero-OS footprint)
FROM scratch

# Set working directory
WORKDIR /app

# The backend expects the frontend dist folder at ../frontend/dist relative to its execution path.
# If we run the binary from /app/backend, we need the dist at /app/frontend/dist.
# So we'll put the binary at /app/backend/server and execute it from /app/backend.
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend /app/backend/server
COPY --from=builder /app/frontend/dist /app/frontend/dist
COPY --from=builder /app/certs /app/backend/certs

# Expose the default ports
EXPOSE 3000
EXPOSE 8443

# Set environment variables
ENV PORT=3000
ENV APP_HOST=0.0.0.0

WORKDIR /app/backend
ENTRYPOINT ["/app/backend/server"]
