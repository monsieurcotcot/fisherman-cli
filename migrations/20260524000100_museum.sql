-- Seconde migration : Création de la table museum_discoveries pour le suivi permanent du Musée

CREATE TABLE IF NOT EXISTS museum_discoveries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER REFERENCES players(id) ON DELETE CASCADE,
    username TEXT NOT NULL,
    fish_name TEXT NOT NULL,
    rarity TEXT NOT NULL,
    max_size REAL NOT NULL,
    max_weight REAL DEFAULT 0,
    best_state TEXT NOT NULL,
    description TEXT,
    total_caught INTEGER DEFAULT 1,
    unlocked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(player_id, fish_name)
);

-- Index pour accélérer le chargement du musée d'un joueur
CREATE INDEX IF NOT EXISTS idx_museum_player_id ON museum_discoveries(player_id);
