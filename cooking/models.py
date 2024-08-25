from datetime import datetime
from typing import List, Tuple

from pydantic import BaseModel


def need_preposition(name: str) -> bool:
    return not name.startswith("d'") and not name.startswith("de ")


def infer_preposition(name: str) -> str:
    split_name = name.split(maxsplit=1)

    if split_name[0][0] in "aeiouyh":
        return "d'"

    return "de "


def split_preposition(name: str) -> Tuple[str, str]:
    if name.startswith("d'"):
        return "d'", name[2:]

    return "de ", name[3:]


class Ingredient(BaseModel):
    name: str
    preposition: str
    quantity: float
    unit: str

    @classmethod
    def parse(cls, raw_ingredient: str) -> "Ingredient":
        parts = raw_ingredient.split(maxsplit=2)

        if len(parts) == 3:
            quantity = float(parts[0].replace(",", "."))
            unit = parts[1]
            name = parts[2]
        else:
            quantity = float(parts[0].replace(",", "."))
            unit = ""
            name = parts[1]

        if not unit:
            preposition = ""
        elif need_preposition(name):
            preposition = infer_preposition(name)
        else:
            preposition, name = split_preposition(name)

        return cls(name=name, preposition=preposition, quantity=quantity, unit=unit)


class IngredientDb(BaseModel):
    id: int
    preposition: str
    name: str
    quantity: float
    unit: str


class RecipeIn(BaseModel):
    name: str
    ingredients: List[Ingredient]
    steps: List[str]


class Recipe(BaseModel):
    id: int
    name: str
    ingredients: List[Ingredient]
    steps: List[str]


class Cart(BaseModel):
    id: int
    created_at: datetime
    recipes: List[Recipe]
