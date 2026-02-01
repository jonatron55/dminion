-- Initial schema for campaign databases.
-- Creates core tables for players, monsters, parties, encounters, maps, and items.
--
-- This migration establishes the foundation for campaign data storage.
-- All foreign keys use appropriate ON DELETE behavior.
-- Portrait fields store base names only; resolution handled by application.

-- Parties (groups of players)
CREATE TABLE IF NOT EXISTS Party (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

-- Players (persistent character data)
CREATE TABLE IF NOT EXISTS Player (
    id INTEGER PRIMARY KEY,
    party_id INTEGER REFERENCES Party(id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    str INTEGER NOT NULL,
    dex INTEGER NOT NULL,
    con INTEGER NOT NULL,
    int INTEGER NOT NULL,
    wis INTEGER NOT NULL,
    cha INTEGER NOT NULL,
    ac INTEGER NOT NULL,
    initiative_bonus INTEGER NOT NULL,
    portrait TEXT,
    notes TEXT
);

-- Player classes (multiclass support)
CREATE TABLE IF NOT EXISTS PlayerClass (
    id INTEGER PRIMARY KEY,
    player_id INTEGER NOT NULL REFERENCES Player(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    level INTEGER NOT NULL
);

-- Monster templates (library entries)
CREATE TABLE IF NOT EXISTS Monster (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    subtype TEXT NOT NULL,
    str INTEGER NOT NULL,
    dex INTEGER NOT NULL,
    con INTEGER NOT NULL,
    int INTEGER NOT NULL,
    wis INTEGER NOT NULL,
    cha INTEGER NOT NULL,
    cr INTEGER NOT NULL,
    ac INTEGER NOT NULL,
    initiative_bonus INTEGER NOT NULL,
    hit_dice TEXT NOT NULL,
    legendary_actions INTEGER NOT NULL,
    portrait TEXT,
    notes TEXT
);

-- Encounters (saved encounter configurations)
CREATE TABLE IF NOT EXISTS Encounter (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    notes TEXT
);

-- Encounter-Monster junction (which monsters are in which encounter)
CREATE TABLE IF NOT EXISTS EncounterMonster (
    id INTEGER PRIMARY KEY,
    encounter_id INTEGER NOT NULL REFERENCES Encounter(id) ON DELETE CASCADE,
    monster_id INTEGER NOT NULL REFERENCES Monster(id) ON DELETE CASCADE,
    count INTEGER NOT NULL
);

-- Encounter-Player junction (which players are in which encounter)
CREATE TABLE IF NOT EXISTS EncounterPlayer (
    id INTEGER PRIMARY KEY,
    encounter_id INTEGER NOT NULL REFERENCES Encounter(id) ON DELETE CASCADE,
    player_id INTEGER NOT NULL REFERENCES Player(id) ON DELETE CASCADE
);

-- Maps (metadata only; images stored externally)
CREATE TABLE IF NOT EXISTS Map (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    image_path TEXT NOT NULL,
    width INTEGER,
    height INTEGER,
    grid_size INTEGER,
    notes TEXT
);

-- Items (library entries for trade system)
CREATE TABLE IF NOT EXISTS Item (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT,
    weight REAL,
    price INTEGER NOT NULL,
    notes TEXT
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_Player_party_id ON Player(party_id);
CREATE INDEX IF NOT EXISTS idx_PlayerClass_player_id ON PlayerClass(player_id);
CREATE INDEX IF NOT EXISTS idx_EncounterMonster_encounter_id ON EncounterMonster(encounter_id);
CREATE INDEX IF NOT EXISTS idx_EncounterMonster_monster_id ON EncounterMonster(monster_id);
CREATE INDEX IF NOT EXISTS idx_EncounterPlayer_encounter_id ON EncounterPlayer(encounter_id);
CREATE INDEX IF NOT EXISTS idx_EncounterPlayer_player_id ON EncounterPlayer(player_id);
