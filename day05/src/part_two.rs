use crate::problem::DatabaseMultiRange;
use crate::problem::Problem;

pub fn part_two(problem: &Problem) -> u64 {
    let mut multi_range = DatabaseMultiRange::new(problem.database_ranges[0]);
    for range in problem.database_ranges.iter() {
        // uncomment lines to debug the merging process
        // println!("Range: {:?}", range);
        multi_range.merge(*range);
        // println!("MR: {}", multi_range);
        // let mut it = String::new();
        // let _ = std::io::stdin().read_line(&mut it).expect("Failed to read input");
    }
    let mut optional_range = Some(Box::new(multi_range));
    let mut size_accumulator = 0u64;
    while let Some(mr) = optional_range {
        size_accumulator += mr.range.range_size();
        optional_range = mr.next;
    }
    size_accumulator
}
