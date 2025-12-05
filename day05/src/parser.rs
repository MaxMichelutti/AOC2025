use crate::problem::DatabaseRange;
use crate::problem::Problem;
use std::io::Read;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ParserStatus {
    RangeStart,
    RangeEnd,
    Ingredient,
}

#[derive(Debug)]
pub struct Parser {
    status: ParserStatus,
    current_range_start: u64,
    current_range_end: u64,
    current_ingredient: u64,
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            current_range_start: 0,
            current_range_end: 0,
            current_ingredient: 0,
            status: ParserStatus::RangeStart,
        }
    }

    pub fn parse<R: Read>(&mut self, mut input: R) -> Result<Problem, &'static str> {
        let mut problem = Problem::new();
        let mut buffer = [0; 1];
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let input_char = buffer[0] as char;
            match input_char {
                '0'..='9' => match self.status {
                    ParserStatus::RangeStart => {
                        self.current_range_start =
                            self.current_range_start * 10 + (input_char as u8 - b'0') as u64;
                    }
                    ParserStatus::RangeEnd => {
                        self.current_range_end =
                            self.current_range_end * 10 + (input_char as u8 - b'0') as u64;
                    }
                    ParserStatus::Ingredient => {
                        self.current_ingredient =
                            self.current_ingredient * 10 + (input_char as u8 - b'0') as u64;
                    }
                },
                '\n' => {
                    match self.status {
                        ParserStatus::RangeStart => {
                            self.status = ParserStatus::Ingredient;
                        }
                        ParserStatus::RangeEnd => {
                            // add range to ranges
                            problem.add_range(DatabaseRange::new(
                                self.current_range_start,
                                self.current_range_end,
                            ));
                            self.current_range_start = 0;
                            self.current_range_end = 0;
                            self.status = ParserStatus::RangeStart;
                        }
                        ParserStatus::Ingredient => {
                            if self.current_ingredient != 0 {
                                problem.add_ingredient(self.current_ingredient);
                                self.current_ingredient = 0;
                            }
                        }
                    }
                }
                '-' => {
                    if self.status == ParserStatus::RangeStart {
                        self.status = ParserStatus::RangeEnd
                    } else {
                        return Err("unexpected \"-\"");
                    }
                }
                _ => {
                    return Err("unexpected character");
                }
            }
        }
        if self.current_ingredient != 0 {
            problem.add_ingredient(self.current_ingredient);
        }
        Ok(problem)
    }
}
