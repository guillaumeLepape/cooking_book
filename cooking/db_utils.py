import sqlite3
from typing import List, Optional

from .db import DATABASE_PATH
from .models import Cart, Ingredient, IngredientDb, Recipe, RecipeIn


def setup_db():
    connection = sqlite3.connect(DATABASE_PATH)

    cursor = connection.cursor()
    cursor.execute("PRAGMA foreign_keys = ON")
    cursor.execute(
        """
            CREATE TABLE IF NOT EXISTS recipes(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT
            )
        """
    )
    cursor.execute(
        """
            CREATE TABLE IF NOT EXISTS ingredients(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                recipe_id INTEGER,
                preposition TEXT,
                name TEXT,
                quantity REAL,
                unit TEXT,
                FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE
            )
        """
    )
    cursor.execute(
        """
            CREATE TABLE IF NOT EXISTS steps(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                recipe_id INTEGER,
                description TEXT,
                FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE
            )
        """
    )
    cursor.execute(
        """
            CREATE TABLE IF NOT EXISTS carts(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                created_at DATETIME NOT NULL
            )
        """
    )
    cursor.execute(
        """
            CREATE TABLE IF NOT EXISTS cart_recipes(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                cart_id INTEGER,
                recipe_id INTEGER,
                FOREIGN KEY(cart_id) REFERENCES carts(id) ON DELETE CASCADE,
                FOREIGN KEY(recipe_id) REFERENCES recipes(id) ON DELETE CASCADE
            )
        """
    )


def fetch_recipe_steps(cursor: sqlite3.Cursor, recipe_id: int) -> List[str]:
    cursor.execute(
        """
            SELECT
                steps.description
            FROM
                steps
            WHERE
                steps.recipe_id = ?
        """,
        (recipe_id,),
    )

    step_rows = cursor.fetchall()

    return [row[0] for row in step_rows]


def fetch_recipe_ingredients(cursor: sqlite3.Cursor, recipe_id: int) -> List[Ingredient]:
    cursor.execute(
        """
            SELECT
                ingredients.preposition,
                ingredients.name,
                ingredients.quantity,
                ingredients.unit
            FROM
                ingredients
            WHERE
                ingredients.recipe_id = ?
        """,
        (recipe_id,),
    )

    ingredient_rows = cursor.fetchall()

    print(ingredient_rows[0])

    return [
        Ingredient(preposition=row[0], name=row[1], quantity=row[2], unit=row[3])
        for row in ingredient_rows
    ]


def fetch_all_ingredients(cursor: sqlite3.Cursor) -> List[IngredientDb]:
    cursor.execute("SELECT id, preposition, name, quantity, unit FROM ingredients")

    return [
        IngredientDb(id=row[0], preposition=row[1], name=row[2], quantity=row[3], unit=row[4])
        for row in cursor.fetchall()
    ]


def fetch_one_cart_and_recipes(cursor: sqlite3.Cursor, cart_id: int) -> Optional[Cart]:
    cart = fetch_one_cart(cursor, cart_id)

    if cart is None:
        return None

    cursor.execute(
        """
            SELECT
                recipes.id,
                recipes.name
            FROM
                carts
                JOIN cart_recipes ON carts.id = cart_recipes.cart_id
                JOIN recipes ON recipes.id = cart_recipes.recipe_id
                WHERE carts.id = ?
        """,
        (cart_id,),
    )

    recipe_rows = cursor.fetchall()

    cart.recipes = [
        Recipe(
            id=row[0],
            name=row[1],
            ingredients=fetch_recipe_ingredients(cursor, row[0]),
            steps=fetch_recipe_steps(cursor, row[0]),
        )
        for row in recipe_rows
    ]

    return cart


def fetch_one_cart(cursor: sqlite3.Cursor, cart_id) -> Optional[Cart]:
    cursor.execute(
        """
            SELECT
                id,
                created_at
            FROM
                carts
            WHERE
                id = ?
        """,
        (cart_id,),
    )

    row = cursor.fetchone()

    if row is None:
        return None

    return Cart(id=row[0], created_at=row[1], recipes=[])


def delete_from_cart(connection: sqlite3.Connection, cart_id: int) -> None:
    cursor = connection.cursor()

    cursor.execute(
        """
            DELETE FROM
                carts
            WHERE
                id = ?
        """,
        (cart_id,),
    )

    connection.commit()


def fetch_one_recipe(cursor: sqlite3.Cursor, recipe_id: int) -> Optional[Recipe]:
    cursor.execute(
        """
            SELECT
                id,
                name
            FROM
                recipes
            WHERE id = ?
        """,
        (recipe_id,),
    )

    row = cursor.fetchone()

    if not row:
        return None

    return Recipe(
        id=row[0],
        name=row[1],
        ingredients=fetch_recipe_ingredients(cursor, row[0]),
        steps=fetch_recipe_steps(cursor, row[0]),
    )


def fetch_all_recipes(cursor: sqlite3.Cursor) -> List[Recipe]:
    cursor.execute("SELECT id, name FROM recipes")

    return [
        Recipe(
            id=row[0],
            name=row[1],
            ingredients=fetch_recipe_ingredients(cursor, row[0]),
            steps=fetch_recipe_steps(cursor, row[0]),
        )
        for row in cursor.fetchall()
    ]


def insert_recipe(connection: sqlite3.Connection, recipe_in: RecipeIn) -> Optional[Recipe]:
    cursor = connection.cursor()

    cursor.execute("INSERT INTO recipes(name) VALUES (?)", (recipe_in.name,))

    recipe_id = cursor.lastrowid

    if recipe_id is None:
        return None

    for ingredient in recipe_in.ingredients:
        cursor.execute(
            "INSERT INTO ingredients(recipe_id, preposition, name, quantity, unit) VALUES (?, ?, ?, ?, ?)",
            (recipe_id, "", ingredient.name, ingredient.quantity, ingredient.unit),
        )

    for step in recipe_in.steps:
        cursor.execute("INSERT INTO steps(recipe_id, description) VALUES (?, ?)", (recipe_id, step))

    connection.commit()

    return Recipe(id=recipe_id, **recipe_in.model_dump())
