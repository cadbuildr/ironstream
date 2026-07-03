// FILE: n_collection_lerp.rs
// occt: NCollection_Lerp

/// Simple linear interpolation tool (LERP).
/// Implements linear interpolation between two values for any type supporting
/// arithmetic operations.
pub struct Lerp<T> {
    start: T,
    end: T,
}

impl<T> Lerp<T>
where
    T: Copy,
    f64: From<T>,
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<f64, Output = T>,
{
    /// Create a new Lerp with start and end values.
    pub fn new(start: T, end: T) -> Self {
        Lerp { start, end }
    }

    /// Interpolate between start and end values.
    /// t=0 returns start, t=1 returns end, values in [0,1] interpolate linearly.
    pub fn interpolate(&self, t: f64) -> T {
        let one_minus_t = 1.0 - t;
        (self.start * one_minus_t) + (self.end * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_integer() {
        let lerp = Lerp::new(0.0, 100.0);
        let result = lerp.interpolate(0.5);
        assert!((result - 50.0).abs() < 0.0001);
    }

    #[test]
    fn test_lerp_at_boundaries() {
        let lerp = Lerp::new(10.0, 20.0);
        assert_eq!(lerp.interpolate(0.0), 10.0);
        assert_eq!(lerp.interpolate(1.0), 20.0);
    }

    #[test]
    fn test_lerp_float() {
        let lerp = Lerp::new(0.0, 1.0);
        let result = lerp.interpolate(0.25);
        assert!((result - 0.25).abs() < 0.0001);
    }
}
