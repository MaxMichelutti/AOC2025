use crate::problem::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ParserStatus {
    PresentIdOrRegionWidth,
    PresentRow1Col1,
    PresentRow1Col2,
    PresentRow1Col3,
    PresentRow2Col1,
    PresentRow2Col2,
    PresentRow2Col3,
    PresentRow3Col1,
    PresentRow3Col2,
    PresentRow3Col3,
    RegionHeight,
    RegionRequirements,
}

pub struct Parser {
    status: ParserStatus,
    region_width: usize,
    region_height: usize,
    region_requirements: Vec<usize>,
    current_integer: isize,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            status: ParserStatus::PresentIdOrRegionWidth,
            region_width: 0,
            region_height: 0,
            region_requirements: Vec::new(),
            current_integer: -1,
        }
    }

    pub fn parse<R: std::io::Read>(&mut self, input: &mut R) -> Result<Problem, String> {
        let mut problem = Problem::new();
        let mut buffer = [0; 1];
        let mut current_present = Present::new(0);
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let input_char = buffer[0] as char;
            match input_char {
                '0'..='9' => {
                    let digit = input_char as u8 - b'0';
                    match self.status {
                        ParserStatus::PresentIdOrRegionWidth
                        | ParserStatus::RegionRequirements
                        | ParserStatus::RegionHeight => {
                            if self.current_integer == -1 {
                                self.current_integer = 0;
                            }
                            // Handle Present ID
                            self.current_integer = self.current_integer * 10 + digit as isize;
                        }
                        _ => {
                            return Err(format!(
                                "Unexpected digit in state {:?}: {}",
                                self.status, input_char
                            ));
                        }
                    }
                }
                ':' => match self.status {
                    ParserStatus::PresentIdOrRegionWidth => {
                        current_present.set_id(self.current_integer as u8);
                        self.current_integer = -1;
                        self.status = ParserStatus::PresentRow1Col1;
                    }
                    ParserStatus::RegionHeight => {
                        self.region_height = self.current_integer as usize;
                        self.current_integer = -1;
                        self.status = ParserStatus::RegionRequirements;
                    }
                    _ => {
                        return Err(format!("Unexpected ':' in state {:?}", self.status));
                    }
                },
                'x' => match self.status {
                    ParserStatus::PresentIdOrRegionWidth => {
                        self.region_width = self.current_integer as usize;
                        self.current_integer = -1;
                        self.status = ParserStatus::RegionHeight;
                    }
                    _ => {
                        return Err(format!("Unexpected 'x' in state {:?}", self.status));
                    }
                },
                ' ' => match self.status {
                    ParserStatus::RegionRequirements => {
                        if self.current_integer != -1 {
                            self.region_requirements.push(self.current_integer as usize);
                            self.current_integer = -1;
                        }
                    }
                    _ => {
                        return Err(format!("Unexpected space in state {:?}", self.status));
                    }
                },
                '.' => match self.status {
                    ParserStatus::PresentRow1Col1 => {
                        self.status = ParserStatus::PresentRow1Col2;
                    }
                    ParserStatus::PresentRow1Col2 => {
                        self.status = ParserStatus::PresentRow1Col3;
                    }
                    ParserStatus::PresentRow1Col3 => {
                        self.status = ParserStatus::PresentRow2Col1;
                    }
                    ParserStatus::PresentRow2Col1 => {
                        self.status = ParserStatus::PresentRow2Col2;
                    }
                    ParserStatus::PresentRow2Col2 => {
                        self.status = ParserStatus::PresentRow2Col3;
                    }
                    ParserStatus::PresentRow2Col3 => {
                        self.status = ParserStatus::PresentRow3Col1;
                    }
                    ParserStatus::PresentRow3Col1 => {
                        self.status = ParserStatus::PresentRow3Col2;
                    }
                    ParserStatus::PresentRow3Col2 => {
                        self.status = ParserStatus::PresentRow3Col3;
                    }
                    ParserStatus::PresentRow3Col3 => {
                        problem.add_present(current_present.clone());
                        current_present.reset_shape();
                        self.status = ParserStatus::PresentIdOrRegionWidth;
                    }
                    _ => {
                        return Err(format!("Unexpected '.' in state {:?}", self.status));
                    }
                },
                '#' => match self.status {
                    ParserStatus::PresentRow1Col1 => {
                        current_present.set_shape_cell(0, 0);
                        self.status = ParserStatus::PresentRow1Col2;
                    }
                    ParserStatus::PresentRow1Col2 => {
                        current_present.set_shape_cell(0, 1);
                        self.status = ParserStatus::PresentRow1Col3;
                    }
                    ParserStatus::PresentRow1Col3 => {
                        current_present.set_shape_cell(0, 2);
                        self.status = ParserStatus::PresentRow2Col1;
                    }
                    ParserStatus::PresentRow2Col1 => {
                        current_present.set_shape_cell(1, 0);
                        self.status = ParserStatus::PresentRow2Col2;
                    }
                    ParserStatus::PresentRow2Col2 => {
                        current_present.set_shape_cell(1, 1);
                        self.status = ParserStatus::PresentRow2Col3;
                    }
                    ParserStatus::PresentRow2Col3 => {
                        current_present.set_shape_cell(1, 2);
                        self.status = ParserStatus::PresentRow3Col1;
                    }
                    ParserStatus::PresentRow3Col1 => {
                        current_present.set_shape_cell(2, 0);
                        self.status = ParserStatus::PresentRow3Col2;
                    }
                    ParserStatus::PresentRow3Col2 => {
                        current_present.set_shape_cell(2, 1);
                        self.status = ParserStatus::PresentRow3Col3;
                    }
                    ParserStatus::PresentRow3Col3 => {
                        current_present.set_shape_cell(2, 2);
                        problem.add_present(current_present.clone());
                        current_present.reset_shape();
                        self.status = ParserStatus::PresentIdOrRegionWidth;
                    }
                    _ => {
                        return Err(format!("Unexpected '#' in state {:?}", self.status));
                    }
                },
                '\n' => {
                    match self.status {
                        ParserStatus::PresentRow1Col1
                        | ParserStatus::PresentRow2Col1
                        | ParserStatus::PresentRow3Col1 => {}
                        ParserStatus::RegionRequirements => {
                            if self.current_integer != -1 {
                                self.region_requirements.push(self.current_integer as usize);
                                self.current_integer = -1;
                            }
                            let region = Region {
                                width: self.region_width,
                                height: self.region_height,
                                requirements: self.region_requirements.clone(),
                            };
                            problem.add_region(region);
                            self.region_requirements.clear();
                            self.status = ParserStatus::PresentIdOrRegionWidth;
                        }
                        ParserStatus::PresentIdOrRegionWidth => {
                            // ignore empty lines between entries
                        }
                        _ => {
                            return Err(format!("Unexpected newline in state {:?}", self.status));
                        }
                    }
                }
                _ => {
                    return Err(format!("Unexpected character: {}", input_char));
                }
            }
        }
        if self.status == ParserStatus::RegionRequirements {
            if self.current_integer != -1 {
                self.region_requirements.push(self.current_integer as usize);
                self.current_integer = -1;
            }
            let region = Region {
                width: self.region_width,
                height: self.region_height,
                requirements: self.region_requirements.clone(),
            };
            problem.add_region(region);
            self.region_requirements.clear();
        }
        Ok(problem)
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
