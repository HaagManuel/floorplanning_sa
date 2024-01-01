use crate::definitions::*;

#[derive(Debug, Clone, Default)]
pub struct ShapeFunction {
    pub points: Vec<Rectangle>
}

impl ShapeFunction {
    pub fn add(&mut self, rectangle: Rectangle) {
        let h = rectangle.height;
        let w = rectangle.width;
        let mut i = 0;
        let mut found_place = false;
        // filter existing points
        while i < self.points.len()  {
            // is domininated
            if w >= self.points[i].width && h >= self.points[i].height {
                // do not add new rectangle
                return;
            }
            // is better
            else if w <= self.points[i].width && h <= self.points[i].height {
                if !found_place {
                    // overwrite existing element
                    found_place = true;
                    self.points[i] = rectangle;
                }
                else {
                    // swap to end and decrease size
                    self.points.swap_remove(i);
                }
            }
            i += 1
        }
        if !found_place {
            self.points.push(rectangle);
        }
    }

    pub fn combine(a: &ShapeFunction, b: &ShapeFunction, v_or_h: ModuleNode) -> ShapeFunction {
        let mut points: Vec<Rectangle> = Vec::new();
        points.reserve_exact(a.points.len() * b.points.len());
        let mut sf = ShapeFunction{points};
        for i in 0..a.points.len() {
            for j in 0..b.points.len() {
                let r1 = a.points[i];
                let r2 = b.points[j];
                sf.add(Rectangle::combine(r1, r2, v_or_h));
            }
        }
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
        assert_eq!(sf.points, vec![Rectangle::new(1, 1)]);
    }

    #[test]
    fn test_pareto_points_1() {
        let mut sf: ShapeFunction = ShapeFunction::default();
        sf.add(Rectangle::new(1, 1));
        sf.add(Rectangle::new(2, 1));
        sf.add(Rectangle::new(3, 1));
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
        assert_eq!(sf.points, vec![
            Rectangle::new(1, 5),
            Rectangle::new(2, 4),
            Rectangle::new(3, 3),
            Rectangle::new(4, 2),
            Rectangle::new(5, 1),
            ]);
    }
}