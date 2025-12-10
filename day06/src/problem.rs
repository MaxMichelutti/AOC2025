#[derive(Debug)]
pub struct Problem{
    pub row_length: usize,
    pub num_rows: usize,
    pub rows: Vec<Vec<u64>>,
    pub operations: Vec<MathOperation>
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MathOperation{
    Add,
    Multiply
}

impl Problem{
    pub fn new() -> Self{
        Problem{
            row_length: 0,
            num_rows: 0,
            rows: Vec::new(),
            operations: Vec::new()
        }
    }

    pub fn add_row(&mut self, row: Vec<u64>){
        let rlen = row.len();
        self.rows.push(row);
        self.num_rows += 1;
        if self.row_length == 0 {
            self.row_length = rlen;
        } else if self.row_length != rlen {
            println!("Expected row length: {}, got: {}", self.row_length, rlen);
            panic!("Inconsistent row lengths in Problem");
        }
    }

    pub fn add_operation(&mut self, op: MathOperation){
        self.operations.push(op);
    }

    pub fn check_problem(&self){
        if self.operations.len() != self.row_length || self.num_rows == 0 {
            panic!("Number of operations does not match row length in Problem");
        }
    }
}