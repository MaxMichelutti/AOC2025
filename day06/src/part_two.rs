use crate::problem2::Problem2;
use crate::problem::MathOperation;

pub fn part_two(problem: &Problem2) -> u64 {
    let mut min_index = 0;
    let mut max_index = 0;
    let mut operation_index = 0;
    let mut accumulator = 0u64;
    loop{
        // look for next empty column
        while max_index < problem.width{
            max_index += 1;
            if max_index >= problem.width{
                break;
            }
            let mut found_empty_column = true;
            for i in 0..problem.height{
                if problem.data[i][max_index] != 0{
                    found_empty_column = false;
                    break;
                }
            }
            if found_empty_column{
                break;
            }
        }
        // skip empty columns
        if min_index + 1 >= max_index{
            min_index += 1;
            max_index += 1;
            continue;
        }
        let mut vertical_values: Vec<u64> = vec![];
        for row in (min_index..max_index).rev() {
            let mut current_value = 0u64;
            for column in 0..problem.height {
                let value_read = problem.data[column][row] as u64;
                if value_read != 0 {
                    current_value = value_read + 10 * current_value;
                }
            }
            vertical_values.push(current_value);
        }
        let current_operation = problem.operations[operation_index];
        operation_index += 1;
        accumulator += match current_operation {
            MathOperation::Add => vertical_values.iter().sum::<u64>(),
            MathOperation::Multiply => vertical_values.iter().product::<u64>()
        };
        // update min_index aand exit if out of bounds
        min_index = max_index + 1;
        if min_index >= problem.width{
            break;
        }
    }
    accumulator
}