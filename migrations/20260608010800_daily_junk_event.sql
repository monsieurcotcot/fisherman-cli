-- Migration: Table pour l'événement quotidien des déchets
CREATE TABLE IF NOT EXISTS daily_stream_stats (
    live_date TEXT PRIMARY KEY,
    total_attempts INTEGER DEFAULT 0,
    junk_target INTEGER DEFAULT 0,
    junk_caught INTEGER DEFAULT 0
);
