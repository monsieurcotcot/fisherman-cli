#!/bin/bash

# FisherAudit - Script d'Audit de Sécurité Universel
# ------------------------------------------------

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}   🛡️  FISHERAUDIT : ANALYSE SÉCURITÉ      ${NC}"
echo -e "${BLUE}==========================================${NC}"

# 1. Vérification des fichiers sensibles
echo -e "\n${YELLOW}[1/4] Vérification des fichiers locaux...${NC}"
if [ -f .env ]; then
    perms=$(stat -c "%a" .env)
    if [ "$perms" -le "600" ]; then
        echo -e "${GREEN}✅ .env est bien protégé ($perms)${NC}"
    else
        echo -e "${RED}❌ ATTENTION : .env a des permissions trop larges ($perms).${NC}"
        echo -e "👉 Conseil : chmod 600 .env"
    fi
else
    echo -e "${BLUE}ℹ️  Aucun fichier .env trouvé.${NC}"
fi

# 2. Audit des Headers HTTP (si URL fournie)
if [ ! -z "$1" ]; then
    echo -e "\n${YELLOW}[2/4] Audit des en-têtes HTTP pour $1...${NC}"
    headers=$(curl -s -I "$1")
    
    if [ -z "$headers" ]; then
        echo -e "${RED}❌ Impossible de joindre l'URL. Vérifiez votre connexion.${NC}"
    else
        check_header() {
            if echo "$headers" | grep -qi "$1"; then
                echo -e "${GREEN}✅ $1 est présent${NC}"
            else
                echo -e "${RED}❌ $1 est MANQUANT !${NC}"
            fi
        }

        check_header "content-security-policy"
        check_header "x-frame-options"
        check_header "x-content-type-options"
        check_header "strict-transport-security"
    fi
else
    echo -e "\n${BLUE}ℹ️  Audit HTTP sauté (fournissez une URL en argument pour tester).${NC}"
    echo -e "👉 Exemple : ./audit.sh https://votre-site.com"
fi

# 3. Audit des dépendances Rust (si Cargo.lock présent)
if [ -f Cargo.lock ]; then
    echo -e "\n${YELLOW}[3/4] Analyse des dépendances Rust (Cargo Audit)...${NC}"
    docker run --rm -v "$(pwd):/app" -w /app rust:latest sh -c "cargo install cargo-audit && cargo audit"
else
    echo -e "\n${BLUE}ℹ️  Audit Rust sauté (aucun Cargo.lock trouvé).${NC}"
fi

# 4. Audit de l'image Docker (si nom fourni en 2e argument)
if [ ! -z "$2" ]; then
    echo -e "\n${YELLOW}[4/4] Analyse de l'image Docker ($2)...${NC}"
    docker run --rm -v /var/run/docker.sock:/var/run/docker.sock aquasec/trivy:latest image --severity CRITICAL,HIGH "$2"
else
    echo -e "\n${BLUE}ℹ️  Audit Docker sauté (fournissez un nom d'image en 2e argument).${NC}"
    echo -e "👉 Exemple : ./audit.sh https://votre-site.com fisherman-cli-bot"
fi

echo -e "\n${BLUE}==========================================${NC}"
echo -e "${GREEN}✅ Audit terminé !${NC}"
echo -e "${BLUE}==========================================${NC}"
