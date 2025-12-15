use crate::problem::Problem2;

enum ParserStatus {
    ReadingNode,
    ReadingNeighbours,
}

pub struct Parser{
    status: ParserStatus,
    current_string: String,
    current_node: String,
    current_node_outputs: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        Parser{
            current_node: String::new(),
            current_string: String::new(),
            current_node_outputs: Vec::new(),
            status: ParserStatus::ReadingNode,
        }
    }

    pub fn parse<R: std::io::Read>(&mut self, input: &mut R) -> Result<Problem2, String> {
        let mut problem = Problem2::new();
        let mut buffer = [0; 1];
        while input
            .read(&mut buffer)
            .map_err(|_| "Failed to read input")?
            > 0
        {
            let input_char = buffer[0] as char;
            match input_char {
                'a'..='z' | 'A'..='Z' => {
                    self.current_string.push(input_char);
                }
                ':' => {
                    if let ParserStatus::ReadingNode = self.status {
                        self.current_node = self.current_string.clone();
                        self.current_string.clear();
                        self.status = ParserStatus::ReadingNeighbours;
                    } else {
                        return Err("Unexpected ':' while reading neighbours".to_string());
                    }
                }
                ' ' => {
                    if let ParserStatus::ReadingNeighbours = self.status {
                        if !self.current_string.is_empty() {
                            self.current_node_outputs.push(self.current_string.clone());
                            self.current_string.clear();
                        }
                    } else {
                        return Err("Unexpected space while reading node name".to_string());
                    }
                }
                '\n' => {
                    if let ParserStatus::ReadingNeighbours = self.status {
                        if !self.current_string.is_empty() {
                            self.current_node_outputs.push(self.current_string.clone());
                            self.current_string.clear();
                        }
                        // Add node to problem
                        problem.add_node(&self.current_node, self.current_node_outputs.iter().map(|s| s.as_str()).collect());
                        // Reset for next node
                        self.current_node.clear();
                        self.current_node_outputs.clear();
                        self.status = ParserStatus::ReadingNode;
                    } else {
                        return Err("Unexpected newline while reading node name".to_string());
                    }
                }
                _ => {return Err(format!("Unexpected character: {}", input_char));}
            }
        }
        Ok(problem)
    }
}