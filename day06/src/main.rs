pub mod parser;
pub mod parser2;
pub mod part_one;
pub mod part_two;
pub mod problem;
pub mod problem2;

pub const PATH: &str = "input/input.txt";

fn main() {
    // get and parse input
    let mut file = std::fs::File::open(PATH).expect("Failed to open input file");
    let mut parser = parser::Parser::new();
    let problem = parser.parse(&mut file).expect("Failed to parse input");

    // part ONE
    let part_one_result = part_one::part_one(&problem);
    println!("Part One Result: {part_one_result}");

    // get and parse input
    let mut file = std::fs::File::open(PATH).expect("Failed to open input file");
    let mut parser2 = parser2::Parser::new();
    let problem2 = parser2.parse(&mut file).expect("Failed to parse input");

    // // part TWO
    let part_two_result = part_two::part_two(&problem2);
    println!("Part Two Result: {part_two_result}");
}
