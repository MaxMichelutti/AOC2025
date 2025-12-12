use crate::problem::Problem;
pub struct Parser{

}

impl Parser {
    pub fn new() -> Self {
        Parser{}
    }

    pub fn parse<R: std::io::Read>(&mut self, input: &mut R) -> Result<Problem, String> {
        let mut problem = Problem::new();
        let mut buffer = [0; 1];
            while input
                .read(&mut buffer)
                .map_err(|_| "Failed to read input")?
                > 0
            {
                let input_char = buffer[0] as char;
                match input_char {
                    _ => {return Err(format!("Unexpected character: {}", input_char));}
                }
            }
        Ok(problem)
    }
}