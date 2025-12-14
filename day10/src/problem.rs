#[derive(Debug, Clone)]
pub struct Problem {
    pub machines: Vec<Machine>,
}

impl Problem {
    pub fn new() -> Self {
        Problem {
            machines: Vec::new(),
        }
    }

    pub fn add_machine(&mut self, machine: Machine) {
        self.machines.push(machine);
    }
}

impl Default for Problem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    pub light_diagram: LightDiagram,
    pub buttons: Vec<Button>,
    pub joltages: Vec<usize>,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            light_diagram: LightDiagram { lights: Vec::new() },
            buttons: Vec::new(),
            joltages: Vec::new(),
        }
    }

    pub fn add_light(&mut self, config: LightConfig) {
        self.light_diagram.lights.push(config);
    }

    pub fn add_button(&mut self, button: Button) {
        self.buttons.push(button);
    }

    pub fn add_to_last_button(&mut self, toggle: usize) {
        if let Some(last_button) = self.buttons.last_mut() {
            last_button.toggles.push(toggle);
        }
    }

    pub fn add_joltage(&mut self, joltage: usize) {
        self.joltages.push(joltage);
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum LightConfig {
    On,
    Off,
}

#[derive(Debug, Clone)]
pub struct LightDiagram {
    pub lights: Vec<LightConfig>,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub toggles: Vec<usize>,
}
