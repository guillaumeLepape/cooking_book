import json
import sqlite3
from pathlib import Path

from cooking.db import DATABASE_PATH
from cooking.db_utils import insert_recipe
from cooking.models import RecipeIn


def main() -> int:
    recipes = json.loads((Path(__file__).parent / "recipes.json").read_text())

    connection = sqlite3.connect(DATABASE_PATH)

    for recipe_in in recipes:
        recipe = insert_recipe(connection, RecipeIn(**recipe_in))

        if recipe is None:
            print(f"Failed to create recipe: {recipe_in}")
            return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
