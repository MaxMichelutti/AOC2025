#[derive(Debug, Clone)]
pub struct Problem {
    pub tiles: Vec<Tile>,
}

impl Problem {
    pub fn new() -> Self {
        Problem { tiles: vec![] }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }
}

impl Default for Problem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
}

impl Tile {
    pub fn new(x: usize, y: usize) -> Self {
        Tile { x, y }
    }

    pub fn compute_area(&self, other: &Tile) -> usize {
        let width = if self.x > other.x {
            self.x - other.x + 1
        } else {
            other.x - self.x + 1
        };
        let height = if self.y > other.y {
            self.y - other.y + 1
        } else {
            other.y - self.y + 1
        };
        width * height
    }
}
