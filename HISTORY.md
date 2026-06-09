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

### ⚠️ Incidents & Résolutions (Permissions Docker & Purge SQL)
*   **Incident 1 : Permissions & Caching Docker**
    *   **Problème** : Erreur `SqliteError: unable to open database file` et panic `index out of bounds: the len is 15 but the index is 15` au démarrage.
    *   **Causes** :
        1.  Conflit d'UID/GID hôte-conteneur : le conteneur démarrait sous l'UID `1000` par défaut alors que le répertoire hôte appartenait à l'utilisateur `1001` (micka).
        2.  Le cache de build de Docker a réutilisé l'ancien binaire compilé avec le vieux schéma SQL à 11 colonnes.
    *   **Solutions** : Ajout des variables `FIX_UID=1001`/`FIX_GID=1001` dans `.env`, création de `fisherman.db` en `644`, et compilation `docker compose build --no-cache`.

*   **Incident 2 : Crash du Bot après une commande `!fish purge` en direct**
    *   **Problème** : Après l'envoi de `!fish purge` suivi de `!fish purge yes` par le streamer dans le chat Twitch, le bot a immédiatement planté dès qu'un joueur a tenté de pêcher (`ColumnNotFound("last_daily_reward_at")`).
    *   **Cause** : La méthode de réinitialisation `purge_all_data()` (ainsi que la méthode de test `setup_db()`) dans `src/db/repository.rs` recréait la table `players` à l'aide d'une requête SQL `CREATE TABLE` **écrite en dur (hardcoded)**. Cette requête utilisait l'ancien schéma à 11 colonnes au lieu du nouveau schéma à 14 colonnes (manquant les colonnes de fidélité quotidienne).
    *   **Solutions** : 
        *   Mise à jour des requêtes `CREATE TABLE players` en dur dans `purge_all_data()` et `setup_db()` de `src/db/repository.rs` pour inclure les 14 colonnes de la V1.1.0.
        *   Reconstruction complète du conteneur sans cache (`docker compose build --no-cache`) pour intégrer ces correctifs de code.
        *   Nettoyage et réinitialisation de `fisherman.db` sur l'hôte.

---

## 📅 Juin 2026 - Version 1.2.6 (Ferraille, Recyclage & Événement Déchets Quotidien)

### 🚀 Objectif
Introduire un système de recyclage des déchets pour récupérer de la ferraille revendable, et augmenter l'engagement lors des streams en créant un événement quotidien automatique qui booste le taux d'apparition des déchets en fonction de l'activité du stream précédent.

### 🛠️ Changements Majeurs

1.  **Système de Ferraille & Recyclage** :
    *   **Recyclage des Déchets** : Analyse de la méthode de recyclage de chaque déchet (poubelle noire, bleue, marron ou décharge). Possibilité de démonter certains déchets pour en extraire des pièces métalliques (ferraille en kg).
    *   **Économie de la Ferraille** : Ajout d'une colonne `scrap_metal` (ferraille possédée) et `total_sold_scrap_metal` dans la table `players`.
    *   **Commande `!fish sell ferrailles`** : Permet de vendre sa ferraille accumulée à un prix dynamique fluctuant à chaque stream.

2.  **Événement Déchets Quotidien (Daily Junk Event)** :
    *   **Objectif dynamique par jour de stream** : À chaque nouveau jour de stream, un objectif de déchets à pêcher est défini (entre 30 et 150).
    *   **Ajustement basé sur l'activité** : Le maximum de cet objectif est calculé dynamiquement sur la base de l'activité du stream précédent : `max_déchets = (lancers_précédents / 10).clamp(30, 150)`. L'objectif du jour est tiré aléatoirement entre 30 et ce maximum.
    *   **Taux de drop quadruplé** : Tant que l'objectif de déchets n'est pas atteint pour la journée en cours, le taux d'apparition des déchets passe de 5 % à **20 % (quadruplé)**. Dès que l'objectif est rempli, il revient à la normale (5 %).
    *   **Notification et Progression** : Ajout d'un suffixe de progression dans le chat Twitch lors de la capture d'un déchet durant l'événement (ex: `(Événement Déchets : 12/69)`).
    *   **Persistence SQL** : Création de la table `daily_stream_stats` pour suivre les statistiques quotidiennes de l'événement (`live_date`, `total_attempts`, `junk_target`, `junk_caught`).

### ⚠️ Incidents & Résolutions (RNG non-Send sous Tokio)
*   **Problème** : Erreur de compilation Cargo `future cannot be sent between threads safely` car le type de retour du futur créé par le bloc asynchrone n'était pas `Send`.
*   **Cause** : Le générateur de nombres aléatoires `rand::thread_rng()` (de type `ThreadRng`, non-`Send`) était gardé en mémoire locale à travers un point d'attente `.await` lors de l'insertion SQL dans la méthode `get_or_update_daily_junk_event`.
*   **Solution** : Encapsulation de l'instanciation et de l'utilisation de `rand::thread_rng()` dans un bloc imbriqué `{ ... }` autonome pour s'assurer que le générateur soit détruit avant le premier point `.await`.

---

## 📅 Juin 2026 - Version 1.2.7 (Badge Unique Premier Millionnaire)

### 🚀 Objectif
Ajouter un badge unique et exclusif de "Premier Millionnaire" attribué de manière permanente et immuable au premier joueur atteignant 1 000 000 pièces d'or.

### 🛠️ Changements Majeurs
1. **Badge Unique & Persistance** :
   * Ajout de la colonne `millionaire_at` (timestamp UTC) dans la table `players`.
   * Migration automatique pour marquer `dozerker` comme le premier millionnaire actuel.
   * Logique atomique et exempte de race condition : validation au sein d'une transaction SQL pour vérifier qu'aucun autre joueur n'a déjà ce titre (`millionaire_at IS NOT NULL`) avant de l'attribuer.
2. **Interface web (HTML/JS)** :
   * Ajout du badge avec un dégradé doré stylé et un emoji 🪙.
   * Toggle automatique de visibilité dans le profil du joueur.
   * Incrémentation des paramètres de version `?v=1.2.7` dans les tags script/link pour forcer le rafraîchissement du cache Cloudflare et du navigateur.
