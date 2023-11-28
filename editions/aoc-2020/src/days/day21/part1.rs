//! Part 1

use super::{common::{parse_dishes, count_ingredients_for_allergens, resolve_allergen_map, extract_ingredients_without_allergens}, INPUT};

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
