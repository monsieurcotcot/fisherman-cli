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
    default_host="fisherman-cli.cotcotuniverse.com"
    echo -e "${BLUE}ℹ️  Configuration du domaine public${NC}"
    read -p "👉 Adresse IP ou Domaine (Défaut: $default_host) : " host_addr
    host_addr=${host_addr:-$default_host}

    # NETTOYAGE de l'entrée utilisateur
    # Retirer http:// ou https:// si présent
    host_addr=$(echo "$host_addr" | sed -e 's|^[^/]*//||')
    # Retirer les slashes à la fin
    host_addr=$(echo "$host_addr" | sed -e 's|/*$||')

    # Déterminer le protocole (https pour les domaines, http pour localhost/IP)
    protocol="http"
    if [[ "$host_addr" == *"."* && "$host_addr" != *"192.168."* ]]; then
        protocol="https"
    fi

    # Construction de l'URL propre
    redirect_uri="$protocol://$host_addr/auth/callback"

    echo -e "\n${YELLOW}2. Configuration de votre application Twitch Dev :${NC}"
    echo -e "${BLUE}ℹ️  Lien : https://dev.twitch.tv/console/apps${NC}"
    echo -e "${GREEN}✅ OAuth Redirect URL à copier dans Twitch :${NC}"
    echo -e "${BLUE}👉 $redirect_uri${NC}"
    echo -e "------------------------------------------"
    
    # 2. Client ID
    read -p "👉 Entrez votre Client ID : " client_id
    while [[ -z "$client_id" ]]; do read -p "⚠️ Le Client ID ne peut pas être vide : " client_id; done

    # 3. Client Secret
    read -p "👉 Entrez votre Client Secret : " client_secret
    while [[ -z "$client_secret" ]]; do read -p "⚠️ Le Client Secret ne peut pas être vide : " client_secret; done

    # 4. Chaîne Twitch
    read -p "👉 Sur quelle chaîne le bot doit-il pêcher ? : " channel
    while [[ -z "$channel" ]]; do read -p "⚠️ Le nom de la chaîne ne peut pas être vide : " channel; done

    # Création du fichier .env propre
    cat <<EOF > .env
# Twitch Configuration
TWITCH_CLIENT_ID=$client_id
TWITCH_CLIENT_SECRET=$client_secret
TWITCH_CHANNEL=$channel
REDIRECT_URI=$redirect_uri

# Database
DATABASE_URL=sqlite:///app/data/fisherman.db

# Logging
RUST_LOG=info
EOF

    echo -e "\n${GREEN}✅ Configuration enregistrée dans le fichier .env !${NC}"
}

# On force la reconfiguration si le .env est partiel ou ancien
if [ ! -f .env ] || grep -q "TWITCH_USERNAME" .env || [ -z "$(grep REDIRECT_URI .env)" ]; then
    configure_env
fi

# Préparation de l'environnement de données (Sécurisé)
mkdir -p data
# Permissions normales : propriétaire (vous) a tous les droits, les autres lisent
chmod 755 data
if [ ! -f data/fisherman.db ]; then
    touch data/fisherman.db
    chmod 644 data/fisherman.db
fi

echo -e "\n${BLUE}🚀 Lancement du conteneur Docker...${NC}"
docker compose down --remove-orphans
FIX_UID=$(id -u) FIX_GID=$(id -g) docker compose up --build -d

# Récupérer l'URL d'auth finale pour affichage
auth_url=$(grep REDIRECT_URI .env | cut -d '=' -f2 | sed 's/\/callback/\/login/')

echo -e "${BLUE}------------------------------------------${NC}"
echo -e "${YELLOW}⚠️  DERNIÈRE ÉTAPE :${NC}"
echo -e "Pour lier le bot à votre compte Twitch, ouvrez ce lien :"
echo -e "${GREEN}👉 $auth_url${NC}"
echo -e "${BLUE}------------------------------------------${NC}"
