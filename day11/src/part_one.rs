use crate::problem::Problem;
use crate::problem::TachyonCell;

pub fn part_one(problem: &Problem) -> u64 {
    let mut problem_clone = problem.clone();
    //println!("Initial Problem State:\n{}", problem_clone);
    let mut total_splittings = 0u64;
    // find source and place tachyon beam beneath it
    for i in 0..problem_clone.cells[0].len() {
        if problem_clone.cells[0][i] == TachyonCell::Source {
            problem_clone.cells[1][i] = TachyonCell::TachyonBeam;
            break;
        }
    }
    //println!("Problem State After Placing Beam:\n{}", problem_clone);
    for i in 1..problem_clone.cells.len()-1 {
        for j in 0..problem_clone.cells[i].len() {
            if problem_clone.cells[i][j] == TachyonCell::TachyonBeam{
                match problem_clone.cells[i+1][j] {
                    TachyonCell::Empty => {
                        problem_clone.cells[i+1][j] = TachyonCell::TachyonBeam;
                    }
                    TachyonCell::Splitter => {
                        total_splittings += 1;
                        if j > 0 {
                            problem_clone.cells[i+1][j-1] = TachyonCell::TachyonBeam;
                        }
                        if j < problem_clone.cells[i].len() - 1 {
                            problem_clone.cells[i+1][j+1] = TachyonCell::TachyonBeam;
                        }
                    }
                    _ => {}
                }
            }
        }
        //println!("Problem State After Row {}:\n{}", i, problem_clone);
    }
    total_splittings
}