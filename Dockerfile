# Image de compilation
FROM rust:latest AS builder

WORKDIR /app

# Copie de tout le projet
COPY . .

# Compilation
RUN cargo build --release

# Image finale légère
FROM debian:bookworm-slim

# Installation des dépendances et création d'un utilisateur non-root
RUN apt-get update && apt-get install -y sqlite3 ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -ms /bin/bash fisherman

WORKDIR /app

# Copie de l'exécutable
COPY --from=builder /app/target/release/fisherman-rust /usr/local/bin/fisherman-rust

# COPIE DU DOSSIER STATIC
COPY --from=builder /app/static ./static

# Donner la propriété du dossier /app à l'utilisateur fisherman
RUN chown -R fisherman:fisherman /app

# Utiliser l'utilisateur non-root
USER fisherman

CMD ["fisherman-rust"]
