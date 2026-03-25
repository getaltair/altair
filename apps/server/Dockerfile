# Multi-stage Dockerfile for altair-server
# Stage 1: Builder
FROM rustlang/rust:nightly-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

# Set working directory
WORKDIR /app

# Copy server source and Cargo.toml
COPY apps/server/Cargo.toml ./
COPY apps/server/src ./src

# Build application in release mode
RUN cargo build --release

# Stage 2: Runtime
FROM alpine:3.20

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata wget

# Create non-root user for security
RUN addgroup -g 1000 altair && \
    adduser -D -u 1000 -G altair altair

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/altair-server /app/altair-server

# Change ownership to non-root user
RUN chown -R altair:altair /app

# Switch to non-root user
USER altair

# Expose default port
EXPOSE 3000

# Set timezone to UTC
ENV TZ=Etc/UTC

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD wget -qO- http://localhost:3000/health || exit 1

# Run application
CMD ["/app/altair-server"]
