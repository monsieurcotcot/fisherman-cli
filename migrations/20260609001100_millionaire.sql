-- Migration: Ajoutez millionaire_at et marquez dozerker comme le premier millionnaire
ALTER TABLE players ADD COLUMN millionaire_at DATETIME DEFAULT NULL;

UPDATE players SET millionaire_at = CURRENT_TIMESTAMP WHERE LOWER(username) = 'dozerker';
