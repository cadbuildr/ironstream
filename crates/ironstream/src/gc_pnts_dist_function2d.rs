// FILE: gc_pnts_dist_function2d.rs
// occt: GCPnts_DistFunction2d

/// Distance function for 2D curve point computation.
pub struct GcPntsDistFunction2d {
    current_dist: f64,
}

impl GcPntsDistFunction2d {
    pub fn new() -> Self {
        GcPntsDistFunction2d {
            current_dist: 0.0,
        }
    }

    pub fn compute_distance(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let dx = x2 - x1;
        let dy = y2 - y1;
        self.current_dist = (dx * dx + dy * dy).sqrt();
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
        let func = GcPntsDistFunction2d::new();
        assert_eq!(func.distance(), 0.0);
    }

    #[test]
    fn test_compute_distance() {
        let mut func = GcPntsDistFunction2d::new();
        func.compute_distance(0.0, 0.0, 3.0, 4.0);
        assert!((func.distance() - 5.0).abs() < 0.01);
    }
}
