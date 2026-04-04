# Image de compilation
FROM rust:latest AS builder

WORKDIR /app

# Copie uniquement le Cargo.toml d'abord
COPY Cargo.toml ./
# Copie le Cargo.lock seulement s'il existe
COPY Cargo.lock* ./

# Création d'un projet vide pour compiler les dépendances (cache Docker)
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Copie du code source
COPY src ./src

# Compilation finale
RUN cargo build --release

# Image finale légère
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y sqlite3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copie de l'exécutable
COPY --from=builder /app/target/release/fisherman-rust /usr/local/bin/fisherman-rust

CMD ["fisherman-rust"]
