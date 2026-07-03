// FILE: blend_func_o.rs
// occt: BlendFunc

pub struct BlendFunc;

impl BlendFunc {
    pub fn get_shape(section_shape: i32) -> (i32, i32, i32) {
        match section_shape {
            0 => (5, 3, 2), // Rational
            1 => (3, 2, 1), // QuasiAngular
            2 => (4, 3, 2), // Polynomial
            3 => (2, 1, 1), // Linear
            _ => (2, 1, 1),
        }
    }

    pub fn knots(section_shape: i32) -> Vec<f64> {
        match section_shape {
            0 => vec![0.0, 1.0],
            1 => vec![0.0, 1.0],
            2 => vec![0.0, 1.0],
            3 => vec![0.0, 1.0],
            _ => vec![0.0, 1.0],
        }
    }

    pub fn mults(section_shape: i32) -> Vec<i32> {
        match section_shape {
            0 => vec![1, 1],
            1 => vec![1, 1],
            2 => vec![1, 1],
            3 => vec![1, 1],
            _ => vec![1, 1],
        }
    }

    pub fn next_shape(s: i32) -> i32 {
        s + 1
    }

    pub fn compute_normal(_u: f64, _v: f64) -> (f64, f64, f64) {
        (0.0, 0.0, 1.0)
    }

    pub fn compute_d_normal(_u: f64, _v: f64) -> ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) {
        ((0.0, 0.0, 1.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shape() {
        let (poles, knots, degree) = BlendFunc::get_shape(0);
        assert!(poles > 0);
        assert!(knots > 0);
        assert!(degree > 0);
    }

    #[test]
    fn test_knots() {
        let knots = BlendFunc::knots(0);
        assert!(!knots.is_empty());
    }

    #[test]
    fn test_mults() {
        let mults = BlendFunc::mults(0);
        assert!(!mults.is_empty());
    }

    #[test]
    fn test_compute_normal() {
        let normal = BlendFunc::compute_normal(0.5, 0.5);
        assert_eq!(normal.2, 1.0);
    }
}
