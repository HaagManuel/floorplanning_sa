

use rand::prelude::*;
use crate::simulated_annealing::*;

// problem: find root of parabola
// toy example to test SA
pub struct Parabola {
    x_current: f64,
    current_cost: f64,
}

#[derive(Debug)]
pub struct ParabolaMove {
    delta: f64,
    delta_cost: f64,
}

impl Parabola {
    pub fn initial_solution() -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let x_current: f64 = rng.gen_range(-10.0..10.0);
        let current_cost: f64 = x_current * x_current;
        Parabola{x_current, current_cost}
    }

    fn eval(&self, x: f64) -> f64 {
        x * x
    }
}

impl SAInstance<ParabolaMove, f64> for Parabola {
    fn get_move(&mut self) -> ParabolaMove {
        let mut rng: ThreadRng = rand::thread_rng();
        let x: f64 = self.x_current;
        let delta: f64 = rng.gen_range(-1.0..1.0);
        let delta_cost: f64 = self.eval(x + delta) - self.eval(x);
        ParabolaMove{delta, delta_cost}
    }
    
    fn apply_move(&mut self, _move: ParabolaMove) {
        self.x_current += _move.delta;
        self.current_cost += _move.delta_cost;
    }
    
    fn current_cost(&self) -> f64 {
        self.current_cost
    }

    fn copy_solution(&self) -> f64 {
        self.x_current
    }
}

impl SAMove for ParabolaMove {
    fn get_delta_cost(&self) -> f64 {
        self.delta_cost
    }
}