import re
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
        regex = re.search(
            "^([0-9]+)[,.]?([0-9]+)?[ ]?(?:(.*?)[ ])?[ ]?(de |d')?(.*)$", raw_ingredient
        )

        if regex is not None:
            print(regex.groups())

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
