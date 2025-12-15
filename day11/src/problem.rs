use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Problem {
    pub id_mappings: HashMap<String, u64>,
    pub nodes: HashMap<u64, Node>,
    pub you_id: u64,
    pub out_id: u64,
    pub svr_id: u64,
    pub dac_id: u64,
    pub fft_id: u64,
    serial: u64,
}

impl Problem {
    pub fn new() -> Self {
        Problem {
            nodes: HashMap::new(),
            id_mappings: HashMap::new(),
            you_id: 0,
            out_id: 0,
            serial: 1,
            svr_id: 0,
            dac_id: 0,
            fft_id: 0,
        }
    }

    pub fn add_node(&mut self, name: &str, neighbours: Vec<&str>) -> u64 {
        let node_id = if let Some(id) = self.id_mappings.get(name) {
            *id
        } else {
            let id = self.serial;
            match name {
                "you" => self.you_id = id,
                "out" => self.out_id = id,
                "svr" => self.svr_id = id,
                "dac" => self.dac_id = id,
                "fft" => self.fft_id = id,
                _ => {}
            }
            self.id_mappings.insert(name.to_string(), id);
            self.nodes.insert(id, Node::new(id, name));
            self.serial += 1;
            id
        };
        for neighbour_name in neighbours {
            let neighbour_id = self.add_node(neighbour_name, vec![]);
            let node_ref = self.nodes.get_mut(&node_id).unwrap();
            node_ref.add_neighbour(neighbour_id);
        }
        node_id
    }

    /// reverse the direction of all edges in the problem
    pub fn reverse(&self) -> Self {
        let mut res = Problem::new();
        res.you_id = self.out_id;
        res.out_id = self.you_id;
        res.svr_id = self.svr_id;
        res.dac_id = self.dac_id;
        res.fft_id = self.fft_id;
        res.id_mappings = self.id_mappings.clone();
        for (id, node) in self.id_mappings.iter() {
            res.nodes.insert(*node, Node::new(*node, id));
        }
        for (id, node) in self.nodes.iter() {
            for &neighbour_id in &node.neighbours {
                res.nodes.get_mut(&neighbour_id).unwrap().add_neighbour(*id);
            }
        }
        res
    }
}

impl Default for Problem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u64,
    pub name: String,
    pub neighbours: Vec<u64>,
}

impl Node {
    pub fn new(id: u64, name: &str) -> Self {
        Node {
            neighbours: Vec::new(),
            name: name.to_string(),
            id,
        }
    }

    pub fn add_neighbour(&mut self, neighbour_id: u64) {
        self.neighbours.push(neighbour_id);
    }
}
