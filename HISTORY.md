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
