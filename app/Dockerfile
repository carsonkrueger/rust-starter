# === Stage 1: Build ===
FROM rust:1.91-alpine3.22 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Build dependencies, but discard the output (the actual application)
# This forces the caching of all dependency compilation
RUN mkdir src \
    && echo "fn main() {println!(\"placeholder\");}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY src ./src

RUN cargo build --release

# === Stage 2: Runtime ===
FROM alpine:3.19 AS runtime

WORKDIR /usr/local/bin

RUN apk add --no-cache postgresql-client bash

COPY --from=builder /app/target/release/app .
COPY entrypoint.sh .

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# CMD ["./your_app_name"]
