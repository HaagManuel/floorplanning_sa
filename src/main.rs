mod simulated_annealing;
mod parabola;
mod knapsack;
mod polish_expression;
mod shape_function;
mod rectangle;
mod instance_generator;

use crate::simulated_annealing::SimulatedAnnealing;
// use crate::parabola::Parabola;
// use crate::knapsack::Knapsack;
// use crate::shape_function::ShapeFunction;
use crate::polish_expression::*;
use crate::rectangle::*;
use crate::instance_generator::*;

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
- only integer rectangles?

1. naive polish expressions
*/

fn main() {
    // TODO benchmark shape function with branch or duplicate -> random generation of instances
    println!("Hello, SA!");
    
    let number_of_modules = 100;
    let min_size = 1;
    let max_size = 4;
    let instance = random_module_list(number_of_modules, min_size, max_size);
    let mut p: PolishExpression = PolishExpression::new(instance);

    // let initial_temperature = SimulatedAnnealing::estimate_initial_temperature(0.99, 100, &mut p);
    let initial_temperature = 2000.0;
    let iterations = 10000;
    let decay = 0.99;
    println!("T: {}", SimulatedAnnealing::estimate_initial_temperature(0.99, 100, &mut p));

    let sa: SimulatedAnnealing = SimulatedAnnealing::new(iterations, initial_temperature, decay);
    sa.run(&mut p);


}
