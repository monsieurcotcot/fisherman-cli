-- Create eco_champions_history table
CREATE TABLE IF NOT EXISTS eco_champions_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    username TEXT NOT NULL,
    crowned_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    dethroned_at DATETIME
);
