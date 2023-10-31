mod simulated_annealing;
mod parabola;
mod knapsack;
mod polish_expression;
mod shape_function;
mod definitions;
mod instance_generator;
mod parser;
mod draw;
mod slicing_tree;
mod sequence_pair;
mod floorplan_common;

use crate::simulated_annealing::SimulatedAnnealing;
// use crate::parabola::Parabola;
// use crate::knapsack::Knapsack;
// use crate::shape_function::ShapeFunction;
use crate::polish_expression::*;
// use crate::definitions::*;
// use crate::instance_generator::*;
use crate::parser::*;
use crate::draw::*;
use crate::floorplan_common::*;
use crate::sequence_pair::*;

fn main() {
    println!("Hello, SA!");

    let(mut blocks, nets) = parse_file("benchmark/n10.floor").unwrap();
    // let(mut blocks, nets) = parse_file("benchmark/n30.floor").unwrap();
    // let(mut blocks, nets) = parse_file("benchmark/n300.floor").unwrap();
    
    
    let net_list = nets.clone();
    let num_blocks = blocks.len();
    blocks.sort_by_key(|rect| rect.width.max(rect.height));

    let alpha: f64 = 0.5;
    let mut sp = SequencePair::new(blocks, nets, alpha);
    let _move = sp.get_random_move();
    sp.apply_move(&_move);
    return;
    

    let mut p: PolishExpression = PolishExpression::new(blocks, nets, alpha);
    // p.set_solution_operator_top();
    p.set_solution_all_vertical();
    
    let num_moves: usize = 3 * num_blocks;
    let initial_prob = 0.95;
    let initial_temperature = SimulatedAnnealing::estimate_initial_temperature(initial_prob, num_moves, &mut p);
    
    let iterations = 1_000_000;
    let decay = SimulatedAnnealing::get_decay_for_n_iterations(iterations, initial_temperature);
    println!("T: {}, it: {}, decay: {}", initial_temperature, iterations, decay);
    
    let plan_before = p.get_floorplan();
    let dead_area_before = p.get_dead_area() * 100.0;
    let wire_before = CostFunction::compute_wirelength(&plan_before, &net_list);
    let sa: SimulatedAnnealing = SimulatedAnnealing::new(iterations, initial_temperature, decay);
    sa.run(&mut p);
    
    let plan_after = p.get_floorplan();
    let dead_area_after = p.get_dead_area() * 100.0;
    let wire_after = CostFunction::compute_wirelength(&plan_after, &net_list);
    let wire_reduction = (wire_after / wire_before) * 100.0;
    println!("dead area before {:.2?}, after {:.2?}%", dead_area_before, dead_area_after);
    println!("{:.2?}% of wirelength before", wire_reduction);
    
    let svg_image1 = "plan_before.svg";
    let svg_image2 = "plan_after.svg";
    let draw_nets = false;
    draw_floorplan(&plan_before, svg_image1, &net_list, draw_nets);
    draw_floorplan(&plan_after, svg_image2, &net_list, draw_nets);
}
