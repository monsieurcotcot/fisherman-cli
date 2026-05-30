-- Migration: Table pour enregistrer les dates des jours où le stream est en ligne
CREATE TABLE IF NOT EXISTS stream_live_dates (
    live_date TEXT PRIMARY KEY
);
