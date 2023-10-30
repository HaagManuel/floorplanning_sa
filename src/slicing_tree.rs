use crate::shape_function::ShapeFunction;
use crate::definitions::*;
use std::collections::VecDeque;

pub type Floorplan = Vec<(usize, usize, Rectangle, ModuleNode)>;

#[derive(Debug, Clone, Default)]
struct SlicingTreeNode {
    left: usize,
    right: usize,
    shape: ShapeFunction,
    module_type: ModuleNode,
}

#[derive(Debug, Default)]
pub struct SlicingTree {
    root: usize,
    nodes: Vec<SlicingTreeNode>,
}

impl SlicingTree {
    pub fn get_bounding_box(&self) -> Rectangle {
        self.nodes[self.root].shape.points
            .iter()
            .min_by_key(|&&r| r.area())
            .unwrap()
            .clone()
    }
    pub fn get_min_area(&self) -> Int {
        self.get_bounding_box().area()
    }

    pub fn get_slicing_tree(solution: &Vec<ModuleNode>, modules: &Vec<Rectangle>) -> SlicingTree {
        let nodes = vec![SlicingTreeNode::default(); solution.len()];
        let mut tree = SlicingTree{root: 0, nodes: nodes};
        let mut stack: Vec<usize> = Vec::new();
        let mut index = 0;
        for module_node in solution.iter() {
            match *module_node {
                ModuleNode::Module(id) => {
                    let module: Rectangle = modules[id];
                    let sf = ShapeFunction::from_iter([module, module.transpose()]);
                    stack.push(index);
                    tree.nodes[index].module_type = *module_node;
                    tree.nodes[index].shape = sf;
                    index += 1;
                }
                _ => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let sf1: &ShapeFunction = &tree.nodes[left].shape;
                    let sf2: &ShapeFunction = &tree.nodes[right].shape;
                    let combined: ShapeFunction = ShapeFunction::combine(sf1, sf2, *module_node);
                    tree.nodes[index].left = left;
                    tree.nodes[index].right = right;
                    tree.nodes[index].module_type = *module_node;
                    tree.nodes[index].shape = combined;
                    stack.push(index);
                    index += 1;
                }
            }
        }
        let root = stack.pop().unwrap();
        debug_assert!(tree.nodes[root].shape.points.len() > 0);
        tree.root = root;
        tree
    }

    // (origin x, origin y, (width, height), ModuleNode)
    pub fn get_floorplan(&self) -> Floorplan {
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