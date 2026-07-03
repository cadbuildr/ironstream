// FILE: extrema_pc_circle.rs
// occt: ExtremaPC_Circle

//! Find extrema between a point and a circle.

use std::f64::consts::PI;

pub struct CircleExtrema {
    radius: f64,
    center: (f64, f64, f64),
}

impl CircleExtrema {
    pub fn new(radius: f64, center: (f64, f64, f64)) -> Self {
        CircleExtrema { radius, center }
    }

    pub fn extrema_distance(&self, point: (f64, f64, f64)) -> (f64, f64) {
        let dx = point.0 - self.center.0;
        let dy = point.1 - self.center.1;
        let dist_to_center = (dx * dx + dy * dy).sqrt();
        let dist_to_circle = (dist_to_center - self.radius).abs();
        (dist_to_circle, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let extrema = CircleExtrema::new(1.0, (0.0, 0.0, 0.0));
        assert_eq!(extrema.radius, 1.0);
    }

    #[test]
    fn test_extrema_at_center() {
        let extrema = CircleExtrema::new(1.0, (0.0, 0.0, 0.0));
        let (dist, _) = extrema.extrema_distance((0.0, 0.0, 0.0));
        assert_eq!(dist, 1.0);
    }
}
