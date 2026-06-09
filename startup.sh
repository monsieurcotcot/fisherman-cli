#!/bin/bash

# Couleurs pour le terminal
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}   🎣 ASSISTANT D'INSTALLATION FISHERMAN   ${NC}"
echo -e "${BLUE}==========================================${NC}"

configure_env() {
    echo -e "\n${YELLOW}1. Configuration de l'accès réseau :${NC}"
    
    # 1. Adresse IP / Hostname
    default_host="fisherman-cli.monsieurcotcot.com"
    echo -e "${BLUE}ℹ️  Configuration du domaine public${NC}"
    read -p "👉 Adresse IP ou Domaine (Défaut: $default_host) : " host_addr
    host_addr=${host_addr:-$default_host}

    host_addr=$(echo "$host_addr" | sed -e 's|^[^/]*//||' -e 's|/*$||')
    protocol="http"
    if [[ "$host_addr" == *"."* && "$host_addr" != *"192.168."* ]]; then protocol="https"; fi
    redirect_uri="$protocol://$host_addr/auth/callback"

    echo -e "\n${YELLOW}2. Configuration de votre application Twitch Dev :${NC}"
    echo -e "${BLUE}ℹ️  Lien : https://dev.twitch.tv/console/apps${NC}"
    echo -e "${GREEN}✅ OAuth Redirect URL à copier dans Twitch :${NC}"
    echo -e "${BLUE}👉 $redirect_uri${NC}"
    echo -e "------------------------------------------"
    
    read -p "👉 Entrez votre Client ID : " client_id
    while [[ -z "$client_id" ]]; do read -p "⚠️ Le Client ID ne peut pas être vide : " client_id; done

    read -p "👉 Entrez votre Client Secret : " client_secret
    while [[ -z "$client_secret" ]]; do read -p "⚠️ Le Client Secret ne peut pas être vide : " client_secret; done

    read -p "👉 Sur quelle chaîne le bot doit-il pêcher ? : " channel
    while [[ -z "$channel" ]]; do read -p "⚠️ Le nom de la chaîne ne peut pas être vide : " channel; done

    # 5. Admin Token (Sécurité)
    admin_token=$(openssl rand -hex 12)

    cat <<EOF > .env
# Twitch Configuration
TWITCH_CLIENT_ID=$client_id
TWITCH_CLIENT_SECRET=$client_secret
TWITCH_CHANNEL=$channel
REDIRECT_URI=$redirect_uri

# Security
ADMIN_TOKEN=$admin_token

# Database
DATABASE_URL=sqlite:///app/data/fisherman.db

# Logging
RUST_LOG=info
EOF

    echo -e "\n${GREEN}✅ Configuration enregistrée dans le fichier .env !${NC}"
}

if [ ! -f .env ] || ! grep -q "ADMIN_TOKEN" .env; then
    configure_env
fi

# Récupérer les infos pour l'affichage final
admin_token=$(grep ADMIN_TOKEN .env | cut -d '=' -f2)
base_url=$(grep REDIRECT_URI .env | cut -d '=' -f2 | sed 's/\/auth\/callback//')

# Préparation de l'environnement de données (Sécurisé)
mkdir -p data
chmod 755 data
if [ ! -f data/fisherman.db ]; then
    touch data/fisherman.db
    chmod 644 data/fisherman.db
fi

echo -e "\n${YELLOW}📢 Notification des utilisateurs Twitch...${NC}"
curl -s -X POST http://localhost:3000/api/maintenance > /dev/null || true

echo -e "\n${BLUE}🚀 Lancement du conteneur Docker...${NC}"
docker compose down --remove-orphans
FIX_UID=$(id -u) FIX_GID=$(id -g) docker compose up --build -d

echo -e "${BLUE}------------------------------------------${NC}"
echo -e "${YELLOW}⚠️  DERNIÈRE ÉTAPE (ADMINISTRATION) :${NC}"
echo -e "Pour connecter le Bot et le Streameur, rendez-vous sur :"
echo -e "\n${BLUE}🔐 Panel Secret (Lien unique) :${NC}"
echo -e "${GREEN}👉 $base_url/admin-cotcot?token=$admin_token${NC}"
echo -e "\n${RED}🔴 NE PARTAGEZ JAMAIS CE LIEN !${NC}"
echo -e "${BLUE}------------------------------------------${NC}"
