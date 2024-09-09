import pytest

from cooking.models import Ingredient


@pytest.mark.parametrize(
    ("test_input", "expected_ingredient"),
    [
        (
            "125 g fromage frais",
            Ingredient(preposition="de ", name="fromage frais", quantity=125.0, unit="g"),
        ),
        (
            "3 brin ciboulette",
            Ingredient(preposition="de ", name="ciboulette", quantity=3.0, unit="brin"),
        ),
        (
            "120 g saumon fumé",
            Ingredient(preposition="de ", name="saumon fumé", quantity=120.0, unit="g"),
        ),
        (
            "120.989g de saumon fumé",
            Ingredient(preposition="de ", name="saumon fumé", quantity=120.989, unit="g"),
        ),
        ("0.5 gousse ail", Ingredient(preposition="d'", name="ail", quantity=0.5, unit="gousse")),
        (
            "3 brin estragon",
            Ingredient(preposition="d'", name="estragon", quantity=3.0, unit="brin"),
        ),
        (
            "2 feuille laurier",
            Ingredient(preposition="de ", name="laurier", quantity=2.0, unit="feuille"),
        ),
        (
            "200 g olive noir",
            Ingredient(preposition="d'", name="olive noir", quantity=200.0, unit="g"),
        ),
        (
            "5 filet anchois à l'huile",
            Ingredient(preposition="d'", name="anchois à l'huile", quantity=5.0, unit="filet"),
        ),
        (
            "5filet d'anchois à l'huile",
            Ingredient(preposition="d'", name="anchois à l'huile", quantity=5.0, unit="filet"),
        ),
        (
            "1,5g de saumon fumé",
            Ingredient(preposition="de ", name="saumon fumé", quantity=1.5, unit="g"),
        ),
        ("1 oignon", Ingredient(preposition="", name="oignon", quantity=1.0, unit="")),
        ("1oignon", Ingredient(preposition="", name="oignon", quantity=1.0, unit="")),
    ],
)
def test_ingredient_parsing(test_input: str, expected_ingredient: Ingredient) -> None:
    assert Ingredient.parse(test_input) == expected_ingredient
