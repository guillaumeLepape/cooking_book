import sys

if sys.version_info >= (3, 9):
    from typing import Annotated
else:
    from typing_extensions import Annotated

from fastapi import FastAPI, Form, HTTPException
from fastapi.responses import HTMLResponse
from fastui import AnyComponent, FastUI, prebuilt_html
from fastui import components as c
from fastui.components.display import DisplayLookup
from fastui.events import BackEvent, GoToEvent, PageEvent

from .db import ConnectionDep
from .db_utils import fetch_all_recipes, fetch_one_recipe, insert_recipe
from .models import Ingredient, Recipe, RecipeIn

app = FastAPI()


@app.get("/api/recipes/", response_model=FastUI, response_model_exclude_none=True)
def recipes_table(connection: ConnectionDep) -> list[AnyComponent]:
    recipes = fetch_all_recipes(connection.cursor())

    return [
        c.Page(
            components=[
                c.Heading(text="Recettes", level=2),
                c.Table(
                    data=recipes,
                    columns=[DisplayLookup(field="name", on_click=GoToEvent(url="/recipes/{id}/"))],
                    data_model=Recipe,
                ),
                c.Heading(text="Nouvelle recette", level=3),
                c.Form(
                    submit_url="/api/recipes/",
                    method="POST",
                    form_fields=[
                        c.FormFieldInput(name="name", title="Name", required=True),
                        c.forms.FormFieldTextarea(
                            name="ingredients",
                            title="Ingrédients",
                            required=True,
                            rows=5,
                        ),
                        c.forms.FormFieldTextarea(
                            name="steps",
                            title="Préparation",
                            required=True,
                            display_mode="inline",
                            rows=5,
                        ),
                    ],
                ),
            ]
        ),
    ]


@app.get("/api/recipes/{recipe_id}/", response_model=FastUI, response_model_exclude_none=True)
def recipe_detail(connection: ConnectionDep, recipe_id: int) -> list[AnyComponent]:
    recipe = fetch_one_recipe(connection.cursor(), recipe_id)

    if recipe is None:
        raise HTTPException(status_code=404, detail="Recipe not found")

    return [
        c.Page(
            components=[
                c.Link(components=[c.Text(text="Retour")], on_click=BackEvent()),
                c.Heading(text=recipe.name, level=2),
                c.Markdown(text="---"),
                c.Heading(text="Ingrédients", level=3),
                c.Div(
                    components=[
                        c.Markdown(
                            text="\n".join(
                                f"{ingredient.quantity} {ingredient.unit} {ingredient.preposition}{ingredient.name.lower()}  "
                                for ingredient in recipe.ingredients
                            )
                        )
                    ]
                ),
                c.Heading(text="Préparation", level=2),
                c.Div(
                    components=[
                        c.Markdown(
                            text="\n".join(
                                f"{i}. {step}" for i, step in enumerate(recipe.steps, start=1)
                            )
                        )
                    ]
                ),
                c.Div(
                    components=[
                        c.Button(
                            text="Supprimer recette", on_click=PageEvent(name="delete-recipe")
                        ),
                        c.Div(
                            components=[
                                c.ServerLoad(
                                    path=f"/recipes/{recipe_id}/",
                                    method="DELETE",
                                    load_trigger=PageEvent(name="delete-recipe"),
                                    components=[],
                                ),
                            ]
                        ),
                    ],
                ),
            ]
        ),
    ]


@app.delete("/api/recipes/{recipe_id}/", response_model=FastUI, response_model_exclude_none=True)
def delete_recipe(connection: ConnectionDep, recipe_id: int) -> list[AnyComponent]:
    recipe = fetch_one_recipe(connection.cursor(), recipe_id)

    if recipe is None:
        raise HTTPException(status_code=404, detail="Recipe not found")

    connection.cursor().execute("DELETE FROM recipes WHERE id = ?", (recipe_id,))
    connection.commit()

    return [
        c.Page(
            components=[
                c.Heading(text=f"{recipe.name} supprimé"),
            ]
        ),
    ]


@app.post("/api/recipes/", response_model=FastUI, response_model_exclude_none=True)
def create_recipe(
    name: Annotated[str, Form()],
    ingredients: Annotated[str, Form()],
    steps: Annotated[str, Form()],
    connection: ConnectionDep,
) -> list[AnyComponent]:
    recipe = insert_recipe(
        connection,
        RecipeIn(
            name=name,
            ingredients=[Ingredient.parse(ingr) for ingr in ingredients.splitlines()],
            steps=[step for step in steps.splitlines() if step],
        ),
    )

    if recipe is None:
        raise HTTPException(status_code=500, detail="Failed to create recipe")

    return c.Page(
        components=[
            c.Link(
                components=[c.Text(text="Display the new recipe")],
                on_click=GoToEvent(url=f"/recipes/{recipe.id}/"),
            ),
        ]
    )


@app.get("/{path:path}")
async def html_landing() -> HTMLResponse:
    """Simple HTML page which serves the React app, comes last as it matches all paths."""
    return HTMLResponse(prebuilt_html(title="FastUI Demo"))
