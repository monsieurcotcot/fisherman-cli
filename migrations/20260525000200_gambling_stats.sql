-- Migration: Ajout des colonnes de statistiques de Coinflip / Hasard
ALTER TABLE players ADD COLUMN coinflip_wins INTEGER DEFAULT 0;
ALTER TABLE players ADD COLUMN coinflip_losses INTEGER DEFAULT 0;
ALTER TABLE players ADD COLUMN coinflip_biggest_win INTEGER DEFAULT 0;
ALTER TABLE players ADD COLUMN coinflip_biggest_loss INTEGER DEFAULT 0;
ALTER TABLE players ADD COLUMN coinflip_gold_won_total INTEGER DEFAULT 0;
ALTER TABLE players ADD COLUMN coinflip_gold_lost_total INTEGER DEFAULT 0;
