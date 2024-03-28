CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    platform TEXT NOT NULL,
    launch TEXT NOT NULL,
    times INTEGER NOT NULL,
    CONSTRAINT unique_game UNIQUE (name, platform)
);