use crate::simulated_annealing::{SAInstance, SAMove};
use crate::definitions::*;
use crate::slicing_tree::*;
use crate::floorplan_common::*;
use rand::prelude::*;
#[derive(Default)]
pub struct PolishExpression {
    solution: Vec<ModuleNode>,
    modules: Vec<Rectangle>,
    nets: Vec<Net>,
    num_operators: Vec<usize>, // to check if op3 is legal
    tree: SlicingTree,
    current_cost: f64,
    avg_wirelength: f64,
    avg_area: f64,
    alpha: f64, // between 0 and 1, -> 1 area more important, -> 0 wirelength more important
}

#[derive(Debug)]
pub enum PEMoveType {
    SwapOperands(usize, usize), 
    InvertChain(usize), // starting from usize
    SwapOperandOperator(usize, usize), 
}

impl PEMoveType {
    fn apply(&self, solution: &mut Vec<ModuleNode>) {
        match *self {
            PEMoveType::SwapOperands(a, b) => solution.swap(a, b),
            PEMoveType::SwapOperandOperator(a, b) => solution.swap(a, b),
            PEMoveType::InvertChain(a) => {
                for i in a..solution.len() {
                    if solution[i].is_module() {
                       break;
                    }
                    else {
                        solution[i] = solution[i].invert();
                    }
                }
            }   
        }
    }
}

#[derive(Debug)]
pub struct PolishExpressionMove {
    move_type: PEMoveType,
    delta_cost: f64,
}

impl PolishExpression {
    pub fn new(modules: Vec<Rectangle>, nets: Vec<Net>, alpha: f64) -> Self {
        let n = modules.len();
        let mut polish_expression = PolishExpression::default();
        polish_expression.modules = modules;
        polish_expression.nets = nets;
        polish_expression.alpha = alpha;
        polish_expression.tree = SlicingTree::new(n);
        polish_expression
    }

    // first all module ids than HVHVHVH
    pub fn set_solution_operator_top(&mut self) {
        let n = self.modules.len();
        let operators_it = (0..n).map(|x| ModuleNode::Module(x));
        let operand_it = [ModuleNode::H(), ModuleNode::V()].into_iter().cycle().take(n - 1);
        let solution: Vec<ModuleNode> = operators_it.chain(operand_it).collect();
        self.set_solution(solution);
    }

    pub fn set_solution_all_vertical(&mut self) {
        let mut solution: Vec<ModuleNode> = vec![ModuleNode::Module(0)];
        for i in 1..self.modules.len() {
            solution.push(ModuleNode::Module(i));
            solution.push(ModuleNode::V());
        }
        self.set_solution(solution);
    }

    pub fn set_solution(&mut self, solution: Vec<ModuleNode>) {
        self.solution = solution;
        let (avg_area, avg_wire) = self.get_avg_wirelenth_avg_area(3 * self.solution.len());
        self.avg_area = avg_area;
        self.avg_wirelength = avg_wire;
        self.current_cost = self.eval_expression();
        self.num_operators = self.get_num_operator();
    }   

    // clones floorplan
    pub fn get_floorplan(&mut self) -> Floorplan {
        self.tree.recompute(&self.solution, &self.modules);
        self.tree.recompute_floorplan();
        self.tree.placement.clone()
    }

    pub fn get_total_area(&mut self) -> Int {
        self.tree.recompute(&self.solution, &self.modules);
        self.tree.get_min_area()
    }

    pub fn get_dead_area(&mut self) -> f64 {
        let occupied_area: usize = self.modules
            .iter()
            .map(|rect| rect.area())
            .sum();
        let total_area = self.get_total_area();
        1.0 - (occupied_area as f64 / total_area as f64)
    }

    fn get_num_operator(&self) -> Vec<usize> {
        self.solution.iter()
        .scan(0, |sum, m| {
                let is_operator = if m.is_module() {0} else {1};
                *sum += is_operator; 
                Some(*sum)
            }
        )
        .collect()
    }

    fn eval_area_wirelength(&mut self) -> (f64, f64) {
        self.tree.recompute(&self.solution, &self.modules);
        self.tree.recompute_floorplan();
        let area = self.tree.get_bounding_box().area() as f64;
        let wirelength = CostFunction::compute_wirelength(&self.tree.placement, &self.nets);
        (area, wirelength)
    }

    pub fn eval_expression(&mut self) -> f64 {
        let (area, wirelength) = self.eval_area_wirelength();
        let area_cost = area / self.avg_area;
        let wire_cost = wirelength / self.avg_wirelength;
        let cost = area_cost * self.alpha + wire_cost * (1.0 - self.alpha);
        cost
        // punish rectangles that are far from a square just for testing packing
        // let cost =  rect.area() + rect.width * rect.width + rect.height * rect.height;
        // cost as f64
    }

