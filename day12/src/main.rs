pub mod parser;
pub mod part_one;
pub mod part_two;
pub mod problem;

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
    let problem = parser.parse(&mut file).expect("Failed to parse input");
    // println!("Parsed Problem:\n{}", problem);

    // part ONE
    let part_one_result = part_one::part_one(&problem);
    println!("Part One Result: {part_one_result}");

    // // part TWO
    let _part_two_result = part_two::part_two(&problem);
}
