use std::sync::{Arc, Mutex};

pub struct Backend {
    pub address: String,
    pub weight: usize,
}

pub struct WeightedRoundRobin {
    backends: Vec<Backend>,
    current_index: usize,
    current_weight: usize,
}

impl WeightedRoundRobin {
    pub fn new(backends: Vec<Backend>) -> Self {
        WeightedRoundRobin {
            backends,
            current_index: 0,
            current_weight: 0,
        }
    }

    pub fn get_next_backend(&mut self) -> &Backend {
        loop {
            self.current_index = (self.current_index + 1) % self.backends.len();
            if self.current_index == 0 {
                self.current_weight = self.current_weight.saturating_sub(1);
                if self.current_weight == 0 {
                    self.current_weight = self.backends.iter().map(|b| b.weight).max().unwrap();
                }
            }

            if self.backends[self.current_index].weight >= self.current_weight {
                return &self.backends[self.current_index];
            }
        }
    }
}

pub type SharedRoundRobin = Arc<Mutex<WeightedRoundRobin>>;
