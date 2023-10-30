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
    
    // let(mut blocks, nets) = parse_file("benchmark/n10.floor").unwrap();
    let(mut blocks, nets) = parse_file("benchmark/n30.floor").unwrap();
    // let(mut blocks, nets) = parse_file("benchmark/n300.floor").unwrap();
    
    blocks.sort_by_key(|rect| rect.width.max(rect.heigth));
    // blocks.sort_by_key(|rect| ((rect.width.max(rect.heigth) as f64 / rect.width.min(rect.heigth) as f64) * 1000.0) as u32);

    let mut p: PolishExpression = PolishExpression::new(blocks, nets);
    // p.set_solution_operator_top();
    p.set_solution_all_vertical();

    let initial_prob = 0.95;
    let num_moves = 1000;
    let initial_temperature = SimulatedAnnealing::estimate_initial_temperature(initial_prob, num_moves, &mut p);

    let iterations = 1000_000;
    let decay = SimulatedAnnealing::get_decay_for_n_iterations(iterations, initial_temperature);;
    println!("T: {}, it: {}, decay: {}", initial_temperature, iterations, decay);
    
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
