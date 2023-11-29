//! Part 2

use super::{
    common::{
        count_ingredients_for_allergens, get_canonical_dangerous_list_as_string, parse_dishes,
        resolve_allergen_map,
    },
    INPUT,
};

pub fn run() -> String {
    let dishes = parse_dishes(INPUT);
    let map = count_ingredients_for_allergens(&dishes);
    let out = resolve_allergen_map(map);
    get_canonical_dangerous_list_as_string(&out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        assert_eq!(
            super::run(),
            "fsr,skrxt,lqbcg,mgbv,dvjrrkv,ndnlm,xcljh,zbhp"
        );
    }
}
