use rand::distributions::{Distribution, Uniform};
use crate::rectangle::*;

pub fn random_module_list(number_of_modules: Int , min_size: Int, max_size: Int) -> Vec<Rectangle> {
    let mut rng = rand::thread_rng();
    let range = Uniform::<Int>::from(min_size..max_size);
    let modules: Vec<Rectangle> = 
            (0..number_of_modules)
            .map(|_| Rectangle::new(range.sample(&mut rng), range.sample(&mut rng)))
            .collect();
    modules
}

