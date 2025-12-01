use crate::{dial::Dial, parser::Action};

pub fn part_one(input: &[Action]) -> u64 {
    let mut dial = Dial::new();
    let mut zero_counter = 0u64;
    for action in input {
        match action.direction {
            crate::parser::Direction::Left => dial.rotate_left(action.steps),
            crate::parser::Direction::Right => dial.rotate_right(action.steps),
        }
        if dial.get_position() == 0 {
            zero_counter += 1;
        }
    }
    zero_counter
}
