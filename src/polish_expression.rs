use core::num;
use std::cmp::Ordering;

use crate::simulated_annealing::{SAInstance, SAMove};
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    heigth: f64,
}
pub struct PolishExpression {
    solution: Vec<ModuleNode>,
    modules: Vec<Rectangle>,
    num_operators: Vec<usize>, // to check if op3 is legal
    current_cost: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum ModuleNode {
    H(),
    V(),
    Module(usize), // id of module
}

impl ModuleNode {
    fn is_module(&self) -> bool {
        match *self {
            ModuleNode::Module(i) => true,
            _ => false 
        }
    }

    fn invert(&self) -> ModuleNode {
        match *self {
            ModuleNode::H() => ModuleNode::V(),
            ModuleNode::V() => ModuleNode::H(),
            _ => panic!("not a H or V node"),
        }
    }

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

impl Rectangle {
    pub fn new(width: f64, heigth: f64) -> Self {
        Rectangle {width, heigth}
    }

    fn transpose(&self) -> Rectangle {
        Rectangle { width: self.heigth, heigth: self.width}
    }

    // b on top of a
    fn combine_h(a: &Rectangle, b: &Rectangle) -> Rectangle {
        let width = a.width.max(b.width);
        let heigth = a.heigth + b.heigth;
        Rectangle {width,  heigth}
    }

    // b on right of a
    fn combine_v(a: &Rectangle, b: &Rectangle) -> Rectangle {
        let width = a.width + b.width;
        let heigth = a.heigth.max(b.heigth);
        Rectangle {width,  heigth}
    }

    fn combine(a: &Rectangle, b: &Rectangle, v_or_h: ModuleNode) -> Rectangle {
        match v_or_h {
            ModuleNode::V() => Rectangle::combine_v(a, b),
            ModuleNode::H() => Rectangle::combine_h(a, b),
            ModuleNode::Module(_) => panic!("only V and H allowed"),
        }
    }

    fn combine_min_area(a: &Rectangle, b: &Rectangle, v_or_h: ModuleNode) -> Rectangle {
        let a2: Rectangle = a.transpose();
        let b2: Rectangle = b.transpose();
        let r1: Rectangle = Rectangle::combine(a, b, v_or_h);
        let r2: Rectangle = Rectangle::combine(&a2, b, v_or_h);
        let r3: Rectangle = Rectangle::combine(a, &b2, v_or_h);
        let r4: Rectangle = Rectangle::combine(&a2, &b2, v_or_h);
        // TODO nicer, remove redundancy, which of the two rotated?
        let vec = vec![r1.area(), r2.area(), r3.area(), r4.area()];
        let vec_rec = vec![r1, r2, r3, r4];
        let i = vec 
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();
        return vec_rec[0].clone();
        // return vec_rec[i].clone();
    }

    fn area(&self) -> f64 {
        self.width * self.heigth
    }
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
        // let solution: Vec<ModuleNode> = vec![
        //     ModuleNode::Module(0),
        //     ModuleNode::Module(1),
        //     ModuleNode::H(),
        //     ModuleNode::Module(2),
        //     ModuleNode::Module(3),
        //     ModuleNode::H(),
        //     ModuleNode::Module(4),
        //     ModuleNode::H(),
        //     ModuleNode::V()
        //     ];
        // -> put into test?
        let current_cost = PolishExpression::eval(&solution, &modules);
        let num_operators: Vec<usize> = PolishExpression::get_num_operator(&solution);
        PolishExpression {solution, modules, current_cost, num_operators}
    }

    pub fn eval(solution: &Vec<ModuleNode>, modules: &Vec<Rectangle>,
    ) -> f64{
        let mut stack: Vec<Rectangle> = Vec::new();
        for module_node in solution {
            match *module_node {
                ModuleNode::Module(id) => {
                    let module: Rectangle = modules[id].clone();
                    stack.push(module);
                }
                _ => {
                    let a: Rectangle = stack.pop().unwrap();
                    let b: Rectangle = stack.pop().unwrap();
                    // let combined: Rectangle = Rectangle::combine(&a, &b, *module_node);
                    let combined: Rectangle = Rectangle::combine_min_area(&a, &b, *module_node);
                    stack.push(combined);
                }
            }
        }
        // for now only area, latter binary tree to compute wirelength
        // last one is bounding box
        let last: Rectangle = stack.pop().unwrap();
        last.area()
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

    pub fn eval_pe(&self) -> f64 {
        PolishExpression::eval(&self.solution, &self.modules)
    }

    fn get_swap_operands(&self) -> PEMoveType {
        let mut rng: ThreadRng = rand::thread_rng();
        let m = self.solution.len();
        loop {
            // TODO later only adjacent operands, i.e no operand between them!
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
        if !self.solution[0].is_module() {
            pos.push(0);
        }
        for i in 1..m {
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
        let r: u64 = rng.gen_range(0..3);
        let move_type: PEMoveType = 
        match r {
            0 => self.get_swap_operands(),
            1 => self.get_invert_chain(),
            2 => self.swap_operand_operator(),
            _ => panic!("only 3 options"),
        };
        let old: f64 = self.eval_pe();
        move_type.apply(&mut self.solution);
        let new: f64 = self.eval_pe();
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
