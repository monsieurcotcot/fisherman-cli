# Image de compilation
FROM rust:latest AS builder

WORKDIR /app

# 1. Copie des fichiers de configuration de compilation
COPY Cargo.toml Cargo.lock build.rs ./

# 2. Création d'un projet dummy et compilation des dépendances
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# 3. Suppression des sources dummy et du binaire temporaire pour forcer la recompilation
RUN rm -rf src/ target/release/fisherman-rust* target/release/deps/fisherman_rust*

# 4. Copie de l'intégralité du projet pour la compilation finale
COPY . .

# 5. Compilation de notre application réelle
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
