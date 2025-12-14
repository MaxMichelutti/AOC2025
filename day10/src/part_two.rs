use std::collections::HashSet;

use crate::problem::*;

pub fn part_two(problem: &Problem) -> u64 {
    // Implement the logic for part two here
    let mut mp = MatrixProblem::new(problem);
    let mut total_moves = 0;
    for (_i, x) in mp.machines.iter_mut().enumerate() {
        // println!("Machine: \n{}", x);
        let free = x.gauss_elimination();
        // println!("After Gauss Elimination: \n{}", x);
        // println!("{i}. Free variables at columns: {:?}", free);
        if free.is_empty() {
            let lcs = x.add_last_column();
            //println!("Moves for machine {}: {}", i+1, lcs as u64);
            total_moves += lcs.round() as u64;
        } else {
            total_moves += x.search_best(&free);
        }
    }
    total_moves
}

#[derive(Debug, Clone)]
struct MatrixProblem {
    pub machines: Vec<MatrixMachine>,
}

#[derive(Debug, Clone)]
struct MatrixMachine {
    pub ab: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
}

impl MatrixProblem {
    pub fn new(problem: &Problem) -> Self {
        let mut machines = Vec::new();
        for machine in &problem.machines {
            machines.push(MatrixMachine::new(machine));
        }
        MatrixProblem { machines }
    }
}

impl MatrixMachine {
    pub fn new(machine: &Machine) -> Self {
        let mut ab = Vec::new();
        let rows = machine.joltages.len();
        let cols = machine.buttons.len();
        for _ in 0..rows {
            ab.push(vec![0f64; cols + 1]);
        }
        for (col, button) in machine.buttons.iter().enumerate() {
            for &toggle in &button.toggles {
                ab[toggle][col] = 1f64;
            }
        }
        for (index, joltage) in machine.joltages.iter().enumerate() {
            ab[index][cols] = *joltage as f64;
        }
        Self {
            ab,
            rows,
            cols: cols + 1,
        }
    }

    pub fn swap_first_non_zero_row(&mut self, start_row: usize, column: usize) -> bool {
        for r in start_row..self.rows {
            if self.ab[r][column] != 0f64 {
                self.ab.swap(r, start_row);
                return true;
            }
        }
        false
    }

    pub fn gauss_elimination(&mut self) -> HashSet<usize> {
        // iterate over diagonal
        let mut col_shift = 0;
        let mut free = HashSet::new();
        for diag in 0..std::cmp::min(self.rows, self.cols - 1) {
            if diag + col_shift >= self.cols - 1 {
                return free;
            }
            while self.ab[diag][diag + col_shift] == 0f64 {
                self.swap_first_non_zero_row(diag, diag + col_shift);
                if self.ab[diag][diag + col_shift] == 0f64 {
                    free.insert(diag + col_shift);
                    //println!("Free variable at column {} detected.", diag + col_shift);
                    col_shift += 1;
                    if diag + col_shift >= self.cols - 1 {
                        return free;
                    }
                } else {
                    break;
                }
            }
            self.normalize_row(diag, diag + col_shift);
            for row in 0..self.rows {
                if row == diag {
                    continue;
                }
                if self.ab[row][diag + col_shift] == 0f64 {
                    continue;
                }
                let factor = -self.ab[row][diag + col_shift];
                let modifier = self.row_multiplied_by(diag, factor);
                self.add_row(row, &modifier);
            }
        }
        if self.cols > self.rows {
            for col in self.rows + col_shift..self.cols - 1 {
                //println!("Extra free variable at column {} detected.", col);
                free.insert(col);
            }
        }
        free
    }

    pub fn normalize_row(&mut self, row: usize, col: usize) {
        let pivot = self.ab[row][col];
        for col in 0..self.cols {
            self.ab[row][col] /= pivot;
        }
    }

    pub fn row_multiplied_by(&mut self, row: usize, factor: f64) -> Vec<f64> {
        let mut row = self.ab[row].clone();
        for col in 0..self.cols {
            row[col] *= factor;
        }
        row
    }

    pub fn add_row(&mut self, target_row: usize, row_to_add: &[f64]) {
        for col in 0..self.cols {
            self.ab[target_row][col] += row_to_add[col];
        }
    }

    pub fn add_last_column(&mut self) -> f64 {
        let mut sum = 0f64;
        for row in 0..self.rows {
            sum += self.ab[row][self.cols - 1];
        }
        sum
    }

    pub fn row_rank(&self, row: usize) -> usize {
        for col in 0..self.cols {
            if self.ab[row][col] != 0f64 {
                return col;
            }
        }
        self.cols
    }

    pub fn search_best(&self, free: &HashSet<usize>) -> u64 {
        // implement later
        let mut current_config = Configuration::new(free.len());
        let mut best_evaluation = u64::MAX;
        while let Some(config) = current_config.next() {
            current_config = config;
            // evaluate config
            let mut evaluation: u64 = current_config.inner.iter().map(|&x| x as u64).sum();
            // let mut solution = vec![0u64; self.cols -1];
            // for (free_index, free_variable) in free.iter().enumerate() {
            //     solution[*free_variable] = current_config.inner[free_index] as u64;
            // }
            let mut ignore_solution = false;
            for row in 0..self.rows {
                // skip empty rows
                if self.row_rank(row) >= self.cols - 1 {
                    continue;
                }
                let mut row_value = self.ab[row][self.cols - 1];
                for (free_index, free_variable) in free.iter().enumerate() {
                    row_value -=
                        self.ab[row][*free_variable] * (current_config.inner[free_index] as f64);
                }
                row_value += 0.0001; // to avoid floating point issues
                if row_value.fract() > 0.0002 || row_value < 0f64 {
                    ignore_solution = true;
                    break;
                } else {
                    //solution[self.row_rank(row)] = row_value.round() as u64;
                    evaluation += row_value.round() as u64;
                }
            }
            if ignore_solution {
                continue;
            }
            if evaluation < best_evaluation {
                // println!("New best solution found with evaluation {}: {:?}", evaluation, solution);
                best_evaluation = evaluation;
            }
        }
        best_evaluation
    }
}

impl std::fmt::Display for MatrixMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.ab {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

struct Configuration {
    pub inner: Vec<u32>,
    yielded_zero: bool,
}

impl Configuration {
    pub fn new(size: usize) -> Self {
        Configuration {
            inner: vec![0u32; size],
            yielded_zero: false,
        }
    }

    pub fn next(self) -> Option<Self> {
        let mut new_config = self.inner.clone();
        if !self.yielded_zero {
            return Some(Configuration {
                inner: new_config,
                yielded_zero: true,
            });
        }
        for i in 0..new_config.len() {
            if new_config[i] < 200 {
                new_config[i] += 1;
                return Some(Configuration {
                    inner: new_config,
                    yielded_zero: true,
                });
            } else {
                new_config[i] = 0;
            }
        }
        None
    }
}
