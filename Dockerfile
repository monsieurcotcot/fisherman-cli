# Image de compilation
FROM rust:latest AS builder

WORKDIR /app

# Copie de tout le projet
COPY . .

# Compilation
RUN cargo build --release

# Image finale légère
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y sqlite3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copie de l'exécutable
COPY --from=builder /app/target/release/fisherman-rust /usr/local/bin/fisherman-rust

CMD ["fisherman-rust"]
