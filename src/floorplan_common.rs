
use crate::{definitions::*, hypergraph::Hypergraph};

pub struct  FloorplanInstance {
    pub modules: Vec<Rectangle>,
    pub nets: Vec<Net>,
}

pub trait Mutation<Move> {
    fn get_random_move(&mut self) -> Move;
    fn apply_move(&mut self, _move: &Move);
}

pub trait Crossover<S: Clone> {
    fn crossover(&self, a: &S, b: &S) -> S;
}

pub trait FloorCost {
    fn get_floor_wire(&self) -> f64;
    fn get_floor_area(&self) -> f64;
}

pub trait FloorPlan {
    fn get_floorplan(&self) -> Floorplan;
}

pub trait Cost {
    fn get_cost(&self) -> f64;
}
pub trait RandomSolution<T: Clone> {
    fn random_solution(&self) -> T;
}
pub trait Solution<T: Clone> {
    fn copy_solution(&self) -> T;
    fn set_solution(&mut self, solution: T);
}

#[derive(Debug)]
pub struct CostFunction {
    pub alpha: f64,
    avg_wirelength: f64,
    avg_area: f64,
}

impl Default for CostFunction {
    fn default() -> Self {
        Self { alpha: 1.0, avg_wirelength: 1.0, avg_area: 1.0 }
    }
}

impl CostFunction {
    pub fn new(alpha: f64, avg_wirelength: f64, avg_area: f64) -> Self {
        CostFunction { alpha, avg_wirelength, avg_area }
    }

    /// computes the cost of a floorplan
    pub fn get_cost(&self, area: f64, wirelength: f64) -> f64 {
        let area_cost = area / self.avg_area;
        let wire_cost = wirelength / self.avg_wirelength;
        let cost = area_cost * self.alpha + wire_cost * (1.0 - self.alpha);
        cost
    }
    
    /// computes the total used wirelength using half-perimenter estimation
    pub fn compute_wirelength(plan: &Floorplan, nets: &Vec<Net>) -> f64 {
        let mut total_wirelength: f64 = 0.0;
        for net in nets.iter() {
            let mut bounding_box = BoundingBox::new(f64::MAX, -f64::MAX, f64::MAX, -f64::MAX);
            for id in net.pins.iter() {
                let (pos_x, pos_y, rect) = plan[*id];
                let (center_x, center_y) = rect.center(pos_x, pos_y);
                bounding_box.extend_point(center_x, center_y);
            }
            // half-perimeter estimation
            total_wirelength += bounding_box.get_width() + bounding_box.get_height();
        }
        total_wirelength
    }
    /// estimates avg-area and -wirelength by perturbation for the cost function
    pub fn compute_mean_parameters<T: Mutation<Move> + FloorCost, Move>(algo: &mut T, repetitions: usize) -> (f64, f64) {
        let mut sum_area = 0.0;
        let mut sum_wirelength = 0.0;
        for _ in 0..repetitions {
            let _move: Move = algo.get_random_move();
            algo.apply_move(&_move);
            let wire = algo.get_floor_wire();
            let area = algo.get_floor_area();
            sum_area += area;
            sum_wirelength += wire;
            algo.apply_move(&_move);
            
        }
        if sum_wirelength == 0.0 {
            sum_wirelength = repetitions as f64;
        }
        if sum_area == 0.0 {
            sum_area = repetitions as f64
        }
        (sum_area / repetitions as f64, sum_wirelength / repetitions as f64)
    }

    /// returns percentage of area not covered by boxes
    pub fn get_dead_area<T: FloorCost>(floor: &T, modules: &Vec<Rectangle>) -> f64{
        let occupied_area: usize = modules.iter()
            .map(|rect| rect.area())
            .sum();
        let total_area = floor.get_floor_area();
        let dead_area = 1.0 - (occupied_area as f64 / total_area as f64);
        dead_area * 100.0
    }
}

/// greedy method to generate a linear ordering of the modules reducing wirelength
/// reduced wirelength a bit, but increase area in experiments
#[allow(dead_code)]
pub fn cluster_growing_order(graph: &Hypergraph, start_node: Int) -> Vec<Int> {
    let mut order: Vec<Int> = vec![start_node];
    let mut placed_nodes: Vec<bool> = vec![false; graph.num_nodes];
    placed_nodes[start_node] = true;

    // place node with highest gain
    // gain = terminating nets - new nets
    for _ in 0..graph.num_nodes - 1 {
        let mut best_gain: i32 = -i32::MAX;
        let mut best_node: Int = 0;
        for v in 0..graph.num_nodes {
            if !placed_nodes[v] {
                let mut terminating_nets = 0; 
                let mut new_nets = 0; 
                for net in graph.out_nets[v].iter() {
                    terminating_nets += net.pins.iter().all(|w| placed_nodes[*w]) as i32;
                    new_nets += net.pins.iter().all(|w| !placed_nodes[*w]) as i32;
                }
                let gain = terminating_nets - new_nets;
                if gain > best_gain {
                    best_gain = gain;
                    best_node = v;
                }
            }
        }
        placed_nodes[best_node] = true;
        order.push(best_node);
    }
    order
}

/// reorders a vector according to a given permutation
#[allow(dead_code)]
pub fn reorder_vec<T: Clone>(permutation: &Vec<usize>, vec: &Vec<T>) -> Vec<T> {
    let mut new_vec: Vec<T> = Vec::new();
    new_vec.reserve_exact(vec.len());
    for i in permutation {
        new_vec.push(vec[permutation[*i]].clone());
    }
    new_vec
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_growth() {
        let nets = vec![
            Net{pins: vec![0, 1],    id: 0},
            Net{pins: vec![0, 3],    id: 1},
            Net{pins: vec![0, 2, 4], id: 2},
            Net{pins: vec![1, 3],    id: 3},
            Net{pins: vec![2, 3, 4], id: 4},
            Net{pins: vec![3, 4],    id: 5},
        ];
        let graph = Hypergraph::from(nets);
        let order = cluster_growing_order(&graph, 0);
        assert_eq!(order, vec![0, 1, 3, 4, 2]);
    }
}