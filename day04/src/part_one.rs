use crate::parser::Problem;
use crate::position::Position;

pub fn part_one(problem: &Problem) -> u64 {
    let mut forkliftable_count: u64 = 0;
    for r in 0..problem.width {
        for c in 0..problem.height {
            let p = Position { row: r, col: c };
            if p.is_forkliftable(problem) {
                // count forkliftable positions
                forkliftable_count += 1;
            }
        }
    }
    forkliftable_count
}
