mod simulated_annealing;
mod parabola;
mod knapsack;
mod polish_expression;
mod shape_function;
mod rectangle;

use crate::simulated_annealing::SimulatedAnnealing;
// use crate::parabola::Parabola;
// use crate::knapsack::Knapsack;
// use crate::shape_function::ShapeFunction;
use crate::polish_expression::*;
use crate::rectangle::*;

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

    // log r < delta / T
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
    

    // best (4 + 4) * (4 + 6) = 8 * 10 = 80, // 12H34H5HV
    let modules: Vec<Rectangle> = vec![
        Rectangle::new(4.0, 6.0),
        Rectangle::new(4.0, 4.0), 
        Rectangle::new(3.0, 4.0), 
        Rectangle::new(4.0, 4.0), 
        Rectangle::new(3.0, 4.0)
        ];

    // let solution: Vec<ModuleNode> = vec![
    //         ModuleNode::Module(0),
    //         ModuleNode::Module(1),
    //         ModuleNode::H(),
    //         ModuleNode::Module(2),
    //         ModuleNode::Module(3),
    //         ModuleNode::H(),
    //         ModuleNode::Module(4),
    //         ModuleNode::H(),
    //         ModuleNode::V()
    // ];
    

    let mut p: PolishExpression = PolishExpression::new(modules);
    // p.set_solution(solution);
    let v = p.eval_expression();
    println!("{}", v);
    // TODO benchmark shape function with branch or duplicate -> random generation of instances
    sa.run(&mut p);

}
