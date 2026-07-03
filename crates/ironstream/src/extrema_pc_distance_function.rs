// FILE: extrema_pc_distance_function.rs
// occt: ExtremaPC_DistanceFunction

/// Distance function for point-to-curve extrema computation.
pub struct ExtremaPcDistanceFunction {
    point: (f64, f64, f64),
    last_value: f64,
    nb_calls: i32,
}

impl ExtremaPcDistanceFunction {
    /// Creates a new distance function.
    pub fn new() -> Self {
        ExtremaPcDistanceFunction {
            point: (0.0, 0.0, 0.0),
            last_value: 0.0,
            nb_calls: 0,
        }
    }

    /// Sets the reference point.
    pub fn set_point(&mut self, x: f64, y: f64, z: f64) {
        self.point = (x, y, z);
    }

    /// Returns the reference point.
    pub fn point(&self) -> (f64, f64, f64) {
        self.point
    }

    /// Computes distance-squared at parameter u.
    pub fn compute(&mut self, u: f64, px: f64, py: f64, pz: f64) -> f64 {
        let dx = self.point.0 - px;
        let dy = self.point.1 - py;
        let dz = self.point.2 - pz;
        let dist_sq = dx * dx + dy * dy + dz * dz;
        self.last_value = dist_sq;
        self.nb_calls += 1;
        dist_sq
    }

    /// Returns last computed value.
    pub fn last_value(&self) -> f64 {
        self.last_value
    }

    /// Returns number of evaluations.
    pub fn nb_calls(&self) -> i32 {
        self.nb_calls
    }

    /// Resets call counter.
    pub fn reset(&mut self) {
        self.nb_calls = 0;
    }
}

impl Default for ExtremaPcDistanceFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = ExtremaPcDistanceFunction::new();
        assert_eq!(func.point(), (0.0, 0.0, 0.0));
        assert_eq!(func.nb_calls(), 0);
    }

    #[test]
    fn test_set_point() {
        let mut func = ExtremaPcDistanceFunction::new();
        func.set_point(1.0, 2.0, 3.0);
        assert_eq!(func.point(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_compute() {
        let mut func = ExtremaPcDistanceFunction::new();
        func.set_point(0.0, 0.0, 0.0);
        let dist = func.compute(0.5, 1.0, 1.0, 1.0);
        assert_eq!(dist, 3.0);
        assert_eq!(func.nb_calls(), 1);
    }

    #[test]
    fn test_distance_calculation() {
        let mut func = ExtremaPcDistanceFunction::new();
        func.set_point(3.0, 4.0, 0.0);
        let dist = func.compute(0.5, 0.0, 0.0, 0.0);
        assert_eq!(dist, 25.0);
    }

    #[test]
    fn test_reset() {
        let mut func = ExtremaPcDistanceFunction::new();
        func.set_point(1.0, 1.0, 1.0);
        func.compute(0.0, 0.0, 0.0, 0.0);
        func.compute(0.5, 1.0, 0.0, 0.0);
        assert_eq!(func.nb_calls(), 2);
        func.reset();
        assert_eq!(func.nb_calls(), 0);
    }
}
