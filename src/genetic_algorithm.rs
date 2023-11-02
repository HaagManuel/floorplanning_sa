use std::marker::PhantomData;

use crate::floorplan_common::*;
use rand::{prelude::*, distributions::WeightedIndex};

pub struct GeneticAlgorithmConfig {
    pub mutation_rate: f64,
    pub population_size: usize,
    pub generations: usize,
}
pub struct GeneticAlgorithm<T, S, Move> 
where
    T: RandomSolution<S>,
    S: Clone
{
    config: GeneticAlgorithmConfig,
    population: Vec<S>,         // population is the set of solutions
    next_population: Vec<S>,    // buffer for the next population
    cost: Vec<f64>,          
    next_cost: Vec<f64>,     
    fitness: Vec<f64>,     
    index: usize,  
    rng: ThreadRng,
    weighted_index: WeightedIndex<f64>, // to generate weigthed probabilities
    phantom1: PhantomData<T>,    // to use type parameter T in implementation
    phantom2: PhantomData<Move>,   
}

impl<T, S, Move> GeneticAlgorithm<T, S, Move> 
where 
    T: Cost + Solution<S> + RandomSolution<S> + Mutation<Move> + Crossover<S>,
    S: Clone + Default
    
{
    pub fn new(config: GeneticAlgorithmConfig) -> Self {
        let population: Vec<S> = vec![S::default(); config.population_size];
        let next_population = vec![S::default(); config.population_size];
        let cost: Vec<f64> = vec![1.0; config.population_size];
        let next_cost = cost.clone();
        let fitness = cost.clone();
        let index = 0;
        let rng = rand::thread_rng();
        let weighted_index = WeightedIndex::new(&fitness).unwrap();
        let phantom1 = PhantomData;
        let phantom2 = PhantomData;
        GeneticAlgorithm { config, population, next_population, cost, next_cost, fitness, index, rng, weighted_index, phantom1, phantom2 }
    }

    fn initialize_populuation(&mut self, instance: &mut T) {
        for i in 0..self.config.population_size {
            let solution = instance.random_solution();
            // TODO remove unnecessary copy
            instance.set_solution(solution.clone());
            self.cost[i] = instance.get_cost();
            self.population[i] = solution;
        }
    }

    fn fitness_function(&self, x: f64) -> f64 {
       1.0 /  x.exp()
    }
    
    fn compute_fitness(&mut self){
        for i in 0..self.config.population_size {
            self.fitness[i] = self.fitness_function(self.cost[i]);
        }
    }

    fn perform_mutations(&mut self, instance: &mut T, mutations: usize) {
        for _ in 0..mutations {
            let i = self.weighted_index.sample(&mut self.rng);
            // TODO remove unnecessary update in set solution
            // TODO avoid unncessary update in instance
            instance.set_solution(self.population[i].clone());
            let _move = instance.get_random_move();
            instance.apply_move(&_move);

            let solution = instance.copy_solution();
            let cost = instance.get_cost();
            
            self.next_population[self.index] = solution;
            self.next_cost[self.index] = cost;
            self.index += 1;
        }
    }

    fn perform_crossover(&mut self, instance: &mut T, crossovers: usize) {
        for _ in 0..crossovers {
            let mut i: usize;
            let mut j: usize;
            loop {
                i = self.weighted_index.sample(&mut self.rng);
                j = self.weighted_index.sample(&mut self.rng);
                if i != j {
                    break;
                }
            }
            let a = &self.population[i];
            let b = &self.population[j];

            let solution = instance.crossover(a, b);
            // TODO remove unnecessary update in set solution
            instance.set_solution(solution.clone());
            let cost = instance.get_cost();
            
            self.next_population[self.index] = solution;
            self.next_cost[self.index] = cost;
            self.index += 1;
        }
    }

    pub fn run(&mut self, instance: &mut T) {
        self.initialize_populuation(instance);
        let mutations = (self.config.mutation_rate * self.config.population_size as f64).round() as usize;
        let crossovers = self.config.population_size - mutations;

        let mut best_solution: S = instance.copy_solution();
        let mut best_cost: f64 = instance.get_cost();
    
        for gen in 0..self.config.generations {
            self.index = 0;
            self.compute_fitness();
            self.weighted_index = WeightedIndex::new(&self.fitness).unwrap();

            self.perform_mutations(instance, mutations);   
            self.perform_crossover(instance, crossovers);   

            for i in 0..self.config.population_size {
                if self.next_cost[i] < best_cost {
                    best_cost = self.next_cost[i];
                    best_solution = self.next_population[i].clone();
                }
            }
            if gen % (self.config.generations.div_ceil(10)) == 0 {
                println!("gen {}, best {:.2}, best this gen {}", gen, best_cost, self.next_cost.iter().map(|x| (x * 1000.0) as i32).min().unwrap());
            }

            std::mem::swap(&mut self.population, &mut self.next_population);
            std::mem::swap(&mut self.cost, &mut self.next_cost);
        }
        self.population.clear();
        instance.set_solution(best_solution);

    }
}

// slower and worse quailty than GA
fn run_genetic_algorithm<T, S, Move>(p: &mut T) 
where 
    T: Cost + Solution<S> + RandomSolution<S> + Mutation<Move> + Crossover<S>,
    S: Clone + Default
{
    let mutation_rate = 0.05;
    let generations = 10_000;
    let population_size = 100;
    println!("mutation rate: {}, generations: {}, population size {}: ", mutation_rate, generations, population_size);
    let config: GeneticAlgorithmConfig = GeneticAlgorithmConfig{mutation_rate, generations, population_size};
    let mut ga = GeneticAlgorithm::new(config);
    ga.run(p);
}