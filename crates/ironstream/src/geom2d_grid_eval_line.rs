// FILE: geom2d_grid_eval_line.rs
// occt: Geom2dGridEval_Line

/// Efficient batch evaluator for 2D line grid points.
/// Line: P(u) = Location + u * Direction
pub struct Geom2dGridEvalLine {
    loc_x: f64,
    loc_y: f64,
    dir_x: f64,
    dir_y: f64,
}

impl Geom2dGridEvalLine {
    pub fn new(loc_x: f64, loc_y: f64, dir_x: f64, dir_y: f64) -> Self {
        // Normalize direction
        let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
        let (dx, dy) = if len > 1e-15 {
            (dir_x / len, dir_y / len)
        } else {
            (1.0, 0.0)
        };

        Geom2dGridEvalLine {
            loc_x,
            loc_y,
            dir_x: dx,
            dir_y: dy,
        }
    }

    pub fn evaluate(&self, u: f64) -> (f64, f64) {
        (
            self.loc_x + u * self.dir_x,
            self.loc_y + u * self.dir_y,
        )
    }

    pub fn evaluate_d1(&self, u: f64) -> ((f64, f64), (f64, f64)) {
        (
            (self.loc_x + u * self.dir_x, self.loc_y + u * self.dir_y),
            (self.dir_x, self.dir_y),
        )
    }

    pub fn evaluate_d2(&self, u: f64) -> ((f64, f64), (f64, f64), (f64, f64)) {
        (
            (self.loc_x + u * self.dir_x, self.loc_y + u * self.dir_y),
            (self.dir_x, self.dir_y),
            (0.0, 0.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRECISION: f64 = 1e-10;

    #[test]
    fn test_line_at_0() {
        let line = Geom2dGridEvalLine::new(1.0, 2.0, 1.0, 0.0);
        let (x, y) = line.evaluate(0.0);
        assert!((x - 1.0).abs() < PRECISION);
        assert!((y - 2.0).abs() < PRECISION);
    }

    #[test]
    fn test_line_at_5() {
        let line = Geom2dGridEvalLine::new(0.0, 0.0, 1.0, 0.0);
        let (x, y) = line.evaluate(5.0);
        assert!((x - 5.0).abs() < PRECISION);
        assert!(y.abs() < PRECISION);
    }

    #[test]
    fn test_line_d1() {
        let line = Geom2dGridEvalLine::new(0.0, 0.0, 1.0, 1.0);
        let ((_x, _y), (dx, dy)) = line.evaluate_d1(0.0);
        let len = (dx * dx + dy * dy).sqrt();
        assert!((len - 1.0).abs() < PRECISION);
    }

    #[test]
    fn test_line_d2() {
        let line = Geom2dGridEvalLine::new(0.0, 0.0, 3.0, 4.0);
        let ((_x, _y), (_dx, _dy), (d2x, d2y)) = line.evaluate_d2(10.0);
        assert!(d2x.abs() < PRECISION);
        assert!(d2y.abs() < PRECISION);
    }
}
