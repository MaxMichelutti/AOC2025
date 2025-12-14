use crate::problem::*;

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
enum ParserStatus {
    LightOpen,
    LightInput,
    LightDone,
    ButtonInput,
    ButtonDone,
    JoltageInput,
    JoltageDone,
}

pub struct Parser {
    status: ParserStatus,
    current_number: usize,
    current_machine: Machine,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            status: ParserStatus::LightOpen,
            current_number: 0,
            current_machine: Machine::new(),
        }
    }

    pub fn parse<R: std::io::Read>(mut self, input: &mut R) -> Result<Problem, String> {
        let mut problem = Problem::new();
        let mut buffer = [0; 1];
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let input_char = buffer[0] as char;
            match input_char {
                '[' => {
                    if self.status != ParserStatus::LightOpen {
                        return Err("Unexpected opening bracket".to_string());
                    }
                    self.status = ParserStatus::LightInput;
                }
                '.' => {
                    if self.status != ParserStatus::LightInput {
                        return Err("Unexpected light input".to_string());
                    }
                    self.current_machine.add_light(LightConfig::Off);
                }
                '#' => {
                    if self.status != ParserStatus::LightInput {
                        return Err("Unexpected light input".to_string());
                    }
                    self.current_machine.add_light(LightConfig::On);
                }
                ']' => {
                    if self.status != ParserStatus::LightInput {
                        return Err("Unexpected closing bracket".to_string());
                    }
                    self.status = ParserStatus::LightDone;
                }
                '(' => {
                    if self.status != ParserStatus::LightDone
                        && self.status != ParserStatus::ButtonDone
                    {
                        return Err("Unexpected opening parenthesis".to_string());
                    }
                    self.current_machine.add_button(Button {
                        toggles: Vec::new(),
                    });
                    self.status = ParserStatus::ButtonInput;
                }
                ',' => match self.status {
                    ParserStatus::ButtonInput => {
                        self.current_machine.add_to_last_button(self.current_number);
                        self.current_number = 0;
                    }
                    ParserStatus::JoltageInput => {
                        self.current_machine.add_joltage(self.current_number);
                        self.current_number = 0;
                    }
                    _ => {
                        return Err("Unexpected comma".to_string());
                    }
                },
                ')' => {
                    if self.status != ParserStatus::ButtonInput {
                        return Err("Unexpected closing parenthesis".to_string());
                    }
                    self.current_machine.add_to_last_button(self.current_number);
                    self.current_number = 0;
                    self.status = ParserStatus::ButtonDone;
                }
                '{' => {
                    if self.status != ParserStatus::ButtonDone {
                        return Err("Unexpected opening brace".to_string());
                    }
                    self.status = ParserStatus::JoltageInput;
                }
                '0'..='9' => {
                    let digit = input_char.to_digit(10).unwrap() as usize;
                    self.current_number = self.current_number * 10 + digit;
                    match self.status {
                        ParserStatus::ButtonDone
                        | ParserStatus::LightDone
                        | ParserStatus::LightOpen
                        | ParserStatus::JoltageDone => {
                            return Err("Unexpected digit after done status".to_string());
                        }
                        _ => {}
                    }
                }
                '}' => {
                    if self.status != ParserStatus::JoltageInput {
                        return Err("Unexpected closing brace".to_string());
                    }
                    self.current_machine.add_joltage(self.current_number);
                    self.current_number = 0;
                    self.status = ParserStatus::JoltageDone;
                }
                '\n' => {
                    if self.status != ParserStatus::JoltageDone {
                        return Err("Unexpected newline".to_string());
                    }
                    problem.add_machine(self.current_machine);
                    self.current_machine = Machine::new();
                    self.status = ParserStatus::LightOpen;
                }
                // ignore white spaces
                ' ' => {}
                _ => {
                    return Err(format!("Unexpected character: {}", input_char));
                }
            }
        }
        if self.status == ParserStatus::JoltageDone {
            problem.add_machine(self.current_machine);
        }
        Ok(problem)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
