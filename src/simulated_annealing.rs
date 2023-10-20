use std::fmt::Debug;

use rand::prelude::*;

pub trait SAMove {
    fn get_delta_cost(&self) -> f64;
}

// minimizes objective function
pub trait SAInstance<Move: SAMove, Solution> {
    fn get_move(&mut self) -> Move;
    fn apply_move(&mut self, _move: Move);
    fn current_cost(&self) -> f64;
    fn copy_solution(&self) -> Solution;
}

pub struct SimulatedAnnealing {
    iterations: u64,
    initial_temperature: f64,
    decay: f64,
}

impl SimulatedAnnealing {

    pub fn new(iterations: u64, initial_temperature: f64, decay: f64) -> SimulatedAnnealing {
        SimulatedAnnealing{
            iterations: iterations,
            initial_temperature: initial_temperature,
            decay,
        }
    }

    pub fn run<T: SAInstance<Move, Solution>, Move: SAMove + Debug, Solution: Debug>(&self, instance: &mut T) {
        let mut temperature: f64 = self.initial_temperature;
        let mut rng: ThreadRng = rand::thread_rng();
        let mut best_cost: f64 = instance.current_cost();
        let mut best_solution: Solution = instance.copy_solution();
        for i in 0..self.iterations {
            if temperature < 0.01 {
                break;
            }
            let _move: Move = instance.get_move();
            let delta: f64 = _move.get_delta_cost();
            if delta <= 0.0 {
                instance.apply_move(_move);
            }
            else {
                // TODO try Uniform, maybe faster
                let r: f64 = rng.gen();
                if r < (-delta / temperature).exp() {
                    instance.apply_move(_move);
                }   
            }

            let cost: f64 = instance.current_cost();
            if cost < best_cost {
                best_cost = cost;
                best_solution = instance.copy_solution();
            }
            if i % 50 == 0 {
                println!("it: {}, T {:.2}, cost {:.2}, delta {:.2}, best {:.2}", i, temperature, cost, delta, best_cost);
            }
            temperature *= self.decay;
        }
        println!("best cost {:?}", best_cost);
        println!("best solution {:?}", best_solution);
    }
}