#[derive(Debug, Clone)]
pub struct Problem {
    pub presents: Vec<Present>,
    pub regions: Vec<Region>,
}

impl Problem {
    pub fn new() -> Self {
        Problem {
            presents: Vec::new(),
            regions: Vec::new(),
        }
    }

    pub fn add_present(&mut self, present: Present) {
        self.presents.push(present);
    }

    pub fn add_region(&mut self, region: Region) {
        self.regions.push(region);
    }
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Problem:")?;
        writeln!(f, "Presents:")?;
        for present in &self.presents {
            writeln!(f, "{}", present)?;
        }
        writeln!(f, "Regions:")?;
        for region in &self.regions {
            writeln!(f, "{}", region)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Present {
    pub id: u8,
    pub shape: [[bool; 3]; 3],
}

impl Present {
    pub fn new(id: u8) -> Self {
        Present {
            id,
            shape: [[false; 3]; 3],
        }
    }

    pub fn set_id(&mut self, id: u8) {
        self.id = id;
    }

    pub fn reset_shape(&mut self) {
        self.shape = [[false; 3]; 3];
    }

    pub fn set_shape_cell(&mut self, row: usize, col: usize) {
        if row < 3 && col < 3 {
            self.shape[row][col] = true;
        }
    }

    pub fn total_size(&self) -> u8 {
        let mut total = 0;
        for row in 0..3 {
            for col in 0..3 {
                if self.shape[row][col] {
                    total += 1;
                }
            }
        }
        total
    }
}

impl std::fmt::Display for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Present ID: {}", self.id)?;
        for row in 0..3 {
            for col in 0..3 {
                let cell = if self.shape[row][col] { '#' } else { '.' };
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    pub width: usize,
    pub height: usize,
    pub requirements: Vec<usize>,
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Region ({} x {}):", self.width, self.height)?;
        writeln!(f, "Requirements: {:?}", self.requirements)?;
        Ok(())
    }
}

impl Default for Problem {
    fn default() -> Self {
        Self::new()
    }
}
