use rand::{distributions::{Distribution, Uniform}, rngs::ThreadRng};
use crate::rectangle::Rectangle;

pub fn random_module_list(number_of_modules: usize , min_size: usize, max_size: usize) -> Vec<Rectangle> {
    let mut rng = rand::thread_rng();
    let range = Uniform::<usize>::from(min_size..max_size);
    let modules: Vec<Rectangle> = 
            (0..number_of_modules)
            .map(|_| Rectangle::new(range.sample(&mut rng) as f64, range.sample(&mut rng) as f64))
            .collect();
    modules
}

