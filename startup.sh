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

# Fonction pour configurer le .env
configure_env() {
    echo -e "\n${YELLOW}Configuration de vos accès Twitch :${NC}"
    
    # 1. Pseudo du Bot
    read -p "👉 Entrez le pseudo Twitch du compte BOT : " username
    while [[ -z "$username" ]]; do
        read -p "⚠️ Le pseudo ne peut pas être vide : " username
    done

    # 2. Token OAuth
    echo -e "\n${YELLOW}⚠️  IMPORTANT : SÉCURITÉ DU TOKEN${NC}"
    echo -e "${BLUE}Pour le développement/test rapide :${NC}"
    echo -e "👉 https://twitchtokengenerator.com/"
    
    echo -e "\n${RED}Pour une utilisation réelle (Production) :${NC}"
    echo -e "Il est fortement recommandé de créer votre propre application sur :"
    echo -e "👉 https://dev.twitch.tv/console"
    echo -e "Cela vous garantit un contrôle total sur vos accès."

    read -p "👉 Entrez votre Access Token (commençant par oauth:) : " oauth
    while [[ ! $oauth == oauth:* ]]; do
        echo -e "${RED}⚠️ Le token doit commencer par 'oauth:'${NC}"
        read -p "👉 Réessayez : " oauth
    done

    # 3. Chaîne Twitch
    read -p "👉 Sur quelle chaîne le bot doit-il pêcher ? (pseudo de la chaine) : " channel
    while [[ -z "$channel" ]]; do
        read -p "⚠️ Le nom de la chaîne ne peut pas être vide : " channel
    done

    # Création du fichier .env
    cat <<EOF > .env
# Twitch Configuration
TWITCH_USERNAME=$username
TWITCH_OAUTH_TOKEN=$oauth
TWITCH_CHANNEL=$channel

# Database (URL used inside Docker)
DATABASE_URL=sqlite:///app/data/fisherman.db

# Logging
RUST_LOG=info
EOF

    echo -e "\n${GREEN}✅ Fichier .env créé avec succès !${NC}"
}

# Vérification de l'existence du .env ou des valeurs par défaut
if [ ! -f .env ]; then
    echo -e "${YELLOW}ℹ️  Premier lancement détecté.${NC}"
    configure_env
else
    # Vérifier si le .env contient encore les placeholders du template
    if grep -q "ton_pseudo_bot" .env; then
        echo -e "${YELLOW}⚠️  Votre fichier .env n'est pas encore configuré.${NC}"
        configure_env
    fi
fi

# Création du dossier data
if [ ! -d data ]; then
    echo -e "\n📂 Création du dossier 'data' pour la base de données..."
    mkdir data
    chmod 777 data
fi

echo -e "\n${BLUE}🚀 Lancement du bot via Docker Compose...${NC}"
docker compose up --build -d

echo -e "${BLUE}------------------------------------------${NC}"
echo -e "${GREEN}✅ INSTALLATION RÉUSSIE !${NC}"
echo -e "🌐 Votre interface web est disponible sur : ${BLUE}http://localhost:3000${NC}"
echo -e "📜 Pour voir ce que fait le bot : ${YELLOW}docker compose logs -f bot${NC}"
echo -e "${BLUE}------------------------------------------${NC}"
