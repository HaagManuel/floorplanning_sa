use crate::shape_function::ShapeFunction;
use crate::definitions::*;

const NO_PARENT: usize = usize::MAX;

#[derive(Debug)]
pub enum ModuleShape {
    Hard(),
    Rotatable(),
    AspectRatios(usize),
}

impl From<String> for ModuleShape {
    fn from(str: String) -> Self {
        if str == "hard" {
            ModuleShape::Hard()
        }
        else if str == "rotatable" {
            ModuleShape::Rotatable()
        }
        else if str == "aspect_ratios" {
            ModuleShape::AspectRatios(1)
        }
        else {
            panic!("unsupported module_shape type")
        }
    }
}
impl ModuleShape {
    pub fn set_min_module_length(&mut self, min_length: usize) {
        if let ModuleShape::AspectRatios(_) = self {
            *self = ModuleShape::AspectRatios(min_length);
        } 
    }
}

impl Default for ModuleShape {
    fn default() -> Self {
        Self::Rotatable()
    }
}

#[derive(Debug, Clone, Default)]
struct SlicingTreeNode {
    left: usize,
    right: usize,
    parent: usize,
    shape: ShapeFunction,
    module_type: ModuleNode,
}

#[derive(Debug, Default)]
pub struct SlicingTree {
    root: usize,
    nodes: Vec<SlicingTreeNode>,
    node_placement: Vec<(usize, usize, Rectangle, ModuleNode)>,
    pub placement: Floorplan,
    stack: Vec<usize>,
    update: Vec<bool>,
    module_shape: ModuleShape,
}

impl SlicingTree {
    pub fn new(num_modules: usize) -> Self {
        let root = 0;
        let num_nodes = 2 * num_modules - 1;
        let nodes = vec![SlicingTreeNode::default(); num_nodes];
        let node_placement = vec![(0,0, Rectangle::new(0,0), ModuleNode::H()); num_nodes];
        let placement = vec![(0,0, Rectangle::new(0,0)); num_modules];
        let stack = Vec::new();
        let update = vec![true; num_nodes];
        let module_shape = ModuleShape::Rotatable();
        SlicingTree{root, nodes, node_placement, placement, stack, update, module_shape}
    }

    pub fn set_module_shape(&mut self, module_shape: ModuleShape) {
        self.module_shape = module_shape;
    }

    pub fn get_module_shape_function(&self, module: Rectangle) -> ShapeFunction{
        match self.module_shape {
            ModuleShape::Hard() => {
                return ShapeFunction::from_iter([module]);
            },
            ModuleShape::Rotatable() => {
                return ShapeFunction::from_iter([module, module.transpose()]);
            },
            ModuleShape::AspectRatios(min_length) => {
                let area = module.area();
                let mut shapes: Vec<Rectangle> = Vec::new();
                let bound = f64::sqrt(area as f64) as usize + 1;
                for a in min_length..bound {
                    let b = area.div_ceil(a);
                    if a * b == area {
                        shapes.push(Rectangle::new(a, b));
                        shapes.push(Rectangle::new(b, a));
                    }
                }
                if shapes.len() == 0{
                    shapes.push(module);
                    shapes.push(module.transpose());
                }
                let shape_function = ShapeFunction{points: shapes};
                return shape_function;
            },
        }
    }

