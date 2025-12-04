#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum GridCell {
    Empty,
    PaperRoll,
}

impl std::fmt::Display for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            GridCell::Empty => '.',
            GridCell::PaperRoll => '@',
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Default)]
pub struct Parser {
    current_row: Vec<GridCell>,
    width: usize,
    height: usize,
    rows: Vec<Vec<GridCell>>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            current_row: Vec::new(),
            width: 0,
            height: 0,
            rows: Vec::new(),
        }
    }

    pub fn parse<R: std::io::Read>(&mut self, mut input: R) -> Result<Problem, &'static str> {
        let mut buffer = [0; 1];
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let item = buffer[0] as char;
            match item {
                '.' => {
                    self.current_row.push(GridCell::Empty);
                }
                '@' => {
                    self.current_row.push(GridCell::PaperRoll);
                }
                '\n' => {
                    let current_row_length = self.current_row.len();
                    let current_row = std::mem::take(&mut self.current_row);
                    self.rows.push(current_row);
                    if self.width == 0 {
                        self.width = current_row_length;
                    } else if self.width != current_row_length {
                        return Err("Inconsistent row lengths");
                    }
                    self.height += 1;
                }
                _ => return Err("Invalid character in input"),
            }
        }

        // final action may not be followed by newline
        if !self.current_row.is_empty() {
            if self.current_row.len() != self.width {
                return Err("Inconsistent row lengths");
            }
            let current_row = std::mem::take(&mut self.current_row);
            self.rows.push(current_row);
            self.height += 1;
        }
        Ok(Problem {
            grid: std::mem::take(&mut self.rows),
            width: self.width,
            height: self.height,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Problem {
    pub grid: Vec<Vec<GridCell>>,
    pub width: usize,
    pub height: usize,
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid ({}x{}):", self.width, self.height)?;
        for row in &self.grid {
            for cell in row {
                let symbol = match cell {
                    GridCell::Empty => '.',
                    GridCell::PaperRoll => '@',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
