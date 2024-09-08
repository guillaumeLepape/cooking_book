use regex::Regex;

fn infer_preposition(name: &str) -> &str {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y', 'h'];

    if vowels.iter().any(|v| name.starts_with(*v)) {
        "d'"
    } else {
        "de "
    }
}

#[derive(Debug)]
pub enum ParsingError {
    InvalidRegex(),
    ParsingError(),
    NoNameFound(),
}

pub fn parse(raw_ingredient: &str) -> Result<(String, String, f32, String), ParsingError> {
    let Ok(re) = Regex::new("^([0-9]+)[,.]?([0-9]+)?[ ]?(?:(.*?)[ ])?[ ]?(de |d')?(.*)$") else {
        return Err(ParsingError::InvalidRegex());
    };

    let Some(caps) = re.captures(raw_ingredient) else {
        return Err(ParsingError::ParsingError());
    };

    let unit = caps.get(3).map_or("", |res| res.as_str());

    let name = match caps.get(5) {
        Some(res) => res.as_str(),
        None => return Err(ParsingError::NoNameFound()),
    };

    let preposition = match unit {
        "" => "",
        _ => caps
            .get(4)
            .map_or_else(|| infer_preposition(name), |res| res.as_str()),
    };

    let integer_part_str = match caps.get(1) {
        Some(res) => res.as_str(),
        None => return Err(ParsingError::ParsingError()),
    };

    let decimal_part = caps.get(2);

    let quantity = match decimal_part {
        Some(res) => match format!("{}.{}", integer_part_str, res.as_str()).parse::<f32>() {
            Ok(val) => val,
            Err(_) => return Err(ParsingError::ParsingError()),
        },
        None => match integer_part_str.parse::<f32>() {
            Ok(val) => val,
            Err(_) => return Err(ParsingError::ParsingError()),
        },
    };

    Ok((
        name.to_owned(),
        preposition.to_owned(),
        quantity,
        unit.to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("125 g fromage frais",("fromage frais", "de ", 125.0, "g"))]
    #[case("1.5 kg de farine", ("farine", "de ", 1.5, "kg"))]
    #[case("3 brin ciboulette",( "ciboulette", "de ",3.0,"brin"))]
    #[case("120 g saumon fum\u{e9}",( "saumon fum\u{e9}", "de ",120.0,"g"))]
    #[case("120.989g de saumon fum\u{e9}",("saumon fum\u{e9}", "de ", 120.989,"g"))]
    #[case("0.5 gousse ail",( "ail", "d'",0.5,"gousse"))]
    #[case("3 brin estragon",("estragon", "d'", 3.0,"brin"))]
    #[case("2 feuille laurier",("laurier", "de ", 2.0,"feuille"))]
    #[case("200 g olive noir",( "olive noir", "d'",200.0,"g"))]
    #[case("5 filet anchois \u{e0} l'huile",("anchois \u{e0} l'huile", "d'", 5.0,"filet"))]
    #[case("5filet d'anchois \u{e0} l'huile",("anchois \u{e0} l'huile", "d'", 5.0,"filet"))]
    #[case("1,5g de saumon fum\u{e9}",("saumon fum\u{e9}", "de ",1.5,"g"))]
    #[case("1 oignon",("oignon","", 1.0,""))]
    #[case("1oignon",("oignon","", 1.0,""))]
    fn ingredient_parsing_test(
        #[case] test_input: &str,
        #[case] expected_ingredient: (&str, &str, f32, &str),
    ) {
        assert_eq!(
            parse(test_input).unwrap(),
            (
                expected_ingredient.0.to_owned(),
                expected_ingredient.1.to_owned(),
                expected_ingredient.2,
                expected_ingredient.3.to_owned()
            )
        );
    }
}