    fn get_swap_adjacent_operands(&self) -> PEMoveType {
        let mut rng: ThreadRng = rand::thread_rng();
        let m = self.solution.len();
        loop {
            let a = rng.gen_range(0..m);
            if self.solution[a].is_module() {
                for b in a+1..self.solution.len() {
                    if  self.solution[b].is_module() {
                        return PEMoveType::SwapOperands(a, b);
                    }
                }
            }
        }
    }

    fn get_swap_operands(&self) -> PEMoveType {
        let mut rng: ThreadRng = rand::thread_rng();
        let m = self.solution.len();
        loop {
            let a = rng.gen_range(0..m);
            let b = rng.gen_range(0..m);
            if self.solution[a].is_module() && self.solution[b].is_module() && a != b {
                    return PEMoveType::SwapOperands(a, b);
            }
        }
    }
    
    fn get_invert_chain(&self) -> PEMoveType {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut pos: Vec<usize> = Vec::new();
        let m = self.solution.len();
        // need atleast to operands
        for i in 2..m {
            if self.solution[i-1].is_module() && !self.solution[i].is_module() {
                pos.push(i);
            } 
        }
        let a = rng.gen_range(0..pos.len());
        PEMoveType::InvertChain(pos[a])
    }

    fn swap_operand_operator(&self) -> Option<PEMoveType> {
        let mut rng: ThreadRng = rand::thread_rng();
        let m = self.solution.len();
        let mut pos: Vec<usize> = Vec::new();
        // operator can not be at position 0 or 1
        for a in  1..m - 1 {
            let l = self.solution[a];
            let r = self.solution[a + 1];
            // check parentheses property because we move operator to the left
            if l.is_module() && !r.is_module() && 2 * self.num_operators[a] < a - 1{
                // previous node always exist because of parentheses property
                // no VV or HH
                let ll  = self.solution[a - 1];
                if r != ll {
                    pos.push(a);
                }
            }
            // operator is moved to the right --> parentheses property can not be violated
            else if !l.is_module() && r.is_module() {
                // no right neighbor that could be cause VV or HH
                if a + 2 >= m {
                    pos.push(a);
                }
                else {
                    // no VV or HH
                    let rr = self.solution[a + 2];
                    if l != rr {
                        pos.push(a);
                    }
                }
            }
        }
        if pos.len() > 0 {
            let i = rng.gen_range(0..pos.len());
            Some(PEMoveType::SwapOperandOperator(pos[i], pos[i] + 1))
        } 
        else {
            None
        }
    }

    fn get_avg_wirelenth_avg_area(&mut self, repetitions: usize) -> (f64, f64) {
        let mut sum_area = 0.0;
        let mut sum_wirelength = 0.0;
        for _ in 0..repetitions {
            let _move = self.get_random_move();
            _move.apply(&mut self.solution);
            let (area, wire) = self.eval_area_wirelength();
            _move.apply(&mut self.solution);
            sum_area += area;
            sum_wirelength += wire;
            
        }
        (sum_area / repetitions as f64, sum_wirelength / repetitions as f64)
    }

    fn get_random_move(&mut self) -> PEMoveType {
        let mut rng: ThreadRng = rand::thread_rng();
        let r: u64 = rng.gen_range(0..3);
        let move_type: PEMoveType = 
        match r {
            0 => self.get_swap_adjacent_operands(),
            1 => self.get_invert_chain(),
            _ => {
                // make sure prefix array is updated
                self.num_operators = self.get_num_operator();
                // can fail if there is no possible swap
                self.swap_operand_operator().unwrap_or(self.get_swap_adjacent_operands())
            },
        };   
        move_type
    }

}

impl SAInstance<PolishExpressionMove, Vec<ModuleNode>> for PolishExpression {
    fn get_move(&mut self) -> PolishExpressionMove {
        let move_type: PEMoveType = self.get_random_move();
        let old: f64 = self.eval_expression();
        move_type.apply(&mut self.solution);
        let new: f64 = self.eval_expression();
        move_type.apply(&mut self.solution);
        let delta_cost: f64 = new - old;
        return PolishExpressionMove{move_type, delta_cost}
    }

    fn apply_move(&mut self, _move: PolishExpressionMove) {
        let move_type: PEMoveType = _move.move_type;
        self.current_cost += _move.delta_cost;
        move_type.apply(&mut self.solution);
    }

    fn current_cost(&self) -> f64 {
        self.current_cost
    }

    fn copy_solution(&self) -> Vec<ModuleNode> {
        self.solution.clone()
    }

    fn set_solution(&mut self, solution: Vec<ModuleNode>) {
        self.set_solution(solution)
    }

}

impl SAMove for PolishExpressionMove {
    fn get_delta_cost(&self) -> f64 {
        self.delta_cost
    }
}
