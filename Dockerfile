# Stage 1: Set muslbuilder
FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
USER root
RUN cargo install cargo-chef
WORKDIR /app

# Stage 2: Prepare planner
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 4: Create release image
FROM alpine:latest
RUN addgroup -S my-server-user && adduser -S my-server-user -G my-server-user

# Copy the build artifact from the build stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /usr/local/bin/server
COPY --from=builder /app/assets /assets
USER my-server-user

# Install upx
#RUN apk add upx --no-cache ca openssl

# Run upx
#RUN upx --ultra-brute /server

# Set the startup command to run your binary
ENTRYPOINT  ["/usr/local/bin/server"]
EXPOSE 9020
