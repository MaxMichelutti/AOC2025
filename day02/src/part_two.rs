use crate::parser::MyRange;

pub fn part_two(ranges: &[MyRange]) -> u64 {
    let highest_end = ranges.iter().map(|r| r.end).max().unwrap_or(0);
    let proofs = generate_proofs(highest_end);
    let mut sum = 0;
    // a bit brute-force, but numbers like 2222 being both 22 - 22 and 2 - 2 - 2 - 2
    // where giving me trouble with part one approach
    for range in ranges.iter() {
        for i in range.start..=range.end {
            for proof in proofs.iter() {
                if i % proof.divider == 0 {
                    let i_divided = i / proof.divider;
                    if i_divided >= proof.start && i_divided <= proof.end {
                        sum += i;
                        break;
                    }
                }
            }
        }
    }
    sum
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
struct Proof {
    pub divider: u64,
    pub start: u64,
    pub end: u64,
}

fn generate_proofs(max_value: u64) -> Vec<Proof> {
    let mut proofs = vec![];
    for repetitions in 2..=10 {
        let mut repetition_power = 1;
        while let Some(divider) = build_divider(repetitions, repetition_power) {
            let start = 10u64.strict_pow(repetition_power - 1);
            let end = 10u64.strict_pow(repetition_power) - 1;
            if divider * start > max_value {
                break;
            }
            proofs.push(Proof {
                divider,
                start,
                end,
            });
            repetition_power += 1;
        }
    }
    proofs
}

// I am dealing with very large number, so I check for overflow when doing powers of 10
fn build_divider(repetitions: u64, power: u32) -> Option<u64> {
    let mut max_exponent = (repetitions as u32 - 1) * power;
    let mut divider = 0u64;
    loop {
        divider += 10u64.checked_pow(max_exponent)?;
        if power > max_exponent {
            break;
        }
        max_exponent -= power;
    }
    Some(divider)
}
