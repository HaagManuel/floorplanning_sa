mod simulated_annealing;
mod parabola;
mod knapsack;
mod polish_expression;
mod shape_function;
mod definitions;
mod instance_generator;
mod parser;
mod draw;

use crate::simulated_annealing::SimulatedAnnealing;
// use crate::parabola::Parabola;
// use crate::knapsack::Knapsack;
// use crate::shape_function::ShapeFunction;
use crate::polish_expression::*;
use crate::definitions::*;
use crate::instance_generator::*;
use crate::parser::*;
use crate::draw::*;


fn main() {
    println!("Hello, SA!");

    // let(blocks, nets) = parse_file("benchmark/n10.floor").unwrap();
    let(blocks, nets) = parse_file("benchmark/n30.floor").unwrap();
    // let(blocks, nets) = parse_file("benchmark/n300.floor").unwrap();
    let mut p: PolishExpression = PolishExpression::new(blocks, nets);
    // p.set_solution_operator_top();
    p.set_solution_all_vertical();
    let initial_prob = 0.95;
    let num_moves = 100;
    let initial_temperature = SimulatedAnnealing::estimate_initial_temperature(initial_prob, num_moves, &mut p);
    // let initial_temperature = 100_000.0;
    let iterations = 10_000;
    let decay = 0.999;
    println!("T: {}", initial_temperature);
    
    let plan_before = p.get_floorplan();
    let before = p.get_dead_area() * 100.0;
    
    let sa: SimulatedAnnealing = SimulatedAnnealing::new(iterations, initial_temperature, decay);
    sa.run(&mut p);
    
    let plan_after = p.get_floorplan();
    let after = p.get_dead_area() * 100.0;
    println!("dead area before {:?}, after {:?}%", before, after);
    
    let svg_image1 = "plan_before.svg";
    let svg_image2 = "plan_after.svg";
    draw_floorplan(&plan_before, svg_image1);
    draw_floorplan(&plan_after, svg_image2);
}
