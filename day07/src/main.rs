pub mod parser;
pub mod part_one;
pub mod part_two;
pub mod problem;

pub const PATH: &str = "input/input.txt";

fn main() {
    // get and parse input
    let mut file = std::fs::File::open(PATH).expect("Failed to open input file");
    let mut parser = parser::Parser::new();
    let problem = parser.parse(&mut file).expect("Failed to parse input");

    // part ONE
    let part_one_result = part_one::part_one(&problem);
    println!("Part One Result: {part_one_result}");

    // // part TWO
    let part_two_result = part_two::part_two(&problem);
    println!("Part Two Result: {part_two_result}");
}
