import re
from datetime import datetime
from typing import List

from pydantic import BaseModel


def infer_preposition(name: str) -> str:
    if name[0] in "aeiouyh":
        return "d'"

    return "de "


class Ingredient(BaseModel):
    name: str
    preposition: str
    quantity: float
    unit: str

    @classmethod
    def parse(cls, raw_ingredient: str) -> "Ingredient":
        regex = re.search(
            "^([0-9]+)[,.]?([0-9]+)?[ ]?(?:(.*?)[ ])?[ ]?(de |d')?(.*)$", raw_ingredient
        )

        if regex is not None:
            groups = regex.groups()

            if groups[2] is None:
                unit = ""
            else:
                unit = groups[2]

            name = groups[4]

            if unit == "":
                preposition = ""
            elif groups[3] is None:
                preposition = infer_preposition(name)
            else:
                preposition = groups[3]

            if groups[1] is None:
                quantity = float(groups[0])
            else:
                quantity = float(groups[0] + "." + groups[1])

            return Ingredient(name=name, preposition=preposition, quantity=quantity, unit=unit)
        else:
            raise ValueError(f"Unable to parse {raw_ingredient}")


class IngredientDb(BaseModel):
    id: int
    preposition: str
    name: str
    quantity: float
    unit: str


class IngredientOut(BaseModel):
    id: int
    preposition: str
    name: str
    quantity: float
    unit: str


class RecipeIn(BaseModel):
    name: str
    ingredients: List[str]
    steps: List[str]


class Recipe(BaseModel):
    id: int
    name: str
    ingredients: List[Ingredient]
    steps: List[str]


class RecipeOut(BaseModel):
    id: int
    name: str
    ingredients: List[IngredientOut]
    steps: List[str]


class Cart(BaseModel):
    id: int
    created_at: datetime
    recipes: List[Recipe]
