use crate::rectangle::*;

#[derive(Debug)]
pub struct ShapeFunction {
    pub points: Vec<Rectangle>
}

impl ShapeFunction {
    pub fn new() -> Self {
        ShapeFunction {points: Vec::new()}
    }

    pub fn add(&mut self, rectangle: Rectangle) {
        self.points.push(rectangle);
    }

    pub fn filter_pareto_points(&mut self) {
        debug_assert!(self.points.len() > 0);
        self.points.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut new_vec: Vec<Rectangle> = Vec::new(); // TODO may optimize
        let mut lowest_heigth = self.points[0].heigth;
        new_vec.push(self.points[0]);
        for i in 1..self.points.len() {
            let last_rect = self.points[i - 1];
            let rect = self.points[i];
            if last_rect.width < rect.width && rect.heigth < lowest_heigth{ 
                new_vec.push(rect);
                lowest_heigth = rect.heigth;
            }
        }
        self.points = new_vec;
    }

    pub fn concat(a: &ShapeFunction, b: &ShapeFunction) -> ShapeFunction {
        let mut sf: ShapeFunction = ShapeFunction::new();
        sf.points.extend(a.points.iter());
        sf.points.extend(b.points.iter());
        sf.filter_pareto_points();
        sf
    }

    pub fn combine(a: &ShapeFunction, b: &ShapeFunction, v_or_h: ModuleNode) -> ShapeFunction {
        let mut sf: ShapeFunction = ShapeFunction::new();
        for i in 0..a.points.len() {
            for j in 0..b.points.len() {
                let r1 = a.points[i];
                let r2 = b.points[j];
                sf.add(Rectangle::combine(r1, r2, v_or_h));
            }
        }
        sf.filter_pareto_points();
        sf
    }
}

impl FromIterator<Rectangle> for ShapeFunction {
    fn from_iter<I: IntoIterator<Item=Rectangle>>(iter: I) -> Self {
        let mut sf = ShapeFunction::new();
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
        let mut sf: ShapeFunction = ShapeFunction::new();
        sf.add(Rectangle::new(1.0, 1.0));
        sf.add(Rectangle::new(1.0, 1.0));
        sf.add(Rectangle::new(1.0, 1.0));
        sf.filter_pareto_points();
        assert_eq!(sf.points, vec![Rectangle::new(1.0, 1.0)]);
    }

    #[test]
    fn test_pareto_points_1() {
        let mut sf: ShapeFunction = ShapeFunction::new();
        sf.add(Rectangle::new(1.0, 1.0));
        sf.add(Rectangle::new(2.0, 1.0));
        sf.add(Rectangle::new(3.0, 1.0));
        sf.filter_pareto_points();
        assert_eq!(sf.points, vec![Rectangle::new(1.0, 1.0)]);
    }

    #[test]
    fn test_pareto_points_2() {
        let mut sf: ShapeFunction = ShapeFunction::new();
        sf.add(Rectangle::new(1.0, 5.0));
        sf.add(Rectangle::new(2.0, 4.0));
        sf.add(Rectangle::new(3.0, 3.0));
        sf.add(Rectangle::new(4.0, 2.0));
        sf.add(Rectangle::new(5.0, 1.0));

        sf.add(Rectangle::new(2.0, 5.0));
        sf.add(Rectangle::new(3.0, 4.0));
        sf.add(Rectangle::new(4.0, 3.0));
        sf.add(Rectangle::new(5.0, 2.0));
        sf.add(Rectangle::new(6.0, 1.0));

        sf.add(Rectangle::new(1.0, 6.0));
        sf.add(Rectangle::new(2.0, 5.0));
        sf.add(Rectangle::new(3.0, 4.0));
        sf.add(Rectangle::new(4.0, 3.0));
        sf.add(Rectangle::new(5.0, 2.0));
        sf.filter_pareto_points();
        assert_eq!(sf.points, vec![
            Rectangle::new(1.0, 5.0),
            Rectangle::new(2.0, 4.0),
            Rectangle::new(3.0, 3.0),
            Rectangle::new(4.0, 2.0),
            Rectangle::new(5.0, 1.0),
            ]);
    }
}