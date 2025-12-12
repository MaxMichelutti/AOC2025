#[derive(Clone, Debug)]
pub struct Problem{
   pub cells: Vec<Vec<TachyonCell>>
}

impl Problem {
    pub fn new() -> Self {
        Problem{
            cells: vec![vec![]]
        }
    }

    pub fn new_row(&mut self) {
        self.cells.push(vec![]);
    }

    pub fn add_cell_to_last_row(&mut self, cell: TachyonCell) {
        if let Some(last_row) = self.cells.last_mut() {
            last_row.push(cell);
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum TachyonCell {
    Empty,
    Splitter,
    Source,
    TachyonBeam,
    Realities(u64)
}

impl TachyonCell {
    pub fn is_empty(&self) -> bool {
        *self == TachyonCell::Empty
    }
}

impl std::fmt::Display for TachyonCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            TachyonCell::Empty => '.',
            TachyonCell::Splitter => '^',
            TachyonCell::Source => 'S',
            TachyonCell::TachyonBeam => '|',
            TachyonCell::Realities(r) => (*r as u8 + b'0') as char, // only really works with less than 10 realities!
        };
        write!(f, "{}", symbol)
    }
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}