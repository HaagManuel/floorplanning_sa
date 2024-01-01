use crate::definitions::*;

#[derive(Debug, Clone, Default)]
pub struct ShapeFunction {
    pub points: Vec<Rectangle>
}

impl ShapeFunction {
    pub fn add(&mut self, rectangle: Rectangle) {
        self.points.push(rectangle);
    }

    pub fn filter_pareto_points(&mut self) {
        debug_assert!(self.points.len() > 0);
        self.points.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut new_vec: Vec<Rectangle> = Vec::new(); // TODO may optimize
        let mut lowest_height = self.points[0].height;
        new_vec.push(self.points[0]);
        for i in 1..self.points.len() {
            let last_rect = self.points[i - 1];
            let rect = self.points[i];
            if last_rect.width < rect.width && rect.height < lowest_height{ 
                new_vec.push(rect);
                lowest_height = rect.height;
            }
        }
        self.points = new_vec;
    }

    pub fn combine(a: &ShapeFunction, b: &ShapeFunction, v_or_h: ModuleNode) -> ShapeFunction {
        let mut points: Vec<Rectangle> = Vec::new();
        points.reserve_exact(a.points.len() * b.points.len());
        for i in 0..a.points.len() {
            for j in 0..b.points.len() {
                let r1 = a.points[i];
                let r2 = b.points[j];
                points.push(Rectangle::combine(r1, r2, v_or_h));
            }
        }
        let mut sf: ShapeFunction = ShapeFunction{points};
        sf.filter_pareto_points();
        sf
    }

    pub fn reconstruct(a: &ShapeFunction, b: &ShapeFunction, v_or_h: ModuleNode, rect: &Rectangle) -> Option<(Rectangle, Rectangle)> {
        for i in 0..a.points.len() {
            for j in 0..b.points.len() {
                let r1 = a.points[i];
                let r2 = b.points[j];
                let combined = Rectangle::combine(r1, r2, v_or_h);
                if combined == *rect {
                    return Some((r1, r2));
                }
            }
        }
        None
    }
}

impl FromIterator<Rectangle> for ShapeFunction {
    fn from_iter<I: IntoIterator<Item=Rectangle>>(iter: I) -> Self {
        let mut sf = ShapeFunction::default();
        for rect in iter {
            sf.add(rect);
        }
        sf
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicates() {
        let mut sf: ShapeFunction = ShapeFunction::default();
        sf.add(Rectangle::new(1, 1));
        sf.add(Rectangle::new(1, 1));
        sf.add(Rectangle::new(1, 1));
        sf.filter_pareto_points();
        assert_eq!(sf.points, vec![Rectangle::new(1, 1)]);
    }

    #[test]
    fn test_pareto_points_1() {
        let mut sf: ShapeFunction = ShapeFunction::default();
        sf.add(Rectangle::new(1, 1));
        sf.add(Rectangle::new(2, 1));
        sf.add(Rectangle::new(3, 1));
        sf.filter_pareto_points();
        assert_eq!(sf.points, vec![Rectangle::new(1, 1)]);
    }

    #[test]
    fn test_pareto_points_2() {
        let mut sf: ShapeFunction = ShapeFunction::default();
        sf.add(Rectangle::new(1, 5));
        sf.add(Rectangle::new(2, 4));
        sf.add(Rectangle::new(3, 3));
        sf.add(Rectangle::new(4, 2));
        sf.add(Rectangle::new(5, 1));

        sf.add(Rectangle::new(2, 5));
        sf.add(Rectangle::new(3, 4));
        sf.add(Rectangle::new(4, 3));
        sf.add(Rectangle::new(5, 2));
        sf.add(Rectangle::new(6, 1));

        sf.add(Rectangle::new(1, 6));
        sf.add(Rectangle::new(2, 5));
        sf.add(Rectangle::new(3, 4));
        sf.add(Rectangle::new(4, 3));
        sf.add(Rectangle::new(5, 2));
        sf.filter_pareto_points();
        assert_eq!(sf.points, vec![
            Rectangle::new(1, 5),
            Rectangle::new(2, 4),
            Rectangle::new(3, 3),
            Rectangle::new(4, 2),
            Rectangle::new(5, 1),
            ]);
    }
}