use std::fmt::Debug;
use rand::prelude::*;

use crate::floorplan_common::*;

// stop if T < T_init * threshold
const TEMPERATURE_THRESHOLD: f64 = 0.000_001;

pub struct SimulatedAnnealingConfig {
    pub iterations: usize,
    pub num_moves_estimation: usize,
    pub inital_prob: f64,
}

pub struct SimulatedAnnealing {
    iterations: usize,
    initial_temperature: f64,
    decay: f64,
}

impl SimulatedAnnealing {

    // estimates initial temperature by: T = -Delta_avg / log p
    // average positive costs
    // p is probability that an inital move is accepted
    pub fn estimate_initial_temperature<T: Mutation<Move> + Cost, Move>(initial_prob: f64, num_moves: usize, instance: &mut T) -> f64 {
        let mut sum: f64 = 0.0;
        let cost = instance.get_cost();
        for _ in 0..num_moves {
            let _move = instance.get_random_move();
            instance.apply_move(&_move);
            let new_cost = instance.get_cost();
            let delta = new_cost - cost;
            sum += delta.abs(); 
            instance.apply_move(&_move);
        }
        let delta_avg = sum / num_moves as f64;
        let e: f64 = 1.0_f64.exp();
        let temperature = -delta_avg / initial_prob.log(e);
        temperature
    } 

    // T * alpha^n < x --> alpha < (x / T)^(1/n)
    pub fn get_decay_for_n_iterations(iterations: usize, initial_temperature: f64) -> f64 {
        let temperature_stop = initial_temperature * TEMPERATURE_THRESHOLD;
        (temperature_stop / initial_temperature).powf(1.0 / iterations as f64)
    }

    pub fn new(iterations: usize, initial_temperature: f64, decay: f64) -> SimulatedAnnealing {
        SimulatedAnnealing{
            iterations: iterations,
            initial_temperature: initial_temperature,
            decay,
        }
    }

    pub fn run<T: Mutation<Move> + Cost + Solution<S>, Move, S: Clone + Debug>(&self, instance: &mut T) {
        let mut temperature: f64 = self.initial_temperature;
        let mut rng: ThreadRng = rand::thread_rng();
        let rng_vector: Vec<f64> = (0..self.iterations).map(|_| rng.gen::<f64>()).collect(); // ~ 10% faster

        let mut best_cost: f64 = instance.get_cost();
        let mut best_solution: S = instance.copy_solution();
        let mut current_cost: f64 = best_cost;
        for i in 0..self.iterations {
            let _move: Move = instance.get_random_move();
            instance.apply_move(&_move);
            let new_cost = instance.get_cost();
            let delta: f64 = new_cost - current_cost;
            
            if delta <= 0.0 || rng_vector[i] < (-delta / temperature).exp() {
                // keep state
                current_cost = new_cost;
            }
            else {
                // reverse move
                instance.apply_move(&_move);
            }

            if new_cost < best_cost {
                best_cost = new_cost;
                best_solution = instance.copy_solution();
            }
            if i % (self.iterations / 10) == 0 {
                eprintln!("it: {}, T {:.2}, cost {:.2}, delta {:.2}, best {:.2}", i, temperature, current_cost, delta, best_cost);
            }
            temperature *= self.decay;
        }
        eprintln!("best cost {:?}", best_cost);
        instance.set_solution(best_solution);
    }

}