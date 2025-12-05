use crate::problem::Problem;

pub fn part_one(problem: &Problem) -> u64 {
    let mut fresh_ingredients = 0;
    for ingredient in problem.ingredient_ids.iter() {
        for range in problem.database_ranges.iter() {
            if range.contains(*ingredient) {
                fresh_ingredients += 1;
                break;
            }
        }
    }
    fresh_ingredients
}
