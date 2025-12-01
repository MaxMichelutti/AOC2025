use crate::{dial::Dial, parser::Action};

pub fn part_two(input: &[Action]) -> u64 {
    let mut dial = Dial::new();
    for action in input {
        if action.steps == 0 {
            // No movement, skip
            continue;
        }
        match action.direction {
            crate::parser::Direction::Left => dial.rotate_left(action.steps),
            crate::parser::Direction::Right => dial.rotate_right(action.steps),
        }
    }
    dial.get_zero_hits()
}
