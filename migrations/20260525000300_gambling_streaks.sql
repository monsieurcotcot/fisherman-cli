-- Migration: Suivi des séries consécutives (Streaks) au Coinflip
ALTER TABLE players ADD COLUMN coinflip_current_win_streak INTEGER DEFAULT 0;
ALTER TABLE players ADD COLUMN coinflip_current_loss_streak INTEGER DEFAULT 0;
ALTER TABLE players ADD COLUMN coinflip_max_win_streak INTEGER DEFAULT 0;
ALTER TABLE players ADD COLUMN coinflip_max_loss_streak INTEGER DEFAULT 0;
