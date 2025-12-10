use crate::problem2::Problem2;
use crate::problem::MathOperation;
use std::io::Read;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ParserStatus {
    Numbers,
    Operations,
    Done
}

#[derive(Debug)]
pub struct Parser {
    status: ParserStatus,
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            status: ParserStatus::Numbers,
        }
    }

    pub fn parse<R: Read>(&mut self, mut input: R) -> Result<Problem2, &'static str> {
        let mut problem = Problem2::new();
        let mut buffer = [0; 1];
        let mut current_row = Vec::new();
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let input_char = buffer[0] as char;
            match input_char {
                '0'..='9' => {
                    match self.status {
                        ParserStatus::Numbers => {
                            current_row.push(input_char as u8 - b'0');
                        },
                        ParserStatus::Operations => {
                            return Err("Unexpected number while parsing operations");
                        }
                        ParserStatus::Done => {
                            return Err("Unexpected number after parsing is done");
                        }
                    }                    
                }
                ' ' => {
                    match self.status{
                        ParserStatus::Numbers => {
                            current_row.push(0);
                        },
                        _=>{}
                    }
                    
                }
                '\n' => {
                    match self.status {
                        ParserStatus::Numbers => {
                            problem.add_row(std::mem::replace(&mut current_row, Vec::new()));
                        }
                        ParserStatus::Operations => {
                            self.status = ParserStatus::Done;
                        }
                        _=>{}
                    }
                }
                '+' | '*' => {
                    let op = match input_char {
                        '+' => MathOperation::Add,
                        '*' => MathOperation::Multiply,
                        _ => unreachable!(),
                    };
                    match self.status {
                        ParserStatus::Numbers => {
                            if !current_row.is_empty() {
                                return Err("Operation found while reading numbers");
                            }
                            self.status = ParserStatus::Operations;
                            problem.add_operation(op);
                        }
                        ParserStatus::Operations => {
                            problem.add_operation(op);
                        }
                        ParserStatus::Done => {
                            return Err("Unexpected operation after parsing is done");
                        }
                    }
                }
                _ => {
                    return Err("unexpected character");
                }
            }
        }
        Ok(problem)
    }
}