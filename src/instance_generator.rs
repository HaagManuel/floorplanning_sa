use rand::distributions::{Distribution, Uniform};
use crate::definitions::*;

#[allow(dead_code)]
pub fn random_instance(num_modules: Int, num_nets: Int, min_size: Int, max_size: Int) -> (Vec<Rectangle>, Vec<Net>) {
    let mut rng = rand::thread_rng();

    let range = Uniform::<Int>::from(min_size..max_size);
    let modules: Vec<Rectangle> = 
    (0..num_modules)
    .map(|_| Rectangle::new(range.sample(&mut rng), range.sample(&mut rng)))
    .collect();
    
    // only nets with two pins for now
    let range = Uniform::<Int>::from(0..num_modules);
    let nets: Vec<Net> = 
            (0..num_nets)
            .map(|i| Net{pins: vec![range.sample(&mut rng), range.sample(&mut rng)], id: i})
            .collect();
    (modules, nets)
}

