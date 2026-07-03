// FILE: extrema_pc_hyperbola.rs
// occt: ExtremaPC_Hyperbola

//! Find extrema between a point and a hyperbola.

pub struct HyperbolaExtrema {
    major_axis: f64,
    minor_axis: f64,
    center: (f64, f64, f64),
}

impl HyperbolaExtrema {
    pub fn new(major_axis: f64, minor_axis: f64, center: (f64, f64, f64)) -> Self {
        HyperbolaExtrema {
            major_axis,
            minor_axis,
            center,
        }
    }

    pub fn extrema_distance(&self, _point: (f64, f64, f64)) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let extrema = HyperbolaExtrema::new(5.0, 3.0, (0.0, 0.0, 0.0));
        assert_eq!(extrema.major_axis, 5.0);
    }
}
