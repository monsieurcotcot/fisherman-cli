-- Migration: Ajout de la jauge de notoriété écolo pour le tri sélectif
ALTER TABLE players ADD COLUMN eco_notoriety INTEGER DEFAULT 1000;
