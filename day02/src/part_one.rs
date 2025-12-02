use crate::parser::MyRange;

pub fn part_one(ranges: &[MyRange]) -> u64 {
    let highest_end = ranges.iter().map(|r| r.end).max().unwrap_or(0);
    let proofs = generate_proofs(highest_end);
    let mut sum = 0;
    for range in ranges.iter() {
        for proof in proofs.iter() {
            let partial_sum = proof.get_sum_of_items_in_range(range);
            sum += partial_sum;
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

impl Proof {
    pub fn get_sum_of_items_in_range(&self, range: &MyRange) -> u64 {
        let mut x = range.start / self.divider + 1;
        if range.start.is_multiple_of(self.divider) {
            // we have to account for exact division
            x -= 1;
        }
        let range_start = std::cmp::max(x, self.start);
        let range_end = std::cmp::min(range.end / self.divider, self.end);
        // no intersection
        if range_start > self.end || range_end < self.start {
            return 0;
        }
        let mut sum = 0;
        for index in range_start..=range_end {
            sum += index * self.divider;
        }
        sum
    }
}

fn generate_proofs(max_value: u64) -> Vec<Proof> {
    let mut proofs = vec![];
    let mut index = 1;
    loop {
        let divider = 10u64.strict_pow(index) + 1;
        let start = 10u64.strict_pow(index - 1);
        let end = 10u64.strict_pow(index) - 1;
        if divider * start > max_value {
            break;
        }
        proofs.push(Proof {
            divider,
            start,
            end,
        });
        index += 1;
    }
    proofs
}
