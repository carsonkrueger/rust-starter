# === Stage 1: Build ===
FROM rust:1.91-slim-bookworm AS builder

WORKDIR /build

RUN apt-get update && apt-get install -y \
    build-essential \
    libpq-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

COPY app/Cargo.toml app/Cargo.lock ./app/
COPY app/src ./app/src
COPY libs ./libs

RUN cargo build --release --manifest-path app/Cargo.toml

# === Stage 2: Runtime ===
FROM debian:bookworm-slim

WORKDIR /usr/local/bin/app

RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    bash \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /usr/local/bin/app/public
RUN mkdir -p /usr/local/bin/app/migrations

COPY --from=builder /usr/local/cargo/bin/diesel .
COPY --from=builder /build/app/target/release/app .
COPY app/public ./public
COPY app/migrations ./migrations
COPY app/diesel.toml ./
COPY app/entrypoint.sh ./

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/app/entrypoint.sh"]
