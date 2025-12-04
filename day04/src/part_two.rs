use crate::parser::GridCell;
use crate::parser::Problem;
use crate::position::Position;

pub fn part_two(problem: &Problem) -> u64 {
    let mut total_removed: u64 = 0;
    let mut problem_cloned = problem.clone();
    loop {
        let removed_this_iteration = simplify_problem(&mut problem_cloned);
        if removed_this_iteration == 0 {
            break;
        }
        total_removed += removed_this_iteration;
    }
    total_removed
}

pub fn simplify_problem(problem: &mut Problem) -> u64 {
    let mut removed_positions = vec![];
    // be careful to count first and then remove or you may remove
    // stuff just because you removed adjacent ones in the same iteration
    for r in 0..problem.width {
        for c in 0..problem.height {
            let p = Position { row: r, col: c };
            if p.is_forkliftable(problem) {
                // count forkliftable positions
                removed_positions.push(p);
            }
        }
    }
    for pos in removed_positions.iter() {
        problem.grid[pos.row][pos.col] = GridCell::Empty;
    }
    removed_positions.len() as u64
}
