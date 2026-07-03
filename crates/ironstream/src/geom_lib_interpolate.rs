// FILE: geom_lib_interpolate.rs
// occt: GeomLib_Interpolate

/// Interpolation utilities for geometric objects.
pub struct GeomLibInterpolate;

impl GeomLibInterpolate {
    /// Perform linear interpolation.
    pub fn linear_interpolate(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
        if (x1 - x0).abs() < 1e-10 {
            return (y0 + y1) / 2.0;
        }
        y0 + (x - x0) * (y1 - y0) / (x1 - x0)
    }

    /// Perform cubic interpolation.
    pub fn cubic_interpolate(
        x: f64,
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        y0p: f64,
        y1p: f64,
    ) -> f64 {
        let h = x1 - x0;
        if h.abs() < 1e-10 {
            return (y0 + y1) / 2.0;
        }
        let t = (x - x0) / h;
        let t2 = t * t;
        let t3 = t2 * t;

        let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
        let h10 = t3 - 2.0 * t2 + t;
        let h01 = -2.0 * t3 + 3.0 * t2;
        let h11 = t3 - t2;

        h00 * y0 + h10 * h * y0p + h01 * y1 + h11 * h * y1p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_interpolate_middle() {
        let y = GeomLibInterpolate::linear_interpolate(0.5, 0.0, 1.0, 0.0, 2.0);
        assert!((y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_linear_interpolate_start() {
        let y = GeomLibInterpolate::linear_interpolate(0.0, 0.0, 1.0, 0.0, 2.0);
        assert!((y - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_linear_interpolate_end() {
        let y = GeomLibInterpolate::linear_interpolate(1.0, 0.0, 1.0, 0.0, 2.0);
        assert!((y - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_cubic_interpolate() {
        let y = GeomLibInterpolate::cubic_interpolate(0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0);
        // Should be close to 0.5 with these parameters
        assert!(y >= 0.0 && y <= 1.0);
    }
}
