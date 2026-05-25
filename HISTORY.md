# 📜 Historique Technique & Évolutions (Fisherman Rust)

Ce document retrace les grandes étapes techniques, les migrations de données et les choix d'architecture effectués pour le projet.

---

## 📅 Mai 2026 - Migration "Unique ID & Auto-Restore"

### 🚀 Objectif
Passer d'un système de stockage simple à une structure RPG plus robuste permettant l'économie (revente/échange) et garantissant la survie des données face aux mises à jour du code.

### 🛠️ Changements Majeurs
1.  **Identifiants Uniques (IDs)** :
    *   Ajout d'une colonne `id` (PRIMARY KEY) dans la table `catches`.
    *   Mise à jour du modèle Rust `Fish` pour inclure cet ID.
    *   Modification du Frontend (`static/index.html`) pour afficher l'ID unique sur chaque objet de l'inventaire.

2.  **Système de Backup JSON Hybride** :
    *   Implémentation d'une tâche de fond asynchrone qui sauvegarde les statistiques critiques des joueurs (`players`) dans `data/players_backup.json` toutes les 5 minutes.
    *   Données sauvegardées : Pseudo, Niveau, XP, Tentatives totales, Succès/Échecs, Statut VIP.

3.  **Migration Automatique (Wipe & Simulate)** :
    *   **Détection** : Le bot vérifie au démarrage si le schéma de la base de données est à jour.
    *   **Logique de Restauration** : Si la base est vide ou obsolète, le bot recrée les tables et lance une simulation.
    *   **Simulation** : Pour chaque joueur restauré, le bot relance un cycle de génération aléatoire basé sur son nombre de tentatives historiques. Cela garantit un inventaire cohérent avec l'XP tout en permettant de mettre à jour les taux de drop sans intervention manuelle.

### ⚠️ Incident & Résolution (Le 7 Mai 2026)
*   **Problème** : Erreur `SqliteError { code: 14, message: "unable to open database file" }` lors du premier wipe manuel.
*   **Cause** : Fichiers temporaires SQLite (`-wal` et `-shm`) orphelins et conflit de permissions entre l'hôte et le conteneur Docker.
*   **Solution** : 
    *   Arrêt complet du conteneur.
    *   Nettoyage forcé du dossier `data/` (suppression de tous les fichiers `.db*`).
    *   Réinitialisation des droits du dossier (`chown 1000:1000`).
    *   Le bot a recréé la base proprement au redémarrage et a simulé avec succès la restauration des 29 joueurs actifs.

---

## 📌 Architecture de Persistence
*   **Source de Vérité 1** : `data/fisherman.db` (SQLite) - Pour les opérations rapides et le Web.
*   **Source de Vérité 2 (Backup)** : `data/players_backup.json` - Pour la survie à long terme et les migrations de schéma.

## 💡 Leçons Apprises
*   **Docker & SQLite** : Toujours s'assurer que le dossier monté en volume appartient à l'UID du conteneur avant que SQLite ne tente de créer les fichiers WAL.
*   **Simulation** : Utiliser `rand::random()` ou s'assurer que le générateur est compatible avec le trait `Send` de Tokio pour éviter de bloquer l'exécuteur asynchrone lors de simulations massives.

---

## 📅 Mai 2026 - Optimisations de Performance & Robustesse VIP

### 🚀 Objectif
Améliorer la réactivité du site web et garantir la fiabilité des mécaniques de jeu (Grades VIP) sur le long terme.

### 🛠️ Changements Majeurs
1.  **Indexation de la Base de Données** :
    *   Ajout d'index SQL sur `catches(player_id)` et `players(username)`.
    *   **Impact** : Temps de réponse du site web divisé par 10 lors de la lecture d'inventaires volumineux.

2.  **Automatisation du Nettoyage VIP** :
    *   Remplacement du système de "sleep" temporaire par une **tâche de fond permanente** (loop) s'exécutant chaque minute.
    *   Le bot vérifie désormais en base de données si des dates d'expiration sont dépassées et communique avec l'API Twitch Helix pour retirer les droits.
    *   **Impact** : Le système VIP est désormais résilient aux redémarrages et aux crashs du bot.

