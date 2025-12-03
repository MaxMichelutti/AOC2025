use crate::parser::Bank;

pub fn part_two(banks: &[Bank]) -> u64 {
    banks.iter().map(best_joltage).sum()
}

fn best_joltage(bank: &Bank) -> u64 {
    let mut index_left = 0;
    let mut accumulator = 0u64;
    for j in 0..12 {
        let mut max_cycle = 0;
        let min_index = index_left;
        let max_index = bank.len() - 12 + j;
        for i in min_index..=max_index {
            if bank.batteries[i] > max_cycle {
                max_cycle = bank.batteries[i];
                index_left = i + 1;
            }
        }
        accumulator = accumulator * 10 + u64::from(max_cycle);
    }
    println!("Best joltage is {}", accumulator);
    accumulator
}
