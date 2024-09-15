-- Your SQL goes here
CREATE TABLE steps (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    recipe_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    FOREIGN KEY (recipe_id) REFERENCES recipes (id) ON DELETE CASCADE
);
