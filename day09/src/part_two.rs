use crate::problem::Problem;
use crate::problem::Tile;

// NOTE: This solution still does not account the fact that 2 lines may be
// one near the other, making the white space between them non-existent.
// That would be much harder to check and would require some clever modifications
// of the original problem (the list of tiles) in order to accomplish.

pub fn part_two(problem: &Problem) -> u64 {
    let tiles = &problem.tiles;
    let total_tiles = tiles.len();
    let mut lines = vec![];
    for i in 0..total_tiles {
        let j = (i + 1) % total_tiles;
        let line = Line::new(&tiles[i], &tiles[j]);
        lines.push(line);
    }
    let mut biggest_square = 0;
    for i in 0..total_tiles {
        for j in (i + 1)..total_tiles {
            let rect = Rectangle::new(&tiles[i], &tiles[j]);
            if rect.area > biggest_square {
                let mut is_ok = true;
                for l in &lines {
                    if rect.is_entered_by(l) || rect.is_crossed_by(l) {
                        is_ok = false;
                        break;
                    }
                }
                if is_ok {
                    biggest_square = rect.area;
                }
            }
        }
    }
    biggest_square as u64
}

struct Rectangle {
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
    pub area: usize,
}

impl Rectangle {
    pub fn new(t1: &Tile, t2: &Tile) -> Self {
        Rectangle {
            start_x: std::cmp::min(t1.x, t2.x),
            start_y: std::cmp::min(t1.y, t2.y),
            end_x: std::cmp::max(t1.x, t2.x),
            end_y: std::cmp::max(t1.y, t2.y),
            area: t1.compute_area(t2),
        }
    }

    // there is a line with an end inside the rectangle
    pub fn is_entered_by(&self, l: &Line) -> bool {
        if l.is_horizontal() {
            // line is within vertical bounds
            l.other >= self.start_y && l.other <= self.end_y &&
            // line start is inside rect    
            ((self.start_x < l.start && l.start < self.end_x) ||
            // line end is inside rect
            (self.start_x < l.end && l.end < self.end_x))
        } else {
            // line is within horizontal bounds
            l.other >= self.start_x && l.other <= self.end_x &&
            // line start is inside rect 
            ((self.start_y < l.start && l.start < self.end_y) ||
            // line end is inside rect
            (self.start_y < l.end && l.end < self.end_y))
        }
    }

    // there is a line that crosses the rectangle from one side to the other
    pub fn is_crossed_by(&self, l: &Line) -> bool {
        if l.is_horizontal() {
            // line crosses the rectangle horizontally
            l.start < self.start_x
                && l.end > self.end_x
                && l.other > self.start_y
                && l.other < self.end_y
        } else {
            // line crosses the rectangle vertically
            l.start < self.start_y
                && l.end > self.end_y
                && l.other > self.start_x
                && l.other < self.end_x
        }
    }
}

struct Line {
    pub start: usize,
    pub end: usize,
    pub other: usize,
    pub is_horizontal: bool,
}

impl Line {
    pub fn new(t1: &Tile, t2: &Tile) -> Self {
        let is_horizontal = t1.y == t2.y;
        let start;
        let end;
        let other;
        if is_horizontal {
            start = std::cmp::min(t1.x, t2.x);
            end = std::cmp::max(t1.x, t2.x);
            other = t1.y;
        } else {
            start = std::cmp::min(t1.y, t2.y);
            end = std::cmp::max(t1.y, t2.y);
            other = t1.x;
        }
        Line {
            start,
            end,
            other,
            is_horizontal,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.is_horizontal
    }
}
