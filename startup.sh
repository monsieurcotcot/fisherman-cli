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
    echo -e "\n${YELLOW}Configuration de votre application Twitch Dev :${NC}"
    echo -e "${BLUE}ℹ️  Créez une application sur : https://dev.twitch.tv/console${NC}"
    echo -e "${BLUE}ℹ️  OAuth Redirect URL : http://localhost:3000/auth/callback${NC}"
    
    # 1. Client ID
    read -p "👉 Entrez votre Client ID : " client_id
    while [[ -z "$client_id" ]]; do
        read -p "⚠️ Le Client ID ne peut pas être vide : " client_id
    done

    # 2. Client Secret
    read -p "👉 Entrez votre Client Secret : " client_secret
    while [[ -z "$client_secret" ]]; do
        read -p "⚠️ Le Client Secret ne peut pas être vide : " client_secret
    done

    # 3. Chaîne Twitch
    read -p "👉 Sur quelle chaîne le bot doit-il pêcher ? : " channel
    while [[ -z "$channel" ]]; do
        read -p "⚠️ Le nom de la chaîne ne peut pas être vide : " channel
    done

    # 4. Adresse IP / Hostname pour le réseau
    default_ip=$(hostname -I | awk '{print $1}')
    read -p "👉 Adresse IP ou Domaine de cette VM (Défaut: $default_ip) : " host_addr
    host_addr=${host_addr:-$default_ip}

    # Création du fichier .env
    cat <<EOF > .env
# Twitch Configuration
TWITCH_CLIENT_ID=$client_id
TWITCH_CLIENT_SECRET=$client_secret
TWITCH_CHANNEL=$channel
REDIRECT_URI=http://$host_addr:3000/auth/callback

# Database
DATABASE_URL=sqlite:///app/data/fisherman.db

# Logging
RUST_LOG=info
EOF

    echo -e "\n${GREEN}✅ Configuration enregistrée !${NC}"
    echo -e "${YELLOW}⚠️  IMPORTANT : Dans la console Twitch Dev, vous DEVEZ ajouter cette URL de redirection :${NC}"
    echo -e "${BLUE}👉 http://$host_addr:3000/auth/callback${NC}"
}

if [ ! -f .env ] || grep -q "TWITCH_USERNAME" .env; then
    configure_env
fi

# Création du dossier data
mkdir -p data
chmod 777 data

echo -e "\n${BLUE}🚀 Lancement du conteneur...${NC}"
docker compose up --build -d

echo -e "${BLUE}------------------------------------------${NC}"
echo -e "${YELLOW}⚠️  DERNIÈRE ÉTAPE :${NC}"
echo -e "Pour connecter le bot à Twitch, ouvrez ce lien dans votre navigateur :"
echo -e "${GREEN}👉 http://localhost:3000/auth${NC}"
echo -e "${BLUE}------------------------------------------${NC}"
