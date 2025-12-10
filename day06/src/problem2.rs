use crate::problem::MathOperation;

#[derive(Debug)]
pub struct Problem2{
    pub data: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
    pub operations: Vec<MathOperation>
}

impl Problem2{
    pub fn new() -> Self{
        Problem2{
            data: Vec::new(),
            width: 0,
            height: 0,
            operations: Vec::new()
        }
    }

    pub fn add_row(&mut self, row: Vec<u8>){
        let rlen = row.len();
        self.data.push(row);
        self.height += 1;
        if self.width == 0 {
            self.width = rlen;
        } else if self.width != rlen {
            println!("Expected row length: {}, got: {}", self.width, rlen);
            panic!("Inconsistent row lengths in Problem2");
        }
    }

    pub fn add_operation(&mut self, op: MathOperation){
        self.operations.push(op);
    }
}