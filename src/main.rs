// mod parabola;
// mod knapsack;
// mod genetic_algorithm;
mod simulated_annealing;
mod polish_expression;
mod shape_function;
mod definitions;
mod instance_generator;
mod parser;
mod draw;
mod slicing_tree;
mod floorplan_common;
mod sequence_pair;
mod hypergraph;
mod time;

use std::fmt::Debug;
use crate::simulated_annealing::*;
use crate::parser::*;
use crate::draw::*;
use crate::floorplan_common::*;
use crate::sequence_pair::*;
use crate::definitions::*;
use crate::hypergraph::*;
use crate::instance_generator::random_instance;
use crate::polish_expression::*;
use crate::time::*;
use clap::Parser;

/// command line arguments
#[derive(Parser, Debug, Clone)]
#[command(author, about, long_about = None)]
struct Args {
    /// path to .floor file
    #[arg(long, default_value_t = String::from("benchmark/n300.floor"))]
    input: String,
    
    /// name of floorplan representation: slicing_tree, sequence_pair
    #[arg(short, long, default_value_t = String::from("sequence_pair"))]
    floorplan_type: String,

    /// alpha in cost function, area_cost * alpha + wire_cost * (1 - alpha);
    #[arg(short, long, default_value_t = 0.5)]
    alpha: f64,

    /// number of SA iteraations
    #[arg(short, long, default_value_t = 1_000_000)]
    iterations: usize,
    
    /// use recursive bisection to get inital solution
    #[arg(short, long)]
    recursive_bisection: bool,

    /// use cluster growing order for recursive bisection
    #[arg(short, long)]
    cluster_growing: bool,

    /// save image of final floorplan
    #[arg(short, long)]
    save_image: bool,

    /// path of output image if save_image is set
    #[arg(short, long, default_value_t = String::from("floorplan.svg"))]
    out_image: String,
}

fn run_simulated_annealing<T, S, Move>(p: &mut T, config: SimulatedAnnealingConfig) 
where 
    T: Mutation<Move> + Cost + Solution<S>,
    S: Clone + Debug,
    {
        let num_moves: usize = config.num_moves_estimation;
        let initial_prob = config.inital_prob;
        let iterations = config.iterations;
        let initial_temperature = SimulatedAnnealing::estimate_initial_temperature(initial_prob, num_moves, p);
        let decay = SimulatedAnnealing::get_decay_for_n_iterations(iterations, initial_temperature);
        
        eprintln!("T: {}, it: {}, decay: {}", initial_temperature, iterations, decay);
        let sa: SimulatedAnnealing = SimulatedAnnealing::new(iterations, initial_temperature, decay);
        sa.run(p);    
    }

fn run_algorithm<T, S, Move>(p: &mut T, sa_config: SimulatedAnnealingConfig, args: Args, instance: FloorplanInstance) 
where 
    T: Mutation<Move> + Cost + Solution<S> + FloorCost + FloorPlan,
    S: Clone + Debug,
    {
        let modules = instance.modules;
        let net_list = instance.nets;

        let dead_area_before = CostFunction::get_dead_area(p, &modules);
        let wire_before = p.get_floor_wire();
        
        let timer = Timer::new();
        run_simulated_annealing(p, sa_config);
        let time_ms = timer.get_passed_ms();

        let plan_after = p.get_floorplan();
        let dead_area_after = CostFunction::get_dead_area(p, &modules);
        let wire_after = p.get_floor_wire();
        let wire_reduction = (wire_after / wire_before) * 100.0;
        let area_after = p.get_floor_area();
    
        eprintln!("");
        eprintln!("dead area before {:.2?}%, after {:.2?}%", dead_area_before, dead_area_after);
        eprintln!("{:.2?}% of wirelength before", wire_reduction);
        eprintln!("total area: {}", area_after);
        eprintln!("total wire: {}", wire_after);
        eprintln!("time [s]: {:.2}", time_ms / 1000.0);
        
        let svg_image = &args.out_image;
        let draw_nets = false;
        if args.save_image {
            draw_floorplan(&plan_after, svg_image, &net_list, draw_nets);
        }

        // output for csv
        // instance, floorplan,alpha,time[ms],total_area,dead_area,total_wire,iterations,cluster_growing,recursive_bisection
        print!("{},", args.input);
        print!("{},", args.floorplan_type);
        print!("{},", args.alpha);
        print!("{:.2},", time_ms);
        print!("{},", area_after);
        print!("{:.2},", dead_area_after);
        print!("{},", wire_after);
        print!("{},", args.iterations);
        print!("{},", args.cluster_growing);
        print!("{}", args.recursive_bisection);
        println!("")
    }

fn cli() {
    let args = Args::parse();
    eprintln!("{:?} \n", args);

    eprintln!("--> reading file: {}", args.input);
    let (blocks, nets) = parse_file(args.input.clone()).unwrap();
    eprintln!("modules: {}, nets: {}, alpha {}", blocks.len(), nets.len(), args.alpha);
    eprintln!("using {} floorplan representation", args.floorplan_type.clone());
    if args.recursive_bisection {
        if args.cluster_growing {
            eprintln!("using recursive bisection with cluster growing");
        }
        else {
            eprintln!("using recursive bisection");
        }
    }
    eprintln!("");

    let graph = Hypergraph::from(nets.clone());
    let mut order: Vec<Int> = (0..graph.num_nodes).collect();

    if args.cluster_growing {
        order = cluster_growing_order(&graph, 0);
    }
    
    let modules = blocks.clone();
    let net_list = nets.clone();
    let instance = FloorplanInstance{modules: modules, nets: net_list};

    let iterations = args.iterations;
    let num_moves_estimation = 3 * blocks.len();
    let inital_prob =  0.95;
    let sa_config = SimulatedAnnealingConfig{iterations, num_moves_estimation, inital_prob};

    if args.floorplan_type == "slicing_tree" {
        let mut p: PolishExpression = PolishExpression::new(blocks, nets, args.alpha);
        if args.recursive_bisection {
            p.set_solution_recursive_bisection(&order);
        } 
        run_algorithm(&mut p, sa_config, args, instance);
    }
    else if args.floorplan_type == "sequence_pair" {
        let mut p = SequencePair::new(blocks, nets, args.alpha);
        if args.recursive_bisection {
            p.set_solution_recursive_bisection(&order);
        } 
        run_algorithm(&mut p, sa_config, args, instance);
    }
    else {
        panic!("unknown floorplan type {}", args.floorplan_type);
    }
}

fn main() {
    eprintln!("Hello, SA!");
    cli();
}
