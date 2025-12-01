use std::io::Read;

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
enum ParserStatus {
    ReadingDirection,
    ReadingDigit,
}

pub struct Parser {
    status: ParserStatus,
    current_direction: Direction,
    current_value: u64,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            status: ParserStatus::ReadingDirection,
            current_value: 0,
            current_direction: Direction::Left,
        }
    }

    pub fn parse<R: Read>(&mut self, mut input: R) -> Result<Vec<Action>, &'static str> {
        let mut buffer = [0; 1];
        let mut items = vec![];
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let item = buffer[0] as char;
            match item {
                'L' => {
                    if self.status != ParserStatus::ReadingDirection {
                        return Err("Unexpected 'L' while reading digit");
                    }
                    self.current_direction = Direction::Left;
                    self.status = ParserStatus::ReadingDigit;
                }
                'R' => {
                    if self.status != ParserStatus::ReadingDirection {
                        return Err("Unexpected 'R' while reading digit");
                    }
                    self.current_direction = Direction::Right;
                    self.status = ParserStatus::ReadingDigit;
                }
                '0'..='9' => {
                    if self.status != ParserStatus::ReadingDigit {
                        return Err("Unexpected digit while reading direction");
                    }
                    self.current_value = self.current_value * 10 + (item as u8 - b'0') as u64;
                }
                '\n' => {
                    if self.status != ParserStatus::ReadingDigit {
                        return Err("Unexpected newline while reading direction");
                    }
                    items.push(self.generate_action());
                    self.status = ParserStatus::ReadingDirection;
                    self.current_value = 0;
                }
                _ => return Err("Invalid character in input"),
            }
        }
        // final action may not be followed by newline
        if self.status == ParserStatus::ReadingDigit {
            items.push(self.generate_action());
        }
        Ok(items)
    }

    fn generate_action(&mut self) -> Action {
        Action::new(self.current_direction, self.current_value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct Action {
    pub direction: Direction,
    pub steps: u64,
}

impl Action {
    pub fn new(direction: Direction, value: u64) -> Action {
        Action {
            direction,
            steps: value,
        }
    }
}
