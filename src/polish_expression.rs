use crate::simulated_annealing::{SAInstance, SAMove};
use crate::shape_function::ShapeFunction;
use crate::definitions::*;
use rand::prelude::*;
use std::cmp::Ordering;

pub struct PolishExpression {
    solution: Vec<ModuleNode>,
    modules: Vec<Rectangle>,
    num_operators: Vec<usize>, // to check if op3 is legal
    current_cost: f64,
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
    pub fn new(modules: Vec<Rectangle>) -> Self {
        // first all module ids than HVHVHVH
        let n = modules.len();
        // TODO maybe smarter initialization
        // TODO index array to operand and operators
        let operators_it = (0..n).map(|x| ModuleNode::Module(x));
        let operand_it = [ModuleNode::H(), ModuleNode::V()].into_iter().cycle().take(n - 1);
        let solution: Vec<ModuleNode> = operators_it.chain(operand_it).collect();
        // -> put into test?
        let current_cost = PolishExpression::eval(&solution, &modules);
        let num_operators: Vec<usize> = PolishExpression::get_num_operator(&solution);
        PolishExpression {solution, modules, current_cost, num_operators}
    }

    pub fn set_solution(&mut self, solution: Vec<ModuleNode>) {
        self.solution = solution;
        self.current_cost = PolishExpression::eval(&self.solution, &self.modules);
        self.num_operators = PolishExpression::get_num_operator(&self.solution);
    }   

    pub fn eval(solution: &Vec<ModuleNode>, modules: &Vec<Rectangle>,
    ) -> f64{
        let mut stack: Vec<ShapeFunction> = Vec::new();
        for module_node in solution {
            match *module_node {
                ModuleNode::Module(id) => {
                    let module: Rectangle = modules[id];
                    let mut sf = ShapeFunction::from_iter([module]);
                    // TODO check if better to add twice before  
                    // each rectangle is rotable by default, maybe change later
                    if module.width != module.heigth {
                        sf.add(module.transpose());  
                    }
                    stack.push(sf);
                }
                _ => {
                    let a: ShapeFunction = stack.pop().unwrap();
                    let b: ShapeFunction = stack.pop().unwrap();
                    let combined: ShapeFunction = ShapeFunction::combine(&a, &b, *module_node);
                    stack.push(combined);
                }
            }
        }
        let last: ShapeFunction = stack.pop().unwrap();
        debug_assert!(last.points.len() > 0);
        let min_area_index = last.points
            .iter()
            .enumerate()
            .max_by(|(_, &a), (_, &b)| a.area().partial_cmp(&b.area()).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();
        last.points[min_area_index].area() as f64
    }

    fn get_num_operator(solution: &Vec<ModuleNode>) -> Vec<usize> {
        solution.iter()
        .scan(0, |sum, m| {
                let is_operator = if m.is_module() {0} else {1};
                *sum += is_operator; 
                Some(*sum)
            }
        )
        .collect()
    }

    pub fn eval_expression(&self) -> f64 {
        PolishExpression::eval(&self.solution, &self.modules)
    }

    fn get_swap_operands(&self) -> PEMoveType {
        let mut rng: ThreadRng = rand::thread_rng();
        let m = self.solution.len();
        loop {
            let a = rng.gen_range(0..m);
            for b in a+1..self.solution.len() {
                if self.solution[a].is_module() && self.solution[b].is_module() {
                    return PEMoveType::SwapOperands(a, b);
                }
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

    fn swap_operand_operator(&self) -> PEMoveType {
        let mut rng: ThreadRng = rand::thread_rng();
        let m = self.solution.len();
        loop {
            let a = rng.gen_range(0..m-1);
            // one operand and one operator
            if self.solution[a].is_module() ^ self.solution[a + 1].is_module() && 2 * self.num_operators[a + 1] <= a  {
                return PEMoveType::SwapOperandOperator(a, a + 1);
            }
        }   
    }
}

impl SAInstance<PolishExpressionMove, Vec<ModuleNode>> for PolishExpression {
    fn get_move(&mut self) -> PolishExpressionMove {
        let mut rng: ThreadRng = rand::thread_rng();
        // let r: u64 = rng.gen_range(0..3);
        let r = 2;
        let move_type: PEMoveType = 
        match r {
            0 => self.get_swap_operands(),
            1 => self.get_invert_chain(),
            2 => {
                // make sure prefix array is updated
                self.num_operators = PolishExpression::get_num_operator(&self.solution);
                self.swap_operand_operator()
            },
            _ => panic!("only 3 options"),
        };
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
        // TODO more efficient
        self.num_operators = PolishExpression::get_num_operator(&self.solution);
    }

    fn current_cost(&self) -> f64 {
        self.current_cost
    }

    fn copy_solution(&self) -> Vec<ModuleNode> {
        self.solution.clone()
    }
}

impl SAMove for PolishExpressionMove {
    fn get_delta_cost(&self) -> f64 {
        self.delta_cost
    }
}
