from datetime import datetime, timezone
from typing import List

from fastapi import FastAPI, HTTPException, status

from .db import ConnectionDep
from .db_utils import (
    delete_from_cart,
    fetch_all_recipes,
    fetch_one_cart_and_recipes,
    fetch_one_recipe,
    insert_recipe,
    setup_db,
)
from .models import Cart, Recipe, RecipeIn, RecipeOut

setup_db()

app = FastAPI()


@app.post("/recipes/")
def create_recipe(recipe_in: RecipeIn, connection: ConnectionDep) -> RecipeOut:
    recipe = insert_recipe(connection, recipe_in)

    if recipe is None:
        raise HTTPException(status_code=500, detail="Failed to create recipe")

    return recipe


@app.get("/recipes/{recipe_id}")
def retrieve_recipe(recipe_id: int, connection: ConnectionDep) -> Recipe:
    cursor = connection.cursor()

    recipe = fetch_one_recipe(cursor, recipe_id)

    if recipe is None:
        raise HTTPException(status_code=404, detail="Recipe not found")

    return recipe


@app.get("/recipes/")
def retrieve_all_recipes(connection: ConnectionDep) -> List[Recipe]:
    cursor = connection.cursor()

    return fetch_all_recipes(cursor)


@app.post("/carts")
def create_cart(connection: ConnectionDep) -> Cart:
    cursor = connection.cursor()

    current_time = datetime.now(timezone.utc)

    cursor.execute("INSERT INTO carts(created_at) VALUES (?)", (current_time,))
    connection.commit()

    if cursor.lastrowid is None:
        raise HTTPException(status_code=500, detail="Failed to create cart")

    return Cart(id=cursor.lastrowid, created_at=current_time, recipes=[])


@app.get("/carts/{cart_id}")
def retrieve_cart(cart_id: int, connection: ConnectionDep) -> Cart:
    cursor = connection.cursor()

    cart = fetch_one_cart_and_recipes(cursor, cart_id)

    if cart is None:
        raise HTTPException(status_code=404, detail="Cart not found")

    return cart


@app.delete("/carts/{cart_id}", status_code=status.HTTP_204_NO_CONTENT)
def delete_cart(cart_id: int, connection: ConnectionDep) -> None:
    return delete_from_cart(connection, cart_id)


@app.post("/carts/{cart_id}/recipes/{recipe_id}")
def add_recipe_to_cart(cart_id: int, recipe_id: int, connection: ConnectionDep) -> Cart:
    cursor = connection.cursor()

    cursor.execute(
        "INSERT INTO cart_recipes(cart_id, recipe_id) VALUES (?, ?)", (cart_id, recipe_id)
    )
    connection.commit()

    cart = fetch_one_cart_and_recipes(cursor, cart_id)

    if cart is None:
        raise HTTPException(status_code=404, detail="Cart not found")

    return cart
