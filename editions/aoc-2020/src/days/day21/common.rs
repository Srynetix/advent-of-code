//! Common

use std::collections::{HashMap, HashSet};

use aoc_sx::{once_cell::sync::Lazy, regex::Regex};

static RGX_DISH: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^(.*?)(?: \(contains (.*?)\))?$"#).unwrap());

/// Allergen.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Allergen(String);

/// Ingredient.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Ingredient(String);

/// Ingredient counts.
pub type IngredientCounts<'a> = HashMap<&'a Ingredient, usize>;

/// Allergen counts.
pub type AllergenCounts<'a> = HashMap<&'a Allergen, IngredientCounts<'a>>;

/// Allergen map.
pub type AllergenMap<'a> = HashMap<&'a Allergen, &'a Ingredient>;

/// Dish.
#[derive(Debug)]
pub struct Dish {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}

/// Parse ingredient.
///
/// # Arguments
///
/// * `input` - Input string
pub fn parse_ingredients(input: &str) -> HashSet<Ingredient> {
    input
        .split_whitespace()
        .map(|i| Ingredient(i.to_string()))
        .collect()
}

/// Parse allergens.
///
/// # Arguments
///
/// * `input` - Input string
pub fn parse_allergens(input: &str) -> HashSet<Allergen> {
    input.split(", ").map(|a| Allergen(a.to_string())).collect()
}

/// Parse dish.
///
/// # Arguments
///
/// * `input` - Input string
pub fn parse_dish(input: &str) -> Dish {
    let captures = RGX_DISH.captures(input.trim()).unwrap();
    let ingredients = parse_ingredients(captures.get(1).unwrap().as_str());
    let allergens = parse_allergens(captures.get(2).map(|x| x.as_str()).unwrap_or_default());

    Dish {
        ingredients,
        allergens,
    }
}

/// Parse multiple dishes.
///
/// # Arguments
///
/// * `input` - Input string
pub fn parse_dishes(input: &str) -> Vec<Dish> {
    input.trim().split('\n').map(parse_dish).collect()
}

/// Count ingredients for allergens.
///
/// # Arguments
///
/// * `input` - Input string
pub fn count_ingredients_for_allergens(dishes: &[Dish]) -> AllergenCounts {
    let mut allergen_map = HashMap::new();

    for d in dishes {
        for a in &d.allergens {
            for i in &d.ingredients {
                let ingredient_map = allergen_map.entry(a).or_insert_with(HashMap::new);
                let ingredient_counter = ingredient_map.entry(i).or_insert(0);
                *ingredient_counter += 1;
            }
        }
    }

    allergen_map
}

/// Resolve allergen map from counts.
///
/// # Arguments
///
/// * `counts` - Allergen counts
pub fn resolve_allergen_map(mut counts: AllergenCounts) -> AllergenMap {
    let mut output = HashMap::new();

    let mut remaining_allergens = counts.keys().copied().collect::<HashSet<_>>();
    while !remaining_allergens.is_empty() {
        let mut allergen_to_remove = None;

        for a in &remaining_allergens {
            if let Some(i) = get_single_max_value_from_ingredient_counts(counts.get(a).unwrap()) {
                output.insert(*a, i);
                allergen_to_remove = Some((*a, i));
                break;
            }
        }

        if let Some((a, i)) = allergen_to_remove {
            // Remove from remaining allergens
            remaining_allergens.remove(a);

            // Remove from count map
            for v in &mut counts.values_mut() {
                v.remove(i);
            }
            counts.remove(a);
        } else {
            panic!("We are looping :(");
        }
    }

    output
}

/// Extract ingredients from dish without allergens.
///
/// # Arguments
///
/// * `dishes` - Dishes
/// * `map` - Allergen map
pub fn extract_ingredients_without_allergens<'a>(
    dishes: &'a [Dish],
    map: AllergenMap<'a>,
) -> Vec<&'a Ingredient> {
    let mut output = vec![];
    let ingrediens_with_allergens = map.into_iter().map(|(_, v)| v).collect::<HashSet<_>>();

    for d in dishes {
        for i in &d.ingredients {
            if !ingrediens_with_allergens.contains(&i) {
                output.push(i)
            }
        }
    }

    output
}

/// Get single max value from ingredient counts.
/// If multiple values are maximum, return None.
///
/// # Arguments
///
/// * `ingredient_counts` - Ingredient count
pub fn get_single_max_value_from_ingredient_counts<'a>(
    ingredient_counts: &IngredientCounts<'a>,
) -> Option<&'a Ingredient> {
    let mut max_value = usize::MIN;
    let mut max_ingredient = None;
    let mut only_one = true;

    for (i, &n) in ingredient_counts {
        match n.cmp(&max_value) {
            std::cmp::Ordering::Greater => {
                max_value = n;
                max_ingredient = Some(i);
                only_one = true;
            }
            std::cmp::Ordering::Equal => {
                only_one = false;
            }
            std::cmp::Ordering::Less => (),
        }
    }

    max_ingredient.and_then(|&i| if only_one { Some(i) } else { None })
}

/// Get canonical dangerous list as string.
///
/// # Arguments
///
/// * `allergen_map` - Allergen map
pub fn get_canonical_dangerous_list_as_string(map: &AllergenMap) -> String {
    let mut allergens = map.keys().copied().collect::<Vec<_>>();
    allergens.sort_unstable();

    allergens
        .iter()
        .map(|a| map.get(a).unwrap().0.clone())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use aoc_sx::indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {"
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)
    "};

    #[test]
    fn test_parse_dishes() {
        let dishes = parse_dishes(SAMPLE);
        assert_eq!(dishes.len(), 4);

        let first_dish = &dishes[0];
        assert_eq!(first_dish.ingredients.len(), 4);
        assert_eq!(first_dish.allergens.len(), 2);
    }

    #[test]
    fn test_map_allergens() {
        let dishes = parse_dishes(SAMPLE);
        let map = count_ingredients_for_allergens(&dishes);
        let out = resolve_allergen_map(map);
        let mut no_allergens = extract_ingredients_without_allergens(&dishes, out);
        no_allergens.sort_unstable();

        assert_eq!(
            no_allergens,
            vec![
                &Ingredient("kfcds".into()),
                &Ingredient("nhms".into()),
                &Ingredient("sbzzf".into()),
                &Ingredient("sbzzf".into()),
                &Ingredient("trh".into()),
            ]
        );
    }

    #[test]
    fn test_canonical_dangerous_list() {
        let dishes = parse_dishes(SAMPLE);
        let map = count_ingredients_for_allergens(&dishes);
        let out = resolve_allergen_map(map);

        assert_eq!(
            get_canonical_dangerous_list_as_string(&out),
            "mxmxvkd,sqjhc,fvjkl".to_string()
        );
    }
}
