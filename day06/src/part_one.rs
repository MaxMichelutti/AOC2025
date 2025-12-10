use crate::problem::Problem;
use crate::problem::MathOperation;

pub fn part_one(problem: &Problem) -> u64 {
    let mut result = 0;
    for i in 0..problem.row_length {
        let operation = problem.operations[i];
        let mut column = Column::new(operation);
        for j in 0..problem.num_rows {
            let element = problem.rows[j][i];
            column.add_element(element);
        }
        result += column.compute_result();
    }
    result
}

#[derive(Debug)]
pub struct Column{
    pub operation: MathOperation,
    pub elements: Vec<u64>
}

impl Column{
    pub fn new(op: MathOperation) -> Self{
        Column{
            operation: op,
            elements: Vec::new()
        }
    }

    pub fn add_element(&mut self, element: u64){
        self.elements.push(element);
    }

    pub fn compute_result(&self) -> u64{
        match self.operation{
            MathOperation::Add => self.elements.iter().sum(),
            MathOperation::Multiply => self.elements.iter().product()
        }
    }
}