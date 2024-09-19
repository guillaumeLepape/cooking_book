use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DieselError;

use crate::db::DBConnection;
use crate::db_utils::insert_recipe;
use crate::models::RecipeIn;

pub fn create_recipes(connection: &mut DBConnection) {
    let recipes = [
        RecipeIn {
            name: "Saucisses aux lentilles".to_owned(),
            ingredients: vec![
                "350 g de Lentilles vertes".to_owned(),
                "300 g de saucisses de Montb\u{e9}liard".to_owned(),
                "200 g de lardons fum\u{e9}s".to_owned(),
                "1 oignon".to_owned(),
                "2 gousse d'ail".to_owned(),
                "2 feuille de laurier".to_owned(),
            ],
            steps: vec![
                "Eplucher et \u{e9}mincer l'oignon. Peler les gousses d'ail.".to_owned(),
                "Dans une cocotte, mettre les lentilles, les saucisses, les lardons, l'oignon \u{e9}minces, les gousses d'ail et les feuilles de laurier. Ajouter 70 cl d'eau, saler et poivrer.".to_owned(),
                "Faire cuire pendant 40 minutes sur feu moyen \u{e0} couvert. Servir bien chaud.".to_owned()
            ],
        },
        RecipeIn {
            name: "Gratin de gnocchi au saumon et \u{e9}pinards".to_owned(),
            ingredients:vec! [
                "400g de gnocchi".to_owned(),
                "300g d'\u{e9}pinards surgel\u{e9}s".to_owned(),
                "200g de pav\u{e9} de saumon".to_owned(),
                "150 g parmesan r\u{e2}p\u{e9}".to_owned(),
                "0.5litre de lait".to_owned(),
                "30.0 g de farine".to_owned(),
                "30g de beurre".to_owned()
            ],
            steps: vec![
                "Faire cuire les gnocchi dans une grande casserole d'eau bouillante sal\u{e9}e en suivant les indications sur le sachet. ".to_owned(),
                "Dans une casserole, faire cuire les \u{e9}pinards avec un peu de beurre pendant 10 minutes.".to_owned(),
                "D\u{e9}couper les pav\u{e9}s de saumon en d\u{e9}s. Pr\u{e9}chauffer le four \u{e0} 180\u{b0}C.".to_owned(),
                "Pr\u{e9}parer la b\u{e9}chamel en faisant fondre le beurre coup\u{e9} en d\u{e9}s dans une casserole. Ajouter la farine en remuant. Verser le lait progressivement en continuant de remuer jusqu'\u{e0} ce que la cr\u{e8}me \u{e9}paississe. Ajouter le parmesan, saler et poivrer.".to_owned(),
                "D\u{e9}poser les gnocchi \u{e9}goutt\u{e9}s dans le fond d'un plat \u{e0} gratin. Ajoutez la moiti\u{e9} de la b\u{e9}chamel. Recouvrir de saumon et d'\u{e9}pinards et ajouter le reste de b\u{e9}chamel. Enfourner pour 20 minutes \u{e0} 180\u{b0}C. Servir aussit\u{f4}t.".to_owned(),
            ]
        },
        RecipeIn {
            name: "Tapenade : la meilleure recette".to_owned(),
            ingredients: vec![
                "200g d'olive noir".to_owned(),
                "8 c\u{e2}pres".to_owned(),
                "5filet anchois \u{e0} l'huile".to_owned(),
                "1 gousse d'ail".to_owned(),
            ],
            steps: vec![
                "Hacher finement la gousse d\u{2019}ail.".to_owned(),
                "Mettre dans le bol d\u{2019}un mixeur les filets d\u{2019}anchois, les c\u{e2}pres, la gousse d\u{2019}ail hach\u{e9}e, les olives noires et l\u{2019}huile d\u{2019}olive et mixer assez fin.".to_owned()
            ]
        }
    ];

    for recipe in recipes {
        match insert_recipe(&recipe, connection) {
            Ok(recipe) => println!("Inserted recipe: {}", recipe.name),
            Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                println!("Recipe already exists: {}", recipe.name);
            }
            Err(error) => {
                eprintln!("Error inserting recipe: {error}");
                std::process::exit(1);
            }
        }
    }
}