    pub fn recompute(&mut self, solution: &Vec<ModuleNode>, modules: &Vec<Rectangle>) {
        let mut index = 0;
        for module_node in solution.iter() {
            match *module_node {
                ModuleNode::Module(id) => {
                    self.stack.push(index);
                    if !self.update[index] {
                        index += 1;
                        continue;
                    }
                    let module: Rectangle = modules[id];
                    let sf: ShapeFunction = self.get_module_shape_function(module);
                    self.nodes[index].module_type = *module_node;
                    self.nodes[index].shape = sf;
                    self.nodes[index].left = 0;
                    self.nodes[index].right = 0;
                    index += 1;
                }
                _ => {
                    let right = self.stack.pop().unwrap();
                    let left  = self.stack.pop().unwrap();
                    self.stack.push(index);
                    if !self.update[index] {
                        index += 1;
                        continue;
                    }
                    self.nodes[left].parent = index;
                    self.nodes[right].parent = index;
                    
                    let sf1: &ShapeFunction = &self.nodes[left].shape;
                    let sf2: &ShapeFunction = &self.nodes[right].shape;
                    let combined: ShapeFunction = ShapeFunction::combine(sf1, sf2, *module_node);
                    self.nodes[index].left = left;
                    self.nodes[index].right = right;
                    self.nodes[index].module_type = *module_node;
                    self.nodes[index].shape = combined;
                    index += 1;
                }
            }
        }
        self.update.fill(false);
        let root: usize = self.stack.pop().unwrap();
        self.nodes[root].parent = NO_PARENT;
        debug_assert!(self.nodes[root].shape.points.len() > 0);
        self.root = root;
    }
    
    // (origin x, origin y, (width, height), ModuleNode)
    pub fn recompute_floorplan(&mut self) {
        let mut v: usize = self.root;
        self.stack.push(v);
        self.node_placement[v] = (0, 0, self.get_bounding_box(), self.nodes[v].module_type);
        while self.stack.len() > 0 {
            v = self.stack.pop().unwrap();
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
            let (x, y, rect, module_type) = self.node_placement[v];
            let (r1, r2) = ShapeFunction::reconstruct(sf1, sf2, module_type, &rect).expect("reconstructing rectangle failed.");
            self.node_placement[l] = (x, y, r1, module_l);
            match module_type {
                ModuleNode::H() => {
                    self.node_placement[r] = (x, y + r1.height, r2, module_r);
                }
                ModuleNode::V() => {
                    self.node_placement[r] = (x + r1.width, y, r2, module_r);
                }
                ModuleNode::Module(_) => panic!("parent should not be a module")
            }
            self.stack.push(l);
            self.stack.push(r);
        }
        // filter modulenodes
        for (x, y, rect, module) in self.node_placement.iter() {
            match *module {
                ModuleNode::Module(i) => {
                    self.placement[i] = (*x, *y, *rect);
                }
                _ => (),
            }
        }
    }

    pub fn get_bounding_box(&self) -> Rectangle {
        self.nodes[self.root].shape.points
            .iter()
            .min_by_key(|&&r| r.area())
            .unwrap()
            .clone()
    }
    pub fn get_min_area(&self) -> f64 {
        self.get_bounding_box().area() as f64
    }

    pub fn mark_path(&mut self, v: usize) {
        let mut w = v;
        while w != NO_PARENT {
            self.update[w] = true;
            w = self.nodes[w].parent;
        }
    }
    
    pub fn update_everything(&mut self) {
        self.update.fill(true);
    }

    pub fn update_swap_leafs(&mut self, left: usize, right: usize) {
        let parent1 = self.nodes[left].parent;
        let parent2 = self.nodes[right].parent;
        if parent1 == parent2 {
            self.mark_path(parent1);
        }
        else {
            self.mark_path(parent1);
            self.mark_path(parent2);
        }
        self.update[left] = true;
        self.update[right] = true;
    }

    pub fn update_invert_chain(&mut self, v: usize) {
        self.mark_path(v);
    }

    pub fn update_swap_operand_operator(&mut self, left: usize, right: usize) {
        self.mark_path(left);
        self.mark_path(right);
    }
    
    pub fn sanity_check(&self, polish_expression: &Vec<ModuleNode>) -> bool{
        if self.nodes.len() != polish_expression.len() {
            return false;
        }
        for i in 0..self.nodes.len() {
            if self.nodes[i].module_type != polish_expression[i] {
                dbg!(self.nodes[i].module_type);
                dbg!(polish_expression[i]);
                return false;
            }
        }
        return true;
    }
}