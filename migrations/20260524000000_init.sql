-- Migration initiale : Création des tables du jeu de pêche RPG

-- 1. Table des joueurs (players)
CREATE TABLE IF NOT EXISTS players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    total_attempts INTEGER DEFAULT 0,
    successful_attempts INTEGER DEFAULT 0,
    failed_attempts INTEGER DEFAULT 0,
    last_fishing_time DATETIME,
    level INTEGER DEFAULT 1,
    xp INTEGER DEFAULT 0,
    vip_until DATETIME,
    profile_image_url TEXT,
    gold INTEGER DEFAULT 0,
    last_daily_reward_at DATETIME,
    consecutive_days INTEGER DEFAULT 0,
    total_days INTEGER DEFAULT 0
);

-- Index pour accélérer la recherche par nom d'utilisateur (classement, profil)
CREATE INDEX IF NOT EXISTS idx_players_username ON players(username);

-- 2. Table des captures (catches)
CREATE TABLE IF NOT EXISTS catches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER REFERENCES players(id) ON DELETE CASCADE,
    fish_name TEXT NOT NULL,
    rarity TEXT NOT NULL,
    size REAL NOT NULL,
    weight REAL DEFAULT 0,
    state TEXT NOT NULL,
    description TEXT,
    stream_title TEXT,
    caught_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    is_junk BOOLEAN DEFAULT 0,
    caught_by TEXT
);

-- Index pour optimiser le chargement de l'inventaire d'un joueur
CREATE INDEX IF NOT EXISTS idx_catches_player_id ON catches(player_id);

-- 3. Table de l'historique des rois de la banane (banana_kings_history)
CREATE TABLE IF NOT EXISTS banana_kings_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER REFERENCES players(id) ON DELETE SET NULL,
    username TEXT NOT NULL,
    crowned_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    dethroned_at DATETIME
);

-- 4. Table des trophées éternels (player_trophies)
CREATE TABLE IF NOT EXISTS player_trophies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER REFERENCES players(id) ON DELETE CASCADE,
    username TEXT NOT NULL,
    season TEXT NOT NULL,
    trophy_tier TEXT NOT NULL,
    level INTEGER DEFAULT 1,
    unlocked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(player_id, season)
);

-- Index pour charger rapidement les trophées éternels sur le profil du joueur
CREATE INDEX IF NOT EXISTS idx_trophies_player_id ON player_trophies(player_id);