3.  **Nettoyage Infrastructure** :
    *   Purge complète des caches Docker (`builder prune -a`).
    *   **Espace libéré** : ~40 Go au total sur la session.

---

## 📅 Mai 2026 - Version 1.1.0-beta (Coût de pêche, Fidélité & Modularisation JSON)

### 🚀 Objectif
Faire évoluer le gameplay avec une économie plus engageante (coût de pêche, fidélité quotidienne) tout en modularisant la base de code pour faciliter l'enrichissement par IA et améliorer les performances.

### 🛠️ Changements Majeurs

1.  **Modularisation du Catalogue de Données (JSON)** :
    *   **Séparation** : Scindé le fichier monolithique de 6000 lignes (`data/game_data.json`) en trois fichiers JSON spécialisés : `data/fish_data.json` (poissons), `data/junk_data.json` (déchets) et `data/fail_messages.json` (phrases d'échec).
    *   **Validation au Build** : Réécriture de `build.rs` pour valider syntaxiquement les fichiers JSON à chaque compilation. Cargo recompile le projet si l'un d'eux change.
    *   **Chargement Asynchrone & Fallback** : Chargement dynamique asynchrone des fichiers JSON au démarrage dans le `OnceLock`, avec un système de repli (`include_str!`) pour garantir la résilience du bot si les fichiers hôtes manquent.
    *   **Script IA** : Adapté `enrich.py` pour cibler exclusivement `fish_data.json`.

2.  **Lancer Payant & Anti-Négatif (Économie)** :
    *   Chaque tentative de pêche (`!fish` / `!peche`) coûte désormais **10 pièces d'or**.
    *   **Sécurité SQL** : Déduction directe en base de données via `gold = MAX(0, gold - 10)` pour éviter tout solde négatif.
    *   **Blocage Gameplay** : Bloque automatiquement la commande si le solde du joueur est < 10 pièces d'or (hors administrateurs et grades `testvip`).

3.  **Récompense Quotidienne de Fidélité (Loyauté)** :
    *   Crédite automatiquement un bonus de pièces d'or lors du premier message de la journée de chaque joueur (UTC NaiveDate).
    *   **Formule progressive** : `Gains = 200 + 50 * min(jours_consecutifs, 10) + 10 * jours_totaux`.
    *   **Optimisation RAM** : Stockage du cache de réclamation dans un `RwLock<HashMap<String, NaiveDate>>` dans `AppState` pour éliminer les requêtes SQL redondantes à chaque message de chat Twitch.

4.  **Version 1.1.0-beta & Cache Busting** :
    *   Mise à jour de la version du paquet dans `Cargo.toml` à `1.1.0-beta`.
    *   Mise à jour du site web (`static/index.html`) pour afficher la version `Beta V1.1.0`.
    *   Busting automatique du cache navigateur en changeant les paramètres de version CSS et JS en `?v=1.1.0`.

### ⚠️ Incident & Résolution (Permissions Docker & Re-compilation)
*   **Problème** : Erreur `SqliteError: unable to open database file` et panic `index out of bounds: the len is 15 but the index is 15`.
*   **Causes** :
    1.  Conflit d'UID/GID hôte-conteneur : le conteneur démarrait sous l'UID `1000` par défaut alors que le répertoire hôte appartenait à l'utilisateur `1001` (micka).
    2.  Le cache de build de Docker a réutilisé l'ancien binaire compilé avec le vieux schéma SQL à 11 colonnes (manquant les 3 nouvelles colonnes de fidélité quotidienne).
*   **Solutions** :
    *   Ajout permanent des variables de permissions dans le fichier de configuration `.env` de l'hôte (`FIX_UID=1001`, `FIX_GID=1001`).
    *   Nettoyage et recréation du fichier de base de données `fisherman.db` sur l'hôte avec les permissions `644`.
    *   Reconstruction complète du conteneur sans cache (`docker compose build --no-cache`) pour forcer Cargo à embarquer la nouvelle migration à 14 colonnes dans le binaire compilé.
    *   Wipe de sécurité et réinitialisation de sauvegarde à blanc (tableau vide `[]`) pour démarrer le cycle V1.1.0 proprement.

