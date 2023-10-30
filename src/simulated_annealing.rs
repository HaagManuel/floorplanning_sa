use std::fmt::Debug;

use rand::prelude::*;

const TEMPERATURE_THRESHOLD: f64 = 0.01;

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

    // estimates initial temperature by: T = -Delta_avg / log p
    // average positive costs
    // p is probability that an inital move is accepted
    pub fn estimate_initial_temperature<T: SAInstance<Move, Solution>, Move: SAMove + Debug, Solution: Debug>(initial_prob: f64, num_moves: usize, instance: &mut T) -> f64 {
        let mut sum = 0.0;
        for _ in 0..num_moves {
            let _move = instance.get_move();
            sum += _move.get_delta_cost().abs(); 
        }
        let delta_avg = sum / num_moves as f64;
        let e: f64 = 1.0_f64.exp();
        let temperature = -delta_avg / initial_prob.log(e);
        temperature
    } 

    // T * alpha^n < x --> alpha < (x / T)^(1/n)
    pub fn get_decay_for_n_iterations(iterations: u64, initial_temperature: f64) -> f64 {
        (TEMPERATURE_THRESHOLD / initial_temperature).powf(1.0 / iterations as f64)
    }

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
            if temperature < TEMPERATURE_THRESHOLD {
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
            if i % (self.iterations / 10) == 0 {
                println!("it: {}, T {:.2}, cost {:.2}, delta {:.2}, best {:.2}", i, temperature, cost, delta, best_cost);
            }
            temperature *= self.decay;
        }
        println!("best cost {:?}", best_cost);
        println!("best solution {:?}", best_solution);
    }
}