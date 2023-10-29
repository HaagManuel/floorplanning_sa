use std::collections::VecDeque;

use crate::simulated_annealing::{SAInstance, SAMove};
use crate::shape_function::ShapeFunction;
use crate::definitions::*;
use rand::prelude::*;

pub type Floorplan = Vec<(usize, usize, Rectangle, ModuleNode)>;
#[derive(Default)]
pub struct PolishExpression {
    solution: Vec<ModuleNode>,
    modules: Vec<Rectangle>,
    nets: Vec<Net>,
    tree: SlicingTree,
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

#[derive(Debug, Clone, Default)]
struct SlicingTreeNode {
    left: usize,
    right: usize,
    shape: ShapeFunction,
    module_type: ModuleNode,
}

#[derive(Debug, Default)]
struct SlicingTree {
    root: usize,
    nodes: Vec<SlicingTreeNode>,
}

impl SlicingTree {
    fn get_bounding_box(&self) -> Rectangle {
        self.nodes[self.root].shape.points
            .iter()
            .min_by_key(|&&r| r.area())
            .unwrap()
            .clone()
    }
    fn get_min_area(&self) -> Int {
        self.get_bounding_box().area()
    }

    // (origin x, origin y, (width, height), ModuleNode)
    fn get_floorplan(&self) -> Floorplan {
        // same position in nodes and placement array
        let mut placement: Vec<(usize, usize, Rectangle, ModuleNode)> = vec![(0,0, Rectangle::new(0,0), ModuleNode::H()); self.nodes.len()];
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut v: usize = self.root;
        queue.push_back(v);
        placement[v] = (0, 0, self.get_bounding_box(), self.nodes[v].module_type);
        while queue.len() > 0 {
            v = queue.pop_front().unwrap();
            let l = self.nodes[v].left;
            let r = self.nodes[v].right;
            if l == r {
                // leaf, Modulenode
                continue;
            }
            let sf1 = &self.nodes[l].shape;
            let sf2 = &self.nodes[r].shape;
            let module_l = self.nodes[l].module_type;
            let module_r = self.nodes[r].module_type;
            let (x, y, rect, module_type) = placement[v];
            let (r1, r2) = ShapeFunction::reconstruct(sf1, sf2, module_type, &rect).expect("reconstructing rectangle failed.");
            placement[l] = (x, y, r1, module_l);
            match module_type {
                ModuleNode::H() => {
                    placement[r] = (x, y + r1.heigth, r2, module_r);
                }
                ModuleNode::V() => {
                    placement[r] = (x + r1.width, y, r2, module_r);
                }
                ModuleNode::Module(_) => panic!("parent should not be a module")
            }
            queue.push_back(l);
            queue.push_back(r);
        }
        // filter modulenodes
        let plan: Floorplan = placement
            .into_iter()
            .filter(|(_, _, _, module)| module.is_module())
            .collect();
        plan
    }
}

impl PolishExpression {
    pub fn new(modules: Vec<Rectangle>, nets: Vec<Net>) -> Self {
        let mut polish_expression = PolishExpression::default();
        polish_expression.modules = modules;
        polish_expression.nets = nets;
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

    pub fn set_solution(&mut self, solution: Vec<ModuleNode>) {
        self.solution = solution;
        self.tree = self.get_slicing_tree();
        self.current_cost = self.tree.get_min_area() as f64;
        self.num_operators = self.get_num_operator();
    }   

    fn get_slicing_tree(&self) -> SlicingTree {
        let nodes = vec![SlicingTreeNode::default(); self.solution.len()];
        let mut tree = SlicingTree{root: 0, nodes: nodes};
        let mut stack: Vec<(ShapeFunction, usize)> = Vec::new();
        let mut index = 0;
        for module_node in self.solution.iter() {
            match *module_node {
                ModuleNode::Module(id) => {
                    let module: Rectangle = self.modules[id];
                    let sf = ShapeFunction::from_iter([module, module.transpose()]);
                    stack.push((sf, index));
                    tree.nodes[index].module_type = *module_node;
                    index += 1;
                }
                _ => {
                    let (sf1, left) = stack.pop().unwrap();
                    let (sf2, right) = stack.pop().unwrap();
                    let combined: ShapeFunction = ShapeFunction::combine(&sf1, &sf2, *module_node);
                    stack.push((combined, index));
                    tree.nodes[left].shape = sf1;
                    tree.nodes[right].shape = sf2;
                    tree.nodes[index].left = left;
                    tree.nodes[index].right = right;
                    tree.nodes[index].module_type = *module_node;

                    index += 1;
                }
            }
        }
        let (sf, root) = stack.pop().unwrap();
        debug_assert!(sf.points.len() > 0);
        tree.root = root;
        tree.nodes[root].shape = sf;
        tree
    }

    pub fn get_floorplan(&self) -> Floorplan {
        self.get_slicing_tree().get_floorplan()
    }

    pub fn get_dead_area(&self) -> f64 {
        let occupied_area: usize = self.modules
            .iter()
            .map(|rect| rect.area())
            .sum();
        let total_area = self.get_slicing_tree().get_min_area();
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

    pub fn eval_expression(&self) -> f64 {
        let tree = self.get_slicing_tree();
        tree.get_min_area() as f64
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

    fn swap_operand_operator(&self) -> PEMoveType {
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
        debug_assert!(pos.len() > 0);
        let i = rng.gen_range(0..pos.len());
        PEMoveType::SwapOperandOperator(pos[i], pos[i] + 1)
        
    }
}

impl SAInstance<PolishExpressionMove, Vec<ModuleNode>> for PolishExpression {
    fn get_move(&mut self) -> PolishExpressionMove {
        let mut rng: ThreadRng = rand::thread_rng();
        let r: u64 = rng.gen_range(0..3);
        // let r: u64 = rng.gen_range(0..5);
        let r: u64 = 3;
        let move_type: PEMoveType = 
        match r {
            0 => self.get_swap_adjacent_operands(),
            1 => self.get_invert_chain(),
            _ => {
                // make sure prefix array is updated
                self.num_operators = self.get_num_operator();
                self.swap_operand_operator()
            },
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
        self.num_operators = self.get_num_operator();
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
