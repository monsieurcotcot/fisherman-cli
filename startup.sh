#!/bin/bash

echo "🎣 Fisherman Twitch Bot - Startup Script"
echo "----------------------------------------"

# 1. Vérifier si le fichier .env existe
if [ ! -f .env ]; then
    echo "❌ Le fichier .env est manquant !"
    echo "👉 Création d'un fichier .env à partir du template..."
    cp .env.template .env
    echo "⚠️  Veuillez remplir vos accès Twitch dans le fichier .env avant de relancer ce script."
    exit 1
fi

# 2. Créer le dossier data s'il n'existe pas (pour la DB)
if [ ! -d data ]; then
    echo "📂 Création du dossier 'data' pour la base de données..."
    mkdir data
    chmod 777 data # S'assurer que Docker peut écrire dedans
fi

# 3. Lancer Docker Compose
echo "🚀 Lancement du bot via Docker Compose..."
docker compose up --build -d

echo "----------------------------------------"
echo "✅ Le bot est en cours de lancement en arrière-plan !"
echo "🌐 Interface web : http://localhost:3000"
echo "📜 Pour voir les logs en temps réel : docker compose logs -f bot"
