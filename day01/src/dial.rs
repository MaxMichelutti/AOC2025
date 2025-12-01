#[derive(Debug, Clone)]
pub struct Dial {
    pub position: u64,
    pub zero_hit_counter: u64,
}

impl Dial {
    pub const DIAL_SIZE: u64 = 100;

    pub fn new() -> Dial {
        Dial {
            position: 50,
            zero_hit_counter: 0,
        }
    }

    pub fn get_position(&self) -> u64 {
        self.position
    }

    pub fn get_zero_hits(&self) -> u64 {
        self.zero_hit_counter
    }

    pub fn rotate_left(&mut self, mut steps: u64) {
        self.zero_hit_counter += steps / Self::DIAL_SIZE;
        steps %= Self::DIAL_SIZE;
        if self.position < steps {
            if self.position != 0 {
                self.zero_hit_counter += 1;
            }
            self.position = Self::DIAL_SIZE - (steps - self.position);
        } else {
            self.position -= steps;
            if self.position == 0 {
                self.zero_hit_counter += 1;
            }
        }
    }

    pub fn rotate_right(&mut self, mut steps: u64) {
        self.zero_hit_counter += steps / Self::DIAL_SIZE;
        steps %= Self::DIAL_SIZE;
        if self.position + steps >= Self::DIAL_SIZE {
            self.zero_hit_counter += 1;
        }
        self.position = (self.position + steps) % Self::DIAL_SIZE;
    }
}
