# 🎣 Fisherman Rust - Bot Twitch & Jeu de Pêche

Application hybride combinant un **Bot Twitch** asynchrone (Rust) et un **Serveur Web** (Axum + Vanilla CSS) pour un jeu de pêche communautaire interactif.

## 🚀 Lancement Rapide

Pour installer ou réinitialiser le projet, utilisez le script interactif :

```bash
chmod +x startup.sh
./startup.sh
```

Ce script configure l'environnement, génère le fichier `.env` et lance les conteneurs Docker.

## 🔐 Administration & Maintenance

Le bot a besoin de tokens Twitch valides pour fonctionner (Chat IRC + API Helix).

### 🔑 Panel d'Administration
Utilisez ce lien unique pour connecter le **Bot** et le **Streameur** (obligatoire pour le statut "En live") :

👉 **[Accès Panel Admin](https://fisherman-cli.monsieurcotcot.com/admin-cotcot?token=39ef4ad8a0c552168e8d8d69)**

*Note : Si le bot ne répond plus ou affiche "Hors live" alors que vous êtes en ligne, reconnectez les deux comptes via ce lien.*

### 🛠 Commandes Docker Utiles

| Action | Commande |
| :--- | :--- |
| **Démarrer / Mettre à jour** | `docker compose up -d --build` |
| **Arrêter** | `docker compose down` |
| **Voir les logs** | `docker compose logs -f fisherman` |
| **Redémarrage rapide** | `docker compose restart fisherman` |

## 📊 Visualisation
Les joueurs peuvent consulter leurs statistiques et le classement mondial ici :
👉 **[Statistiques Fisherman](https://fisherman-cli.monsieurcotcot.com/)**

## 🏗 Stack Technique
- **Langage** : Rust (Tokio runtime)
- **Serveur Web** : Axum
- **Base de données** : SQLite (via SQLx) avec mode WAL activé.
- **Conteneurisation** : Docker & Docker Compose.
- **Exposition** : Cloudflare Tunnel (port 3000).

---
*Dernière mise à jour : Avril 2026 - Migration vers monsieurcotcot.com effectuée.*
