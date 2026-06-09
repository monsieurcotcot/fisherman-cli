#!/bin/bash
# Couleurs pour le terminal
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${YELLOW}📢 Notification des utilisateurs Twitch (Début de la mise à jour)...${NC}"
# Appel de l'API locale du bot pour poster le message d'attente sur Twitch
curl -s -X POST http://localhost:3000/api/maintenance > /dev/null || true

echo -e "${BLUE}🐳 Lancement de Docker Compose (Reconstruction & Redémarrage)...${NC}"
FIX_UID=$(id -u) FIX_GID=$(id -g) docker compose up --build -d
