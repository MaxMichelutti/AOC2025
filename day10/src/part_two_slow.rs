use crate::problem::*;

pub fn part_two(problem: &Problem) -> u64 {
    // Implement the logic for part two here
    let prob = SmartProblem::new(problem);
    let mut total_moves = 0;
    for machine in &prob.machines{
        print!("Machine: {:?} ", machine);
        let moves = machine.minimum_moves_for_target();
        println!("=> Minimum Moves: {}", moves);
        total_moves += moves;
    }
    total_moves
}

#[derive(Debug, Clone)]
struct SmartProblem{
    machines: Vec<SmartMachine>,
}

#[derive(Debug, Clone)]
struct SmartMachine{
    target: Configuration,
    buttons: Vec<u32>
}

impl SmartMachine{
    fn new(machine: &Machine) -> Self{
        let mut target = Vec::new();
        target = machine.joltages.iter().map(|&j| j as u32).collect();
        let mut buttons = Vec::new();
        for button in &machine.buttons{
            let mut acc = 0u32;
            for toggle in &button.toggles{
                acc |= 1 << toggle;
            }
            buttons.push(acc);
        }
        SmartMachine{
            target: Configuration{joltages: target},
            buttons
        }
    }

    // use a bad BFS to find the minimum moves
    fn minimum_moves_for_target(&self) -> u64 {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0u64, Configuration::new(self.target.joltages.len())));
        let mut configs_set = std::collections::HashSet::new();
        loop {
            let (moves, config) = queue.pop_front().unwrap();
            for button in &self.buttons {
                let mut new_config = config.clone();
                new_config.apply_button(*button);
                if new_config.is_over_target(&self.target){
                    continue;
                }
                if new_config.is_target(&self.target) {
                    return moves + 1;
                }
                if configs_set.contains(&new_config.joltages) {
                    continue;
                }
                configs_set.insert(new_config.joltages.clone());
                queue.push_back((moves + 1, new_config));
            }
        }
    }
}

impl SmartProblem{
    fn new(problem: &Problem) -> Self{
        let mut machines = Vec::new();
        for machine in &problem.machines{
            machines.push(SmartMachine::new(machine));
        }
        SmartProblem{
            machines
        }
    }
}

#[derive(Debug, Clone)]
pub struct Configuration{
    pub joltages: Vec<u32>,
}

impl Configuration{
    pub fn new(size: usize) -> Self{
        Configuration{
            joltages: vec![0u32; size],
        }
    }

    pub fn apply_button(&mut self, button: u32) {
        for (i , joltage) in self.joltages.iter_mut().enumerate(){
            // read bit at corresponding index
            if (button & (1 << i)) != 0{
                *joltage += 1;
            }
        }
    }

    pub fn is_target(&self, target: &Configuration) -> bool{
        &self.joltages == &target.joltages
    }

    pub fn is_over_target(&self, target: &Configuration) -> bool{
        for (a, b) in self.joltages.iter().zip(target.joltages.iter()){
            if a > b {
                return true;
            }
        }
        false
    }
}