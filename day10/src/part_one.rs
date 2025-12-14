use crate::problem::*;

pub fn part_one(problem: &Problem) -> u64 {
    // Implement the logic for part one here
    let prob = SmartProblem::new(problem);

    let mut total_moves = 0;
    for machine in &prob.machines {
        //println!("Evaluating machine: {:?}", machine);
        let moves = machine.minimum_moves_for_target();
        //println!("Minimum moves for machine: {}", moves);
        total_moves += moves;
    }
    total_moves as u64
}

#[derive(Debug, Clone)]
struct SmartProblem {
    machines: Vec<SmartMachine>,
}

#[derive(Debug, Clone)]
struct SmartMachine {
    lights: u32,
    buttons: Vec<u32>,
    mask: u32,
}

impl SmartMachine {
    fn new(machine: &Machine) -> Self {
        let mut lights = 0u32;
        for (i, light) in machine.light_diagram.lights.iter().enumerate() {
            if *light == LightConfig::On {
                lights |= 1 << i;
            }
        }
        let mut buttons = Vec::new();
        for button in &machine.buttons {
            let mut acc = 0u32;
            for toggle in &button.toggles {
                acc |= 1 << toggle;
            }
            buttons.push(acc);
        }
        let mask = (1 << machine.light_diagram.lights.len()) - 1;
        SmartMachine {
            lights,
            buttons,
            mask,
        }
    }

    // use a bad BFS to find the minimum moves
    fn minimum_moves_for_target(&self) -> u32 {
        if self.lights == 0 {
            return 0;
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0u32, 0u32));
        loop {
            let (moves, config) = queue.pop_front().unwrap();
            for button in &self.buttons {
                // bitwise xor to find next configuration
                let new_config = (config ^ button) & self.mask;
                if new_config == self.lights {
                    return moves + 1;
                }
                queue.push_back((moves + 1, new_config));
            }
        }
    }
}

impl SmartProblem {
    fn new(problem: &Problem) -> Self {
        let mut machines = Vec::new();
        for machine in &problem.machines {
            machines.push(SmartMachine::new(machine));
        }
        SmartProblem { machines }
    }
}
