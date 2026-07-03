// FILE: ais_bad_edge_filter.rs
// occt: AIS_BadEdgeFilter

#[derive(Clone, Debug, Default)]
pub struct BadEdgeFilter {
    tolerance: f64,
}

impl BadEdgeFilter {
    pub fn new() -> Self { Self { tolerance: 1e-7 } }
    pub fn tolerance(&self) -> f64 { self.tolerance }
    pub fn set_tolerance(&mut self, tol: f64) { self.tolerance = tol.max(0.0); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let f = BadEdgeFilter::new();
        assert!(f.tolerance() > 0.0);
    }
}
