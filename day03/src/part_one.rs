use crate::parser::Bank;

pub fn part_one(banks: &[Bank]) -> u64 {
    banks.iter().map(|bank| best_joltage(bank) as u64).sum()
}

fn best_joltage(bank: &Bank) -> u8 {
    let mut index_left = 0;
    let mut max_decina = 0;
    let mut max_unita = 0;
    for i in 0..bank.len() - 1 {
        if bank.batteries[i] > max_decina {
            max_decina = bank.batteries[i];
            index_left = i;
        }
    }
    for j in index_left + 1..=bank.len() - 1 {
        if bank.batteries[j] > max_unita {
            max_unita = bank.batteries[j];
        }
    }
    let res = max_decina * 10 + max_unita;
    println!("Best joltage is {}", res);
    res
}
