use crate::definitions::*;

pub trait Mutation<Move> {
    fn get_random_move(&mut self) -> Move;
    fn apply_move(&mut self, _move: &Move);
}

pub trait FloorCost {
    fn get_floor_wire(&self) -> f64;
    fn get_floor_area(&self) -> f64;
}

pub trait FloorPlan {
    fn get_floorplan(&self) -> Floorplan;
}

pub trait Cost {
    fn get_cost(&self) -> f64;
}

pub trait Solution<T: Clone> {
    fn copy_solution(&self) -> T;
    fn set_solution(&mut self, solution: T);
}

#[derive(Debug)]
pub struct CostFunction {
    alpha: f64,
    avg_wirelength: f64,
    avg_area: f64,
}

impl Default for CostFunction {
    fn default() -> Self {
        Self { alpha: 1.0, avg_wirelength: 1.0, avg_area: 1.0 }
    }
}

impl CostFunction {
    pub fn new(alpha: f64, avg_wirelength: f64, avg_area: f64) -> Self {
        CostFunction { alpha, avg_wirelength, avg_area }
    }

    pub fn get_cost(&self, area: f64, wirelength: f64) -> f64 {
        let area_cost = area / self.avg_area;
        let wire_cost = wirelength / self.avg_wirelength;
        let cost = area_cost * self.alpha + wire_cost * (1.0 - self.alpha);
        cost
    }
    
    pub fn compute_wirelength(plan: &Floorplan, nets: &Vec<Net>) -> f64 {
        let mut total_wirelength: f64 = 0.0;
        for net in nets.iter() {
            let mut bounding_box = BoundingBox::new(f64::MAX, -f64::MAX, f64::MAX, -f64::MAX);
            for id in net.pins.iter() {
                let (pos_x, pos_y, rect) = plan[*id];
                let (center_x, center_y) = rect.center(pos_x, pos_y);
                bounding_box.extend_point(center_x, center_y);
            }
            // half perimeter estimation
            total_wirelength += bounding_box.get_width() + bounding_box.get_height();
        }
        total_wirelength
    }

    pub fn compute_mean_parameters<T: Mutation<Move> + FloorCost, Move>(algo: &mut T, repetitions: usize) -> (f64, f64) {
        let mut sum_area = 0.0;
        let mut sum_wirelength = 0.0;
        for _ in 0..repetitions {
            let _move: Move = algo.get_random_move();
            algo.apply_move(&_move);
            let wire = algo.get_floor_wire();
            let area = algo.get_floor_area();
            sum_area += area;
            sum_wirelength += wire;
            algo.apply_move(&_move);
            
        }
        if sum_wirelength == 0.0 {
            sum_wirelength = repetitions as f64;
        }
        if sum_area == 0.0 {
            sum_area = repetitions as f64
        }
        (sum_area / repetitions as f64, sum_wirelength / repetitions as f64)
    }

    // returns dead area in percent
    pub fn get_dead_area<T: FloorCost>(floor: &T, modules: &Vec<Rectangle>) -> f64{
        let occupied_area: usize = modules.iter()
            .map(|rect| rect.area())
            .sum();
        let total_area = floor.get_floor_area();
        let dead_area = 1.0 - (occupied_area as f64 / total_area as f64);
        dead_area * 100.0
    }
}
