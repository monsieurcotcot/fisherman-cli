-- Migration: Ajout de la préférence de langue pour chaque joueur
ALTER TABLE players ADD COLUMN language TEXT DEFAULT NULL;
