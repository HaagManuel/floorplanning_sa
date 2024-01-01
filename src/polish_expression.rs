use crate::definitions::*;
use crate::slicing_tree::*;
use crate::floorplan_common::*;
use rand::prelude::*;

pub type PolishExpressionSolution = Vec<ModuleNode>;

#[derive(Debug)]
pub enum PEMoveType {
    InvertChain(usize), // starting from usize
    SwapOperands(usize, usize), 
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

#[derive(Default)]
pub struct PolishExpression {
    solution: PolishExpressionSolution,
    modules: Vec<Rectangle>,
    nets: Vec<Net>,
    num_operators: Vec<usize>, // to check if op3 is legal
    tree: SlicingTree,

    cost_function: CostFunction,
    current_cost: f64,
    current_area: f64,
    current_wire: f64,
}


impl PolishExpression {
    pub fn new(modules: Vec<Rectangle>, nets: Vec<Net>, alpha: f64) -> Self {
        let n = modules.len();
        let mut pe = PolishExpression::default();
        
        // initialize data structures
        pe.modules = modules;
        pe.nets = nets;
        pe.tree = SlicingTree::new(n);
        pe.cost_function = CostFunction::new(alpha, 1.0, 1.0);

        pe.set_solution_all_vertical();
        pe
    }

    pub fn update_cost_function(&mut self) {
        // compute averages for cost function
        let repetitions = 3 * self.modules.len();
        let (avg_wirelength,avg_area) = CostFunction::compute_mean_parameters(self, repetitions);
        self.cost_function = CostFunction::new(self.cost_function.alpha, avg_wirelength, avg_area);
        self.current_cost = self.cost_function.get_cost(self.current_area, self.current_wire);
    }

    pub fn update(&mut self) {
        self.tree.recompute(&self.solution, &self.modules);
        self.tree.recompute_floorplan();
        self.current_area = self.tree.get_min_area();
        self.current_wire = CostFunction::compute_wirelength(&self.tree.placement, &self.nets);
        self.current_cost = self.cost_function.get_cost(self.current_area, self.current_wire);
        self.num_operators = self.get_num_operator();   
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

    pub fn set_solution_recursive_bisection(&mut self, order: &Vec<Int>) {
        let solution = self.recursive_bisection(&order, 0, self.modules.len(), ModuleNode::H());
        self.set_solution(solution);
    }

    // intervall [l, r)
    fn recursive_bisection(&self, order: &Vec<Int>, left: usize, right: usize, split: ModuleNode) -> PolishExpressionSolution {
        if right - left == 1 {
            return vec![ModuleNode::Module(order[left])];
        }
        else if right - left == 2 {
            return vec![ModuleNode::Module(order[left]), ModuleNode::Module(order[left + 1]), split];
        }
        else {
            let mid = (left + right).div_ceil(2);
            let mut s1 = self.recursive_bisection(order, left, mid, split.invert());
            let mut s2 = self.recursive_bisection(order, mid, right, split.invert());
            s1.append(&mut s2);
            s1.push(split);
            return s1;
        }
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
}

impl Mutation<PEMoveType> for PolishExpression {
    fn get_random_move(&mut self) -> PEMoveType {
        let mut rng: ThreadRng = rand::thread_rng();
        let r: u64 = rng.gen_range(0..3);
        let move_type: PEMoveType = 
        match r {
            0 => self.get_swap_adjacent_operands(),
            1 => self.get_invert_chain(),
            _ => {
                // make sure prefix array is updated
                // only needed for move 3
                // self.num_operators = self.get_num_operator();
                // can fail if there is no possible swap
                self.swap_operand_operator().unwrap_or(self.get_swap_adjacent_operands())
            },
        };   
        move_type
    }

    fn apply_move(&mut self, _move: &PEMoveType, update: bool) {
        debug_assert!(self.tree.sanity_check(&self.solution));
        _move.apply(&mut self.solution);
        match _move {
                PEMoveType::InvertChain(a) => self.tree.update_invert_chain(*a),
                PEMoveType::SwapOperands(a, b) => self.tree.update_swap_leafs(*a, *b),
                PEMoveType::SwapOperandOperator(a, b) => self.tree.update_swap_operand_operator(*a, *b),
        }
        if update {
            self.update();
        }
        else {
            // bring tree into a consistent state, but saves floorplan and wire computation
            self.tree.recompute(&self.solution, &self.modules)
        }
    }
}

impl FloorCost for PolishExpression {
    fn get_floor_wire(&self) -> f64 {
        self.current_wire
    }
    
    fn get_floor_area(&self) -> f64 {
        self.current_area
    }
}

impl FloorPlan for PolishExpression {
    fn get_floorplan(&self) -> Floorplan {
        self.tree.placement.clone()
    }
}

impl Cost for PolishExpression {
    fn get_cost(&self) -> f64 {
        self.current_cost
    }
}

impl Solution<PolishExpressionSolution> for PolishExpression {
    fn copy_solution(&self) -> PolishExpressionSolution {
        self.solution.clone()
    }

    fn set_solution(&mut self, solution: PolishExpressionSolution) {
        self.solution = solution;
        self.tree.update_everything();
        self.update();
        self.update_cost_function()
    }
}
