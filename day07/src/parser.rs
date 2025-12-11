use crate::problem::Problem;
use crate::problem::TachyonCell;

pub struct Parser{

}

impl Parser {
    pub fn new() -> Self {
        Parser{}
    }

    pub fn parse<R: std::io::Read>(&mut self, input: &mut R) -> Result<Problem, String> {
        let mut problem = Problem::new();
        let mut buffer = [0; 1];
        let mut has_source = false;
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let input_char = buffer[0] as char;
            match input_char {
                'S' | 's' => {
                    if problem.cells.len() > 1 || has_source {
                        return Err("Source cell can only be in the first row".to_string());
                    }
                    has_source = true;
                    problem.add_cell_to_last_row(TachyonCell::Source);
                }
                '.' => {
                    problem.add_cell_to_last_row(TachyonCell::Empty);
                }
                '^' => {
                    problem.add_cell_to_last_row(TachyonCell::Splitter);
                }
                '\n' => {
                    problem.new_row();
                }
                _ => {
                    return Err("Unexpected character".to_string());
                }
            }
        }
        Ok(problem)
    }
}