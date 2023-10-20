use rand::prelude::*;
use crate::simulated_annealing::*;

// knapsack problem with penalty for violating constraint
pub struct Knapsack {
    weights: Vec<f64>,
    gains: Vec<f64>,
    penalty: f64,
    max_weight: f64,
    selected_items: Vec<bool>,
    current_cost: f64,
    current_weight: f64,
}
#[derive(Debug)]
pub struct KnapsackMove {
    flip_item: usize,
    delta_cost: f64,
    delta_weight: f64,
}

impl Knapsack {
    pub fn initial_solution(weights: Vec<f64>, gains: Vec<f64>, penalty: f64, max_weight: f64) -> Self {
        // initially select no items
        // if too many items are selected it might be hard to get the constraint satisfied again
        let selected_items: Vec<bool> = vec![false; weights.len()];
        let current_cost = 0.0;
        let current_weight = 0.0;
        Knapsack{weights, gains, penalty, max_weight, selected_items, current_cost, current_weight}
    }
}

impl SAInstance<KnapsackMove, Vec<bool>> for Knapsack {
    
    fn get_move(&mut self) -> KnapsackMove {
        let mut rng: ThreadRng = rand::thread_rng();
        // index to flip
        let flip_item: usize = rng.gen_range(0..self.weights.len());
        let sign: f64 = if self.selected_items[flip_item] {-1.0} else {1.0};
        let delta_gain: f64 = self.gains[flip_item] * sign;
        let delta_weight: f64 = self.weights[flip_item] * sign;
        let penalty: f64 = if self.current_weight + delta_weight > self.max_weight {self.penalty} else {0.0};
        let delta_cost = -delta_gain + penalty;
        KnapsackMove{flip_item, delta_cost, delta_weight}
    }
    
    fn apply_move(&mut self, _move: KnapsackMove) {
        self.selected_items[_move.flip_item] ^= true;
        self.current_cost += _move.delta_cost;
        self.current_weight += _move.delta_weight;
    }
    
    fn current_cost(&self) -> f64 {
        self.current_cost
    }
    
    fn copy_solution(&self) -> Vec<bool> {
        self.selected_items.clone()
    }
}

impl SAMove for KnapsackMove {
    fn get_delta_cost(&self) -> f64 {
        self.delta_cost
    }
}