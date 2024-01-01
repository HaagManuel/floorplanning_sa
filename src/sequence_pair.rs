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
            SPMoveType::SwapLeftSide(a, b) =>{
                sequence_pair.x_sequence.swap(a, b);
                sequence_pair.index_x.swap(sequence_pair.x_sequence[a], sequence_pair.x_sequence[b]);
            }    
            SPMoveType::SwapRightSide(a, b) => {
                sequence_pair.y_sequence.swap(a, b);
                sequence_pair.index_y.swap(sequence_pair.y_sequence[a], sequence_pair.y_sequence[b]);
            },
            SPMoveType::SwapBothSides(a, b, c, d) => {
                sequence_pair.x_sequence.swap(a, b); 
                sequence_pair.y_sequence.swap(c, d); 
                sequence_pair.index_x.swap(sequence_pair.x_sequence[a], sequence_pair.x_sequence[b]);
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
    len_vec: Vec<Int>,
    index_x: Vec<usize>, // index of number i in x_sequence
    index_y: Vec<usize>, // index of number i in y_sequence
    placement: Floorplan,
    
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
        sp.modules = modules;
        sp.nets = nets;

        // initial sequence
        sp.x_sequence = (0..n).collect();
        sp.y_sequence = (0..n).collect();
        sp.len_vec = (0..n).collect();
        sp.index_x  = (0..n).collect();
        sp.index_y  = (0..n).collect();

        // update cost parameter
        sp.cost_function = CostFunction::new(alpha, 1.0, 1.0);
        sp.update();
        sp.update_cost_function();

        sp
    }

    pub fn set_solution_recursive_bisection(&mut self, order: &Vec<Int>) {
        let (x_sequence, y_sequence) = self.recursive_bisection(&order, 0, self.modules.len(), true);
        self.set_solution((x_sequence, y_sequence, self.modules.clone()));
    }

    // intervall [l, r)
    fn recursive_bisection(&self, order: &Vec<Int>, left: usize, right: usize, split_horizontal: bool) -> (Vec<Int>, Vec<Int>) {
        if right - left == 1 {
            let i = order[left];
            return (vec![i], vec![i]);
        }
        else if right - left == 2 {
            let i = order[left];
            let j = order[left + 1];
            if split_horizontal {
                return (vec![i, j], vec![i, j]);
            }
            else {
                return (vec![j, i], vec![i, j]);
            }
        }
        else {
            let mid = (left + right).div_ceil(2);
            let (mut x1, mut y1) = self.recursive_bisection(order, left, mid, split_horizontal ^ true);
            let (mut x2, mut y2) = self.recursive_bisection(order, mid, right, split_horizontal ^ true);
            if split_horizontal {
                x1.append(&mut x2);
                y1.append(&mut y2);
                return (x1, y1);
            }
            else {
                x2.append(&mut x1);
                y1.append(&mut y2);
                return (x2, y1);
            }
        }
    }

    pub fn update(&mut self) {
        self.compute_floorplan();
        self.current_area = self.bounding_box.area() as f64;
        self.current_wire = CostFunction::compute_wirelength(&self.placement, &self.nets);
        self.current_cost = self.compute_cost();
    }

    pub fn update_cost_function(&mut self) {
        // compute averages for cost function
        let repetitions = 3 * self.modules.len();
        let (avg_wirelength,avg_area) = CostFunction::compute_mean_parameters(self, repetitions);
        self.cost_function = CostFunction::new(self.cost_function.alpha, avg_wirelength, avg_area);
        self.current_cost = self.cost_function.get_cost(self.current_area, self.current_wire);
    }

    pub fn compute_cost(&mut self) -> f64 {
        self.cost_function.get_cost(self.current_area, self.current_wire)
    }
    
    pub fn compute_floorplan(&mut self) {
        let n = self.placement.len();

        // x-coordinates
        self.len_vec.fill(0);
        for i in 0..n {
            let x_id = self.x_sequence[i];
            let pos_y = self.index_y[x_id];
            let l = self.len_vec[pos_y];
            self.placement[x_id].0 = l;
            let t = l + self.modules[x_id].width;
            for j in pos_y..n {
                if t > self.len_vec[j] {
                    self.len_vec[j] = t;
                }
                else { 
                    break;
                }
            }
        }

        // y-coordinates
        self.len_vec.fill(0);
        for i in 0..n {
            let y_id = self.y_sequence[i];
            let pos_x = n - 1 - self.index_x[y_id]; // reversing x sequence -> lca(x^R, y)
            let l = self.len_vec[pos_x];
            self.placement[y_id].1 = l;
            let t = l + self.modules[y_id].height;
            for j in pos_x..n {
                if t > self.len_vec[j] {
                    self.len_vec[j] = t;
                }
                else { 
                    break;
                }
            }
        }
        
        // write rotation of rectangles into floorplan
        for i in 0..n {
            let width = self.modules[i].width;
            let height = self.modules[i].height;
            self.placement[i].2 = Rectangle::new(width, height);
        }

        // compute bounding box
        self.bounding_box.width = self.placement.iter().map(|(x, _, r)| *x + (*r).width).max().unwrap();
        self.bounding_box.height = self.placement.iter().map(|(_, y, r)| *y + (*r).height).max().unwrap();
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
    fn apply_move(&mut self, _move: &SPMoveType, update: bool) {
        _move.apply(self);
        if update {
            self.update()
        }
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
        for (pos, id) in self.x_sequence.iter().enumerate() {
            self.index_x[*id] = pos;
        }
        for (pos, id) in self.y_sequence.iter().enumerate() {
            self.index_y[*id] = pos;
        }
        self.update();
        self.update_cost_function()
    }
}

impl RandomSolution<SequencePairSolution> for SequencePair {
    fn random_solution(&self) -> SequencePairSolution {
        let mut x_sequence: Vec<usize> = (0..self.x_sequence.len()).collect();
        let mut y_sequence: Vec<usize> = x_sequence.clone();
        let mut rect = self.modules.clone();
        for i in 0..rect.len() {
            if thread_rng().gen_bool(0.5) {
                rect[i] = rect[i].transpose();
            }
        }
        x_sequence.shuffle(&mut thread_rng());
        y_sequence.shuffle(&mut thread_rng());
        rect.shuffle(&mut thread_rng());
        (x_sequence, y_sequence, rect)
    }
}

impl Crossover<SequencePairSolution> for SequencePair {
    fn crossover(&self, a: &SequencePairSolution, b: &SequencePairSolution) -> SequencePairSolution {
        
        let n = self.modules.len();
        let n_half = n.div_ceil(2);
        let mut selected = vec![false; n_half];
        let mut n2 = vec![true; n - n_half];
        selected.append(&mut n2);

        let mut x_sequence: Vec<usize> = Vec::new();
        let mut y_sequence: Vec<usize> = Vec::new();
        let mut rect: Vec<Rectangle> = Vec::new();
        x_sequence.reserve_exact(n);
        y_sequence.reserve_exact(n);
        rect.reserve_exact(n);

        for i in 0..n {
            if selected[a.0[i]] {
                x_sequence.push(a.0[i]);
            }
            if selected[a.1[i]] {
                y_sequence.push(a.1[i]);
            }
            if selected[i] {
                rect.push(a.2[i]);
            }
            if !selected[b.0[i]] {
                x_sequence.push(b.0[i]);
            }
            if !selected[b.1[i]] {
                y_sequence.push(b.1[i]);
            }
            if !selected[i] {
                rect.push(a.2[i]);
            }
        }
        (x_sequence, y_sequence, rect)
    }
}