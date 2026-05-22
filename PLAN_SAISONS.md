# Concept de Saisons et Trophées Éternels : Spécifications et Architecture

Ce document définit les spécifications complètes du système de **saisons**, de **trophées éternels**, et de **réinitialisations** pour le bot de pêche Twitch `fisherman-cli`.

---

## 1. Fonctionnement des Saisons (A, B, C...)

Pour relancer la hype et maintenir la communauté engagée, le streamer peut diviser l'année en plusieurs saisons (ex: **Saison 2026 A**, **Saison 2026 B**, etc.).
- **Données de Saison (Actives) :** Niveaux, XP, inventaires de poissons actifs et bananes. Ces données sont remises à zéro à chaque fin de saison.
- **Données Éternelles :** Archivées à chaque fin de saison dans la table `player_trophies`. Ces récompenses restent affichées à vie sur le profil web du joueur et dans le chat Twitch sous forme de badges.

---

## 2. Liste des Trophées et Badges Éternels

### A. Trophées de Niveau de Fin de Saison
Décernés automatiquement à tous les participants en fonction de leur niveau actif final à la clôture de la saison :
- **Bronze 🥉 :** Niveau 10 à 19 (Badge 🥉 dans le chat)
- **Argent 🥈 :** Niveau 20 à 39 (Badge 🥈 dans le chat)
- **Or 🥇 :** Niveau 40 à 69 (Badge 🥇 dans le chat)
- **Platinium 💎 :** Niveau 70 à 99 (Badge 💎 dans le chat)
- **Diamant ❄️ :** Niveau 100 à 149 (Badge ❄️ dans le chat)
- **Obsidienne 🌌 :** Niveau 150+ (Badge 🌌 dans le chat)

### B. Trophées Spéciaux de Saison
Ces trophées uniques récompensent les comportements et styles de jeu marquants de la saison :
1. **Trophée de la Night 🌙 :** Décerné au joueur ayant effectué le plus de pêches réussies après 22h00 pendant la saison. Parfait pour récompenser les habitués de fin de stream !
2. **Le Roi des Voleurs 🍌 :** Décerné au joueur ayant détenu ou volé la banane sacrée le plus de fois dans la saison.
3. **L'Éboueur des Mers 🧹 :** Décerné au joueur ayant pêché le plus grand nombre de déchets (junk) dans la saison.
4. **Le Pêcheur Divin 👑 :** Décerné au joueur ayant capturé le plus grand nombre de poissons de rareté Divine, Mythique ou Légendaire.

*Chaque trophée spécial octroie un titre et un badge unique dans le chat Twitch (ex: `[🌙 Night 2026A]`, `[🍌 Banane 2026A]`).*

---

## 3. Les Commandes de Réinitialisation

Trois niveaux de reset sont définis pour offrir une flexibilité maximale :

### A. `!fish reset` (Viewer) - Réinitialisation Souple
- **Objectif :** Permet à un viewer de repartir à zéro sur la saison active.
- **Effets :** Vide son inventaire actif de saison, remet son niveau à 1, son XP à 0 et ses compteurs saisonniers à 0.
- **Sécurité :** Conserve intacts tous ses **trophées éternels** obtenus lors des saisons précédentes.
- **Confirmation :** Le joueur doit taper `!fish yes` dans les 2 minutes pour valider.

### B. `!fish reset all` (Viewer) - Réinitialisation Totale
- **Objectif :** Effacer définitivement toute trace du compte.
- **Effets :** Efface les données actives ET détruit définitivement tout son historique de trophées éternels.
- **Confirmation :** Le joueur doit confirmer explicitement avec `!fish yes all` pour éviter toute fausse manipulation.

### C. `!admin season_reset <nom_saison>` (Admin - monsieurcotcot)
- **Objectif :** Clôturer officiellement la saison en cours et en démarrer une nouvelle (ex: `Saison 2026 A`).
- **Déroulement automatique :**
  1. Le bot calcule et attribue les trophées de niveau à tous les joueurs actifs (Niveau >= 10).
  2. Le bot détermine les vainqueurs des trophées spéciaux (ex: *Trophée de la Night 🌙*, *Roi des Voleurs 🍌*) et leur décerne leurs badges.
  3. Le bot vide la table `catches`, réinitialise les niveaux/XP des joueurs dans `players` et remet à zéro l'historique de bananes actives.
  4. Les trophées éternels dans `player_trophies` restent intacts.
  5. Le bot annonce la clôture dans le chat avec une célébration : `🏆 La Saison <nom_saison> est close ! Les exploits ont été gravés éternellement, place à la nouvelle saison ! 🎣`

---

## 4. Visualisation Premium (Web UI)

Le site web affichera une vitrine de trophées ultra-premium sur le profil de chaque joueur :
- **Design Glassmorphism :** Des cartes en verre translucide qui laissent transparaître des lueurs colorées selon la rareté du badge.
- **Effets Harmonieux (HSL) :**
  - **Obsidienne 🌌 :** Lueur violette/noire pulsante mystique.
  - **Diamant ❄️ :** Éclats bleu glacier scintillants.
  - **Platinium 💎 :** Blanc métallisé futuriste pur.
  - **Or/Argent/Bronze :** Gradients classiques ultra-propres et chaleureux.
- **Informations au survol :** Survoler un trophée affiche le niveau atteint à l'époque, la date exacte de déblocage, et la saison correspondante.
