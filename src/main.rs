mod simulated_annealing;
mod parabola;
mod knapsack;
mod polish_expression;

use crate::simulated_annealing::SimulatedAnnealing;
// use crate::parabola::Parabola;
// use crate::knapsack::Knapsack;
use crate::polish_expression::*;

/*
benchmark sets
 https://s2.smu.edu/~manikas/Benchmarks/MCNC_Benchmark_Netlists.html

 */

/* TODO
- more efficient way to generate random numbers
- maybe tsp? 
- restarts of SA
- maybe wheels?
- shape functions: hard, soft, continous -> first hard, need to be combinable
- implement binary tree
- implement polish expression to check equivalence
- analytical approach for comparison
- state of the art method online?

1. naive polish expressions
*/

fn main() {
    println!("Hello, SA!");

    // high temperature random, low greedy

    // log r < delta / T

    // setter?
    // init of temperature with avg
    let iterations = 10000;
    let initial_temperature = 2500.0;
    let decay = 0.99;
    let sa: SimulatedAnnealing = SimulatedAnnealing::new(iterations, initial_temperature, decay);

    // let p: Parabola = Parabola::initial_solution();
    
    // let weights = vec![1.0, 2.0, 4.0, 8.0, 8.0, 8.0];
    // let gains =   vec![1.0, 2.0, 4.0, 6.0, 6.0, 6.0 ];
    // let penalty = 10_000.0;
    // let max_weight = 8.0;
    // let p: Knapsack = Knapsack::initial_solution(weights, gains, penalty, max_weight);
    
    // Tablle 10.1 

    // best (4 + 4) * (4 + 6) = 8 * 10 = 80, // 12H34H5HV
    let modules: Vec<Rectangle> = vec![
        Rectangle::new(4.0, 6.0),
        Rectangle::new(4.0, 4.0), 
        Rectangle::new(3.0, 4.0), 
        Rectangle::new(4.0, 4.0), 
        Rectangle::new(3.0, 4.0)
        ];
    // with rotated pieces
    // let modules: Vec<Rectangle> = vec![
    //     Rectangle::new(4.0, 6.0),
    //     Rectangle::new(4.0, 4.0), 
    //     Rectangle::new(4.0, 3.0), // rotated 
    //     Rectangle::new(4.0, 4.0), 
    //     Rectangle::new(4.0, 3.0) // rotated
    //     ];
    // todo how to rotate? our always rotate rectangles with min area
    let mut p: PolishExpression = PolishExpression::new(modules);
    sa.run(&mut p);

    


}
