
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rectangle {
    pub width: f64,
    pub heigth: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum ModuleNode {
    H(),
    V(),
    Module(usize), // id of module
}

impl Rectangle {
    pub fn new(width: f64, heigth: f64) -> Self {
        Rectangle {width, heigth}
    }

    pub fn transpose(&self) -> Rectangle {
        Rectangle { width: self.heigth, heigth: self.width}
    }

    // b on top of a
    fn combine_h(a: Rectangle, b: Rectangle) -> Rectangle {
        let width = a.width.max(b.width);
        let heigth = a.heigth + b.heigth;
        Rectangle {width,  heigth}
    }

    // b on right of a
    fn combine_v(a: Rectangle, b: Rectangle) -> Rectangle {
        let width = a.width + b.width;
        let heigth = a.heigth.max(b.heigth);
        Rectangle {width,  heigth}
    }

    pub fn combine(a: Rectangle, b: Rectangle, v_or_h: ModuleNode) -> Rectangle {
        match v_or_h {
            ModuleNode::V() => Rectangle::combine_v(a, b),
            ModuleNode::H() => Rectangle::combine_h(a, b),
            ModuleNode::Module(_) => panic!("only V and H allowed"),
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.heigth
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