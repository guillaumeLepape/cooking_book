-- Your SQL goes here
CREATE TABLE cart_recipes(
    cart_id INTEGER NOT NULL,
    recipe_id INTEGER NOT NULL,
    PRIMARY KEY(cart_id, recipe_id),
    FOREIGN KEY(cart_id) REFERENCES carts(id) ON DELETE CASCADE,
    FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE
);