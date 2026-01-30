-- Players and Monsters

CREATE TABLE Players (
    id INTEGER PRIMARY KEY,
    party_id INTEGER REFERENCES Parties(id),
    name TEXT NOT NULL,
    str INTEGER NOT NULL,
    dex INTEGER NOT NULL,
    con INTEGER NOT NULL,
    int INTEGER NOT NULL,
    wis INTEGER NOT NULL,
    cha INTEGER NOT NULL,
    ac INTEGER NOT NULL,
    initiative_bonus INTEGER NOT NULL,
    small_portrait TEXT,
    full_portrait TEXT,
    notes TEXT
);

CREATE TABLE Classes (
    id INTEGER PRIMARY KEY,
    player_id INTEGER NOT NULL REFERENCES Players(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    level INTEGER NOT NULL
);

CREATE TABLE Parties (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE Monsters (
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
    small_portrait TEXT,
    full_portrait TEXT,
    notes TEXT
);


-- Trade

CREATE TABLE TradeItems (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    measure REAL,
    unit TEXT,
    price INTEGER NOT NULL
);
