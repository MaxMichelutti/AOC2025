use crate::problem::Problem;
use crate::problem::Tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParserStatus {
    ReadingFirst,
    ReadingSecond,
}

pub struct Parser {
    first: usize,
    second: usize,
    status: ParserStatus,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            first: 0,
            second: 0,
            status: ParserStatus::ReadingFirst,
        }
    }

    pub fn reset_tile(&mut self) {
        self.first = 0;
        self.second = 0;
        self.status = ParserStatus::ReadingFirst;
    }

    pub fn parse<R: std::io::Read>(&mut self, input: &mut R) -> Result<Problem, String> {
        let mut problem = Problem::new();
        let mut buffer = [0; 1];
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let input_char = buffer[0] as char;
            match input_char {
                '0'..='9' => {
                    let digit = input_char.to_digit(10).unwrap() as usize;
                    match self.status {
                        ParserStatus::ReadingFirst => {
                            self.first = self.first * 10 + digit;
                        }
                        ParserStatus::ReadingSecond => {
                            self.second = self.second * 10 + digit;
                        }
                    }
                }
                ',' => {
                    if let ParserStatus::ReadingFirst = self.status {
                        self.status = ParserStatus::ReadingSecond;
                    } else {
                        return Err("Unexpected comma while reading second number".to_string());
                    }
                }
                '\n' => {
                    if let ParserStatus::ReadingSecond = self.status {
                        problem.add_tile(Tile::new(self.first, self.second));
                        self.reset_tile();
                    } else {
                        return Err("Unexpected newline while reading first number".to_string());
                    }
                }
                _ => {
                    return Err(format!("Unexpected character: {}", input_char));
                }
            }
        }
        if self.status == ParserStatus::ReadingSecond {
            problem.add_tile(Tile::new(self.first, self.second));
        }
        Ok(problem)
    }
}
