use crate::problem::Problem;
use crate::problem::TachyonCell;

pub fn part_two(problem: &Problem) -> u64 {
    let mut problem_clone = problem.clone();
    //println!("Initial Problem State:\n{}", problem_clone);
    // find source and place tachyon beam beneath it
    for i in 0..problem_clone.cells[0].len() {
        if problem_clone.cells[0][i] == TachyonCell::Source {
            problem_clone.cells[1][i] = TachyonCell::Realities(1);
            break;
        }
    }
    //println!("Problem State After Placing Beam:\n{}", problem_clone);
    for i in 1..problem_clone.cells.len()-1 {
        for j in 0..problem_clone.cells[i].len() {
            if let TachyonCell::Realities(r) = problem_clone.cells[i][j]{
                match problem_clone.cells[i+1][j] {
                    TachyonCell::Empty => {
                        problem_clone.cells[i+1][j] = TachyonCell::Realities(r);
                    }
                    TachyonCell::Splitter => {
                        if j > 0 {
                            match problem_clone.cells[i+1][j-1] {
                                TachyonCell::Realities(existing_r) => {
                                    problem_clone.cells[i+1][j-1] = TachyonCell::Realities(existing_r + r);
                                }
                                _ => {
                                    problem_clone.cells[i+1][j-1] = TachyonCell::Realities(r);
                                }
                            }
                        }
                        if j < problem_clone.cells[i].len() - 1 {
                            match problem_clone.cells[i+1][j+1] {
                                TachyonCell::Realities(existing_r) => {
                                    problem_clone.cells[i+1][j+1] = TachyonCell::Realities(existing_r + r);
                                }
                                _ => {
                                    problem_clone.cells[i+1][j+1] = TachyonCell::Realities(r);
                                }
                            }
                        }
                    }
                    TachyonCell::Realities(existing_r) => {
                        problem_clone.cells[i+1][j] = TachyonCell::Realities(existing_r + r);
                    }
                    _=> {
                        panic!("Unexpected cell type below realities cell");
                    }
                }
            }
        }
        //println!("Problem State After Row {}:\n{}", i, problem_clone);
    }
    let mut total_realities = 0u64;
    for i in 0..problem_clone.cells[problem_clone.cells.len()-1].len(){
        if let TachyonCell::Realities(r) = problem_clone.cells[problem_clone.cells.len()-1][i]{
            total_realities += r;
        }
    }
    total_realities
}