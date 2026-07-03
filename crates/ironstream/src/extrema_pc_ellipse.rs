// FILE: extrema_pc_ellipse.rs
// occt: ExtremaPC_Ellipse

//! Find extrema between a point and an ellipse.

pub struct EllipseExtrema {
    major_radius: f64,
    minor_radius: f64,
    center: (f64, f64, f64),
}

impl EllipseExtrema {
    pub fn new(major_radius: f64, minor_radius: f64, center: (f64, f64, f64)) -> Self {
        EllipseExtrema {
            major_radius,
            minor_radius,
            center,
        }
    }

    pub fn extrema_parameter(&self, _point: (f64, f64, f64)) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let extrema = EllipseExtrema::new(5.0, 3.0, (0.0, 0.0, 0.0));
        assert_eq!(extrema.major_radius, 5.0);
        assert_eq!(extrema.minor_radius, 3.0);
    }
}
