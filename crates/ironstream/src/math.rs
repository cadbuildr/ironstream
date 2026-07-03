// FILE: math.rs
// occt: math

//! Mathematical solvers and utilities.

pub const EPSILON: f64 = 1e-15;
pub const PI: f64 = 3.14159265358979323846;
pub const SQRT_2: f64 = 1.41421356237309504880;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(EPSILON > 0.0);
        assert!(PI > 3.0);
        assert!(SQRT_2 > 1.4);
    }
}
