use crate::problem::JunctionBox;
use crate::problem::Problem;

pub struct Parser {
    first_digit: usize,
    second_digit: usize,
    third_digit: usize,
    status: ParserStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParserStatus {
    FirstDigit,
    SecondDigit,
    ThirdDigit,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            first_digit: 0,
            second_digit: 0,
            third_digit: 0,
            status: ParserStatus::FirstDigit,
        }
    }

    fn reset_digits(&mut self) {
        self.first_digit = 0;
        self.second_digit = 0;
        self.third_digit = 0;
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
                    let digit = (input_char as u8 - b'0') as usize;
                    match self.status {
                        ParserStatus::FirstDigit => {
                            self.first_digit = self.first_digit * 10 + digit
                        }
                        ParserStatus::SecondDigit => {
                            self.second_digit = self.second_digit * 10 + digit
                        }
                        ParserStatus::ThirdDigit => {
                            self.third_digit = self.third_digit * 10 + digit
                        }
                    }
                }
                ',' => match self.status {
                    ParserStatus::FirstDigit => self.status = ParserStatus::SecondDigit,
                    ParserStatus::SecondDigit => self.status = ParserStatus::ThirdDigit,
                    ParserStatus::ThirdDigit => {
                        return Err("Unexpected comma".to_string());
                    }
                },
                '\n' => {
                    if self.status != ParserStatus::ThirdDigit {
                        return Err("Incomplete junction box definition".to_string());
                    }
                    problem.add_junction_box(JunctionBox::new(
                        self.first_digit,
                        self.second_digit,
                        self.third_digit,
                    ));
                    self.reset_digits();
                    self.status = ParserStatus::FirstDigit;
                }
                _ => {
                    return Err("Unexpected character".to_string());
                }
            }
        }
        if self.status != ParserStatus::ThirdDigit {
            return Err("Incomplete junction box definition".to_string());
        }
        problem.add_junction_box(JunctionBox::new(
            self.first_digit,
            self.second_digit,
            self.third_digit,
        ));
        Ok(problem)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
