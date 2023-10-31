
pub type Int = usize;
pub type Floorplan = Vec<(usize, usize, Rectangle)>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Rectangle {
    pub width: Int,
    pub height: Int,
}

pub struct BoundingBox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModuleNode {
    H(),
    V(),
    Module(usize), // id of module
}

impl Default for ModuleNode {
    fn default() -> Self {
        ModuleNode::Module(0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Net {
    pub pins: Vec<usize>,
}

impl Rectangle {
    pub fn new(width: Int, height: Int) -> Self {
        Rectangle {width, height}
    }

    pub fn transpose(&self) -> Rectangle {
        Rectangle { width: self.height, height: self.width}
    }

    // b on top of a
    fn combine_h(a: Rectangle, b: Rectangle) -> Rectangle {
        let width = a.width.max(b.width);
        let height = a.height + b.height;
        Rectangle {width,  height}
    }

    // b on right of a
    fn combine_v(a: Rectangle, b: Rectangle) -> Rectangle {
        let width = a.width + b.width;
        let height = a.height.max(b.height);
        Rectangle {width,  height}
    }

    pub fn combine(a: Rectangle, b: Rectangle, v_or_h: ModuleNode) -> Rectangle {
        match v_or_h {
            ModuleNode::V() => Rectangle::combine_v(a, b),
            ModuleNode::H() => Rectangle::combine_h(a, b),
            ModuleNode::Module(_) => panic!("only V and H allowed"),
        }
    }

    pub fn area(&self) -> Int {
        self.width * self.height
    }
    
    // assumes position is at bottom left corner
    pub fn center(&self, pos_x: Int, pos_y: Int) -> (f64, f64) {
        let center_x = pos_x as f64 + (self.width as f64 / 2.0);
        let center_y = pos_y as f64 + (self.height as f64 / 2.0);
        (center_x, center_y)
    }
}

impl BoundingBox {
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        BoundingBox { min_x, max_x, min_y, max_y }
    }

    pub fn extend_point(&mut self, x: f64, y: f64) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
    } 

    pub fn get_width(&self) -> f64 {
        self.max_x - self.min_x
    }

    pub fn get_height(&self) -> f64 {
        self.max_y - self.min_y
    }
}

impl ModuleNode {
    pub fn is_module(&self) -> bool {
        match *self {
            ModuleNode::Module(_) => true,
            _ => false 
        }
    }

    pub fn invert(&self) -> ModuleNode {
        match *self {
            ModuleNode::H() => ModuleNode::V(),
            ModuleNode::V() => ModuleNode::H(),
            _ => panic!("not a H or V node"),
        }
    }
}

impl Net {
    pub fn new(pins: Vec<usize>) -> Self {
        Net{pins}
    }
}