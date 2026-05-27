-- Migration: Suivi des statistiques pour les trophees/medailles
ALTER TABLE players ADD COLUMN gold_given_total INTEGER DEFAULT 0;
ALTER TABLE players ADD COLUMN max_gold_held INTEGER DEFAULT 0;
