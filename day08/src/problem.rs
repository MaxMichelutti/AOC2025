#[derive(Debug, Clone)]
pub struct Problem {
    pub junction_boxes: Vec<JunctionBox>,
}

impl Problem {
    pub fn new() -> Self {
        Problem {
            junction_boxes: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.junction_boxes.len()
    }

    pub fn add_junction_box(&mut self, junction_box: JunctionBox) {
        self.junction_boxes.push(junction_box);
    }
}

impl Default for Problem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JunctionBox {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl JunctionBox {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        JunctionBox { x, y, z }
    }

    /// returns the squared distance to another junction box
    pub fn distance_to(&self, other: &JunctionBox) -> usize {
        let dx = self.x as i64 - other.x as i64;
        let dy = self.y as i64 - other.y as i64;
        let dz = self.z as i64 - other.z as i64;
        (dx * dx + dy * dy + dz * dz) as usize
    }
}
