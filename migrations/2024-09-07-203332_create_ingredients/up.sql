-- Your SQL goes here
CREATE TABLE ingredients (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    recipe_id INTEGER NOT NULL,
    preposition TEXT NOT NULL,
    name TEXT NOT NULL,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    FOREIGN KEY (recipe_id) REFERENCES recipes (id) ON DELETE CASCADE
);
