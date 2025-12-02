use std::io::Read;

#[derive(Debug, Clone, Eq, Copy, PartialEq)]
enum ParserStatus {
    ReadingFirstNumber,
    ReadingSecondNumber,
}

pub struct Parser {
    status: ParserStatus,
    current_first: u64,
    current_second: u64,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            status: ParserStatus::ReadingFirstNumber,
            current_first: 0,
            current_second: 0,
        }
    }

    pub fn parse<R: Read>(&mut self, mut input: R) -> Result<Vec<MyRange>, &'static str> {
        let mut buffer = [0; 1];
        let mut items = vec![];
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let item = buffer[0] as char;
            match item {
                '0'..='9' => match self.status {
                    ParserStatus::ReadingFirstNumber => {
                        self.current_first = self.current_first * 10 + (item as u8 - b'0') as u64;
                    }
                    ParserStatus::ReadingSecondNumber => {
                        self.current_second = self.current_second * 10 + (item as u8 - b'0') as u64;
                    }
                },
                '-' => {
                    if self.status != ParserStatus::ReadingFirstNumber {
                        return Err("Unexpected '-' while reading first number");
                    }
                    self.status = ParserStatus::ReadingSecondNumber;
                }
                ',' => {
                    if self.status != ParserStatus::ReadingSecondNumber {
                        return Err("Unexpected ',' while reading second number");
                    }
                    items.push(self.generate_range());
                    self.current_first = 0;
                    self.current_second = 0;
                    self.status = ParserStatus::ReadingFirstNumber;
                }
                '\n' => {}
                _ => return Err("Invalid character in input"),
            }
        }
        // final action may not be followed by newline
        if self.status == ParserStatus::ReadingSecondNumber {
            items.push(self.generate_range());
        }
        Ok(items)
    }

    fn generate_range(&mut self) -> MyRange {
        MyRange::new(self.current_first, self.current_second)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct MyRange {
    pub start: u64,
    pub end: u64,
}

impl MyRange {
    pub fn new(start: u64, end: u64) -> MyRange {
        MyRange { start, end }
    }
}
