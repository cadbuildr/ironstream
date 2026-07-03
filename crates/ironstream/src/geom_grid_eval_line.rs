// FILE: geom_grid_eval_line.rs
// occt: GeomGridEval_Line

//! Evaluates a line segment on a parameter grid.

pub struct LineEval {
    start: (f64, f64, f64),
    end: (f64, f64, f64),
}

impl LineEval {
    pub fn new(start: (f64, f64, f64), end: (f64, f64, f64)) -> Self {
        LineEval { start, end }
    }

    pub fn evaluate(&self, parameter: f64) -> (f64, f64, f64) {
        let t = parameter.clamp(0.0, 1.0);
        (
            self.start.0 + t * (self.end.0 - self.start.0),
            self.start.1 + t * (self.end.1 - self.start.1),
            self.start.2 + t * (self.end.2 - self.start.2),
        )
    }

    pub fn first_parameter(&self) -> f64 {
        0.0
    }

    pub fn last_parameter(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_start() {
        let line = LineEval::new((0.0, 0.0, 0.0), (10.0, 0.0, 0.0));
        let (x, y, z) = line.evaluate(0.0);
        assert_eq!((x, y, z), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_line_end() {
        let line = LineEval::new((0.0, 0.0, 0.0), (10.0, 0.0, 0.0));
        let (x, y, z) = line.evaluate(1.0);
        assert_eq!((x, y, z), (10.0, 0.0, 0.0));
    }

    #[test]
    fn test_line_midpoint() {
        let line = LineEval::new((0.0, 0.0, 0.0), (10.0, 0.0, 0.0));
        let (x, y, z) = line.evaluate(0.5);
        assert_eq!((x, y, z), (5.0, 0.0, 0.0));
    }

    #[test]
    fn test_line_3d() {
        let line = LineEval::new((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));
        let (x, y, z) = line.evaluate(0.5);
        assert_eq!((x, y, z), (2.5, 3.5, 4.5));
    }

    #[test]
    fn test_parameters() {
        let line = LineEval::new((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        assert_eq!(line.first_parameter(), 0.0);
        assert_eq!(line.last_parameter(), 1.0);
    }
}
