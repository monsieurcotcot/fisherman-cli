-- Migration : Ajout de la gestion de la ferraille pour les joueurs
ALTER TABLE players ADD COLUMN scrap_metal REAL DEFAULT 0.0;
ALTER TABLE players ADD COLUMN total_sold_scrap_metal REAL DEFAULT 0.0;
