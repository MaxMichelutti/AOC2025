use std::io::Read;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse<R: Read>(&mut self, mut input: R) -> Result<Vec<Bank>, &'static str> {
        let mut buffer = [0; 1];
        let mut items = vec![];
        let mut current_bank = Bank::new();
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let item = buffer[0] as char;
            match item {
                '0'..='9' => {
                    current_bank.add_battery(item.to_digit(10).unwrap() as u8);
                }
                '\n' => {
                    items.push(current_bank);
                    current_bank = Bank::new();
                }
                _ => return Err("Invalid character in input"),
            }
        }

        // final action may not be followed by newline
        if !current_bank.is_empty() {
            items.push(current_bank);
        }
        Ok(items)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Bank {
    pub batteries: Vec<u8>,
}

impl Bank {
    pub fn new() -> Bank {
        Bank { batteries: vec![] }
    }

    pub fn add_battery(&mut self, capacity: u8) {
        self.batteries.push(capacity);
    }

    pub fn is_empty(&self) -> bool {
        self.batteries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.batteries.len()
    }
}
