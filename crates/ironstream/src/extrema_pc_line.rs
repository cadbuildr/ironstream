// FILE: extrema_pc_line.rs
// occt: ExtremaPC_Line

//! Find extrema between a point and a line.

pub struct LineExtrema {
    start: (f64, f64, f64),
    end: (f64, f64, f64),
}

impl LineExtrema {
    pub fn new(start: (f64, f64, f64), end: (f64, f64, f64)) -> Self {
        LineExtrema { start, end }
    }

    pub fn extrema_parameter(&self, point: (f64, f64, f64)) -> f64 {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        let dz = self.end.2 - self.start.2;
        let len_sq = dx * dx + dy * dy + dz * dz;

        if len_sq < 1e-12 {
            0.0
        } else {
            let t = ((point.0 - self.start.0) * dx
                + (point.1 - self.start.1) * dy
                + (point.2 - self.start.2) * dz)
                / len_sq;
            t.clamp(0.0, 1.0)
        }
    }

    pub fn distance_to_point(&self, point: (f64, f64, f64)) -> f64 {
        let t = self.extrema_parameter(point);
        let closest = (
            self.start.0 + t * (self.end.0 - self.start.0),
            self.start.1 + t * (self.end.1 - self.start.1),
            self.start.2 + t * (self.end.2 - self.start.2),
        );
        let dx = point.0 - closest.0;
        let dy = point.1 - closest.1;
        let dz = point.2 - closest.2;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let extrema = LineExtrema::new((0.0, 0.0, 0.0), (1.0, 0.0, 0.0));
        assert_eq!(extrema.start, (0.0, 0.0, 0.0));
        assert_eq!(extrema.end, (1.0, 0.0, 0.0));
    }

    #[test]
    fn test_extrema_parameter() {
        let extrema = LineExtrema::new((0.0, 0.0, 0.0), (10.0, 0.0, 0.0));
        let t = extrema.extrema_parameter((5.0, 0.0, 0.0));
        assert_eq!(t, 0.5);
    }

    #[test]
    fn test_distance_to_point() {
        let extrema = LineExtrema::new((0.0, 0.0, 0.0), (10.0, 0.0, 0.0));
        let dist = extrema.distance_to_point((5.0, 3.0, 4.0));
        assert_eq!(dist, 5.0);
    }
}
