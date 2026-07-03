// FILE: gc_pnts_dist_function.rs
// occt: GCPnts_DistFunction

/// Distance function for curve point computation.
pub struct GcPntsDistFunction {
    current_dist: f64,
}

impl GcPntsDistFunction {
    pub fn new() -> Self {
        GcPntsDistFunction {
            current_dist: 0.0,
        }
    }

    pub fn compute_distance(&mut self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dz = z2 - z1;
        self.current_dist = (dx * dx + dy * dy + dz * dz).sqrt();
    }

    pub fn distance(&self) -> f64 {
        self.current_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let func = GcPntsDistFunction::new();
        assert_eq!(func.distance(), 0.0);
    }

    #[test]
    fn test_compute_distance() {
        let mut func = GcPntsDistFunction::new();
        func.compute_distance(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        assert!((func.distance() - 1.732).abs() < 0.01);
    }
}
