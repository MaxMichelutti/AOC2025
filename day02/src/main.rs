pub mod parser;
pub mod part_one;
pub mod part_two;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: cargo run -- <input_file_path>");
        std::process::exit(1);
    }
    let path = &args[0];

    // get and parse input
    let mut file = std::fs::File::open(path).expect("Failed to open input file");
    let mut parser = parser::Parser::new();
    let ranges = parser.parse(&mut file).expect("Failed to parse input");

    let part_one_result = part_one::part_one(&ranges);
    println!("Part one result: {}", part_one_result);

    let part_two_result = part_two::part_two(&ranges);
    println!("Part two result: {}", part_two_result);
}
