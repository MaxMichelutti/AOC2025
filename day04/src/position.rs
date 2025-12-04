use crate::parser::GridCell;
use crate::parser::Problem;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Default)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    pub fn is_forkliftable(&self, problem: &Problem) -> bool {
        if problem.grid[self.row][self.col] == GridCell::Empty {
            return false;
        }
        let adjacent = self.generate_adjacent(problem.width, problem.height);
        let mut occupied_count = 0;
        for pos in adjacent {
            if problem.grid[pos.row][pos.col] == GridCell::PaperRoll {
                occupied_count += 1;
            }
        }
        occupied_count < 4
    }

    fn generate_adjacent(&self, width: usize, height: usize) -> Vec<Position> {
        let mut positions = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let new_row = self.row as isize + i;
                let new_col = self.col as isize + j;
                if new_row >= 0
                    && new_row < width as isize
                    && new_col >= 0
                    && new_col < height as isize
                {
                    positions.push(Position {
                        row: new_row as usize,
                        col: new_col as usize,
                    });
                }
            }
        }
        positions
    }
}
