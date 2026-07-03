// FILE: geom_grid_eval_circle.rs
// occt: GeomGridEval_Circle

//! Evaluates a circle on a parameter grid.

use std::f64::consts::PI;

pub struct CircleEval {
    radius: f64,
}

impl CircleEval {
    pub fn new(radius: f64) -> Self {
        CircleEval { radius }
    }

    pub fn evaluate(&self, parameter: f64) -> (f64, f64, f64) {
        let x = self.radius * parameter.cos();
        let y = self.radius * parameter.sin();
        (x, y, 0.0)
    }

    pub fn first_parameter(&self) -> f64 {
        0.0
    }

    pub fn last_parameter(&self) -> f64 {
        2.0 * PI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_evaluation() {
        let circle = CircleEval::new(1.0);
        let (x, y, z) = circle.evaluate(0.0);
        assert!((x - 1.0).abs() < 1e-10);
        assert!(y.abs() < 1e-10);
        assert_eq!(z, 0.0);
    }

    #[test]
    fn test_circle_quarter() {
        let circle = CircleEval::new(1.0);
        let (x, y, _) = circle.evaluate(PI / 2.0);
        assert!(x.abs() < 1e-10);
        assert!((y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_circle_radius() {
        let circle = CircleEval::new(5.0);
        let (x, y, _) = circle.evaluate(0.0);
        assert!((x - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_parameters() {
        let circle = CircleEval::new(1.0);
        assert_eq!(circle.first_parameter(), 0.0);
        assert!((circle.last_parameter() - 2.0 * PI).abs() < 1e-10);
    }
}
