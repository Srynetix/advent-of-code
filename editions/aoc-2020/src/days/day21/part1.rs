//! Part 1

use super::{
    common::{
        count_ingredients_for_allergens, extract_ingredients_without_allergens, parse_dishes,
        resolve_allergen_map,
    },
    INPUT,
};

pub fn run() -> usize {
    let dishes = parse_dishes(INPUT);
    let map = count_ingredients_for_allergens(&dishes);
    let out = resolve_allergen_map(map);
    extract_ingredients_without_allergens(&dishes, out).len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(super::run(), 2389);
    }
}
