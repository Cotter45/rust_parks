# ---- Build Stage ----
FROM rust:1.71.0 as builder
WORKDIR /app

# Copy the source code and data
COPY . .

# Build the application in release mode
RUN cargo build --release

# ---- Run Stage ----
FROM debian:buster-slim
WORKDIR /app

# Copy the binary from the build stage
COPY --from=builder /app/target/release/hello-parks ./hello-parks
COPY --from=builder /app/data ./data

# Set execute permissions on the binary
RUN chmod +x ./hello-parks

# Expose the application on port 8080
EXPOSE 8080

# Command to run the application
CMD ["./hello-parks"]