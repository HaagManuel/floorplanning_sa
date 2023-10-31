
use crate::definitions::*;
use crate::floorplan_common::*;
use rand::prelude::*;

pub type SequencePairSolution = (Vec<Int>, Vec<Int>, Vec<Rectangle>);

#[derive(Debug)]
pub enum SPMoveType {
    RotateModule(usize), 
    SwapLeftSide(usize, usize), 
    SwapRightSide(usize, usize), 
    SwapBothSides(usize, usize, usize, usize), 
}

impl SPMoveType {
    fn apply(&self, sequence_pair: &mut SequencePair) {
        match *self {
            SPMoveType::RotateModule(a) => sequence_pair.modules[a] = sequence_pair.modules[a].transpose(),
            SPMoveType::SwapLeftSide(a, b) => sequence_pair.x_sequence.swap(a, b),
            SPMoveType::SwapRightSide(a, b) => {
                sequence_pair.y_sequence.swap(a, b);
                sequence_pair.index_y.swap(sequence_pair.y_sequence[a], sequence_pair.y_sequence[b]);
            },
            SPMoveType::SwapBothSides(a, b, c, d) => {
                sequence_pair.x_sequence.swap(a, b); 
                sequence_pair.y_sequence.swap(c, d); 
                sequence_pair.index_y.swap(sequence_pair.y_sequence[c], sequence_pair.y_sequence[d]);
            }   
        }
    }
}

#[derive(Debug, Default)]
pub struct SequencePair {
    modules: Vec<Rectangle>,
    nets: Vec<Net>,
    x_sequence: Vec<Int>,
    y_sequence: Vec<Int>,
    index_y: Vec<usize>, // index of number i in y_sequence
    placement: Floorplan,
    lca_array: Vec<Vec<Int>>, // later try 1-dim
    
    cost_function: CostFunction,
    current_cost: f64,
    current_wire: f64,
    current_area: f64,
    bounding_box: Rectangle,
}

impl SequencePair {
    pub fn new(modules: Vec<Rectangle>, nets: Vec<Net>, alpha: f64) -> Self {
        let n = modules.len();
        let mut sp = SequencePair::default();
        
        // initialize data structures
        sp.placement = vec![(0,0, Rectangle::new(0, 0)); n];
        sp.lca_array = vec![vec![0; n + 2]; n + 2]; // border of zeros 
        sp.modules = modules;
        sp.nets = nets;

        // initial sequence
        sp.x_sequence = (0..n).collect();
        sp.y_sequence = (0..n).collect();
        sp.index_y  = (0..n).collect();

        // update cost parameter
        sp.update();

        // compute averages for cost function
        let repetitions = 3 * n;
        let (avg_wirelength,avg_area) = CostFunction::compute_mean_parameters(&mut sp, repetitions);
        sp.cost_function = CostFunction::new(alpha, avg_wirelength, avg_area);
        sp.current_cost = sp.compute_cost();
        sp
    }

    pub fn update(&mut self) {
        self.compute_floorplan();
        self.current_area = self.bounding_box.area() as f64;
        self.current_wire = CostFunction::compute_wirelength(&self.placement, &self.nets);
        self.current_cost = self.compute_cost();
    }

    pub fn compute_cost(&mut self) -> f64 {
        self.cost_function.get_cost(self.current_area, self.current_wire)
    }
    
    // O(n^2) later maybe O(n log n) with better algorithm
    fn compute_lca<F, W>(&mut self, f_i: F, weight: W) where 
    F: Fn(usize) -> usize,
    W: Fn(Rectangle) -> usize {
        let n = self.x_sequence.len();
        for a in 1..n+1{
            for b in 1..n+1 {
                let i = f_i(a); // normal or reversed order
                let j = b - 1;
                let id = self.x_sequence[i];
                if id == self.y_sequence[j] {
                    self.lca_array[a][b] = self.lca_array[a - 1][b - 1] + weight(self.modules[id]); // width or heigth
                }
                else {
                    self.lca_array[a][b] = self.lca_array[a - 1][b].max(self.lca_array[a][b - 1]);
                }
            }
        }
    }

    pub fn compute_floorplan(&mut self) {
        let n = self.placement.len();
        let shift = |x: usize| {x - 1};
        let reverse = |x: usize| {n - x};
        let get_width  = |rect: Rectangle| {rect.width};
        let get_height = |rect: Rectangle| {rect.height};
        
        // lca (x, y, widths)    --> x-coords
        self.compute_lca(shift, get_width);
        self.bounding_box.width = self.lca_array[n][n];
        for i in 0..n {
            let id = self.x_sequence[i];
            let pos_y = self.index_y[id];

            // lca array starts at 1
            let x_coord = self.lca_array[i + 1][pos_y + 1];
            let width = self.modules[id].width;
            let height = self.modules[id].height;
            self.placement[id] = (x_coord, 0, Rectangle::new(width, height));
        }
        
        // lca (x^R, y, heights) --> y-coords
        self.compute_lca(reverse, get_height);
        self.bounding_box.height = self.lca_array[n][n];
        for i in 0..n {
            let id = self.x_sequence[i];
            let pos_y = self.index_y[id];

            // lca array starts at 1
            let y_coord = self.lca_array[n - i + 1][pos_y + 1];
            self.placement[id].1 = y_coord;
        }
    }

    
}
impl Mutation<SPMoveType> for SequencePair {
    fn get_random_move(&mut self) -> SPMoveType {
        let mut rng: ThreadRng = rand::thread_rng();
        macro_rules! random {
            ($m:expr) => {{
                rng.gen_range(0..$m)
            }};
         }
         
        macro_rules! two_random {
        ($m:expr) => {{
            let (c, d): (usize, usize);
            loop {
                let a = random!($m);
                let b = random!($m);
                if a != b {
                    (c, d) = (a, b);
                    break;
                }
            }
            (c, d)
        }};
        }
        let n = self.modules.len();
        let r = random!(4);
        let move_type: SPMoveType = match r {
            0 => SPMoveType::RotateModule(random!(n)),
            1 => {
                let (a, b) = two_random!(n);
                SPMoveType::SwapLeftSide(a, b)
            },  
            2 => {
                let (a, b) = two_random!(n);
                SPMoveType::SwapRightSide(a, b)
            },
            _ => {
                let (a, b) = two_random!(n);
                let (c, d) = (self.index_y[a], self.index_y[b]);
                SPMoveType::SwapBothSides(a, b, c, d)
            },
        };
        move_type
    }
    fn apply_move(&mut self, _move: &SPMoveType) {
        _move.apply(self);
        self.update()
    }
}

impl FloorCost for SequencePair {
    fn get_floor_area(&self) -> f64 {
        self.current_area
    }

    fn get_floor_wire(&self) -> f64 {
        self.current_wire
    }   
}

impl FloorPlan for SequencePair {
    fn get_floorplan(&self) -> Floorplan {
        self.placement.clone()
    }
}

impl Cost for SequencePair {
    fn get_cost(&self) -> f64 {
        self.current_cost
    }
}

// x_sequence, y_sequence, rotated rectangles
impl Solution<SequencePairSolution> for SequencePair {
    fn copy_solution(&self) -> SequencePairSolution {
        (self.x_sequence.clone(), self.y_sequence.clone(), self.modules.clone())
    }

    fn set_solution(&mut self, solution: SequencePairSolution) {
        (self.x_sequence, self.y_sequence, self.modules) = solution;
        self.update();
    }
}